use std::collections::HashMap;

use handle_errors::Error;
use tracing::{event, instrument, Level};
use warp::{
    http::StatusCode,
    reject::{custom, Rejection},
    reply, Reply,
};

use crate::store::Store;
use crate::types::pagination::{extraction_pagination, Pagination};
use crate::types::question::{NewQuestion, UpdateQuestion};

fn validate_question_fields(title: &str, content: &str) -> Result<(), Error> {
    if title.trim().is_empty() {
        return Err(Error::ValidationError(
            "title must not be empty".to_string(),
        ));
    }
    if content.trim().is_empty() {
        return Err(Error::ValidationError(
            "content must not be empty".to_string(),
        ));
    }
    Ok(())
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "web-service", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extraction_pagination(params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn get_question(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.get_question(id).await {
        Ok(res) => Ok(reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn get_answers(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.get_answers(id).await {
        Ok(res) => Ok(reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl Reply, Rejection> {
    validate_question_fields(&new_question.title, &new_question.content).map_err(custom)?;

    let new_question = NewQuestion {
        title: new_question.title.trim().to_string(),
        content: new_question.content.trim().to_string(),
        tags: new_question.tags,
    };

    match store.add_question(new_question).await {
        Ok(question) => Ok(reply::with_status(
            reply::json(&question),
            StatusCode::CREATED,
        )),
        Err(e) => Err(custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: UpdateQuestion,
) -> Result<impl Reply, Rejection> {
    validate_question_fields(&question.title, &question.content).map_err(custom)?;

    let question = UpdateQuestion {
        title: question.title.trim().to_string(),
        content: question.content.trim().to_string(),
        tags: question.tags,
    };

    match store.update_question(question, id).await {
        Ok(res) => Ok(reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.delete_question(id).await {
        Ok(true) => Ok(reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Ok(false) => Err(custom(Error::NotFound)),
        Err(e) => Err(custom(e)),
    }
}
