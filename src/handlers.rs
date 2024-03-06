/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

use std::net::SocketAddr;
use axum::{
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use axum::extract::{ConnectInfo, Path};
use diesel::RunQueryDsl;
use rand::distributions::{Alphanumeric, DistString};
use crate::{
    domain::{CreateTinyUrl, Data},
    tools::get_current_datetime,
};
use crate::cache::domain::CacheHandler;
use crate::database::{db_create_tiny_url, db_get_tiny_url, establish_connection};
use crate::domain::DataInsert;
use crate::schema::data_tiny::dsl::data_tiny;
use crate::schema::data_tiny::short_url;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_tiny_url(
    url_path: Path<String>
) -> (StatusCode, String) {
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
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<CreateTinyUrl>,
) -> (StatusCode, Json<Data>) {
    log::info!("Request from: {}", addr);
    let tiny_url_gen: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);

    let to_insert = DataInsert {
        base_url: payload.url.clone(),
        short_url: tiny_url_gen,
        created_at: get_current_datetime(),
        created_by_ip: Some(addr.to_string()),
    };

    match db_create_tiny_url(to_insert) {
        Ok(data) => (StatusCode::CREATED, Json(data)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Data::default()))
    }
}
