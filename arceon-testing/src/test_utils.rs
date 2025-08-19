/*!
# Test Utilities

Helper functions and utilities for test execution and validation.
*/

use anyhow::Result;
use std::time::{Duration, Instant};

/// Test execution utilities
pub struct TestUtils;

impl TestUtils {
    /// Wait for condition with timeout
    pub async fn wait_for_condition<F>(
        condition: F,
        timeout: Duration,
        check_interval: Duration,
    ) -> Result<bool>
    where
        F: Fn() -> bool,
    {
        let start = Instant::now();
        
        while start.elapsed() < timeout {
            if condition() {
                return Ok(true);
            }
            tokio::time::sleep(check_interval).await;
        }
        
        Ok(false)
    }

    /// Generate random test data
    pub fn generate_random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789";
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Measure execution time
    pub async fn measure_execution_time<F, Fut, T>(f: F) -> (T, Duration)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        (result, duration)
    }
}