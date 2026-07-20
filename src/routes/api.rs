use warp::{Filter, Rejection, Reply};

use crate::store::Store;

/// API routes (no static files, CORS, or error recovery).
///
/// Compose with `.recover(handle_errors::return_error)` in `main` and tests.
pub fn api(store: Store) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let auth = super::auth::require_auth(store.clone());
    let store_filter = warp::any().map(move || store.clone());

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(super::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_questions request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let get_question_answers = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(super::question::get_answers)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_answers request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(super::question::get_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_question request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(auth.clone())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(super::question::add_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "add_question request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth.clone())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(super::question::update_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "update_question request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth.clone())
        .and(store_filter.clone())
        .and_then(super::question::delete_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "delete_question request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(auth.clone())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(super::answer::add_answer)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "add_answer request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let delete_answer = warp::delete()
        .and(warp::path("answers"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth.clone())
        .and(store_filter.clone())
        .and_then(super::answer::delete_answer)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "delete_answer request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and(warp::addr::remote())
        .and_then(super::auth::register)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "register request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and(warp::addr::remote())
        .and_then(super::auth::login)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "login request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let guest_token = warp::post()
        .and(warp::path("auth"))
        .and(warp::path("guest-token"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::addr::remote())
        .and_then(super::auth::guest_token)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "guest_token request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let logout = warp::post()
        .and(warp::path("logout"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::header::optional::<String>("authorization"))
        .and_then(super::auth::logout)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "logout request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let me = warp::get()
        .and(warp::path("me"))
        .and(warp::path::end())
        .and(auth)
        .and_then(super::auth::me)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "me request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    get_questions
        .or(get_question_answers)
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .or(add_answer)
        .or(delete_answer)
        .or(register)
        .or(login)
        .or(guest_token)
        .or(logout)
        .or(me)
        .or(delete_question)
}
