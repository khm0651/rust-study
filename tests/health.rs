use actix::run;
use actix_web::{test, web, App};

#[actix_rt::test]
async fn health_check_test() {
    let server = run().expect("fail to run server");
    let req = test::TestRequest::get().uri("/health").to_request();
    assert!(resp.status().is_success())
}
