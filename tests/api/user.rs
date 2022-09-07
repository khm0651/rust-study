use crate::helpers::{spawn_app, USER_LIST};
use actix::model::User;

static API_VER: &str = "api/v1";

#[tokio::test]
async fn get_users_test() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/{}/user", &app.address, API_VER))
        .send()
        .await
        .expect("Failed to send get_users request");

    assert!(response.status().is_success());

    let result = response.json::<Vec<User>>().await.expect("parsing error");
    let expect = unsafe { USER_LIST.clone() };
    assert_eq!(result, expect);
}
