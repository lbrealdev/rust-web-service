use handle_errors::Error;
use warp::{http::StatusCode, reject::custom, reply, Rejection, Reply};

use crate::store::Store;
use crate::types::answer::{NewAnswer, UpdateAnswer};
use crate::types::user::AuthUser;

pub async fn add_answer(
    user: AuthUser,
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl Reply, Rejection> {
    if new_answer.content.trim().is_empty() {
        return Err(custom(Error::ValidationError(
            "content must not be empty".to_string(),
        )));
    }

    let new_answer = NewAnswer {
        content: new_answer.content.trim().to_string(),
        question_id: new_answer.question_id,
    };

    match store.add_answer(new_answer, user.id).await {
        Ok(answer) => Ok(reply::with_status(
            reply::json(&answer),
            StatusCode::CREATED,
        )),
        Err(e) => Err(custom(e)),
    }
}

pub async fn update_answer(
    id: i32,
    user: AuthUser,
    store: Store,
    answer: UpdateAnswer,
) -> Result<impl Reply, Rejection> {
    if answer.content.trim().is_empty() {
        return Err(custom(Error::ValidationError(
            "content must not be empty".to_string(),
        )));
    }

    let existing = store.get_answer(id).await.map_err(custom)?;
    if !user.can_modify(existing.author_id) {
        return Err(custom(Error::Forbidden));
    }

    let answer = UpdateAnswer {
        content: answer.content.trim().to_string(),
    };

    match store.update_answer(answer, id).await {
        Ok(res) => Ok(reply::json(&res)),
        Err(e) => Err(custom(e)),
    }
}

pub async fn delete_answer(id: i32, user: AuthUser, store: Store) -> Result<impl Reply, Rejection> {
    let existing = store.get_answer(id).await.map_err(custom)?;
    if !user.can_modify(existing.author_id) {
        return Err(custom(Error::Forbidden));
    }

    match store.delete_answer(id).await {
        Ok(true) => Ok(reply::with_status(
            format!("Answer {} deleted", id),
            StatusCode::OK,
        )),
        Ok(false) => Err(custom(Error::NotFound)),
        Err(e) => Err(custom(e)),
    }
}
