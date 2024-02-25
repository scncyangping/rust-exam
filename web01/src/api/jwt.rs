use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let exp = SystemTime::now() + Duration::seconds(15 * 24 * 50 * 60);
        let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        Claims { sub, exp }
    }
}

pub enum AuthError {
    TokenCreation,
}
