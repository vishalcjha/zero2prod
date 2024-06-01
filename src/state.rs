use std::sync::Arc;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    connection: Arc<PgPool>,
}

impl AppState {
    pub fn new(connection: PgPool) -> Self {
        Self {
            connection: Arc::new(connection),
        }
    }

    pub fn get_db_connection(&self) -> &PgPool {
        &self.connection
    }
}
