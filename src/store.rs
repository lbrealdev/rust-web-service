use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::auth_crypto::{hash_password, hash_token, verify_password};
use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{NewQuestion, Question, QuestionId, UpdateQuestion},
    user::{AuthUser, UserRole},
};
use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

fn map_db_error(e: sqlx::Error) -> Error {
    match e {
        sqlx::Error::RowNotFound => Error::NotFound,
        sqlx::Error::Database(db) if db.constraint() == Some("users_username_key") => {
            Error::Conflict("username already taken".into())
        }
        other => {
            tracing::event!(tracing::Level::ERROR, "{:?}", other);
            Error::DatabaseQueryError
        }
    }
}

fn map_user_row(row: PgRow) -> Result<AuthUser, Error> {
    let role_str: String = row.get("role");
    let role = UserRole::parse(&role_str).ok_or(Error::InternalServerError)?;
    Ok(AuthUser {
        id: row.get("id"),
        role,
        username: row.get("username"),
        display_name: row.get("display_name"),
    })
}

fn map_question_row(row: PgRow) -> Question {
    Question {
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
        author_id: row.get("author_id"),
    }
}

impl Store {
    pub async fn new(db_url: &str, max_connections: u32) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn ensure_bootstrap_admin(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), Error> {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin')")
                .fetch_one(&self.connection)
                .await
                .map_err(map_db_error)?;

        if exists {
            return Ok(());
        }

        let password_hash = hash_password(password)?;
        sqlx::query(
            "INSERT INTO users (role, username, password_hash, display_name)
             VALUES ('admin', $1, $2, $1)",
        )
        .bind(username)
        .bind(password_hash)
        .execute(&self.connection)
        .await
        .map_err(map_db_error)?;

        tracing::info!("bootstrap admin created: {}", username);
        Ok(())
    }

    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        role: UserRole,
    ) -> Result<AuthUser, Error> {
        let password_hash = hash_password(password)?;
        let row = sqlx::query(
            "INSERT INTO users (role, username, password_hash, display_name)
             VALUES ($1, $2, $3, $2)
             RETURNING id, role, username, display_name",
        )
        .bind(role.as_str())
        .bind(username)
        .bind(password_hash)
        .fetch_one(&self.connection)
        .await
        .map_err(map_db_error)?;

        map_user_row(row)
    }

    pub async fn create_token_user(&self) -> Result<(AuthUser, String), Error> {
        let sign_in_token = crate::auth_crypto::random_token();
        let token_hash = hash_token(&sign_in_token);

        let mut tx = self.connection.begin().await.map_err(map_db_error)?;

        let row = sqlx::query(
            "INSERT INTO users (role, username, password_hash, display_name)
             VALUES ('token', NULL, NULL, 'guest-pending')
             RETURNING id, role, username, display_name",
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(map_db_error)?;

        let mut user = map_user_row(row)?;
        let display_name = format!("guest-{}", user.id);
        sqlx::query("UPDATE users SET display_name = $1 WHERE id = $2")
            .bind(&display_name)
            .bind(user.id)
            .execute(&mut *tx)
            .await
            .map_err(map_db_error)?;
        user.display_name = Some(display_name);

        sqlx::query("INSERT INTO sign_in_tokens (user_id, token_hash) VALUES ($1, $2)")
            .bind(user.id)
            .bind(token_hash)
            .execute(&mut *tx)
            .await
            .map_err(map_db_error)?;

        tx.commit().await.map_err(map_db_error)?;
        Ok((user, sign_in_token))
    }

    pub async fn authenticate_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthUser, Error> {
        let row = sqlx::query(
            "SELECT id, role, username, display_name, password_hash
             FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.connection)
        .await
        .map_err(map_db_error)?;

        let Some(row) = row else {
            return Err(Error::Unauthorized);
        };

        let password_hash: String = row.get("password_hash");
        if !verify_password(password, &password_hash)? {
            return Err(Error::Unauthorized);
        }

        map_user_row(row)
    }

    pub async fn authenticate_sign_in_token(&self, sign_in_token: &str) -> Result<AuthUser, Error> {
        let token_hash = hash_token(sign_in_token);
        let row = sqlx::query(
            "SELECT u.id, u.role, u.username, u.display_name
             FROM sign_in_tokens t
             JOIN users u ON u.id = t.user_id
             WHERE t.token_hash = $1",
        )
        .bind(token_hash)
        .fetch_optional(&self.connection)
        .await
        .map_err(map_db_error)?;

        let Some(row) = row else {
            return Err(Error::Unauthorized);
        };
        map_user_row(row)
    }

    pub async fn create_session(&self, user_id: i32) -> Result<String, Error> {
        let token = crate::auth_crypto::random_token();
        let token_hash = hash_token(&token);
        sqlx::query(
            "INSERT INTO sessions (user_id, token_hash, expires_at)
             VALUES ($1, $2, NOW() + INTERVAL '30 days')",
        )
        .bind(user_id)
        .bind(token_hash)
        .execute(&self.connection)
        .await
        .map_err(map_db_error)?;
        Ok(token)
    }

    pub async fn user_from_session(&self, session_token: &str) -> Result<AuthUser, Error> {
        let token_hash = hash_token(session_token);
        let row = sqlx::query(
            "SELECT u.id, u.role, u.username, u.display_name
             FROM sessions s
             JOIN users u ON u.id = s.user_id
             WHERE s.token_hash = $1 AND s.expires_at > NOW()",
        )
        .bind(token_hash)
        .fetch_optional(&self.connection)
        .await
        .map_err(map_db_error)?;

        let Some(row) = row else {
            return Err(Error::Unauthorized);
        };
        map_user_row(row)
    }

    pub async fn delete_session(&self, session_token: &str) -> Result<(), Error> {
        let token_hash = hash_token(session_token);
        sqlx::query("DELETE FROM sessions WHERE token_hash = $1")
            .bind(token_hash)
            .execute(&self.connection)
            .await
            .map_err(map_db_error)?;
        Ok(())
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, Error> {
        match sqlx::query(
            "SELECT id, title, content, tags, author_id FROM questions LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .map(map_question_row)
        .fetch_all(&self.connection)
        .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn get_question(&self, question_id: i32) -> Result<Question, Error> {
        match sqlx::query("SELECT id, title, content, tags, author_id FROM questions WHERE id = $1")
            .bind(question_id)
            .map(map_question_row)
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn add_question(
        &self,
        new_question: NewQuestion,
        author_id: i32,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags, author_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, content, tags, author_id",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .bind(author_id)
        .map(map_question_row)
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn update_question(
        &self,
        question: UpdateQuestion,
        question_id: i32,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions
            SET title = $1, content = $2, tags = $3
            WHERE id = $4
            RETURNING id, title, content, tags, author_id",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(map_question_row)
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(result) => Ok(result.rows_affected() > 0),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn get_answers(&self, question_id: i32) -> Result<Vec<Answer>, Error> {
        match sqlx::query(
            "SELECT id, content, question_id, author_id FROM answers WHERE question_id = $1 ORDER BY id",
        )
        .bind(question_id)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
            author_id: row.get("author_id"),
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(answers) => Ok(answers),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn get_answer(&self, answer_id: i32) -> Result<Answer, Error> {
        match sqlx::query("SELECT id, content, question_id, author_id FROM answers WHERE id = $1")
            .bind(answer_id)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
                author_id: row.get("author_id"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer, author_id: i32) -> Result<Answer, Error> {
        match sqlx::query(
            "INSERT INTO answers (content, question_id, author_id)
            VALUES ($1, $2, $3)
            RETURNING id, content, question_id, author_id",
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .bind(author_id)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
            author_id: row.get("author_id"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => Err(map_db_error(e)),
        }
    }

    pub async fn delete_answer(&self, answer_id: i32) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM answers WHERE id = $1")
            .bind(answer_id)
            .execute(&self.connection)
            .await
        {
            Ok(result) => Ok(result.rows_affected() > 0),
            Err(e) => Err(map_db_error(e)),
        }
    }
}
