use octantis::configuration::get_configuration;
use octantis::startup::run;
use sqlx::{PgConnection, Connection};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let settings = get_configuration().expect("Failed to read application configuration");
    let connection = PgConnection::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("0.0.0.0:{}", settings.application_port);
    let listener = std::net::TcpListener::bind(address).expect("Failed to bind to random port");
    run(listener, connection)?.await
}
