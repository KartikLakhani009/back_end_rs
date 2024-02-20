#[allow(dead_code)]
pub const SESSION_COOKIE_NAME: &str = "axum_svelte_session";
pub const SERVER_PORT: &str = "8383";
pub const SERVER_HOST: &str = "127.0.0.1";

use dotenv::dotenv;
use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DATABASE_URL: String = set_database();
    pub static ref SERVER_ADDRESS: String = set_server_address();
}

fn set_database() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn set_server_address() -> String {
    dotenv().ok();
    let port = env::var("SERVER_PORT")
        .ok()
        .unwrap_or_else(|| SERVER_PORT.to_string());
    let host = env::var("SERVER_HOST")
        .ok()
        .unwrap_or_else(|| SERVER_HOST.to_string());
    let addr: String = format!("{host}:{port}")
        .parse()
        .expect("Can not parse address and port");

    addr
}
