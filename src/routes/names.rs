use crate::domain::key::KeyName;
use crate::domain::key::PemPublicKey;
use crate::NameMapState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Params {
    name: String,
}
pub async fn get_key(
    Path(params): Path<Params>,
    State(namemap): State<NameMapState>,
) -> Result<Json<PemPublicKey>, StatusCode> {
    let lock = namemap.read().await;
    let key = lock.get(&params.name);
    match key {
        Some(key) => Ok(Json(PemPublicKey(key.clone()))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
pub async fn list_namemap(State(namemap): State<NameMapState>) -> Json<Vec<KeyName>> {
    Json(namemap.read().await.keys().cloned().collect())
}
