use core::fmt;

/// Borrowed metric row parsed from CSV-like text:
/// `service,status,latency_ms`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricRow<'a> {
    pub service: &'a str,
    pub status: &'a str,
    pub latency_ms: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricParseError {
    EmptyLine,
    MissingField { field: &'static str },
    ExtraFields,
    InvalidLatency { value: String },
}

impl fmt::Display for MetricParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLine => write!(f, "line is empty"),
            Self::MissingField { field } => write!(f, "missing field: {field}"),
            Self::ExtraFields => write!(f, "expected exactly three fields"),
            Self::InvalidLatency { value } => write!(f, "invalid latency value: {value}"),
        }
    }
}

impl std::error::Error for MetricParseError {}

/// Parse one line without allocating for string fields.
pub fn parse_metric_row(line: &str) -> Result<MetricRow<'_>, MetricParseError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(MetricParseError::EmptyLine);
    }

    let mut parts = trimmed.split(',');

    let service = parts
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or(MetricParseError::MissingField { field: "service" })?;

    let status = parts
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or(MetricParseError::MissingField { field: "status" })?;

    let latency_raw = parts
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or(MetricParseError::MissingField {
            field: "latency_ms",
        })?;

    if parts.next().is_some() {
        return Err(MetricParseError::ExtraFields);
    }

    let latency_ms = latency_raw
        .parse::<u32>()
        .map_err(|_| MetricParseError::InvalidLatency {
            value: latency_raw.to_string(),
        })?;

    Ok(MetricRow {
        service,
        status,
        latency_ms,
    })
}

/// Parse all non-empty rows from input, returning borrowed slices.
pub fn parse_metric_rows(
    input: &str,
) -> impl Iterator<Item = Result<MetricRow<'_>, MetricParseError>> + '_ {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_metric_row)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slice_is_within_parent(slice: &str, parent: &str) -> bool {
        let parent_start = parent.as_ptr() as usize;
        let parent_end = parent_start + parent.len();
        let slice_start = slice.as_ptr() as usize;
        let slice_end = slice_start + slice.len();
        slice_start >= parent_start && slice_end <= parent_end
    }

    #[test]
    fn parse_single_row() {
        let row = parse_metric_row("api,200,37").unwrap();
        assert_eq!(
            row,
            MetricRow {
                service: "api",
                status: "200",
                latency_ms: 37
            }
        );
    }

    #[test]
    fn parse_rows_skips_blank_lines() {
        let input = "api,200,10\n\nweb,500,900\n";
        let rows: Vec<_> = parse_metric_rows(input).collect();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|item| item.is_ok()));
    }

    #[test]
    fn parse_row_rejects_invalid_latency() {
        let err = parse_metric_row("api,200,slow").unwrap_err();
        assert_eq!(
            err,
            MetricParseError::InvalidLatency {
                value: "slow".into()
            }
        );
    }

    #[test]
    fn borrowed_fields_point_into_original_input() {
        let line = "gateway,200,123";
        let row = parse_metric_row(line).unwrap();
        assert!(slice_is_within_parent(row.service, line));
        assert!(slice_is_within_parent(row.status, line));
    }
}
