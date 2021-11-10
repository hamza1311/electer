use crate::models::{Poll, PollOption, User};
use anyhow::Result;
use serde::Serialize;
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn create(
    conn: &mut PgConnection,
    creator: &User,
    title: &str,
    options: &[String],
) -> Result<Uuid> {
    let uuid = Uuid::new_v4();
    let inserted = sqlx::query!(
        r#"
insert into polls (id, title, creator)
values ($1, $2, $3)
returning id;
    "#,
        uuid,
        title,
        creator.id
    )
    .fetch_one(&mut *conn)
    .await?;
    let poll_id = inserted.id;

    for option in options {
        let uuid = Uuid::new_v4();
        sqlx::query!(
            r#"
            insert into poll_options (id, poll_id, text)
            values ($1, $2, $3);
        "#,
            uuid,
            poll_id,
            option
        )
        .execute(&mut *conn)
        .await?;
    }
    Ok(poll_id)
}

macro_rules! map_view_to_poll {
    ($it:ident) => {{
        Poll {
            id: $it.poll_id.unwrap(),
            title: $it.poll_title.unwrap(),
            creator: User {
                id: $it.user_id.unwrap(),
                username: $it.user_username.unwrap(),
                password: $it.user_password.unwrap(),
                create_time: $it.user_create_time.unwrap(),
            },
            create_time: $it.poll_create_time.unwrap(),
        }
    }};
}

pub async fn fetch_by_id(conn: &mut PgConnection, poll_id: Uuid) -> Result<Poll> {
    let poll = sqlx::query!(
        r#"
SELECT *
FROM polls_view
WHERE poll_id = $1;
    "#,
        poll_id
    )
    .map(|it| map_view_to_poll!(it))
    .fetch_one(conn)
    .await?;

    Ok(poll)
}

pub async fn fetch_many(conn: &mut PgConnection, limit: u64) -> Result<Vec<Poll>> {
    let poll = sqlx::query!(
        r#"
SELECT *
FROM polls_view
ORDER BY poll_create_time DESC
LIMIT $1;
    "#,
        (limit as i64)
    )
    .map(|it| map_view_to_poll!(it))
    .fetch_all(conn)
    .await?;

    Ok(poll)
}

pub async fn fetch_options(conn: &mut PgConnection, poll: &Poll) -> Result<Vec<PollOption>> {
    let options = sqlx::query!(
        r#"
    select id, text
    from poll_options
    where poll_id = $1;
    "#,
        poll.id
    )
    .map(|it| PollOption {
        id: it.id,
        poll: poll.clone(),
        text: it.text,
    })
    .fetch_all(conn)
    .await?;
    Ok(options)
}

pub async fn add_vote(
    conn: &mut PgConnection,
    voter_id: Uuid,
    poll_id: Uuid,
    option_id: Uuid,
) -> Result<()> {
    sqlx::query!(
        r#"
insert into votes (voter, poll, option)
values ($1, $2, $3);
    "#,
        voter_id,
        poll_id,
        option_id
    )
    .execute(conn)
    .await?;

    Ok(())
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct PollResult {
    option: Uuid,
    votes: u64,
}

pub async fn fetch_results(conn: &mut PgConnection, poll_id: Uuid) -> Result<Vec<PollResult>> {
    let results = sqlx::query!(
        r#"
select count(option), option
from votes
where poll = $1
group by option;
    "#,
        poll_id
    )
    .map(|it| PollResult {
        option: it.option,
        votes: it.count.unwrap() as u64,
    })
    .fetch_all(conn)
    .await?;
    Ok(results)
}
