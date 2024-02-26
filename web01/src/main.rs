mod db;

use askama::Template;
use axum::extract::Path;
use axum::response::{Html, IntoResponse, Response};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use web01::api;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();
    let pool = db::establish_connection().await;
    // build our application with a route
    let app = Router::new()
        // 模版路径
        .route("/tem1", get(template_str))
        .route("/tem2", get(template_string))
        .route("/tem3", get(template_include))
        .route("/tem4/:name", get(askama_template))
        .route("/api/login", post(api::users::login))
        .with_state(pool);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn template_string() -> Html<String> {
    Html("<h1>hello template_string world</h1>".to_string())
}

async fn template_str() -> Html<&'static str> {
    Html("<h1>hello template_str world</h1>")
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
