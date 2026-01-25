use crate::RateLimiter;
use std::time::{Duration, SystemTime};

const LIMIT_COUNT: usize = 5;
const LIMIT_DURATION: Duration = Duration::from_secs(10);
const LIMIT_DURATION_SHORT: Duration = Duration::from_millis(250);

#[test]
fn test_get_wait_duration_empty() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION);

    // Act
    let wait = limiter.get_wait_duration();
    print_duration("Wait", wait);

    // Assert
    assert!(wait.is_none());
}

#[test]
fn test_get_wait_duration_available() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION);
    let now = SystemTime::now();
    for _ in 0..(LIMIT_COUNT - 1) {
        limiter.requests.push_back(now);
    }

    // Act
    let wait = limiter.get_wait_duration();
    print_duration("Wait", wait);

    // Assert
    assert!(wait.is_none());
}

#[test]
fn test_get_wait_duration_full() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION);
    let now = SystemTime::now();
    for _ in 0..LIMIT_COUNT {
        limiter.requests.push_back(now);
    }

    // Act
    let wait = limiter.get_wait_duration();
    print_duration("Wait", wait);

    // Assert
    assert!(wait.is_some());
    assert!(approximately_equals(
        wait.unwrap(),
        LIMIT_DURATION,
        Duration::from_millis(100)
    ));
}

#[tokio::test]
async fn test_execute_available() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION);
    let now = SystemTime::now();
    for _ in 0..(LIMIT_COUNT - 1) {
        limiter.requests.push_back(now);
    }

    // Act
    let now = SystemTime::now();
    let wait = limiter.execute().await;
    let elapsed = now.elapsed().expect("elapsed should not fail");
    print_duration("Wait", wait);
    print_duration("Elapsed", Some(elapsed));

    // Assert
    assert!(wait.is_none());
    assert!(approximately_equals(
        elapsed,
        Duration::from_secs(0),
        Duration::from_millis(50)
    ));
}

#[tokio::test]
async fn test_execute_full() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION_SHORT);
    let now = SystemTime::now();
    for _ in 0..LIMIT_COUNT {
        limiter.requests.push_back(now);
    }

    // Act
    let now = SystemTime::now();
    let wait = limiter.execute().await;
    let elapsed = now.elapsed().expect("elapsed should not fail");
    print_duration("Wait", wait);
    print_duration("Elapsed", Some(elapsed));

    // Assert
    assert!(wait.is_some());
    assert!(approximately_equals(
        wait.unwrap(),
        LIMIT_DURATION_SHORT,
        Duration::from_millis(50)
    ));
    assert!(approximately_equals(
        elapsed,
        LIMIT_DURATION_SHORT,
        Duration::from_millis(50)
    ));
}

fn approximately_equals(d1: Duration, d2: Duration, tolerance: Duration) -> bool {
    if d1 > d2 {
        d1.checked_sub(d2).unwrap() <= tolerance
    } else {
        d2.checked_sub(d1).unwrap() <= tolerance
    }
}

fn print_duration(name: &str, duration: Option<Duration>) {
    if let Some(duration) = duration {
        println!("{name} duration: {:.3} seconds", duration.as_secs_f64());
    } else {
        println!("{name} duration: None");
    }
}

// Edge case tests

#[test]
fn test_constructor_initializes_empty_queue() {
    // Arrange & Act
    let limiter = RateLimiter::new(10, Duration::from_secs(60));

    // Assert
    assert!(limiter.requests.is_empty());
    assert_eq!(limiter.rate.num, 10);
    assert_eq!(limiter.rate.per, Duration::from_secs(60));
}

#[test]
fn test_remove_stale_clears_old_requests() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, Duration::from_millis(50));
    let now = SystemTime::now();
    // Add requests that are already stale
    let old_time = now - Duration::from_millis(100);
    for _ in 0..LIMIT_COUNT {
        limiter.requests.push_back(old_time);
    }
    assert_eq!(limiter.requests.len(), LIMIT_COUNT);

    // Act - get_wait_duration calls remove_stale internally
    let wait = limiter.get_wait_duration();

    // Assert - stale requests should be removed, no wait needed
    assert!(wait.is_none());
    assert!(limiter.requests.is_empty());
}

#[test]
fn test_partial_stale_removal() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, Duration::from_millis(100));
    let now = SystemTime::now();

    // Add some stale and some fresh requests
    let stale_time = now - Duration::from_millis(200);
    for _ in 0..3 {
        limiter.requests.push_back(stale_time);
    }
    for _ in 0..2 {
        limiter.requests.push_back(now);
    }
    assert_eq!(limiter.requests.len(), 5);

    // Act
    let wait = limiter.get_wait_duration();

    // Assert - stale removed, only fresh remain, under limit so no wait
    assert!(wait.is_none());
    assert_eq!(limiter.requests.len(), 2);
}

#[tokio::test]
async fn test_execute_adds_request_to_queue() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION);
    assert!(limiter.requests.is_empty());

    // Act
    limiter.execute().await;

    // Assert
    assert_eq!(limiter.requests.len(), 1);
}

#[tokio::test]
async fn test_multiple_executes_fill_queue() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION);

    // Act
    for _ in 0..(LIMIT_COUNT - 1) {
        limiter.execute().await;
    }

    // Assert - queue should have LIMIT_COUNT - 1 entries
    assert_eq!(limiter.requests.len(), LIMIT_COUNT - 1);

    // Act - one more should not require waiting
    let wait = limiter.execute().await;

    // Assert
    assert!(wait.is_none());
    assert_eq!(limiter.requests.len(), LIMIT_COUNT);
}
