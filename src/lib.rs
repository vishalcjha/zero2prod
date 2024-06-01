pub mod configuration;
pub mod routes;
pub mod startup;
pub mod state;
use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

async fn greet(Query(query): Query<GreetQuery>) -> String {
    let query = query.name.unwrap_or_else(|| "World".into());
    format!("Hello {}", query)
}
