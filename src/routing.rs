/*
 * Copyright (c) 2024. Extragornax (gaspard at extragornax.fr)
 */


use axum::{
    Router,
    routing::{get, post},
};
use crate::{
    handlers::{create_tiny_url, get_tiny_url, root},
    rate_limit::RateLimiter
};

pub fn get_routes(_rate: RateLimiter) -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /redirect` goes to `get_tiny_url`
        .route("/redirect/*url", get(get_tiny_url).with_state(_rate.clone()))
        // `POST /users` goes to `create_user`
        .route("/create", post(create_tiny_url).with_state(_rate.clone()))
}
