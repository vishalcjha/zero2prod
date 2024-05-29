use tokio::sync::oneshot;
use zero2prod::run;

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
