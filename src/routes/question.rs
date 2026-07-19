use std::collections::HashMap;

use tracing::{event, instrument, Level};
use warp::{
    http::StatusCode,
    reject::{custom, Rejection},
};

use crate::store::Store;
use crate::types::pagination::{extraction_pagination, Pagination};
use crate::types::question::{NewQuestion, Question};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, Rejection> {
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
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn get_question(id: i32, store: Store) -> Result<impl warp::Reply, Rejection> {
    match store.get_question(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn get_answers(id: i32, store: Store) -> Result<impl warp::Reply, Rejection> {
    match store.get_answers(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, Rejection> {
    match store.add_question(new_question).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Question added".to_string(),
            StatusCode::OK,
        )),
        Err(e) => Err(custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(custom(e)),
    }
}
