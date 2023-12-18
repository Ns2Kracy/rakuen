use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/light_novel", Router::new())
}
