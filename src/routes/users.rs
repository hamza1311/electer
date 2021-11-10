use crate::models::User;
use axum::{extract, Json};

pub async fn get_me(ex: extract::Extension<User>) -> Json<User> {
    Json(ex.0)
}
