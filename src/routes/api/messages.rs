use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

use crate::domain::key::KeyName;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PublishMessage {
    /// base64 encoded string
    pub content: String,
    /// recipient name
    #[schema(value_type = String)]
    pub recipient: KeyName,
}

#[utoipa::path(
    post,
    path = "/api/publish",
    request_body = PublishMessage,
    responses(
        (status = OK, description = "Message published succesfully", body = ()),
        (status = NOT_FOUND, description = "recipient name not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Error publishing message", body = ())
    )
)]
#[tracing::instrument(skip(pool, msg), name = "publishing new message")]
pub async fn publish_message(
    State(pool): State<PgPool>,
    Json(msg): Json<PublishMessage>,
) -> StatusCode {
    match insert_msg(&pool, msg).await {
        Ok(()) => StatusCode::OK,
        Err(sqlx::Error::Database(err)) if err.is_foreign_key_violation() => {
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            tracing::error!("error publishing message: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, utoipa::ToSchema)]
pub struct GetMessages {
    /// recipient name
    #[schema(value_type = String)]
    pub recipient: KeyName,
    /// max messages to fetch
    pub limit: Option<u32>,
}

#[utoipa::path(
    post,
    path = "/api/messages",
    request_body = GetMessages,
    responses(
        (status = OK, description = "Message fetched succesfully", body = Vec<Message>),
        (status = INTERNAL_SERVER_ERROR, description = "Error fetching message", body = ()),
    ),
)]
#[tracing::instrument(skip(pool), name = "get published messages")]
pub async fn get_messages(
    State(pool): State<PgPool>,
    Json(get_msg): Json<GetMessages>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    match get_sent_msgs(&pool, get_msg).await {
        Ok(msgs) => Ok(Json(msgs)),
        Err(e) => {
            tracing::error!("error getting messages: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn insert_msg(pool: &PgPool, msg: PublishMessage) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO MESSAGES (id, recipient, sent_at, content)
        VALUES (gen_random_uuid(), $1, now(), $2)"#,
        msg.recipient.name(),
        msg.content
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub sent_at: DateTime<Utc>,
}
async fn get_sent_msgs(pool: &PgPool, get_msg: GetMessages) -> sqlx::Result<Vec<Message>> {
    let limit = match get_msg.limit {
        Some(l) => l.min(200),
        None => 10,
    };
    let msgs = sqlx::query!(
        r#"
        SELECT content, sent_at FROM messages WHERE recipient = $1 ORDER BY sent_at LIMIT $2
        "#,
        get_msg.recipient.name(),
        limit as i32
    )
    .fetch_all(pool)
    .await?;
    Ok(msgs
        .into_iter()
        .map(|r| Message {
            content: r.content,
            sent_at: r.sent_at,
        })
        .collect())
}
