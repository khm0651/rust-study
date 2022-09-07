use crate::model::User;
use actix_web::web::{Json, Path};
use actix_web::{HttpResponse, Responder};

pub static mut USER_LIST: Vec<User> = vec![];

pub async fn get_users() -> impl Responder {
    let result = serde_json::to_string(unsafe { &USER_LIST });
    match result {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(e) => HttpResponse::from_error(e),
    }
}

pub async fn post_user(user: Json<User>) -> impl Responder {
    let user = user.into_inner();
    unsafe { USER_LIST.push(user.clone()) }
    let result = serde_json::to_string(&user);
    match result {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(e) => HttpResponse::from_error(e),
    }
}

pub async fn delete_user(user_id: Path<u16>) -> impl Responder {
    let user_id = user_id.into_inner();
    let remove_result: User;
    unsafe {
        let position = USER_LIST
            .binary_search_by_key(&user_id, |user| user.user_id)
            .expect("not found user");
        remove_result = USER_LIST.remove(position)
    }
    let result = serde_json::to_string(&remove_result);
    match result {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(e) => HttpResponse::from_error(e),
    }
}

pub async fn patch_user(user: Json<User>) -> impl Responder {
    let user = user.into_inner();
    unsafe {
        let position = USER_LIST
            .binary_search_by_key(&user.user_id, |user| user.user_id)
            .expect("not found user");
        USER_LIST.remove(position);
        USER_LIST.insert(position, user.clone());
    }
    let result = serde_json::to_string(&user);
    match result {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(e) => HttpResponse::from_error(e),
    }
}
