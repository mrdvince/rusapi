#[tokio::test]
async fn health_check_works() {
    // the health check is exposed at /health_check;
    // the health check is behind a GET method;
    // the health check always returns a 200;
    // the health check’s response has no body

    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = rusapi::run().expect("Failed to start bind address");
    let _ = tokio::spawn(server);
}
