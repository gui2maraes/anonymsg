use crate::domain::key::{JsonPublicKey, KeyName};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use sqlx::PgPool;
use tracing::instrument;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RegisterInfo {
    name: KeyName,
    #[serde(rename = "publicKey")]
    public_key: JsonPublicKey,
}

#[instrument(skip(pool, info), fields(name = %info.name))]
pub async fn register(State(pool): State<PgPool>, Json(info): Json<RegisterInfo>) -> Response {
    match register_key(&pool, info).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(e) => match e.into_database_error() {
            Some(e) if e.is_unique_violation() => StatusCode::CONFLICT.into_response(),
            Some(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::new(e.to_string()))
                .unwrap(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    }
}

#[instrument(skip(pool, info) name="registering new key")]
async fn register_key(pool: &PgPool, info: RegisterInfo) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO keymap (name, public_key) VALUES ($1, $2)",
        info.name.name(),
        serde_json::to_value(info.public_key).unwrap()
    )
    .execute(pool)
    .await?;
    Ok(())
}
