use crate::model::User;
use actix_web::web::Json;
use actix_web::{HttpResponse, Responder};

static mut USER_LIST: Vec<User> = vec![];

pub async fn get_users() -> impl Responder {
    let result = serde_json::to_string(unsafe { &USER_LIST });
    match result {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(e) => HttpResponse::from_error(e),
    }
}
