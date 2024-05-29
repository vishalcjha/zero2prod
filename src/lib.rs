use axum::{extract::Query, http::StatusCode, routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Ok")
}
async fn greet(Query(query): Query<GreetQuery>) -> String {
    let query = query.name.unwrap_or_else(|| "World".into());
    format!("Hello {}", query)
}

pub async fn run(listener: TcpListener) {
    let app = Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check));

    axum::serve(listener, app).await.unwrap();
}
