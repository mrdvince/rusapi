use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // the health check is exposed at /health_check;
    // the health check is behind a GET method;
    // the health check always returns a 200;
    // the health check’s response has no body

    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = rusapi::run(listener).expect("Failed to start bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
