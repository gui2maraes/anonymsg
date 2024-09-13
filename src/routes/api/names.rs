use crate::domain::key::{PublicJwk, KeyName};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Params {
    #[schema(value_type = String)]
    name: KeyName,
}
#[utoipa::path(
    post,
    path = "/api/namefetch",
    request_body = Params,
    responses(
        (status = OK, description = "Key fetched succesfully", body = PublicJwk),
        (status = NOT_FOUND, description = "Key with name not found", body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),

    )
)]
#[tracing::instrument(skip(pool), name = "get public_key by name")]
pub async fn get_key(
    State(pool): State<PgPool>,
    Json(params): Json<Params>,
) -> Result<Json<PublicJwk>, StatusCode> {
    let key = match get_key_by_name(&pool, params.name.name()).await {
        Ok(k) => k,
        Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("database error: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(key))
}

#[utoipa::path(
    post,
    path = "/api/namesearch",
    request_body = Params,
    responses(
        (status = OK, description = "List of similar names", body = Vec<String>),
        (status = INTERNAL_SERVER_ERROR, body = ()),

    )
)]
#[tracing::instrument(skip(pool), name = "name fuzzy search")]
pub async fn name_search(
    State(pool): State<PgPool>,
    Json(params): Json<Params>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let names = match name_fuzzy_search(&pool, params.name.name()).await {
        Ok(names) => names,
        Err(e) => {
            tracing::error!("database error: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(names))
}

async fn get_key_by_name(pool: &PgPool, name: &str) -> sqlx::Result<PublicJwk> {
    let key_str = sqlx::query!(
        r#"SELECT public_key as "key: sqlx::types::Json<PublicJwk>"
        FROM keymap WHERE name = $1"#,
        name
    )
    .fetch_one(pool)
    .await?;
    Ok(key_str.key.0)
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
