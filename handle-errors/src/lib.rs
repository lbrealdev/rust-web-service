use std::num::ParseIntError;

use thiserror::Error;
use tracing::{event, instrument, Level};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot parse parameter: {0}")]
    ParseError(#[from] ParseIntError),
    #[error("Missing parameter")]
    MissingParameters,
    #[error("{0}")]
    ValidationError(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Cannot update, invalid data.")]
    DatabaseQueryError,
    #[error("Internal server error")]
    InternalServerError,
}

impl Reject for Error {}

pub(crate) fn status_for_error(error: &Error) -> StatusCode {
    match error {
        Error::ParseError(_) | Error::MissingParameters | Error::ValidationError(_) => {
            StatusCode::BAD_REQUEST
        }
        Error::Unauthorized => StatusCode::UNAUTHORIZED,
        Error::NotFound => StatusCode::NOT_FOUND,
        Error::DatabaseQueryError => StatusCode::UNPROCESSABLE_ENTITY,
        Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        let status = status_for_error(error);
        match error {
            Error::DatabaseQueryError | Error::InternalServerError => {
                event!(Level::ERROR, "{}", error);
            }
            Error::NotFound => {
                event!(Level::WARN, "{}", error);
            }
            _ => {
                event!(Level::ERROR, "{}", error);
            }
        }
        Ok(warp::reply::with_status(error.to_string(), status))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS forbidden error: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "Cannot deserizalize request body: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[cfg(test)]
mod handle_errors_tests {
    use super::*;
    use warp::reply::Reply;

    #[test]
    fn status_for_error_maps_variants() {
        let parse_err = "x".parse::<i32>().unwrap_err();
        assert_eq!(
            status_for_error(&Error::ParseError(parse_err)),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            status_for_error(&Error::MissingParameters),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            status_for_error(&Error::ValidationError("bad".into())),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            status_for_error(&Error::Unauthorized),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(status_for_error(&Error::NotFound), StatusCode::NOT_FOUND);
        assert_eq!(
            status_for_error(&Error::DatabaseQueryError),
            StatusCode::UNPROCESSABLE_ENTITY
        );
        assert_eq!(
            status_for_error(&Error::InternalServerError),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn return_error_uses_mapped_status_and_body() {
        use http_body_util::BodyExt;

        let rejection = warp::reject::custom(Error::NotFound);
        let response = return_error(rejection).await.unwrap().into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Not found");
    }

    #[tokio::test]
    async fn return_error_unknown_rejection_is_not_found() {
        let rejection = warp::reject::reject();
        let response = return_error(rejection).await.unwrap().into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
