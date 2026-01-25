use crate::Rate;
use colored::Colorize;
use log::trace;
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// A sliding window rate limiter for throttling API requests.
///
/// Tracks request timestamps in a queue and enforces a maximum number of
/// requests within a sliding time window. When the limit is reached, callers
/// are delayed until the oldest request falls outside the window.
///
/// Uses interior mutability via [`tokio::sync::Mutex`] to allow shared
/// access (`&self`) across async tasks. Locks are held only for brief
/// in-memory operations, never across await points or sleeps.
pub struct RateLimiter {
    pub(crate) rate: Rate,
    pub(crate) requests: Mutex<VecDeque<SystemTime>>,
}

impl RateLimiter {
    /// Create a new [`RateLimiter`]
    #[must_use]
    pub fn new(num: usize, per: Duration) -> Self {
        Self {
            rate: Rate { num, per },
            requests: Mutex::new(VecDeque::new()),
        }
    }

    /// Wait if required then execute
    ///
    /// Returns `None` if there was no wait, else the duration.
    pub async fn execute(&self) -> Option<Duration> {
        let wait_duration = self.get_wait_duration().await;
        if let Some(wait) = wait_duration {
            trace!(
                "{} {:.3} for rate limiter",
                "Waiting".bold(),
                wait.as_secs_f64()
            );
            sleep(wait).await;
            self.requests.lock().await.pop_front();
        }
        self.requests.lock().await.push_back(SystemTime::now());
        wait_duration
    }

    /// Get the duration to wait before a request can be made.
    ///
    /// Returns `None` if no wait, else the duration.
    pub async fn get_wait_duration(&self) -> Option<Duration> {
        let mut requests = self.requests.lock().await;
        if requests.len() < self.rate.num {
            return None;
        }
        Self::remove_stale(&mut requests, self.rate.per);
        if requests.len() < self.rate.num {
            return None;
        }
        let request = requests.front()?;
        let elapsed = request.elapsed().expect("elapsed should not fail");
        if elapsed > self.rate.per {
            return None;
        }
        Some(
            self.rate
                .per
                .checked_sub(elapsed)
                .expect("duration should not overflow"),
        )
    }

    /// Remove requests older than the rate duration.
    fn remove_stale(requests: &mut VecDeque<SystemTime>, per: Duration) {
        let cutoff = SystemTime::now() - per;
        requests.retain(|&request| request > cutoff);
    }
}
