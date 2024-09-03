pub mod messages;
pub mod names;
pub mod register;

use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/names/search", get(names::name_search))
        .route("/name", get(names::get_key))
        .route("/register", post(register::register))
        .route("/publish", post(messages::publish_message))
        .route("/messages", get(messages::get_messages))
}
