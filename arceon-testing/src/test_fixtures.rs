/*!
# Test Fixtures

Reusable test data and setup utilities for consistent testing across all modules.
*/

use uuid::Uuid;
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;

/// Common test data generators
pub struct TestFixtures;

impl TestFixtures {
    /// Generate test player data
    pub fn create_test_player() -> HashMap<String, Value> {
        let mut player = HashMap::new();
        player.insert("id".to_string(), serde_json::json!(Uuid::new_v4()));
        player.insert("username".to_string(), serde_json::json!("TestPlayer"));
        player.insert("level".to_string(), serde_json::json!(1));
        player.insert("experience".to_string(), serde_json::json!(0));
        player.insert("created_at".to_string(), serde_json::json!(Utc::now()));
        player
    }

    /// Generate test guild data
    pub fn create_test_guild() -> HashMap<String, Value> {
        let mut guild = HashMap::new();
        guild.insert("id".to_string(), serde_json::json!(Uuid::new_v4()));
        guild.insert("name".to_string(), serde_json::json!("TestGuild"));
        guild.insert("member_count".to_string(), serde_json::json!(1));
        guild.insert("created_at".to_string(), serde_json::json!(Utc::now()));
        guild
    }

    /// Generate test item data
    pub fn create_test_item() -> HashMap<String, Value> {
        let mut item = HashMap::new();
        item.insert("id".to_string(), serde_json::json!(Uuid::new_v4()));
        item.insert("name".to_string(), serde_json::json!("Test Sword"));
        item.insert("rarity".to_string(), serde_json::json!("Common"));
        item.insert("damage".to_string(), serde_json::json!(10));
        item
    }
}