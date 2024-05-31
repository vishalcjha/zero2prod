use reqwest::StatusCode;

pub async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Ok")
}
