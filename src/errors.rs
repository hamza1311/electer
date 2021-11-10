use axum::body::Body;
use axum::response::IntoResponse;
use http::Response;
use std::ops::Deref;

pub struct AnyError(anyhow::Error);

impl IntoResponse for AnyError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        Response::builder()
            .status(500)
            .body(format!("{}", self.0).into())
            .unwrap()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AnyError {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl Deref for AnyError {
    type Target = anyhow::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type HandlerResult<T> = Result<T, AnyError>;
