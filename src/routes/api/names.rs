use crate::domain::key::KeyName;
use crate::domain::key::PemPublicKey;
use crate::NameMapState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Params {
    name: String,
}
#[tracing::instrument(skip(pool), name = "get public_key by name")]
pub async fn get_key(
    Path(params): Path<Params>,
    State(pool): State<PgPool>,
) -> Result<Json<PemPublicKey>, StatusCode> {
    let k = match get_key_by_name(&pool, &params.name).await {
        Ok(k) => k,
        Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("database error: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let key = match PemPublicKey::from_pem(&k) {
        Ok(k) => k,
        Err(e) => {
            tracing::error!("failed to parse stored PEM public key: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(key))
}
#[tracing::instrument(skip(pool), name = "name fuzzy search")]
pub async fn name_search(
    Path(params): Path<Params>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let names = match name_fuzzy_search(&pool, &params.name).await {
        Ok(names) => names,
        Err(e) => {
            tracing::error!("database error: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(names))
}

async fn get_key_by_name(pool: &PgPool, name: &str) -> sqlx::Result<String> {
    let key_str = sqlx::query!("SELECT public_key FROM keymap WHERE name = $1", name)
        .fetch_one(pool)
        .await?;
    Ok(key_str.public_key)
}

async fn name_fuzzy_search(pool: &PgPool, name: &str) -> sqlx::Result<Vec<String>> {
    let names = sqlx::query!(
        r#"SELECT name
        FROM keymap
        WHERE name % $1
        ORDER BY similarity(name, $1) DESC, name
        LIMIT 10"#,
        name
    )
    .fetch_all(pool)
    .await?;
    Ok(names.into_iter().map(|r| r.name).collect())
}
