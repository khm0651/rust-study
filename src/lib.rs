use actix_web::dev::Server;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, patch, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    user_id: u32,
    name: String,
    phone: String,
}

static mut USER_LIST: Vec<User> = vec![];

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(get_users)
            .service(get_user)
            .service(post_user)
            .service(delete_user)
            .service(patch_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("health check ok")
}

#[get("api/v1/user")]
pub async fn get_users() -> impl Responder {
    unsafe {
        let res_body = serde_json::to_string(&USER_LIST).expect("parsing error");
        HttpResponse::Ok().body(res_body)
    }
}

#[get("api/v1/user/{user_id}")]
pub async fn get_user(user_id: Path<u32>) -> impl Responder {
    unsafe {
        let user_id = user_id.into_inner();
        let index = USER_LIST
            .binary_search_by_key(&user_id, |user| user.user_id)
            .expect("not found user");
        let user = USER_LIST.get(index).expect("not found user");
        let res_body = serde_json::to_string(user).expect("parsing error");
        HttpResponse::Ok().body(res_body)
    }
}

#[post("api/v1/user")]
pub async fn post_user(data: Json<User>) -> impl Responder {
    unsafe {
        let user = data.into_inner();
        USER_LIST.push(User {
            user_id: user.clone().user_id,
            name: user.clone().name,
            phone: user.clone().phone,
        });
        let res_body = serde_json::to_string(&user).expect("parsing error");
        HttpResponse::Ok().body(res_body)
    }
}

#[delete("api/v1/user/{user_id}")]
pub async fn delete_user(user_id: Path<u32>) -> impl Responder {
    unsafe {
        let user_id = user_id.into_inner();
        let user_index = USER_LIST
            .binary_search_by_key(&user_id, |user| user.user_id)
            .expect("not found user");
        // let user_index = USER_LIST
        //     .iter()
        //     .position(|user| user.user_id == user_id)
        //     .expect("not found user");
        USER_LIST.remove(user_index);
        let res_body = serde_json::to_string(&USER_LIST).expect("parsing error");
        HttpResponse::Ok().body(res_body)
    }
}

#[patch("api/v1/user")]
pub async fn patch_user(data: Json<User>) -> impl Responder {
    unsafe {
        let user = data.into_inner();
        let user_index = USER_LIST
            .binary_search_by_key(&user.user_id, |user| user.user_id)
            .expect("not found user");
        USER_LIST.remove(user_index);
        USER_LIST.insert(user_index, user);
        let res_body = serde_json::to_string(&USER_LIST).expect("parsing error");
        HttpResponse::Ok().body(res_body)
    }
}
