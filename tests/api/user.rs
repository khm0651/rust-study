use crate::helpers::spawn_app;
use actix::api::USER_LIST;
use actix::model::User;

static API_VER: &str = "api/v1";

#[tokio::test]
async fn should_get_users_function_return_user_list_all_data() {
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

#[tokio::test]
async fn should_post_user_function_return_same_value() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let dummy_data = User {
        user_id: 10,
        name: String::from("hamin"),
        phone: String::from("010-4938-0651"),
    };

    let response = client
        .post(&format!("{}/{}/user", &app.address, API_VER))
        .json(&dummy_data)
        .send()
        .await
        .expect("Failed to send post_user request");

    assert!(response.status().is_success());

    let result = response.json::<User>().await.expect("parsing error");
    let expect = dummy_data.clone();
    assert_eq!(result, expect);
}
