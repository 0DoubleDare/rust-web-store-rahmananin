use sqlx::{Pool, mysql::MySqlPool};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::mysql::MySqlPool,
}
