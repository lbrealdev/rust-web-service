use warp::{
    filters::{
        cors::CorsForbidden,
        body::BodyDeserializeError,
    },
    reject::Reject,
    Rejection,
    Reply,
    http::StatusCode
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Error::ParseError(ref err) => {
            write!(f, "Cannot parse parameter: {}", err)
        },
        Error::MissingParameters => write!(f, "Missing parameter"),
        Error::QuestionNofFound => write!(f, "Question not found"),
    }
}

pub async fn return_error(r: Rejection)
    -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

impl Reject for Error {}
