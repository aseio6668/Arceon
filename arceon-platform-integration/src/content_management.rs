/*!
# Content Management

Platform content and DLC management system.
*/

use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Content item (DLC, expansions, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content_type: ContentType,
    pub price: Option<Price>,
    pub owned: bool,
    pub installed: bool,
    pub download_size: Option<u64>,
    pub install_path: Option<String>,
    pub platform_specific_data: HashMap<String, serde_json::Value>,
}

/// Content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    DLC,
    Expansion,
    Season,
    Cosmetic,
    Character,
    Map,
    Weapon,
    Other(String),
}

/// Price information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub amount: f64,
    pub currency: String,
    pub formatted: String,
}

/// Content library management
pub struct ContentLibrary {
    pub items: Vec<ContentItem>,
    pub platform_libraries: HashMap<PlatformId, Vec<ContentItem>>,
}

impl ContentLibrary {
    /// Create new content library
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            platform_libraries: HashMap::new(),
        }
    }
    
    /// Add content item
    pub fn add_content(&mut self, content: ContentItem) {
        self.items.push(content);
    }
    
    /// Get owned content
    pub fn get_owned_content(&self) -> Vec<&ContentItem> {
        self.items.iter().filter(|item| item.owned).collect()
    }
    
    /// Get installed content
    pub fn get_installed_content(&self) -> Vec<&ContentItem> {
        self.items.iter().filter(|item| item.installed).collect()
    }
}

impl Default for ContentLibrary {
    fn default() -> Self {
        Self::new()
    }
}