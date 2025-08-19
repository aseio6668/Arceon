/*!
# User Profile Management

Cross-platform user profile synchronization and management.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub creation_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
    pub platform_specific_data: HashMap<String, serde_json::Value>,
}

/// Friend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
    pub user_id: String,
    pub display_name: String,
    pub status: FriendStatus,
    pub platform_specific_data: HashMap<String, serde_json::Value>,
}

/// Friend status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FriendStatus {
    Friend,
    Pending,
    Blocked,
    Online,
    Away,
    Busy,
    Offline,
    Unknown,
}

/// Launch options for content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchOptions {
    pub arguments: Vec<String>,
    pub environment: HashMap<String, String>,
    pub working_directory: Option<String>,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            arguments: Vec::new(),
            environment: HashMap::new(),
            working_directory: None,
        }
    }
}