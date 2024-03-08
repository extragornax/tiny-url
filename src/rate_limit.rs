use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::IpAddr;
use chrono::{DateTime, Utc};

// This will be the request limit (per minute) for a user to access an endpoint
// If the user attempts to go beyond this limit, we should return an error
const REQUEST_LIMIT: usize = 120;

#[derive(Clone, Default)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<IpAddr, Vec<DateTime<Utc>>>>>,
}

impl RateLimiter {
    pub fn check_if_rate_limited(&self, ip_addr: IpAddr) -> Result<(), String> {
        log::info!("Checking if IP is rate limited: {}", ip_addr);
        // we only want to keep timestamps from up to 60 seconds ago
        let throttle_time_limit = Utc::now() - std::time::Duration::from_secs(60);

        let mut requests_hashmap = self.requests.lock().unwrap();

        let mut requests_for_ip = requests_hashmap
            // grab the entry here and allow us to modify it in place
            .entry(ip_addr)
            // if the entry is empty, insert a vec with the current timestamp
            .or_insert(Vec::new());

        requests_for_ip.retain(|x| x.to_utc() > throttle_time_limit);
        requests_for_ip.push(Utc::now());

        log::info!("Requests for IP: {}", requests_for_ip.len());

        if requests_for_ip.len() > REQUEST_LIMIT {
            return Err("IP is rate limited :(".to_string());
        }

        Ok(())
    }
}
