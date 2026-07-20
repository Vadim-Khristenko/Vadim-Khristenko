//! Shared HTTP retry with backoff + rate-limit awareness.
//!
//! Every provider sends through `send_retrying`, so a transient failure (network
//! blip, 429, 5xx, or a GitHub secondary rate limit) rides out a short backoff
//! instead of silently dropping a card. Waits honor `Retry-After` and
//! `X-RateLimit-Reset`, but are capped so a CI run never hangs on a long limit —
//! past the cap it returns the response and the caller degrades gracefully.

use anyhow::{anyhow, Result};
use reqwest::blocking::{RequestBuilder, Response};
use std::thread::sleep;
use std::time::Duration;

use crate::log;

const MAX_ATTEMPTS: u32 = 4;
const MAX_WAIT_SECS: u64 = 30;

/// Send a request, retrying transient failures with exponential backoff.
pub fn send_retrying(req: RequestBuilder) -> Result<Response> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        let this = req
            .try_clone()
            .ok_or_else(|| anyhow!("request body not cloneable — cannot retry"))?;
        match this.send() {
            Ok(resp) => {
                let code = resp.status().as_u16();
                let retryable = code == 429
                    || resp.status().is_server_error()
                    || (code == 403 && rate_limited(&resp));
                if retryable && attempt < MAX_ATTEMPTS {
                    let wait = retry_after(&resp).unwrap_or_else(|| backoff(attempt));
                    log::warn(&format!(
                        "HTTP {code} — retry {attempt}/{} in {}s",
                        MAX_ATTEMPTS - 1,
                        wait.as_secs()
                    ));
                    sleep(wait);
                    continue;
                }
                return Ok(resp);
            }
            Err(e) if attempt < MAX_ATTEMPTS => {
                let wait = backoff(attempt);
                log::warn(&format!(
                    "request failed ({e}) — retry {attempt}/{} in {}s",
                    MAX_ATTEMPTS - 1,
                    wait.as_secs()
                ));
                sleep(wait);
            }
            Err(e) => return Err(e.into()),
        }
    }
}

/// 1s, 2s, 4s, … capped at MAX_WAIT_SECS.
fn backoff(attempt: u32) -> Duration {
    let secs = 1u64.checked_shl(attempt - 1).unwrap_or(MAX_WAIT_SECS);
    Duration::from_secs(secs.min(MAX_WAIT_SECS))
}

fn rate_limited(resp: &Response) -> bool {
    resp.headers()
        .get("x-ratelimit-remaining")
        .and_then(|h| h.to_str().ok())
        == Some("0")
}

/// Prefer an explicit `Retry-After` (seconds); fall back to the epoch
/// `X-RateLimit-Reset` when a primary limit is exhausted. Always capped.
fn retry_after(resp: &Response) -> Option<Duration> {
    let h = resp.headers();
    if let Some(secs) = h
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
    {
        return Some(Duration::from_secs(secs.min(MAX_WAIT_SECS)));
    }
    if rate_limited(resp) {
        if let Some(reset) = h
            .get("x-ratelimit-reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.trim().parse::<i64>().ok())
        {
            let delta = (reset - chrono::Utc::now().timestamp()).max(0) as u64;
            return Some(Duration::from_secs(delta.min(MAX_WAIT_SECS)));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_is_exponential_and_capped() {
        assert_eq!(backoff(1), Duration::from_secs(1));
        assert_eq!(backoff(2), Duration::from_secs(2));
        assert_eq!(backoff(3), Duration::from_secs(4));
        assert_eq!(backoff(99), Duration::from_secs(MAX_WAIT_SECS));
    }
}
