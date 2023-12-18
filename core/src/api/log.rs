use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/log", Router::new())
}
