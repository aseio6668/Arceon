/*!
# Arceon Platform Integration

Provides seamless integration with gaming platforms including:
- Epic Games Store
- Steam
- Generic third-party platform adapters
- Windows launcher support

## Features

### Platform Support
- **Epic Games**: OAuth authentication, user profiles, achievements, DLC management
- **Steam**: Steamworks integration, user authentication, workshop, overlay
- **Generic Platforms**: Flexible adapter system for custom integrations
- **Windows Launcher**: Registry integration, process management, auto-updates

### Integration Types
- **Authentication**: Single sign-on with platform credentials
- **User Profiles**: Import user data, friends lists, achievements
- **DLC/Content**: Manage downloadable content and purchases
- **Social Features**: Friends, communities, messaging
- **Analytics**: Platform-specific telemetry and metrics

## Example Usage

```rust
use arceon_platform_integration::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize platform manager
    let mut platform_manager = PlatformManager::new();
    
    // Register Epic Games integration
    platform_manager.register_platform(Box::new(EpicGamesAdapter::new())).await?;
    
    // Register Steam integration
    platform_manager.register_platform(Box::new(SteamAdapter::new())).await?;
    
    // Authenticate user
    let user = platform_manager.authenticate_user("epic", "user_token").await?;
    
    // Sync achievements
    platform_manager.sync_achievements(&user.platform_id).await?;
    
    Ok(())
}
```
*/

pub mod platform_manager;
pub mod epic_games;
pub mod steam;
pub mod generic_adapter;
pub mod windows_launcher;
pub mod authentication;
pub mod achievements;
pub mod user_profile;
pub mod content_management;
pub mod analytics;
pub mod errors;

// Re-exports
pub use platform_manager::*;
pub use epic_games::*;
pub use steam::*;
pub use generic_adapter::*;
pub use windows_launcher::*;
pub use authentication::*;
pub use achievements::*;
pub use user_profile::*;
pub use content_management::*;
pub use analytics::*;
pub use errors::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main result type for platform integration operations
pub type PlatformResult<T> = Result<T, PlatformError>;

/// Unique identifier for platform integrations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlatformId(pub String);

impl PlatformId {
    pub fn new(platform: &str) -> Self {
        Self(platform.to_string())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PlatformId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for PlatformId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Platform capability flags
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformCapabilities {
    pub authentication: bool,
    pub user_profiles: bool,
    pub achievements: bool,
    pub friends_list: bool,
    pub content_management: bool,
    pub analytics: bool,
    pub overlay_support: bool,
    pub cloud_saves: bool,
    pub voice_chat: bool,
    pub workshop: bool,
}

impl Default for PlatformCapabilities {
    fn default() -> Self {
        Self {
            authentication: true,
            user_profiles: true,
            achievements: false,
            friends_list: false,
            content_management: false,
            analytics: false,
            overlay_support: false,
            cloud_saves: false,
            voice_chat: false,
            workshop: false,
        }
    }
}

/// Platform integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub platform_id: PlatformId,
    pub app_id: String,
    pub api_endpoint: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub sandbox_mode: bool,
    pub capabilities: PlatformCapabilities,
    pub custom_settings: HashMap<String, serde_json::Value>,
}

impl PlatformConfig {
    pub fn new(platform_id: PlatformId, app_id: String) -> Self {
        Self {
            platform_id,
            app_id,
            api_endpoint: None,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            sandbox_mode: false,
            capabilities: PlatformCapabilities::default(),
            custom_settings: HashMap::new(),
        }
    }
    
    pub fn with_oauth(mut self, client_id: String, client_secret: String, redirect_uri: String) -> Self {
        self.client_id = Some(client_id);
        self.client_secret = Some(client_secret);
        self.redirect_uri = Some(redirect_uri);
        self
    }
    
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.api_endpoint = Some(endpoint);
        self
    }
    
    pub fn with_capabilities(mut self, capabilities: PlatformCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }
    
    pub fn sandbox_mode(mut self) -> Self {
        self.sandbox_mode = true;
        self
    }
    
    pub fn set_custom_setting<T: Serialize>(&mut self, key: &str, value: T) -> Result<()> {
        self.custom_settings.insert(key.to_string(), serde_json::to_value(value)?);
        Ok(())
    }
    
    pub fn get_custom_setting<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        if let Some(value) = self.custom_settings.get(key) {
            Ok(Some(serde_json::from_value(value.clone())?))
        } else {
            Ok(None)
        }
    }
}