use crate::Rate;
use colored::Colorize;
use log::trace;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct RateLimiter {
    pub(crate) rate: Rate,
    pub(crate) requests: VecDeque<SystemTime>,
}

impl RateLimiter {
    /// Create a new [`RateLimiter`]
    #[must_use]
    pub fn new(num: usize, per: Duration) -> Self {
        Self {
            rate: Rate { num, per },
            requests: VecDeque::new(),
        }
    }

    /// Wait if required then execute
    ///
    /// Returns `None` if there was no wait, else the duration.
    pub fn execute(&mut self) -> Option<Duration> {
        let wait_duration = self.get_wait_duration();
        if let Some(wait) = wait_duration {
            trace!(
                "{} {:.3} for rate limiter",
                "Waiting".bold(),
                wait.as_secs_f64()
            );
            sleep(wait);
            self.requests.pop_front();
        }
        self.requests.push_back(SystemTime::now());
        wait_duration
    }

    /// Get the duration to wait before a request can be made.
    ///
    /// Returns `None` if no wait, else the duration.
    pub fn get_wait_duration(&mut self) -> Option<Duration> {
        if self.requests.len() < self.rate.num {
            return None;
        }
        self.remove_stale();
        if self.requests.len() < self.rate.num {
            return None;
        }
        let request = self.requests.front()?;
        let elapsed = request.elapsed().expect("elapsed should not fail");
        if elapsed > self.rate.per {
            return None;
        }
        Some(self.rate.per - elapsed)
    }

    /// Remove requests older than the rate duration.
    fn remove_stale(&mut self) {
        let cutoff = SystemTime::now() - self.rate.per;
        self.requests.retain(|&request| request > cutoff);
    }
}
