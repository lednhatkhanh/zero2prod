use zero2prod::{
    configuration::get_configuration,
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = get_connection_pool(&configuration.database);

    let application = Application::build(configuration, connection_pool).await?;
    application.run_until_stopped().await?;
    Ok(())
}
