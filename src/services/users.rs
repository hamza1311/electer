use crate::models::User;
use anyhow::Result;
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn create(conn: &mut PgConnection, username: &str, password: &str) -> Result<User> {
    let uuid = Uuid::new_v4();
    let user = sqlx::query_as!(
        User,
        r#"
            INSERT INTO users(id, username, password) VALUES
            ($1, $2, $3)
            returning *;
        "#,
        uuid,
        username,
        password
    )
    .fetch_one(conn)
    .await?;

    Ok(user)
}

pub async fn fetch_by_id(conn: &mut PgConnection, id: Uuid) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        select *
        from users
        where id = $1;
        "#,
        id
    )
    .fetch_one(conn)
    .await?;

    Ok(user)
}

pub async fn fetch_by_username(conn: &mut PgConnection, username: &str) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        select *
        from users
        where username = $1;
        "#,
        username
    )
    .fetch_one(conn)
    .await?;

    Ok(user)
}
