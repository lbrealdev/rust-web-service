use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Token,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Token => "token",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            "token" => Some(UserRole::Token),
            _ => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub role: UserRole,
    pub username: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i32,
    pub role: UserRole,
    pub username: Option<String>,
    pub display_name: Option<String>,
}

impl AuthUser {
    pub fn to_public(&self) -> User {
        User {
            id: self.id,
            role: self.role.clone(),
            username: self.username.clone(),
            display_name: self.display_name.clone(),
        }
    }

    pub fn can_modify(&self, author_id: Option<i32>) -> bool {
        if self.role.is_admin() {
            return true;
        }
        match author_id {
            Some(id) => id == self.id,
            None => false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub sign_in_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_token: Option<String>,
}
