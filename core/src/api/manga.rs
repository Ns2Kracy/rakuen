use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/manga", Router::new())
}
