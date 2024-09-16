pub mod messages;
pub mod alias;
pub mod register;

use axum::routing::post;
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;


pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/search/:alias", get(alias::search_alias))
        .route("/registry/:alias", get(alias::fetch_alias))
        .route("/register", post(register::register))
        .route("/publish", post(messages::publish_message))
        .route("/messages", get(messages::get_messages))
}
