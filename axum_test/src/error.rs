use axum::{
    async_trait,
    extract::{
        rejection::{JsonRejection, QueryRejection},
        FromRequest, Query, Request,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use validator::Validate;

// The kinds of errors we can hit in our application.
#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    AxumHttp(#[from] AxumHttpError),
    #[error("{0}")]
    MsgError(String),
    #[error(transparent)]
    DbError(#[from] DbErr),
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
    #[error(transparent)]
    OtherError(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        AppError::MsgError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 格式化时间为字符串，格式为：2022-02-02T18:12:23.443
        AppJson(ErrorResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            msg: self.to_string(),
            timestamp: chrono::Local::now()
                .format("%Y-%m-%dT%H:%M:%S%.3f")
                .to_string(),
        })
        .into_response()
    }
}

#[derive(Error, Debug)]
pub enum AxumHttpError {
    // The request body contained invalid JSON
    #[error("{0}")]
    JsonRejection(JsonRejection),
    #[error("{0}")]
    QueryRejection(QueryRejection),
    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("token does not exist")]
    MissingToken,
    #[error("token expired")]
    TokenExpired,
    #[error("token check error")]
    InvalidToken,
}

// How we want errors responses to be serialized
#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    msg: String,
    timestamp: String,
}

impl From<JsonRejection> for AxumHttpError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<QueryRejection> for AxumHttpError {
    fn from(rejection: QueryRejection) -> Self {
        Self::QueryRejection(rejection)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AppJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for AppJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            //.map_err(AxumHttpError::JsonRejection)?;
            .map_err(|e| {
                // 打印详细错误日志
                eprintln!("Failed to deserialize JSON: {:?}", e);
                AxumHttpError::JsonRejection(e)
            })?;
        value.validate().map_err(AxumHttpError::ValidationError)?;
        Ok(AppJson(value))
    }
}

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

/// 自定义Query解析器
///
/// 添加对Query参数解析
#[derive(Debug, Clone, Copy, Default)]
pub struct AppQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for AppQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state)
            .await
            .map_err(AxumHttpError::QueryRejection)?;
        value.validate().map_err(AxumHttpError::ValidationError)?;
        Ok(AppQuery(value))
    }
}
