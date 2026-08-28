use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;
use sqlx::Row;
use web_service::store::Store;

#[derive(Debug, Deserialize)]
struct SeedQuestion {
    title: String,
    content: String,
    #[serde(default)]
    tags: Option<Vec<String>>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "questions.json".to_string());

    let store = Store::new(&database_url, 2).await;
    sqlx::migrate!()
        .run(&store.connection)
        .await
        .expect("migrations failed");

    let raw = std::fs::read_to_string(Path::new(&path))
        .unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
    let items: HashMap<String, SeedQuestion> =
        serde_json::from_str(&raw).unwrap_or_else(|e| panic!("failed to parse {path}: {e}"));

    let mut entries: Vec<_> = items.into_iter().collect();
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut inserted = 0usize;
    let mut skipped = 0usize;

    for (_, q) in entries {
        let title = q.title.trim();
        let content = q.content.trim();
        if title.is_empty() || content.is_empty() {
            eprintln!("skipping seed row with empty title or content");
            skipped += 1;
            continue;
        }

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM questions WHERE title = $1)")
                .bind(title)
                .fetch_one(&store.connection)
                .await
                .expect("exists check failed");

        if exists {
            println!("skip (exists): {title}");
            skipped += 1;
            continue;
        }

        let row = sqlx::query(
            "INSERT INTO questions (title, content, tags)
             VALUES ($1, $2, $3)
             RETURNING id",
        )
        .bind(title)
        .bind(content)
        .bind(&q.tags)
        .fetch_one(&store.connection)
        .await
        .expect("insert failed");

        let id: i32 = row.get("id");
        println!("inserted #{id}: {title}");
        inserted += 1;
    }

    println!("seed complete: {inserted} inserted, {skipped} skipped");
}
