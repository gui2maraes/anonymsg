use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

use crate::domain::key::KeyName;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PublishPost {
    pub base64_content: String,
    pub recipient: KeyName,
}

#[tracing::instrument(skip(pool, post), name = "publishing new post")]
pub async fn publish_post(State(pool): State<PgPool>, Json(post): Json<PublishPost>) -> StatusCode {
    match insert_post(&pool, post).await {
        Ok(()) => StatusCode::OK,
        Err(sqlx::Error::Database(err)) if err.is_foreign_key_violation() => {
            StatusCode::BAD_REQUEST
        }
        Err(e) => {
            tracing::error!("error publishing post: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn insert_post(pool: &PgPool, post: PublishPost) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO POSTS (id, recipient, sent_at, content)
        VALUES (gen_random_uuid(), $1, now(), $2)"#,
        post.recipient.name(),
        post.base64_content
    )
    .execute(pool)
    .await?;
    Ok(())
}
