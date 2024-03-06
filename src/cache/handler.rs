/*
 * Copyright (c) 2024. Extragornax (gaspard@extragornax.fr)
 */

use std::{
    env,
    sync::{Arc, Mutex},
};

use serde::Serialize;

use crate::cache::domain::{CacheError, CacheHandler, TimeToLive};

fn connect() -> redis::Connection {
    dotenv::dotenv().ok();

    let redis_host = env::var("REDIS_HOST").expect("Missing environment variable REDIS_HOST");
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    let uri_scheme = match env::var("IS_TLS").unwrap_or("false".to_string()) {
        e if e == "true".to_string() => "rediss",
        _ => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("Failed to connect to Redis")
}

impl CacheHandler {
    pub fn new() -> Box<CacheHandler> {
        let t = CacheHandler {
            conn: Arc::new(Mutex::new(connect())),
        };

        Box::new(t)
    }

    pub fn has(self, needle: &str) -> bool {
        let start_time = std::time::Instant::now();

        let mut conn = self.conn.lock().unwrap();

        let val: u8 = redis::cmd("EXISTS")
            .arg(needle)
            .query(&mut conn)
            .unwrap_or_else(|_| panic!("Failed to get value for cache key: {}", needle));

        let elapsed = start_time.elapsed();
        log::debug!("Cache|Has|Elapsed time: {:?}", elapsed);
        val != 0
    }

    pub fn get<T>(self, needle: &str) -> Result<T, CacheError>
        where T: Serialize + for<'a> serde::Deserialize<'a>,
    {
        let start_time = std::time::Instant::now();
        let mut conn = self.conn.lock().unwrap();

        let val: redis::RedisResult<Option<String>> = redis::cmd("GET").arg(&needle).query(&mut conn);

        match val {
            Ok(val) => match val {
                Some(val) => {
                    let elapsed = start_time.elapsed();
                    log::debug!("Cache|Get|Elapsed time: {:?}", elapsed);
                    match serde_json::from_str::<T>(&val) {
                        Ok(res) => return Ok(res),
                        Err(_) => {
                            let _ = self.clone().del(needle);
                            return Err(CacheError::new("Failed to deserialize value"));
                        }
                    };
                }
                None => {
                    let elapsed = start_time.elapsed();
                    log::debug!("Cache|Get|Elapsed time: {:?}", elapsed);
                    Err(CacheError::new("Cache key not found"))
                }
            },
            Err(_) => {
                let elapsed = start_time.elapsed();
                log::debug!("Cache|Get|Elapsed time: {:?}", elapsed);
                Err(CacheError::new("Failed to get value for cache key"))
            }
        }
    }

    pub fn set<T>(self, key: &str, value: T) -> Result<(), CacheError>
        where T: Serialize,
    {
        let value_as_string = serde_json::to_string(&value)?;
        Self::set_with_ttl(self, key, &value_as_string, TimeToLive::default())
    }

    pub fn set_ttl<T>(self, key: &str, value: T, ttl_in_second: TimeToLive) -> Result<(), CacheError>
        where T: Serialize,
    {
        let value_as_string = serde_json::to_string(&value)?;
        Self::set_with_ttl(self, key, &value_as_string, ttl_in_second)
    }

    fn set_with_ttl(
        self,
        key: &str,
        value: &str,
        ttl_in_second: TimeToLive,
    ) -> Result<(), CacheError> {
        let start_time = std::time::Instant::now();

        let mut conn = self.conn.lock().unwrap();

        let _: () = redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_in_second as u32)
            .arg(value)
            .query(&mut conn)
            .unwrap_or_else(|_| panic!("Failed to set value for cache key: {}", key));

        let elapsed = start_time.elapsed();
        log::debug!("Cache|Set|Elapsed time: {:?}", elapsed);

        Ok(())
    }

    pub fn del(self, key: &str) -> Result<(), CacheError> {
        // log::debug!("Deleting cache key: \"{}\"...", key);
        let start_time = std::time::Instant::now();

        let mut conn = self.conn.lock().unwrap();

        let _: () = redis::cmd("DEL")
            .arg(key)
            .query(&mut conn)
            .unwrap_or_else(|_| panic!("Failed to delete cache key: {}", key));

        let elapsed = start_time.elapsed();
        log::debug!("Cache|Del|Elapsed time: {:?}", elapsed);
        Ok(())
    }
}
