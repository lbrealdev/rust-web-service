use handle_errors::return_error;
use serde_json::Value;
use warp::http::StatusCode;
use warp::Filter;
use web_service::routes::api;
use web_service::store::Store;
use web_service::types::question::{NewQuestion, Question};

/// DB-backed tests — run with `just test-db` (ignored by default so `just test` stays offline).
fn database_url() -> String {
    dotenvy::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for DB integration tests")
}

async fn test_store() -> Store {
    let store = Store::new(&database_url(), 2).await;
    sqlx::migrate!()
        .run(&store.connection)
        .await
        .expect("migrations failed");
    store
}

async fn cleanup_question(store: &Store, id: i32) {
    let _ = store.delete_question(id).await;
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn store_question_crud() {
    let store = test_store().await;

    let created = store
        .add_question(NewQuestion {
            title: "integration store title".into(),
            content: "integration store content".into(),
            tags: Some(vec!["test".into()]),
        })
        .await
        .expect("add_question");

    let id = created.id.0;
    let fetched = store.get_question(id).await.expect("get_question");
    assert_eq!(fetched.title, "integration store title");
    assert_eq!(fetched.content, "integration store content");

    let deleted = store.delete_question(id).await.expect("delete_question");
    assert!(deleted);

    let missing = store.get_question(id).await;
    assert!(matches!(missing, Err(handle_errors::Error::NotFound)));
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn api_create_get_question_and_404() {
    let store = test_store().await;
    let routes = api(store.clone()).recover(return_error);

    let create = warp::test::request()
        .method("POST")
        .path("/questions")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "title": "integration api title",
            "content": "integration api content",
            "tags": ["api"]
        }))
        .reply(&routes)
        .await;

    assert_eq!(create.status(), StatusCode::CREATED);
    let created: Question = serde_json::from_slice(create.body()).expect("decode created question");
    let id = created.id.0;

    let get = warp::test::request()
        .method("GET")
        .path(&format!("/questions/{id}"))
        .reply(&routes)
        .await;

    assert_eq!(get.status(), StatusCode::OK);
    let body: Value = serde_json::from_slice(get.body()).expect("decode get body");
    assert_eq!(body["title"], "integration api title");

    let missing = warp::test::request()
        .method("GET")
        .path("/questions/2147483647")
        .reply(&routes)
        .await;

    assert_eq!(missing.status(), StatusCode::NOT_FOUND);

    cleanup_question(&store, id).await;
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn api_rejects_empty_question_fields() {
    let store = test_store().await;
    let routes = api(store).recover(return_error);

    let res = warp::test::request()
        .method("POST")
        .path("/questions")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "title": "   ",
            "content": "ok",
            "tags": null
        }))
        .reply(&routes)
        .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
