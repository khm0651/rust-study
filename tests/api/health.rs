use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to send health_check request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
