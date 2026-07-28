use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use once_cell::sync::Lazy;

static LOGIN_ATTEMPTS: Lazy<Mutex<HashMap<String, Vec<Instant>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

const WINDOW: Duration = Duration::from_secs(60);
const MAX_FAILURES: usize = 10;

pub fn client_key(addr: Option<SocketAddr>) -> String {
    addr.map(|a| a.ip().to_string())
        .unwrap_or_else(|| "unknown".into())
}

/// Returns false if the client is currently rate-limited.
pub fn allow_attempt(key: &str) -> bool {
    let mut map = LOGIN_ATTEMPTS.lock().unwrap();
    let now = Instant::now();
    let entries = map.entry(key.to_string()).or_default();
    entries.retain(|t| now.duration_since(*t) < WINDOW);
    entries.len() < MAX_FAILURES
}

pub fn record_failure(key: &str) {
    let mut map = LOGIN_ATTEMPTS.lock().unwrap();
    map.entry(key.to_string()).or_default().push(Instant::now());
}

pub fn clear_failures(key: &str) {
    let mut map = LOGIN_ATTEMPTS.lock().unwrap();
    map.remove(key);
}
