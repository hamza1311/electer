use crate::errors::HandlerResult;
use crate::models::{Poll, PollOption, User};
use crate::{services, AppState};
use axum::{extract, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePollRequestPayload {
    title: String,
    options: Vec<String>,
}

#[derive(Serialize)]
pub struct PollPayload {
    #[serde(flatten)]
    poll: Poll,
    options: Vec<PollOption>,
}

pub async fn create(
    state: extract::Extension<AppState>,
    user: extract::Extension<User>,
    body: Json<CreatePollRequestPayload>,
) -> HandlerResult<Json<PollPayload>> {
    let mut conn = state.pool.acquire().await?;
    let mut tx = conn.begin().await?;
    let created_id = services::polls::create(&mut tx, &user, &body.title, &body.options).await?;
    tx.commit().await?;

    get_poll_by_id(&mut conn, created_id).await
}

async fn get_poll_by_id(
    conn: &mut PgConnection,
    poll_id: Uuid,
) -> HandlerResult<Json<PollPayload>> {
    let poll = services::polls::fetch_by_id(&mut *conn, poll_id).await?;
    let options = services::polls::fetch_options(conn, &poll).await?;

    Ok(Json(PollPayload { poll, options }))
}

pub async fn get(
    state: extract::Extension<AppState>,
    poll_id: extract::Path<Uuid>,
) -> HandlerResult<Json<PollPayload>> {
    let mut conn = state.pool.acquire().await?;
    get_poll_by_id(&mut conn, *poll_id).await
}

pub async fn list(
    state: extract::Extension<AppState>,
    limit: Option<extract::Query<u64>>,
) -> HandlerResult<Json<Vec<PollPayload>>> {
    let mut conn = state.pool.acquire().await?;
    let polls = services::polls::fetch_many(&mut conn, limit.map(|it| *it).unwrap_or(64)).await?;
    let mut resp = Vec::with_capacity(polls.len());
    for poll in polls {
        let options = services::polls::fetch_options(&mut conn, &poll).await?;
        resp.push(PollPayload { poll, options })
    }
    Ok(Json(resp))
}

#[derive(Deserialize)]
pub struct PollVoteRequestPayload {
    option: Uuid,
}

pub async fn vote(
    state: extract::Extension<AppState>,
    user: extract::Extension<User>,
    poll_id: extract::Path<Uuid>,
    Json(PollVoteRequestPayload { option }): Json<PollVoteRequestPayload>,
) -> HandlerResult<StatusCode> {
    let mut conn = state.pool.acquire().await?;
    let mut tx = conn.begin().await?;
    services::polls::add_vote(&mut tx, user.id, *poll_id, option).await?;
    tx.commit().await?;
    Ok(StatusCode::CREATED)
}

pub async fn results(
    state: extract::Extension<AppState>,
    poll_id: extract::Path<Uuid>,
) -> HandlerResult<Json<Value>> {
    let mut conn = state.pool.acquire().await?;
    let results = services::polls::fetch_results(&mut conn, *poll_id).await?;
    Ok(Json(serde_json::to_value(results)?))
}
