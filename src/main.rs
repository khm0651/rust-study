use actix::configuration::get_configuration;
use actix::startup::Application;
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    tokio::select! {
        o = application_task => report_exit("API", o),
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            println!("{} has exited.", task_name)
        }
        Ok(Err(e)) => {
            println!("{} failed. {:#?}", task_name, e)
        }
        Err(e) => {
            println!("{}' task failed to complete. {:#?}", task_name, e)
        }
    }
}
