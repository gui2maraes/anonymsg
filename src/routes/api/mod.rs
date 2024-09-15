pub mod messages;
pub mod names;
pub mod register;

use axum::routing::{post};
use axum::{Router};
use sqlx::PgPool;


pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/namesearch", post(names::name_search))
        .route("/namefetch", post(names::get_key))
        .route("/register", post(register::register))
        .route("/publish", post(messages::publish_message))
        .route("/messages", post(messages::get_messages))
}
