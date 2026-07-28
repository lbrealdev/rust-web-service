use handle_errors::return_error;
use serde_json::Value;
use warp::http::StatusCode;
use warp::Filter;
use web_service::routes::api;
use web_service::store::Store;
use web_service::types::question::{NewQuestion, Question};
use web_service::types::user::UserRole;

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
        .ensure_bootstrap_admin("testadmin", "testadminpass")
        .await
        .expect("bootstrap admin");
    store
}

async fn cleanup_question(store: &Store, id: i32) {
    let _ = store.delete_question(id).await;
}

async fn guest_session(store: &Store) -> (String, String) {
    let (user, sign_in) = store.create_token_user().await.expect("token user");
    let token = store.create_session(user.id).await.expect("session");
    (format!("Bearer {token}"), sign_in)
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn store_question_crud() {
    let store = test_store().await;
    let (user, _) = store.create_token_user().await.expect("token user");

    let created = store
        .add_question(
            NewQuestion {
                title: "integration store title".into(),
                content: "integration store content".into(),
                tags: Some(vec!["test".into()]),
            },
            user.id,
        )
        .await
        .expect("add_question");

    let id = created.id.0;
    let fetched = store.get_question(id).await.expect("get_question");
    assert_eq!(fetched.title, "integration store title");
    assert_eq!(fetched.author_id, Some(user.id));

    let deleted = store.delete_question(id).await.expect("delete_question");
    assert!(deleted);

    let missing = store.get_question(id).await;
    assert!(matches!(missing, Err(handle_errors::Error::NotFound)));
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn api_requires_auth_for_create() {
    let store = test_store().await;
    let routes = api(store).recover(return_error);

    let create = warp::test::request()
        .method("POST")
        .path("/questions")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "title": "no auth",
            "content": "should fail",
            "tags": null
        }))
        .reply(&routes)
        .await;

    assert_eq!(create.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn api_guest_token_create_get_and_404() {
    let store = test_store().await;
    let routes = api(store.clone()).recover(return_error);
    let (auth, _) = guest_session(&store).await;

    let create = warp::test::request()
        .method("POST")
        .path("/questions")
        .header("content-type", "application/json")
        .header("authorization", &auth)
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
async fn api_rejects_empty_question_fields_when_authed() {
    let store = test_store().await;
    let (auth, _) = guest_session(&store).await;
    let routes = api(store).recover(return_error);

    let res = warp::test::request()
        .method("POST")
        .path("/questions")
        .header("content-type", "application/json")
        .header("authorization", &auth)
        .json(&serde_json::json!({
            "title": "   ",
            "content": "ok",
            "tags": null
        }))
        .reply(&routes)
        .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn api_register_login_and_ownership_forbidden() {
    let store = test_store().await;
    let routes = api(store.clone()).recover(return_error);

    let suffix = uuid::Uuid::new_v4().simple().to_string();
    let username = format!("user_{suffix}");

    let reg = warp::test::request()
        .method("POST")
        .path("/register")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "username": username,
            "password": "password123"
        }))
        .reply(&routes)
        .await;
    assert_eq!(reg.status(), StatusCode::CREATED);
    let reg_body: Value = serde_json::from_slice(reg.body()).unwrap();
    let auth1 = format!("Bearer {}", reg_body["token"].as_str().unwrap());

    let create = warp::test::request()
        .method("POST")
        .path("/questions")
        .header("content-type", "application/json")
        .header("authorization", &auth1)
        .json(&serde_json::json!({
            "title": "owned question",
            "content": "mine",
            "tags": null
        }))
        .reply(&routes)
        .await;
    assert_eq!(create.status(), StatusCode::CREATED);
    let q: Question = serde_json::from_slice(create.body()).unwrap();

    let (auth2, _) = guest_session(&store).await;
    let del = warp::test::request()
        .method("DELETE")
        .path(&format!("/questions/{}", q.id.0))
        .header("authorization", &auth2)
        .reply(&routes)
        .await;
    assert_eq!(del.status(), StatusCode::FORBIDDEN);

    cleanup_question(&store, q.id.0).await;
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn api_login_with_sign_in_token() {
    let store = test_store().await;
    let (_, sign_in) = guest_session(&store).await;
    let routes = api(store).recover(return_error);

    let login = warp::test::request()
        .method("POST")
        .path("/login")
        .header("content-type", "application/json")
        .json(&serde_json::json!({ "sign_in_token": sign_in }))
        .reply(&routes)
        .await;

    assert_eq!(login.status(), StatusCode::OK);
    let body: Value = serde_json::from_slice(login.body()).unwrap();
    assert!(body["token"].as_str().is_some());
    assert_eq!(body["user"]["role"], "token");
}

#[tokio::test]
#[ignore = "requires Postgres; run with just test-db"]
async fn bootstrap_admin_role() {
    let store = test_store().await;
    let user = store
        .authenticate_password("testadmin", "testadminpass")
        .await
        .expect("admin login");
    assert_eq!(user.role, UserRole::Admin);
}
