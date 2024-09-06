use axum::Router;
use tokio::net::ToSocketAddrs;

use crate::routes;

pub fn application(pool: sqlx::PgPool) -> Router {
    Router::new()
        .nest("/api", routes::api::router())
        .with_state(pool)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback_service(routes::ui::ui_server())
}

pub async fn run(addr: impl ToSocketAddrs, pool: sqlx::PgPool) {
    let app = application(pool);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}
