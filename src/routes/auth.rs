use crate::auth::generate_jwt;
use crate::auth::REFRESH_KEYS;
use crate::errors::HandlerResult;
use crate::models::User;
use crate::utils::random_string;
use crate::{services, AppState};
use anyhow::anyhow;
use axum::{extract, response, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Connection;

#[derive(Deserialize)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Serialize, Clone)]
pub struct AuthenticatedResponse {
    token: String,
    refresh: String,
    duration: DateTime<Utc>,
}

pub async fn signup(
    state: extract::Extension<AppState>,
    credentials: extract::Json<UserCredentials>,
) -> HandlerResult<Json<AuthenticatedResponse>> {
    let mut conn = state.pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let hashed = hash(&credentials.password, DEFAULT_COST)?;
    let user = services::users::create(&mut tx, &credentials.username, &hashed).await?;
    tx.commit().await?;
    Ok(Json(sign_user_in(&user).await))
}

pub async fn signin(
    state: extract::Extension<AppState>,
    credentials: extract::Json<UserCredentials>,
) -> HandlerResult<Json<AuthenticatedResponse>> {
    let mut conn = state.pool.acquire().await?;
    let user = services::users::fetch_by_username(&mut conn, &credentials.username).await?;
    let valid = verify(&credentials.password, &user.password)?;

    if !valid {
        return Err(anyhow!("invalid password").into());
    }

    let resp = sign_user_in(&user).await;
    Ok(Json(resp))
}

async fn sign_user_in(user: &User) -> AuthenticatedResponse {
    let (jwt, duration) = generate_jwt(user.id).unwrap();

    let refresh_token = base64::encode(random_string());
    REFRESH_KEYS
        .lock()
        .await
        .insert(refresh_token.clone(), user.id);
    AuthenticatedResponse {
        token: jwt,
        refresh: refresh_token,
        duration,
    }
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenPayload {
    refresh: String,
}

pub async fn refresh_token(
    refresh_token: extract::Json<RefreshTokenPayload>,
) -> HandlerResult<response::Json<AuthenticatedResponse>> {
    let keys = REFRESH_KEYS.lock().await;
    match keys.get(&refresh_token.refresh) {
        Some(uuid) => {
            let (jwt, duration) = generate_jwt(*uuid).unwrap();
            Ok(response::Json(AuthenticatedResponse {
                token: jwt,
                refresh: refresh_token.refresh.clone(),
                duration,
            }))
        }
        None => Err(anyhow!("invalid refresh token").into()),
    }
}
