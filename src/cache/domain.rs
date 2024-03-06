/*
 * Copyright (c) 2024. Extragornax (gaspard@extragornax.fr)
 */


use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};

pub enum TimeToLive {
    OneHour = 60 * 60,
    OneWeek = 60 * 60 * 24 * 7,
}

impl Default for TimeToLive {
    fn default() -> Self {
        TimeToLive::OneHour
    }
}

#[derive(Clone)]
pub struct CacheHandler {
    pub conn: Arc<Mutex<redis::Connection>>,
}

#[derive(Debug)]
pub struct CacheError {
    details: String,
}

impl Error for CacheError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(e: serde_json::Error) -> CacheError {
        CacheError {
            details: format!("Failed to serialize value: {}", e),
        }
    }
}

impl CacheError {
    pub(crate) fn new(msg: &str) -> CacheError {
        CacheError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
