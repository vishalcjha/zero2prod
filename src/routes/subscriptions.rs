use axum::Form;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubscriberInfo {
    name: String,
    email: String,
}

pub async fn subscribe(Form(subscriber_info): Form<SubscriberInfo>) {}
