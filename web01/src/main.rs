mod db;

use askama::Template;
use axum::body::Body;
use axum::extract::{Path, Request};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::any_service;
use axum::{
    http::StatusCode,
    middleware,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use futures::future::Lazy;
use http::HeaderName;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use time::Instant;
use tokio::signal;
use tokio::time::sleep;
use tower::{service_fn, ServiceBuilder};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use web01::api::chat::ChatState;
use web01::{api, AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    //tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web01=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let pool = db::establish_connection().await;
    let chat_state = ChatState::new(100);
    let app_state = Arc::new(AppState { pool, chat_state });

    // build our application with a route
    let template_route = Router::new()
        .route("/tem1", get(template_str))
        .route("/tem2", get(template_string))
        .route("/tem3", get(template_include))
        // askama template
        .route("/tem4/:name", get(askama_template));

    let api_route = Router::new()
        .route("/login", post(api::users::login))
        .route(
            "/service02",
            any_service(service_fn(|req: Request<Body>| async move {
                let body = Body::from(format!("Hi from `{} {}` /", req.method(), req.uri()));
                let res = Response::new(body);
                Ok::<_, Infallible>(res)
            })),
        );

    let app = Router::new()
        // 模版路径
        .route("/", get(index))
        .route("/auth", post(api::jwt::authorize))
        .route("/claims", post(api::jwt::claims))
        .nest("/tem", template_route)
        .nest("/api", api_route)
        .route("/websocket", get(api::chat::websocket_handler))
        .fallback(fallback)
        .layer(middleware::from_fn(self_middleware))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                // 超时时间
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
                // 设置请求头
                .layer(SetRequestIdLayer::new(
                    HeaderName::from_static("x-request-id"),
                    MakeRequestUuid,
                ))
                .layer(CorsLayer::new().allow_origin(Any)),
        )
        .with_state(app_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// 优雅关闭
async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.expect("failed to install Ctrl+C") };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {
            tracing::debug!("ctrl c stop")
        },
        _= terminate => {
            tracing::debug!("terminate stop")
        }
    }
}

async fn self_middleware(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    tracing::debug!("{}", request.uri().path());
    Ok(next.run(request).await)
}

async fn index(req: Request<Body>) -> String {
    if rand::random() {
        sleep(Duration::new(0, 300000)).await;
    }

    if let Some(req_id) = req.headers().get("x-request-id") {
        format!("requestId[{:?}]", req_id)
    } else {
        String::from("none")
    }
}

async fn fallback() -> Html<String> {
    Html("<h1>fallback</h1>".to_string())
}

async fn template_string() -> Html<String> {
    Html("<h1>hello template_string world</h1>".to_string())
}

async fn template_str() -> Html<&'static str> {
    Html(include_str!("../templates/chat.html"))
}

async fn template_include() -> Html<&'static str> {
    Html(include_str!("../templates/hello.html"))
}
#[derive(Template, Default)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

struct TemplateResponse<T>(T);

impl<T> IntoResponse for TemplateResponse<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render templates. Error: {err}"),
            )
                .into_response(),
        }
    }
}

async fn askama_template(Path(name): Path<String>) -> impl IntoResponse {
    let tpl = HelloTemplate { name };
    TemplateResponse(tpl)
}
