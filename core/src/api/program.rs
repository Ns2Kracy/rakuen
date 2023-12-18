use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/program", Router::new())
}
