/*!
# Platform Analytics

Cross-platform analytics and telemetry collection.
*/

use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Platform analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformAnalytics {
    pub platform_id: PlatformId,
    pub user_id: String,
    pub session_count: u64,
    pub total_playtime_hours: f64,
    pub achievements_unlocked: u64,
    pub friends_count: u64,
    pub last_activity: Option<chrono::DateTime<chrono::Utc>>,
    pub platform_specific_metrics: HashMap<String, serde_json::Value>,
}

/// Analytics event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event_name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub platform_id: PlatformId,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Analytics collector
pub struct AnalyticsCollector {
    pub events: Vec<AnalyticsEvent>,
    pub platform_analytics: HashMap<PlatformId, PlatformAnalytics>,
}

impl AnalyticsCollector {
    /// Create new analytics collector
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            platform_analytics: HashMap::new(),
        }
    }
    
    /// Track event
    pub fn track_event(&mut self, event: AnalyticsEvent) {
        self.events.push(event);
    }
    
    /// Update platform analytics
    pub fn update_platform_analytics(&mut self, analytics: PlatformAnalytics) {
        self.platform_analytics.insert(analytics.platform_id.clone(), analytics);
    }
    
    /// Get analytics for platform
    pub fn get_platform_analytics(&self, platform_id: &PlatformId) -> Option<&PlatformAnalytics> {
        self.platform_analytics.get(platform_id)
    }
}

impl Default for AnalyticsCollector {
    fn default() -> Self {
        Self::new()
    }
}