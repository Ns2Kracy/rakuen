mod anime;
mod auth;
mod light_novel;
mod log;
mod manga;
mod program;
mod user;

pub fn mount() -> axum::Router {
    axum::Router::new().nest(
        "/api",
        axum::Router::new()
            .merge(auth::mount())
            .merge(user::mount())
            .merge(anime::mount())
            .merge(manga::mount())
            .merge(light_novel::mount())
            .merge(log::mount())
            .merge(program::mount()),
    )
}
