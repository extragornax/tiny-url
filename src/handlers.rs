/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

use std::net::SocketAddr;
use axum::{
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use axum::extract::{ConnectInfo, Path, State};
use diesel::RunQueryDsl;
use rand::distributions::{Alphanumeric, DistString};
use crate::{
    domain::{CreateTinyUrl, Data},
    tools::get_current_datetime,
};
use crate::cache::domain::CacheHandler;
use crate::database::{db_create_tiny_url, db_get_tiny_url, establish_connection};
use crate::domain::DataInsert;
use crate::rate_limit::RateLimiter;
use crate::schema::data_tiny::dsl::data_tiny;
use crate::schema::data_tiny::short_url;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_tiny_url(
    State(rate_limiter): State<RateLimiter>,
    url_path: Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> (StatusCode, String) {

    match rate_limiter.check_if_rate_limited(addr.ip()) {
        Ok(_) => {}
        Err(e) => {
            return (StatusCode::TOO_MANY_REQUESTS, e);
        }
    }

    let parsed = url_path.to_string();

    if parsed.is_empty() {
        return (StatusCode::NOT_FOUND, "Not Found".to_string());
    }

    let cache = CacheHandler::new();
    match cache.clone().get::<String>(&parsed) {
        Ok(data) => {
            return (StatusCode::OK, data);
        }
        Err(_) => {}
    }


    let results: Data = match db_get_tiny_url(parsed) {
        Ok(data) => data,
        Err(_) => {
            return (StatusCode::NOT_FOUND, "Not Found".to_string());
        }
    };

    let _ = cache.set::<String>(&results.short_url, results.base_url.clone());

    (StatusCode::OK, results.base_url)
}

pub async fn create_tiny_url(
    State(rate_limiter): State<RateLimiter>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<CreateTinyUrl>,
) -> (StatusCode, Result<Json<Data>, Json<String>>) {
    log::info!("Request from: {}", addr);

    match rate_limiter.check_if_rate_limited(addr.ip()) {
        Ok(_) => {}
        Err(e) => {
            return (StatusCode::TOO_MANY_REQUESTS, Err(Json(e.to_string())));
        }
    }


    let tiny_url_gen: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);

    let to_insert = DataInsert {
        base_url: payload.url.clone(),
        short_url: tiny_url_gen,
        created_at: get_current_datetime(),
        created_by_ip: Some(addr.to_string()),
    };

    match db_create_tiny_url(to_insert) {
        Ok(data) => (StatusCode::CREATED, Ok(Json(data))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Err(Json(e.to_string()))),
    }
}
