pub mod names;
pub mod publish;
pub mod register;

use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/names/search/:name", get(names::name_search))
        .route("/names/:name", get(names::get_key))
        .route("/register", post(register::register))
}
