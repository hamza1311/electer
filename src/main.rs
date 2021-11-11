mod auth;
mod errors;
mod models;
mod routes;
mod services;
mod utils;

use crate::auth::JwtAuth;
use axum::error_handling::HandleErrorExt;
use axum::handler::Handler;
use axum::routing::any;
use axum::{
    routing::{get, post},
    Router,
};
use http::StatusCode;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::auth::{AsyncRequireAuthorization, AsyncRequireAuthorizationLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

const DATABASE_URL: &str = "postgresql://electer:password@localhost:5432/electer";
const ISS: &str = "electer";

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let _ = &*auth::ENCODING_KEY;
    let database_url = std::env::var("DATABASE_URL");
    let database_url = database_url.as_deref().unwrap_or(DATABASE_URL);
    let pool = PgPool::connect(database_url).await.unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to apply migrations");

    let app_state = AppState { pool: pool.clone() };

    let auth = Router::new()
        .route("/signup", post(routes::auth::signup))
        .route("/signin", post(routes::auth::signin))
        .route("/refresh", post(routes::auth::refresh_token));

    let users = Router::new()
        .route("/me", get(routes::users::get_me))
        .layer(AsyncRequireAuthorizationLayer::new(JwtAuth));

    let api = Router::new()
        .nest("/auth", auth)
        .nest("/users", users)
        .nest(
            "/polls",
            Router::new()
                .route(
                    "/",
                    axum::routing::service_method_routing::get(routes::polls::list.into_service())
                        .post(AsyncRequireAuthorization::new(
                            any(routes::polls::create),
                            JwtAuth,
                        )),
                )
                .nest(
                    "/:poll_id",
                    Router::new()
                        .route("/", get(routes::polls::get))
                        .route(
                            "/vote",
                            AsyncRequireAuthorization::new(post(routes::polls::vote), JwtAuth),
                        )
                        .route("/results", get(routes::polls::results)),
                ),
        )
        .layer(AddExtensionLayer::new(app_state));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|it| it.parse::<u16>().ok())
        .unwrap_or(8000);

    let dist_dir = std::env::var("DIST_DIR");
    let dist_dir = dist_dir.as_deref().unwrap_or("./frontend/dist");

    let app = Router::new().nest("/api", api).fallback(
        axum::routing::service_method_routing::get(ServeDir::new(dist_dir)).handle_error(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", e),
            )
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
