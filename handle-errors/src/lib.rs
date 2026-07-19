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

fn status_for_error(error: &Error) -> StatusCode {
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
