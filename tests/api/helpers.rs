use actix::api::USER_LIST;
use actix::configuration::get_configuration;
use actix::model::User;
use actix::startup::Application;

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

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

pub fn setting_two_dummy_data() -> &'static User {
    unsafe { USER_LIST.clear() }
    let dummy_data1 = User {
        user_id: 10,
        name: String::from("hamin1"),
        phone: String::from("010-4938-0651"),
    };

    let dummy_data2 = User {
        user_id: 20,
        name: String::from("hamin2"),
        phone: String::from("010-5788-0651"),
    };

    unsafe {
        USER_LIST.push(dummy_data1);
        USER_LIST.push(dummy_data2);
        return USER_LIST.get(0).expect("out of index");
    }
}
