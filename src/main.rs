#![warn(clippy::all)]

use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use web_service::{routes, store};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "handle_errors=warn,web_service=info,warp=error".to_owned());

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_connections = std::env::var("DB_POOL_MAX")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(5);

    let bootstrap_username = std::env::var("BOOTSTRAP_ADMIN_USERNAME")
        .or_else(|_| std::env::var("ADMIN_USERNAME"))
        .unwrap_or_else(|_| "admin".into());
    let bootstrap_password = std::env::var("BOOTSTRAP_ADMIN_PASSWORD")
        .or_else(|_| std::env::var("ADMIN_PASSWORD"))
        .expect("BOOTSTRAP_ADMIN_PASSWORD or ADMIN_PASSWORD must be set");

    let store = store::Store::new(&database_url, max_connections).await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    store
        .ensure_bootstrap_admin(&bootstrap_username, &bootstrap_password)
        .await
        .expect("Cannot bootstrap admin user");

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let index = warp::path::end().and(warp::fs::file("static/index.html"));
    let static_files = warp::fs::dir("static");

    let routes = index
        .or(static_files)
        .or(routes::api(store))
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
