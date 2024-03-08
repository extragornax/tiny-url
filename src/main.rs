/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

#![recursion_limit = "256"]
extern crate openssl;
// DO NOT MOVE THIS LINE
// #[macro_use]
// extern crate diesel;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::rate_limit::RateLimiter;

mod domain;
mod handlers;
mod routing;
mod tools;
mod database;
mod schema;
mod cache;
mod rate_limit;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let listen_url = format!("0.0.0.0:{}", env::var("PORT").unwrap_or("3000".to_string()));
    let rate_limiter: RateLimiter = RateLimiter::default();

    log::info!("Listening on {}", listen_url);
    let routes = routing::get_routes(rate_limiter);
    let listener = tokio::net::TcpListener::bind(listen_url).await.unwrap();
    axum::serve(listener, routes.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
