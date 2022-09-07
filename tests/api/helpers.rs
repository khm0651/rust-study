use actix::configuration::get_configuration;
use actix::model::User;
use actix::startup::Application;

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub static mut USER_LIST: Vec<User> = vec![];

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut conf = get_configuration().expect("Failed to read configuration.");
        conf.application.port = 0;
        conf
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build test app.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    };

    test_app
}
