/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */

use axum::{
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use axum::extract::Path;
use diesel::RunQueryDsl;
use crate::{
    domain::{CreateTinyUrl, Data},
    tools::get_current_datetime,
};
use crate::database::establish_connection;
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

    let connection = &mut establish_connection();

    use crate::schema::data_tiny::dsl::*;

    let results: Data = match data_tiny
        .filter(short_url.eq(parsed))
        .select((
            id,
            base_url,
            short_url,
            created_at,
            created_by_ip,
        ))
        .first(connection) {
        Ok(data) => data,
        Err(_) => {
            return (StatusCode::NOT_FOUND, "Not Found".to_string());
        }
    };

    log::info!("Results: {:?}", results);

    (StatusCode::OK, results.base_url)
}

pub async fn create_tiny_url(
    Json(payload): Json<CreateTinyUrl>,
) -> (StatusCode, Json<Data>) {
    (StatusCode::CREATED, Json(Data {
        id: 1,
        base_url: payload.url,
        short_url: "http://tinyurl.com/1".to_string(),
        created_at: get_current_datetime(),
        created_by_ip: Some("0.0.0.0".to_string()),
    }))
}
