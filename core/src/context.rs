use std::sync::Arc;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: DatabaseConnection,
}

pub type Ctx = Arc<Context>;
