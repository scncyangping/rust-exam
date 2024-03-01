use crate::api;
use axum::body::Bytes;
use axum::extract::FromRequestParts;
use axum::response::{IntoResponse, Response};
use axum::{Json, RequestPartsExt};
use axum_extra::headers::authorization::{Bearer, Credentials};
use axum_extra::headers::{Authorization, Error};
use axum_extra::TypedHeader;
use http::request::Parts;
use http::{HeaderValue, StatusCode};
use jsonwebtoken::{decode, encode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::from_utf8_unchecked;
use std::time::{SystemTime, UNIX_EPOCH};
use time::Duration;
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: jsonwebtoken::EncodingKey,
    pub decoding: jsonwebtoken::DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: jsonwebtoken::EncodingKey::from_secret(secret),
            decoding: jsonwebtoken::DecodingKey::from_secret(secret),
        }
    }
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims::new(String::from("cao"));
    let token = encode(&jsonwebtoken::Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(serde_json::json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub async fn claims(claims: Claims) -> Result<String, AuthError> {
    tracing::debug!(?claims);
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}
#[axum::async_trait]
impl<T> FromRequestParts<T> for Claims
where
    T: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &T) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<TokenAuthCode>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.decoding,
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

// 自定义Auth头
pub struct TokenAuthCode(SelfToken);

impl Credentials for TokenAuthCode {
    const SCHEME: &'static str = "TOKEN";

    fn decode(value: &HeaderValue) -> Option<Self> {
        debug_assert!(
            value.as_bytes()[..Self::SCHEME.len()].eq_ignore_ascii_case(Self::SCHEME.as_bytes()),
            "HeaderValue to decode should start with \"TOKEN ..\", received = {:?}",
            value,
        );
        SelfToken::from_val(value).ok().map(TokenAuthCode)
    }

    fn encode(&self) -> HeaderValue {
        (&self.0).into()
    }
}

impl TokenAuthCode {
    /// View the token part as a `&str`.
    pub fn token(&self) -> &str {
        self.0.as_str()["TOKEN ".len()..].trim_start()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfToken {
    value: HeaderValue,
}

impl<'a> From<&'a SelfToken> for HeaderValue {
    fn from(src: &'a SelfToken) -> HeaderValue {
        src.value.clone()
    }
}

impl SelfToken {
    pub(crate) fn from_val(val: &HeaderValue) -> Result<Self, String> {
        if val.to_str().is_ok() {
            Ok(SelfToken { value: val.clone() })
        } else {
            Err("error".to_string())
        }
    }
    pub(crate) fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.value.as_bytes()) }
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let exp = SystemTime::now() + Duration::seconds(15 * 24 * 50 * 60);
        let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        Claims {
            sub,
            exp,
            company: String::from("com"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
