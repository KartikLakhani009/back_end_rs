#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use axum::{routing::get, Router};
use tracing::log::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let _db_conn_str = utils::constants::DATABASE_URL.clone();

    let addr = utils::constants::SERVER_ADDRESS.clone();

    let app = Router::new().route("/test", get(routes::notimplement));

    let listener = tokio::net::TcpListener::bind(addr.clone())
        .await
        .expect("listening has issue");

    tracing::info!("listening on http://{:?}", &addr);

    axum::serve(listener, app)
        .await
        .expect("server is not able to serve");
}
