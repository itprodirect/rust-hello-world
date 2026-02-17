use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpClientConfig {
    pub base_url: String,
    pub timeout_ms: u64,
    pub retries: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    EmptyBaseUrl,
    TimeoutMustBePositive,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBaseUrl => write!(f, "base_url must not be empty"),
            Self::TimeoutMustBePositive => write!(f, "timeout_ms must be greater than zero"),
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone, Copy, Default)]
pub struct Missing;

#[derive(Debug, Clone)]
pub struct Present<T>(T);

/// Builder with compile-time required fields (`base_url`, `timeout_ms`).
///
/// ```compile_fail
/// use power_blocks::typestate_builder::HttpClientConfigBuilder;
///
/// // `build()` is not available until required fields are set.
/// let _ = HttpClientConfigBuilder::new().build();
/// ```
#[derive(Debug, Clone)]
pub struct HttpClientConfigBuilder<BaseUrl = Missing, Timeout = Missing> {
    base_url: BaseUrl,
    timeout_ms: Timeout,
    retries: u8,
}

impl HttpClientConfigBuilder<Missing, Missing> {
    pub fn new() -> Self {
        Self {
            base_url: Missing,
            timeout_ms: Missing,
            retries: 2,
        }
    }
}

impl<Timeout> HttpClientConfigBuilder<Missing, Timeout> {
    pub fn base_url(
        self,
        base_url: impl Into<String>,
    ) -> HttpClientConfigBuilder<Present<String>, Timeout> {
        HttpClientConfigBuilder {
            base_url: Present(base_url.into()),
            timeout_ms: self.timeout_ms,
            retries: self.retries,
        }
    }
}

impl<BaseUrl> HttpClientConfigBuilder<BaseUrl, Missing> {
    pub fn timeout_ms(self, timeout_ms: u64) -> HttpClientConfigBuilder<BaseUrl, Present<u64>> {
        HttpClientConfigBuilder {
            base_url: self.base_url,
            timeout_ms: Present(timeout_ms),
            retries: self.retries,
        }
    }
}

impl<BaseUrl, Timeout> HttpClientConfigBuilder<BaseUrl, Timeout> {
    pub fn retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }
}

impl HttpClientConfigBuilder<Present<String>, Present<u64>> {
    pub fn build(self) -> Result<HttpClientConfig, ConfigError> {
        let base_url = self.base_url.0.trim().to_string();
        if base_url.is_empty() {
            return Err(ConfigError::EmptyBaseUrl);
        }

        let timeout_ms = self.timeout_ms.0;
        if timeout_ms == 0 {
            return Err(ConfigError::TimeoutMustBePositive);
        }

        Ok(HttpClientConfig {
            base_url,
            timeout_ms,
            retries: self.retries,
        })
    }
}

impl Default for HttpClientConfigBuilder<Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_happy_path() {
        let cfg = HttpClientConfigBuilder::new()
            .base_url("https://api.example.com")
            .timeout_ms(1_500)
            .retries(5)
            .build()
            .unwrap();

        assert_eq!(
            cfg,
            HttpClientConfig {
                base_url: "https://api.example.com".into(),
                timeout_ms: 1_500,
                retries: 5
            }
        );
    }

    #[test]
    fn default_retries_is_small_and_safe() {
        let cfg = HttpClientConfigBuilder::new()
            .base_url("https://api.example.com")
            .timeout_ms(1_000)
            .build()
            .unwrap();

        assert_eq!(cfg.retries, 2);
    }

    #[test]
    fn empty_base_url_is_rejected() {
        let result = HttpClientConfigBuilder::new()
            .base_url("   ")
            .timeout_ms(1_000)
            .build();

        assert_eq!(result, Err(ConfigError::EmptyBaseUrl));
    }

    #[test]
    fn zero_timeout_is_rejected() {
        let result = HttpClientConfigBuilder::new()
            .base_url("https://api.example.com")
            .timeout_ms(0)
            .build();

        assert_eq!(result, Err(ConfigError::TimeoutMustBePositive));
    }
}
