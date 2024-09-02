use crate::domain::key::KeyName;
use crate::domain::key::PemPublicKey;
use crate::NameMapState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RegisterInfo {
    name: KeyName,
    key: PemPublicKey,
}
pub async fn register(
    State(namemap): State<NameMapState>,
    Json(info): Json<RegisterInfo>,
) -> StatusCode {
    let mut lock = namemap.write().await;
    if lock.contains(info.name.name()) {
        return StatusCode::CONFLICT;
    }

    lock.insert(info.name, info.key.0);
    StatusCode::CREATED
}

async fn register_key(pool: &PgPool, info: RegisterInfo) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO keymap (name, public_key) VALUES ($1, $2)",
        info.name,
        info.key.pem_string()
    )
}
