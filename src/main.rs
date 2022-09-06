use actix::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().expect("fail to run server").await
}
