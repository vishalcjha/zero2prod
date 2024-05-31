pub mod configuration;
pub mod routes;
pub mod startup;
use axum::{
    extract::Query,
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

async fn greet(Query(query): Query<GreetQuery>) -> String {
    let query = query.name.unwrap_or_else(|| "World".into());
    format!("Hello {}", query)
}
