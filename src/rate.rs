use std::time::Duration;

/// Rate limit configuration specifying requests allowed per time window
pub struct Rate {
    /// Number of requests allowed per duration.
    pub num: usize,
    /// Duration before the limit resets.
    pub per: Duration,
}
