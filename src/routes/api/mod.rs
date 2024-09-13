pub mod messages;
pub mod names;
pub mod register;

use axum::routing::{get, post};
use axum::{Json, Router};
use sqlx::PgPool;
use utoipa::OpenApi;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        names::name_search,
        names::get_key,
        register::register,
        messages::get_messages,
        messages::publish_message
    ),
    components(schemas(names::Params, register::RegisterInfo, messages::GetMessages, messages::PublishMessage))
)]
struct ApiDoc;
#[utoipa::path(
    get,
    path = "/api/openapi.json",
    responses(
        (status = OK, description = "JSON file", body = utoipa::openapi::OpenApi)
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/namesearch", post(names::name_search))
        .route("/namefetch", post(names::get_key))
        .route("/register", post(register::register))
        .route("/publish", post(messages::publish_message))
        .route("/messages", post(messages::get_messages))
        .route("/openapi.json", get(openapi))
}
