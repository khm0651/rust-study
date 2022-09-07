use crate::api::{get_users, health_check};
use crate::configuration::Settings;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(settions: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            settions.application.host, settions.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, settions.application.base_url).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

async fn run(listener: TcpListener, base_url: String) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/api/v1/user", web::get().to(get_users))
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
