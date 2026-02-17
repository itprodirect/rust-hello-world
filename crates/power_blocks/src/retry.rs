use std::thread;
use std::time::Duration;

/// Retry policy for fallible operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub fixed_backoff: Duration,
}

impl RetryPolicy {
    pub fn with_fixed_backoff(max_attempts: usize, fixed_backoff: Duration) -> Self {
        Self {
            max_attempts: max_attempts.max(1),
            fixed_backoff,
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            fixed_backoff: Duration::ZERO,
        }
    }
}

/// Retries an operation based on policy and a retry predicate.
///
/// The operation receives the 1-based attempt number.
pub fn retry<T, E, Operation, ShouldRetry>(
    policy: RetryPolicy,
    mut operation: Operation,
    should_retry: ShouldRetry,
) -> Result<T, E>
where
    Operation: FnMut(usize) -> Result<T, E>,
    ShouldRetry: Fn(&E) -> bool,
{
    let max_attempts = policy.max_attempts.max(1);

    for attempt in 1..=max_attempts {
        match operation(attempt) {
            Ok(value) => return Ok(value),
            Err(error) => {
                if attempt == max_attempts || !should_retry(&error) {
                    return Err(error);
                }

                if !policy.fixed_backoff.is_zero() {
                    thread::sleep(policy.fixed_backoff);
                }
            }
        }
    }

    unreachable!("loop always returns")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum DemoError {
        Temporary,
        Permanent,
    }

    #[test]
    fn succeeds_after_transient_failures() {
        let mut attempts = 0usize;
        let result = retry(
            RetryPolicy::with_fixed_backoff(5, Duration::ZERO),
            |_| {
                attempts += 1;
                if attempts < 3 {
                    Err(DemoError::Temporary)
                } else {
                    Ok("ok")
                }
            },
            |err| matches!(err, DemoError::Temporary),
        );

        assert_eq!(result, Ok("ok"));
        assert_eq!(attempts, 3);
    }

    #[test]
    fn stops_immediately_for_non_retryable_errors() {
        let mut attempts = 0usize;
        let result: Result<(), DemoError> = retry(
            RetryPolicy::with_fixed_backoff(10, Duration::ZERO),
            |_| {
                attempts += 1;
                Err(DemoError::Permanent)
            },
            |err| matches!(err, DemoError::Temporary),
        );

        assert_eq!(result, Err(DemoError::Permanent));
        assert_eq!(attempts, 1);
    }

    #[test]
    fn stops_at_max_attempts() {
        let mut attempts = 0usize;
        let result: Result<(), DemoError> = retry(
            RetryPolicy::with_fixed_backoff(3, Duration::ZERO),
            |_| {
                attempts += 1;
                Err(DemoError::Temporary)
            },
            |err| matches!(err, DemoError::Temporary),
        );

        assert_eq!(result, Err(DemoError::Temporary));
        assert_eq!(attempts, 3);
    }

    #[test]
    fn zero_max_attempts_is_treated_as_one_attempt() {
        let mut attempts = 0usize;
        let result: Result<(), DemoError> = retry(
            RetryPolicy::with_fixed_backoff(0, Duration::ZERO),
            |_| {
                attempts += 1;
                Err(DemoError::Temporary)
            },
            |err| matches!(err, DemoError::Temporary),
        );

        assert_eq!(result, Err(DemoError::Temporary));
        assert_eq!(attempts, 1);
    }
}
