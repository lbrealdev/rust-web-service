use handle_errors::Error;
use warp::{
    http::StatusCode,
    reject::{custom, Rejection},
    reply, Filter, Reply,
};

use crate::rate_limit::{allow_attempt, clear_failures, client_key, record_failure};
use crate::store::Store;
use crate::types::user::{AuthResponse, AuthUser, LoginRequest, RegisterRequest, UserRole};

pub fn require_auth(store: Store) -> impl Filter<Extract = (AuthUser,), Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());
    warp::header::optional::<String>("authorization")
        .and(store_filter)
        .and_then(|auth_header: Option<String>, store: Store| async move {
            let header = auth_header.ok_or_else(|| custom(Error::Unauthorized))?;
            let token = header
                .strip_prefix("Bearer ")
                .or_else(|| header.strip_prefix("bearer "))
                .map(str::trim)
                .filter(|t| !t.is_empty())
                .ok_or_else(|| custom(Error::Unauthorized))?;

            store.user_from_session(token).await.map_err(custom)
        })
}

fn bearer_from_header(auth_header: Option<String>) -> Option<String> {
    auth_header.and_then(|h| {
        h.strip_prefix("Bearer ")
            .or_else(|| h.strip_prefix("bearer "))
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
    })
}

pub async fn register(
    store: Store,
    body: RegisterRequest,
    addr: Option<std::net::SocketAddr>,
) -> Result<impl Reply, Rejection> {
    let key = client_key(addr);
    if !allow_attempt(&key) {
        return Err(custom(Error::TooManyRequests));
    }

    let username = body.username.trim().to_string();
    if username.len() < 3 || username.len() > 64 {
        return Err(custom(Error::ValidationError(
            "username must be 3-64 characters".into(),
        )));
    }
    if body.password.len() < 8 {
        return Err(custom(Error::ValidationError(
            "password must be at least 8 characters".into(),
        )));
    }

    match store
        .create_user(&username, &body.password, UserRole::User)
        .await
    {
        Ok(user) => {
            clear_failures(&key);
            let token = store.create_session(user.id).await.map_err(custom)?;
            Ok(reply::with_status(
                reply::json(&AuthResponse {
                    token,
                    user: user.to_public(),
                    sign_in_token: None,
                }),
                StatusCode::CREATED,
            ))
        }
        Err(Error::Conflict(_)) => {
            record_failure(&key);
            Err(custom(Error::Conflict("username already taken".into())))
        }
        Err(e) => Err(custom(e)),
    }
}

pub async fn login(
    store: Store,
    body: LoginRequest,
    addr: Option<std::net::SocketAddr>,
) -> Result<impl Reply, Rejection> {
    let key = client_key(addr);
    if !allow_attempt(&key) {
        return Err(custom(Error::TooManyRequests));
    }

    let user = if let Some(sign_in_token) = body
        .sign_in_token
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        store.authenticate_sign_in_token(sign_in_token).await
    } else if let (Some(username), Some(password)) = (
        body.username
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty()),
        body.password.as_ref().filter(|s| !s.is_empty()),
    ) {
        store.authenticate_password(username, password).await
    } else {
        return Err(custom(Error::ValidationError(
            "provide username+password or sign_in_token".into(),
        )));
    };

    match user {
        Ok(user) => {
            clear_failures(&key);
            let token = store.create_session(user.id).await.map_err(custom)?;
            Ok(reply::json(&AuthResponse {
                token,
                user: user.to_public(),
                sign_in_token: None,
            }))
        }
        Err(Error::Unauthorized) => {
            record_failure(&key);
            Err(custom(Error::Unauthorized))
        }
        Err(e) => Err(custom(e)),
    }
}

pub async fn guest_token(
    store: Store,
    addr: Option<std::net::SocketAddr>,
) -> Result<impl Reply, Rejection> {
    let key = client_key(addr);
    if !allow_attempt(&key) {
        return Err(custom(Error::TooManyRequests));
    }

    let (user, sign_in_token) = store.create_token_user().await.map_err(custom)?;
    clear_failures(&key);
    let token = store.create_session(user.id).await.map_err(custom)?;
    Ok(reply::with_status(
        reply::json(&AuthResponse {
            token,
            user: user.to_public(),
            sign_in_token: Some(sign_in_token),
        }),
        StatusCode::CREATED,
    ))
}

pub async fn logout(store: Store, auth_header: Option<String>) -> Result<impl Reply, Rejection> {
    if let Some(token) = bearer_from_header(auth_header) {
        store.delete_session(&token).await.map_err(custom)?;
    }
    Ok(reply::with_status("logged out", StatusCode::OK))
}

pub async fn me(user: AuthUser) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&user.to_public()))
}
