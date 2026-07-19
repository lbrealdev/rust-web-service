use std::env;

use warp::http::StatusCode;

use crate::types::login::{LoginRequest, LoginResponse};

pub async fn login(login_req: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    let response = if login_req.password == admin_password {
        LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        }
    } else {
        LoginResponse {
            success: false,
            message: "Invalid password".to_string(),
        }
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        if response.success {
            StatusCode::OK
        } else {
            StatusCode::UNAUTHORIZED
        },
    ))
}
