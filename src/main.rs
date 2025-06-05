#![warn(clippy::all)]

use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "handle_errors=warn,web_service=info,warp=error".to_owned());

    let store = store::Store::new("postgres://admin:localpsql2025@localhost:5432/rustwebdev").await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_question request",
                method = %info.method(),
                path = %info.method(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "add_question request",
                method = %info.method(),
                path = %info.method(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "update_question request",
                method = %info.method(),
                path = %info.method(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "delete_question request",
                method = %info.method(),
                path = %info.method(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "add_answer request",
                method = %info.method(),
                path = %info.method(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let index = warp::path::end()
        .and(warp::fs::file("static/index.html"));

    let static_files = warp::fs::dir("static");

    let routes = index
        .or(static_files)
        .or(get_questions)
        .or(add_question)
        .or(update_question)
        .or(add_answer)
        .or(delete_question)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
