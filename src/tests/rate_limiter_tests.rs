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

#[test]
fn test_execute_available() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION);
    let now = SystemTime::now();
    for _ in 0..(LIMIT_COUNT - 1) {
        limiter.requests.push_back(now);
    }

    // Act
    let now = SystemTime::now();
    let wait = limiter.execute();
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

#[test]
fn test_execute_full() {
    // Arrange
    let mut limiter = RateLimiter::new(LIMIT_COUNT, LIMIT_DURATION_SHORT);
    let now = SystemTime::now();
    for _ in 0..LIMIT_COUNT {
        limiter.requests.push_back(now);
    }

    // Act
    let now = SystemTime::now();
    let wait = limiter.execute();
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
        d1 - d2 <= tolerance
    } else {
        d2 - d1 <= tolerance
    }
}

fn print_duration(name: &str, duration: Option<Duration>) {
    if let Some(duration) = duration {
        println!("{name} duration: {:.3} seconds", duration.as_secs_f64());
    } else {
        println!("{name} duration: None");
    }
}
