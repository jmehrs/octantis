use axum::{extract::State, http::StatusCode, Json};
use sqlx::postgres::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
}

pub async fn create_customer(
    State(connection): State<PgConnection>,
    Json(payload): Json<FormData>,
) -> StatusCode {
    StatusCode::OK
}
