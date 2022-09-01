use actix::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().expect("fail to run server").await
}

// #[get("/")]
// pub async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello Actix")
// }
//
// #[get("/json")]
// pub async fn hello_json() -> Result<impl Responder> {
//     let person = Person {
//         name: String::from("hamin"),
//     };
//     Ok(web::Json(person))
// }
//
// #[derive(Serialize, Debug)]
// struct Person {
//     name: String,
// }
