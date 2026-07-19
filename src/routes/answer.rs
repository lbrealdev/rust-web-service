use warp::http::StatusCode;

use crate::store::Store;
use crate::types::answer::NewAnswer;

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_answer(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_answer(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Answer {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
