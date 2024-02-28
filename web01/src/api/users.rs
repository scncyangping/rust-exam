use crate::api::jwt::Claims;
use crate::api::ApiError;
use crate::db::User;
use crate::AppState;
use anyhow::anyhow;
use axum::extract::State;
use axum::routing::any;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{Error, MySql, Pool};
use std::sync::Arc;
use std::task::Poll;

#[derive(Deserialize)]
pub struct LoginPayload {
    code: String,
}

#[derive(Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}
impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
    // step1. 使用code从wx获取token
    let wx_user = wx_login(payload.code).await?;
    let user = sqlx::query_as::<_, User>("select * from users where openid= ?")
        .bind(&wx_user.open_id)
        .fetch_one(&state.pool)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            let id = 1;
            let res = sqlx::query("insert into users (id,openid,session_key) value (?,?)")
                .bind(&id)
                .bind(&wx_user.open_id)
                .bind(&wx_user.session_key)
                .execute(&state.pool)
                .await?;

            sqlx::query_as::<_, User>("select * from users where openid= ?")
                .bind(&wx_user.open_id)
                .fetch_one(&state.pool)
                .await?
        }
        Err(e) => return Err(ApiError::from(e)),
    };

    let clams = Claims::new(user.id.to_string());
    let token = encode(
        &Header::default(),
        &clams,
        &EncodingKey::from_secret(b"secret"),
    )?;
    Ok(Json(AuthBody::new(token)))
}

#[derive(Deserialize, Default)]
struct WxUser {
    pub open_id: String,
    pub session_key: String,
}
async fn wx_login(code: String) -> Result<WxUser, ApiError> {
    Ok(WxUser::default())
}
