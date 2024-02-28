use sqlx::{MySql, Pool};

pub mod api;
pub mod db;
pub struct AppState {
    pub pool: Pool<MySql>,
    pub chat_state: api::chat::ChatState,
}
