use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let serve_dir = ServeDir::new("assets2").not_found_service(ServeFile::new("assets/html"));
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .nest_service("/assets2", serve_dir.clone())
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
#[derive(Debug, Deserialize)]
struct InputParams {
    foo: i32,
    bar: String,
    third: Option<i32>,
}

async fn query(Query(params): Query<InputParams>) -> impl IntoResponse {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query </h3>")
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello,World!</h1>")
}
