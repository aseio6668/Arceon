/*!
# Mock Services

Test doubles and mock implementations for external services and dependencies.
*/

use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug)]
pub struct MockServiceManager {
    pub services: HashMap<String, MockService>,
}

#[derive(Debug)]
pub struct MockService {
    pub service_name: String,
    pub mock_type: MockType,
    pub responses: HashMap<String, MockResponse>,
}

#[derive(Debug)]
pub enum MockType {
    HTTP,
    Database,
    MessageQueue,
    Cache,
}

#[derive(Debug)]
pub struct MockResponse {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl MockServiceManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn add_mock_service(&mut self, service: MockService) {
        self.services.insert(service.service_name.clone(), service);
    }

    pub async fn start_all_mocks(&self) -> Result<()> {
        // Start all mock services
        Ok(())
    }

    pub async fn stop_all_mocks(&self) -> Result<()> {
        // Stop all mock services
        Ok(())
    }
}