use std::time::Duration;

pub struct Rate {
    /// Number of requests allowed per duration.
    pub num: usize,
    /// Duration before the limit resets.
    pub per: Duration,
}
