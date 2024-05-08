use sqlx::{FromRow, MySql, MySqlPool, Pool};
use std::env;
// use time::PrimitiveDateTime;

pub async fn establish_connection() -> Pool<MySql> {
    let db_url = env::var("DATABASE_URL").expect("not found");
    let pool = MySqlPool::connect(&db_url).await.expect("connect error");
    pool
}

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub openid: String,
    pub session_key: String,
    //pub created_at: PrimitiveDateTime,
    //pub updated_at: PrimitiveDateTime,
}
