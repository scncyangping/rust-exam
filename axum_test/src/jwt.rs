use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{AppError, AuthError};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    #[serde(default = "default_expire_time")]
    pub expire_time: u64,
}
#[inline]
fn default_expire_time() -> u64 {
    3600
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub iat: u64,
    pub exp: u64,
    pub sub: String,
    pub phone: String,
    pub name: String,
    pub email: String,
    pub user_id: String,
    pub username: String,
}

impl Claims {
    pub fn new(exp: u64, sub: String) -> Result<Self, String> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Time went backwards".to_string())?
            .as_secs();
        let expiration = time + exp;

        let ss: Claims = Self {
            sub,
            iat: time,
            exp: expiration,
            ..Default::default()
        };
        Ok(ss)
    }
    pub fn with_user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = user_id;
        self
    }

    pub fn with_email(&mut self, email: String) -> &mut Self {
        self.email = email;
        self
    }

    pub fn with_phone(&mut self, phone: String) -> &mut Self {
        self.phone = phone;
        self
    }

    pub fn with_username(&mut self, name: String) -> &mut Self {
        self.username = name;
        self
    }
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn generate_token(&self, jwt_key: &[u8]) -> Result<String, String> {
        let header = Header::new(jsonwebtoken::Algorithm::HS512);
        encode(&header, &self, &EncodingKey::from_secret(jwt_key))
            .map_err(|e| format!("JWT encoding error: {:?}", e))
    }
}

pub fn validate_jwt_token(token: &str, jwt_key: &[u8]) -> Result<Claims, AppError> {
    let mut validator = Validation::default();
    validator.algorithms = vec![Algorithm::HS512];
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(jwt_key), &validator)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;
    Ok(token_data.claims)
}
