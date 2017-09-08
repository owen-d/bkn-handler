use std::env;
use super::errors::*;

// Need to supply ROCKET_ENV & ROCKET_PORT
#[derive(Default)]
pub struct Env {
    // cassandra address
    pub cass_addr: String,
    pub cass_port: u16,
    pub cass_pool_size: u32,
    // valid list of referrer headers to accept. Since we redirect from the page itself. Can be `*` for allowing any referer if it exists
    pub referrers: Vec<String>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn load_env(&mut self) -> Self {
        Env {
            cass_addr: env::var("CASSANDRA_ADDRESS").unwrap_or("127.0.0.1".to_string()),
            cass_port: env::var("CASSANDRA_PORT")
                .chain_err(|| "no port specified")
                .and_then(|a| a.parse::<u16>().chain_err(|| "not valid u16"))
                .unwrap_or(9042),
            cass_pool_size: env::var("CASSANDRA_POOL_SIZE")
                .chain_err(|| "no port specified")
                .and_then(|a| a.parse::<u32>().chain_err(|| "not valid u32"))
                .unwrap_or(15),
            referrers: env::var("ALLOWED_REFERRERS")
                .unwrap_or("*".to_string())
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        }
    }
}
