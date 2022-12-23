use std::net::TcpListener;
use std::sync::Arc;

use axum::routing::{get, post, IntoMakeService};
use axum::{Router, Server};
use hyper::server::conn::AddrIncoming;
use hyper::Body;
use sqlx::PgConnection;

use crate::routes;

type AppServer = Server<AddrIncoming, IntoMakeService<Router<(), Body>>>;

pub fn run(listener: TcpListener, connection: PgConnection) -> hyper::Result<AppServer> {

    let connection = Arc::new(connection);

    let app = Router::new()
        .route("/status", get(routes::status))
        .route("/customers", post(routes::create_customer))
        .with_state(connection);

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
