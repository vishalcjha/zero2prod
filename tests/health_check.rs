use reqwest::Client;
use sqlx::{Connection, PgConnection};
use tokio::sync::oneshot;
use zero2prod::{configuration::get_configuration, startup::run};

fn spawn_app() -> oneshot::Receiver<u16> {
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:0");
    let (tx, rw) = oneshot::channel();
    let _ = tokio::spawn(async move {
        let tcp_listener = tcp_listener.await.unwrap();
        let _ = tx.send(tcp_listener.local_addr().unwrap().port());
        run(tcp_listener).await;
    });
    rw
}

#[tokio::test]
async fn health_check_works() {
    let port_receiver = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "http://localhost:{}/health_check",
            port_receiver.await.unwrap()
        ))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let port = spawn_app().await.unwrap();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres.");
    let client = Client::new();

    let body = "name=vishal%20kumar&email=vishalcjha%40gmail.com";
    let response = client
        .post(format!("http://localhost:{}/subscribe", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to send subscribe message");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "vishalcjha@gmail.com");
    assert_eq!(saved.name, "vishal kumar");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let port = spawn_app().await.unwrap();
    let client = Client::new();

    let data = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, error_msg) in data {
        let response = client
            .post(format!("http://localhost:{}/subscribe", port))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        assert_eq!(
            422,
            response.status().as_u16(),
            "Api did not fail with 400 when payload was {}",
            error_msg
        );
    }
}
