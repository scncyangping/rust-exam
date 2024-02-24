use sqlx::{MySql, MySqlPool, Pool};
use std::env;

pub async fn establish_connection() -> Pool<MySql> {
    let db_url = env::var("DATABASE_URL").expect("not found");
    let pool = MySqlPool::connect(&db_url).await.expect("connect error");
    pool
}
