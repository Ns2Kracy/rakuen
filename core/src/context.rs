use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: DatabaseConnection,
}
