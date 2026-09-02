use std::time::{Duration, Instant};

/// Helper to measure the execution time of a closure.
pub fn time_execution<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}

/// Calculates throughput (ops per second) given a number of iterations and total duration.
pub fn calculate_throughput(iterations: u64, duration: Duration) -> f64 {
    let secs = duration.as_secs_f64();
    if secs > 0.0 {
        iterations as f64 / secs
    } else {
        0.0
    }
}
