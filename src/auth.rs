use crate::models::User;
use crate::utils::random_string;
use crate::ISS;
use crate::{services, AppState};
use axum::body::BoxBody;
use chrono::{DateTime, Duration, Utc};
use futures::future::BoxFuture;
use hmac::{Hmac, Mac, NewMac};
use http::{header, Request, Response, StatusCode};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::auth::AsyncAuthorizeRequest;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: i64,
    iss: String,
    iat: i64,
    sub: Uuid,
}

lazy_static! {
    pub static ref SECRET: Vec<u8> = {
        let s = random_string();
        Hmac::<Sha256>::new_from_slice(s.as_bytes())
            .unwrap()
            .finalize()
            .into_bytes()
            .to_vec()
    };
    pub static ref REFRESH_KEYS: Arc<Mutex<HashMap<String, Uuid>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(SECRET.as_slice());
}

pub fn generate_jwt(user_id: Uuid) -> jsonwebtoken::errors::Result<(String, DateTime<Utc>)> {
    let duration = Duration::minutes(5);
    let now = Utc::now();
    let exp = now + duration;
    let claims = Claims {
        iss: ISS.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
        sub: user_id,
    };

    jsonwebtoken::encode(&Header::default(), &claims, &*ENCODING_KEY).map(|it| (it, exp))
}

pub async fn decode_jwt(conn: &mut PoolConnection<Postgres>, token: &str) -> anyhow::Result<User> {
    let validation = Validation {
        iss: Some(ISS.to_string()),
        ..Validation::default()
    };
    let decoding_key = DecodingKey::from_secret(SECRET.as_slice());

    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?;
    let user = services::users::fetch_by_id(conn, token_data.claims.sub).await?;
    Ok(user)
}

#[derive(Clone, Copy)]
pub struct JwtAuth;

impl AsyncAuthorizeRequest for JwtAuth {
    type Output = User;
    type Future = BoxFuture<'static, Option<User>>;
    type ResponseBody = BoxBody;

    fn authorize<B>(&mut self, request: &Request<B>) -> Self::Future {
        let token = request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|it| it.to_str().ok())
            .and_then(|it| it.strip_prefix("Bearer "))
            .map(|it| it.to_string());

        let state = request
            .extensions()
            .get::<AppState>()
            .expect("state not set");
        let pool = state.pool.clone();
        Box::pin(async move {
            match token {
                Some(token) => {
                    // for authenticated requests, the pool is acquired twice
                    // if the request handler makes use of database
                    // todo make it one
                    let mut conn = pool.acquire().await.ok()?;
                    decode_jwt(&mut conn, &token).await.ok()
                }
                None => None,
            }
        })
    }

    fn on_authorized<B>(&mut self, request: &mut Request<B>, user: User) {
        request.extensions_mut().insert(user);
    }

    fn unauthorized_response<B>(&mut self, _request: &Request<B>) -> Response<BoxBody> {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(BoxBody::default())
            .unwrap()
    }
}
