use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct extracted from query params.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Pagination {
    /// Max number of items to return (when provided with `offset`).
    pub limit: Option<i32>,
    /// Number of items to skip.
    pub offset: i32,
}

/// Extract pagination from `/questions` query params.
///
/// Both `limit` and `offset` must be present together, e.g. `/questions?limit=10&offset=0`.
pub fn extraction_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
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
                limit: Some(10),
                offset: 5,
            }
        );
    }

    #[test]
    fn missing_both_returns_missing_parameters() {
        let err = extraction_pagination(HashMap::new()).unwrap_err();
        assert!(matches!(err, Error::MissingParameters));
    }

    #[test]
    fn missing_offset_returns_missing_parameters() {
        let err = extraction_pagination(params(&[("limit", "10")])).unwrap_err();
        assert!(matches!(err, Error::MissingParameters));
    }

    #[test]
    fn missing_limit_returns_missing_parameters() {
        let err = extraction_pagination(params(&[("offset", "0")])).unwrap_err();
        assert!(matches!(err, Error::MissingParameters));
    }

    #[test]
    fn non_numeric_limit_returns_parse_error() {
        let err = extraction_pagination(params(&[("limit", "x"), ("offset", "0")])).unwrap_err();
        assert!(matches!(err, Error::ParseError(_)));
    }

    #[test]
    fn non_numeric_offset_returns_parse_error() {
        let err = extraction_pagination(params(&[("limit", "1"), ("offset", "x")])).unwrap_err();
        assert!(matches!(err, Error::ParseError(_)));
    }
}
