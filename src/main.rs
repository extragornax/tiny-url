/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

#![recursion_limit = "256"]
extern crate openssl;
// DO NOT MOVE THIS LINE
// #[macro_use]
// extern crate diesel;

use std::env;
use std::net::SocketAddr;

mod domain;
mod handlers;
mod routing;
mod tools;
mod database;
mod schema;
mod cache;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listen_url = format!("0.0.0.0:{}", env::var("PORT").unwrap_or("3000".to_string()));

    log::info!("Listening on {}", listen_url);

    let routes = routing::get_routes();
    let listener = tokio::net::TcpListener::bind(listen_url).await.unwrap();
    axum::serve(listener, routes.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
