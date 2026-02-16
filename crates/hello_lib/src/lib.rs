/// Returns a greeting string for the given name.
///
/// # Examples
///
/// ```
/// assert_eq!(hello_lib::greet("world"), "Hello, world!");
/// assert_eq!(hello_lib::greet("Rust"), "Hello, Rust!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Trims whitespace from the input and validates that it is non-empty.
///
/// Returns the trimmed name on success, or an error message if the
/// input is empty or contains only whitespace.
///
/// # Examples
///
/// ```
/// assert_eq!(hello_lib::parse_name("  Alice  "), Ok("Alice"));
/// assert!(hello_lib::parse_name("").is_err());
/// ```
pub fn parse_name(input: &str) -> Result<&str, &'static str> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Err("name must not be empty")
    } else {
        Ok(trimmed)
    }
}

/// Returns the crate version from Cargo metadata.
///
/// # Examples
///
/// ```
/// let v = hello_lib::version();
/// assert!(!v.is_empty());
/// ```
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_simple_name() {
        assert_eq!(greet("world"), "Hello, world!");
    }

    #[test]
    fn greet_unicode_name() {
        assert_eq!(greet("世界"), "Hello, 世界!");
    }

    #[test]
    fn greet_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn parse_name_valid() {
        assert_eq!(parse_name("Alice"), Ok("Alice"));
    }

    #[test]
    fn parse_name_trims_whitespace() {
        assert_eq!(parse_name("  Bob  "), Ok("Bob"));
    }

    #[test]
    fn parse_name_rejects_empty() {
        assert_eq!(parse_name(""), Err("name must not be empty"));
    }

    #[test]
    fn parse_name_rejects_whitespace_only() {
        assert_eq!(parse_name("   "), Err("name must not be empty"));
    }

    #[test]
    fn parse_name_unicode() {
        assert_eq!(parse_name(" Ñoño "), Ok("Ñoño"));
    }

    #[test]
    fn version_is_not_empty() {
        assert!(!version().is_empty());
    }

    #[test]
    fn version_is_semver() {
        let v = version();
        assert!(v.contains('.'), "version should contain a dot: {v}");
    }
}
