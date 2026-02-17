use std::fmt;
use std::io::BufRead;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A single parsed log entry.
///
/// Format: `TIMESTAMP LEVEL MESSAGE`
/// Example: `2024-01-15T10:30:00Z INFO Server started on port 8080`
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

/// Error returned when a line cannot be parsed.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub line: String,
    pub reason: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error on {:?}: {}", self.line, self.reason)
    }
}

impl std::error::Error for ParseError {}

// ---------------------------------------------------------------------------
// Line parser
// ---------------------------------------------------------------------------

/// Parse a single log line into a [`LogEntry`].
///
/// Expected format: `TIMESTAMP LEVEL MESSAGE` where the first two tokens are
/// whitespace-delimited and everything after the second token is the message.
///
/// # Examples
///
/// ```
/// let entry = log_parser::parse_line("2024-01-15T10:30:00Z INFO hello").unwrap();
/// assert_eq!(entry.level, "INFO");
/// ```
pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ParseError {
            line: line.to_string(),
            reason: "empty line".into(),
        });
    }

    // Parse timestamp token.
    let first_ws = trimmed
        .char_indices()
        .find(|(_, c)| c.is_whitespace())
        .map(|(i, _)| i)
        .ok_or_else(|| ParseError {
            line: line.to_string(),
            reason: "missing level and message".into(),
        })?;
    let timestamp = &trimmed[..first_ws];

    // Skip inter-token whitespace, then parse level token.
    let level_start = trimmed[first_ws..]
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| first_ws + i)
        .ok_or_else(|| ParseError {
            line: line.to_string(),
            reason: "missing level and message".into(),
        })?;
    let level_end = trimmed[level_start..]
        .char_indices()
        .find(|(_, c)| c.is_whitespace())
        .map(|(i, _)| level_start + i)
        .ok_or_else(|| ParseError {
            line: line.to_string(),
            reason: "missing message".into(),
        })?;
    let level = &trimmed[level_start..level_end];

    // Skip trailing whitespace after level; the rest is message.
    let message_start = trimmed[level_end..]
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| level_end + i)
        .ok_or_else(|| ParseError {
            line: line.to_string(),
            reason: "missing message".into(),
        })?;
    let message = &trimmed[message_start..];

    Ok(LogEntry {
        timestamp: timestamp.to_string(),
        level: level.to_string(),
        message: message.to_string(),
    })
}

// ---------------------------------------------------------------------------
// Allocating parser
// ---------------------------------------------------------------------------

/// Parse all lines in `input`, collecting results into a `Vec`.
///
/// Every line produces a `String` allocation for each field. Simple and
/// correct, but holds the entire result set in memory.
pub fn parse_log(input: &str) -> Vec<LogEntry> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| parse_line(l).ok())
        .collect()
}

// ---------------------------------------------------------------------------
// Streaming parser
// ---------------------------------------------------------------------------

/// Parse log entries one at a time from any buffered reader.
///
/// Returns an iterator that yields one `Result<LogEntry, ParseError>` per
/// non-empty line. The caller decides what to do with each entry — filter,
/// count, aggregate — without holding the entire log in memory.
///
/// Uses a single reusable line buffer internally.
pub fn parse_log_streaming<R: BufRead>(
    reader: R,
) -> impl Iterator<Item = Result<LogEntry, ParseError>> {
    StreamingParser {
        reader,
        buf: String::new(),
    }
}

struct StreamingParser<R> {
    reader: R,
    buf: String,
}

impl<R: BufRead> Iterator for StreamingParser<R> {
    type Item = Result<LogEntry, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buf.clear();
            match self.reader.read_line(&mut self.buf) {
                Ok(0) => return None, // EOF
                Ok(_) => {
                    if self.buf.trim().is_empty() {
                        continue; // skip blank lines
                    }
                    return Some(parse_line(&self.buf));
                }
                Err(e) => {
                    return Some(Err(ParseError {
                        line: String::new(),
                        reason: format!("I/O error: {e}"),
                    }));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Generate `n` synthetic log lines for testing and benchmarking.
pub fn generate_log(n: usize) -> String {
    let levels = ["DEBUG", "INFO", "WARN", "ERROR"];
    let mut out = String::with_capacity(n * 60);
    for i in 0..n {
        let level = levels[i % levels.len()];
        out.push_str(&format!(
            "2024-01-15T10:{:02}:{:02}Z {level} Request {i} processed successfully\n",
            (i / 60) % 60,
            i % 60,
        ));
    }
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_line_valid() {
        let entry = parse_line("2024-01-15T10:30:00Z INFO Server started").unwrap();
        assert_eq!(entry.timestamp, "2024-01-15T10:30:00Z");
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "Server started");
    }

    #[test]
    fn parse_line_message_with_spaces() {
        let entry = parse_line("2024-01-15T10:30:00Z ERROR disk full on /dev/sda1").unwrap();
        assert_eq!(entry.message, "disk full on /dev/sda1");
    }

    #[test]
    fn parse_line_handles_multiple_spaces_between_fields() {
        let entry = parse_line("2024-01-15T10:30:00Z   INFO   server started").unwrap();
        assert_eq!(entry.timestamp, "2024-01-15T10:30:00Z");
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "server started");
    }

    #[test]
    fn parse_line_handles_tabs_between_fields() {
        let entry = parse_line("2024-01-15T10:30:00Z\tWARN\tspike detected").unwrap();
        assert_eq!(entry.level, "WARN");
        assert_eq!(entry.message, "spike detected");
    }

    #[test]
    fn parse_line_empty_is_error() {
        assert!(parse_line("").is_err());
        assert!(parse_line("   ").is_err());
    }

    #[test]
    fn parse_line_missing_message_is_error() {
        assert!(parse_line("2024-01-15T10:30:00Z INFO").is_err());
    }

    #[test]
    fn parse_line_single_token_is_error() {
        assert!(parse_line("justonetoken").is_err());
    }

    #[test]
    fn parse_line_missing_level_and_message_is_error() {
        assert!(parse_line("2024-01-15T10:30:00Z   ").is_err());
    }

    #[test]
    fn parse_log_collects_all_entries() {
        let input = generate_log(100);
        let entries = parse_log(&input);
        assert_eq!(entries.len(), 100);
    }

    #[test]
    fn parse_log_skips_blank_lines() {
        let input = "2024-01-15T10:00:00Z INFO a\n\n2024-01-15T10:00:01Z WARN b\n";
        let entries = parse_log(input);
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn parse_log_skips_invalid_lines() {
        let input =
            "2024-01-15T10:00:00Z INFO ok\nthis_is_invalid\n2024-01-15T10:00:01Z WARN still_ok\n";
        let entries = parse_log(input);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].message, "ok");
        assert_eq!(entries[1].message, "still_ok");
    }

    #[test]
    fn streaming_matches_allocating() {
        let input = generate_log(500);
        let allocating = parse_log(&input);
        let streaming: Vec<LogEntry> = parse_log_streaming(Cursor::new(&input))
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(allocating, streaming);
    }

    #[test]
    fn streaming_skips_blank_lines() {
        let input = "2024-01-15T10:00:00Z INFO a\n\n\n2024-01-15T10:00:01Z WARN b\n";
        let entries: Vec<_> = parse_log_streaming(Cursor::new(input))
            .filter_map(|r| r.ok())
            .collect();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn streaming_emits_error_for_invalid_line_and_continues() {
        let input = "2024-01-15T10:00:00Z INFO ok\ninvalid\n2024-01-15T10:00:01Z WARN still_ok\n";
        let results: Vec<_> = parse_log_streaming(Cursor::new(input)).collect();
        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
        assert!(results[2].is_ok());
    }

    #[test]
    fn generate_log_produces_correct_count() {
        let log = generate_log(1000);
        let count = log.lines().count();
        assert_eq!(count, 1000);
    }
}
