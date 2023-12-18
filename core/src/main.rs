use std::sync::Arc;

use axum::{Extension, Router};
use rakuen_core::{api, common::logging::init_tracing, context};
use sea_orm::Database;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    init_tracing().await;

    let db = Database::connect("sqlite://rakuen.db?mode=rwc").await.unwrap();

    let app = Router::new()
        .layer(Extension(Arc::new(context::Context { db })))
        .merge(api::mount())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7590")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
