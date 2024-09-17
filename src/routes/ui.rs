use tower_http::services::ServeDir;

pub fn ui_server() -> ServeDir {
    #[cfg(debug_assertions)]
    let path = "web/dist";
    #[cfg(not(debug_assertions))]
    let path = "dist";
    ServeDir::new(path)
}
