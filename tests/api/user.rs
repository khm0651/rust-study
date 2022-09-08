use crate::helpers::{setting_two_dummy_data, spawn_app};
use actix::api::USER_LIST;
use actix::model::User;

static API_VER: &str = "api/v1";

#[tokio::test]
async fn should_get_users_function_if_success_return_user_list_all_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    setting_two_dummy_data();

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
async fn should_get_user_function_if_success_return_specific_user_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let dummy = setting_two_dummy_data();
    let response = client
        .get(&format!(
            "{}/{}/user/{}",
            &app.address, API_VER, dummy.user_id
        ))
        .send()
        .await
        .expect("Failed to send get_users request");

    assert!(response.status().is_success());

    let result = response.json::<User>().await.expect("parsing error");
    let expect = dummy.clone();
    assert_eq!(result, expect);
}

#[tokio::test]
async fn should_post_user_function_if_success_return_same_value() {
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

#[tokio::test]
async fn should_delete_user_function_if_success_return_same_value() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let dummy = setting_two_dummy_data();
    let expect = dummy.clone();
    let delete_user_response = client
        .delete(&format!(
            "{}/{}/user/{}",
            &app.address, API_VER, dummy.user_id
        ))
        .json(&dummy.clone())
        .send()
        .await
        .expect("Failed to send post_user request");

    assert!(delete_user_response.status().is_success());

    let result = delete_user_response
        .json::<User>()
        .await
        .expect("parsing error");

    assert_eq!(result, expect);
}

#[tokio::test]
async fn should_delete_user_function_if_success_return_size_is_1() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let dummy_data = setting_two_dummy_data();

    let delete_user_response = client
        .delete(&format!(
            "{}/{}/user/{}",
            &app.address, API_VER, dummy_data.user_id
        ))
        .json(&dummy_data)
        .send()
        .await
        .expect("Failed to send get_users request");

    assert!(delete_user_response.status().is_success());

    let get_users_response = client
        .get(&format!("{}/{}/user", &app.address, API_VER))
        .send()
        .await
        .expect("Failed to send get_users request");

    assert!(get_users_response.status().is_success());

    let result = get_users_response
        .json::<Vec<User>>()
        .await
        .expect("parsing error");

    assert_eq!(result.len(), 1)
}

#[tokio::test]
async fn should_patch_user_function_if_success_return_patch_value() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    setting_two_dummy_data();

    let dummy_data = User {
        user_id: 20,
        name: String::from("hamin343434"),
        phone: String::from("010-5222222222222788-0651"),
    };

    let patch_user_response = client
        .patch(&format!("{}/{}/user", &app.address, API_VER))
        .json(&dummy_data)
        .send()
        .await
        .expect("parsing error");

    let result = patch_user_response
        .json::<User>()
        .await
        .expect("parsing error");

    assert_eq!(result, dummy_data)
}
