use axum::{Extension, Router};
use migration::{Migrator, MigratorTrait};
use rakuen_core::{api, common::logging::init_tracing, context};
use sea_orm::Database;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    init_tracing().await;

    let db = Database::connect("sqlite://rakuen.db?mode=rwc")
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to database: {}", e);
            e
        })
        .unwrap();

    Migrator::up(&db, None).await.unwrap();

    let app = Router::new()
        .merge(api::mount())
        .layer(Extension(Arc::new(context::Context { db })))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7590")
        .await
        .unwrap();

    tracing::info!(
        "Rakuen is running on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
