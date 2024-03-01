use sqlx::{MySql, Pool};

pub mod api;
pub mod db;
pub struct AppState {
    pub pool: Pool<MySql>,
    pub chat_state: api::chat::ChatState,
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    pub struct A<T>(T);
    #[test]
    fn test() {
        let x: Result<i32, String> = Ok(23);
        let xx = x.map(A);
        println!("{:?}", xx)
    }
}
