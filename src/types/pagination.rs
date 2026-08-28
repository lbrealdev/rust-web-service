use handle_errors::Error;
use std::collections::HashMap;

/// Default page size when `limit` is omitted.
pub const DEFAULT_LIMIT: i32 = 100;

/// Upper bound so `LIMIT NULL` cannot return the full table.
pub const MAX_LIMIT: i32 = 100;

/// Pagination struct extracted from query params.
#[derive(Debug, PartialEq, Eq)]
pub struct Pagination {
    /// Max number of items to return.
    pub limit: i32,
    /// Number of items to skip.
    pub offset: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: DEFAULT_LIMIT,
            offset: 0,
        }
    }
}

/// Extract pagination from `/questions` query params.
///
/// Each parameter is optional and independent, e.g. `/questions?limit=10`,
/// `/questions?offset=20`, or `/questions?limit=10&offset=0`.
///
/// Omitted `limit` defaults to [`DEFAULT_LIMIT`]. Omitted `offset` defaults to `0`.
/// `limit` is capped at [`MAX_LIMIT`].
pub fn extraction_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    let mut pagination = Pagination::default();

    if let Some(raw) = params.get("limit") {
        let parsed = raw.parse::<i32>().map_err(Error::ParseError)?;
        if parsed < 0 {
            return Err(Error::ValidationError(
                "limit must be greater than or equal to 0".to_string(),
            ));
        }
        pagination.limit = parsed.min(MAX_LIMIT);
    }

    if let Some(raw) = params.get("offset") {
        let parsed = raw.parse::<i32>().map_err(Error::ParseError)?;
        if parsed < 0 {
            return Err(Error::ValidationError(
                "offset must be greater than or equal to 0".to_string(),
            ));
        }
        pagination.offset = parsed;
    }

    Ok(pagination)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn extracts_limit_and_offset() {
        let p = extraction_pagination(params(&[("limit", "10"), ("offset", "5")])).unwrap();
        assert_eq!(
            p,
            Pagination {
                limit: 10,
                offset: 5,
            }
        );
    }

    #[test]
    fn missing_both_uses_defaults() {
        let p = extraction_pagination(HashMap::new()).unwrap();
        assert_eq!(
            p,
            Pagination {
                limit: DEFAULT_LIMIT,
                offset: 0,
            }
        );
    }

    #[test]
    fn limit_only_defaults_offset() {
        let p = extraction_pagination(params(&[("limit", "10")])).unwrap();
        assert_eq!(
            p,
            Pagination {
                limit: 10,
                offset: 0,
            }
        );
    }

    #[test]
    fn offset_only_defaults_limit() {
        let p = extraction_pagination(params(&[("offset", "20")])).unwrap();
        assert_eq!(
            p,
            Pagination {
                limit: DEFAULT_LIMIT,
                offset: 20,
            }
        );
    }

    #[test]
    fn limit_is_capped_at_max() {
        let p = extraction_pagination(params(&[("limit", "1000")])).unwrap();
        assert_eq!(p.limit, MAX_LIMIT);
    }

    #[test]
    fn negative_limit_is_validation_error() {
        let err = extraction_pagination(params(&[("limit", "-1")])).unwrap_err();
        assert!(matches!(err, Error::ValidationError(_)));
    }

    #[test]
    fn negative_offset_is_validation_error() {
        let err = extraction_pagination(params(&[("offset", "-5")])).unwrap_err();
        assert!(matches!(err, Error::ValidationError(_)));
    }

    #[test]
    fn non_numeric_limit_returns_parse_error() {
        let err = extraction_pagination(params(&[("limit", "x")])).unwrap_err();
        assert!(matches!(err, Error::ParseError(_)));
    }

    #[test]
    fn non_numeric_offset_returns_parse_error() {
        let err = extraction_pagination(params(&[("offset", "x")])).unwrap_err();
        assert!(matches!(err, Error::ParseError(_)));
    }
}
