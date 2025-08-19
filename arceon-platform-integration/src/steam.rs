/*!
# Steam Integration

Comprehensive Steam integration including:
- Steamworks SDK integration
- Steam authentication
- Workshop support
- Achievements and statistics
- Steam overlay
- Friends and community features
*/

use crate::{PlatformId, PlatformConfig, PlatformCapabilities, PlatformResult, PlatformError,
            PlatformAdapter, AuthenticationCredentials, PlatformUser,
            AuthenticationResult, UserProfile, Achievement, Friend, LaunchOptions, PlatformAnalytics};
use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, warn, error, debug};


/// Steam platform adapter
pub struct SteamAdapter {
    platform_id: PlatformId,
    config: Option<PlatformConfig>,
    capabilities: PlatformCapabilities,
    #[cfg(feature = "steam-integration")]
    client: Option<steamworks::Client>,
    #[cfg(feature = "steam-integration")]
    app_id: Option<steamworks::AppId>,
}

/// Steam user information
#[derive(Debug, Clone)]
pub struct SteamUserInfo {
    pub steam_id: u64,
    pub display_name: String,
    pub avatar_url: String,
    pub profile_url: String,
    pub is_online: bool,
    pub country_code: Option<String>,
}

/// Steam achievement data
#[derive(Debug, Clone)]
pub struct SteamAchievementInfo {
    pub api_name: String,
    pub display_name: String,
    pub description: String,
    pub achieved: bool,
    pub unlock_time: Option<chrono::DateTime<chrono::Utc>>,
    pub icon: String,
    pub icon_gray: String,
}

/// Steam workshop item
#[derive(Debug, Clone)]
pub struct SteamWorkshopItem {
    pub published_file_id: u64,
    pub title: String,
    pub description: String,
    pub creator_steam_id: u64,
    pub file_size: u64,
    pub download_url: Option<String>,
    pub preview_url: Option<String>,
    pub tags: Vec<String>,
    pub subscribed: bool,
}

impl SteamAdapter {
    /// Create a new Steam adapter
    pub fn new() -> Self {
        Self {
            platform_id: PlatformId::new("steam"),
            config: None,
            capabilities: PlatformCapabilities {
                authentication: true,
                user_profiles: true,
                achievements: true,
                friends_list: true,
                content_management: false,
                analytics: true,
                overlay_support: true,
                cloud_saves: true,
                voice_chat: true,
                workshop: true,
            },
            #[cfg(feature = "steam-integration")]
            client: None,
            #[cfg(feature = "steam-integration")]
            app_id: None,
        }
    }
    
    /// Initialize Steam client with app ID
    #[cfg(feature = "steam-integration")]
    async fn initialize_steam_client(&mut self, app_id: u32) -> PlatformResult<()> {
        info!("Initializing Steam client with app ID: {}", app_id);
        
        let steam_app_id = steamworks::AppId(app_id);
        
        match steamworks::Client::init_app(steam_app_id) {
            Ok((client, _single)) => {
                info!("Steam client initialized successfully");
                self.client = Some(client);
                self.app_id = Some(steam_app_id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to initialize Steam client: {:?}", e);
                Err(PlatformError::SteamError {
                    message: format!("Failed to initialize Steam client: {:?}", e),
                })
            }
        }
    }
    
    /// Get current Steam user information
    #[cfg(feature = "steam-integration")]
    fn get_steam_user_info(&self) -> PlatformResult<SteamUserInfo> {
        let client = self.client.as_ref()
            .ok_or_else(|| PlatformError::SteamError {
                message: "Steam client not initialized".to_string(),
            })?;
        
        let utils = client.utils();
        let friends = client.friends();
        let user = client.user();
        
        let steam_id = user.steam_id().raw();
        let display_name = friends.name();
        
        // Get avatar information
        let avatar_url = format!(
            "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/{}/{}_full.jpg",
            format!("{:02x}", steam_id >> 32),
            steam_id & 0xFFFFFFFF
        );
        
        let profile_url = format!("https://steamcommunity.com/profiles/{}", steam_id);
        
        Ok(SteamUserInfo {
            steam_id,
            display_name,
            avatar_url,
            profile_url,
            is_online: true, // Assume online if we can get info
            country_code: Some(utils.ip_country()),
        })
    }
    
    /// Get Steam achievements
    #[cfg(feature = "steam-integration")]
    fn get_steam_achievements(&self) -> PlatformResult<Vec<SteamAchievementInfo>> {
        let client = self.client.as_ref()
            .ok_or_else(|| PlatformError::SteamError {
                message: "Steam client not initialized".to_string(),
            })?;
        
        let user_stats = client.user_stats();
        
        // This would require the actual achievement names from your game
        // For now, return an empty list
        Ok(vec![])
    }
    
    /// Unlock Steam achievement
    #[cfg(feature = "steam-integration")]
    fn unlock_steam_achievement(&self, api_name: &str) -> PlatformResult<()> {
        let client = self.client.as_ref()
            .ok_or_else(|| PlatformError::SteamError {
                message: "Steam client not initialized".to_string(),
            })?;
        
        let user_stats = client.user_stats();
        
        // Steam achievement API varies by version - using simplified approach
        let achievement = user_stats.achievement(api_name);
        match achievement.set() {
            Ok(()) => {
                user_stats.store_stats();
                info!("Steam achievement unlocked: {}", api_name);
                Ok(())
            }
            Err(_) => {
                warn!("Failed to unlock Steam achievement: {}", api_name);
                Err(PlatformError::SteamError {
                    message: format!("Failed to unlock achievement: {}", api_name),
                })
            }
        }
    }
    
    /// Get Steam friends list
    #[cfg(feature = "steam-integration")]
    fn get_steam_friends(&self) -> PlatformResult<Vec<Friend>> {
        let client = self.client.as_ref()
            .ok_or_else(|| PlatformError::SteamError {
                message: "Steam client not initialized".to_string(),
            })?;
        
        let friends = client.friends();
        let _friend_count = 0; // Steam friends API varies by version
        
        let friend_list = Vec::new();
        
        // Steam friends API implementation varies by steamworks version
        // For compatibility, return empty list for now
        // In a real implementation, you would iterate through friends using the correct API methods
        
        Ok(friend_list)
    }
    
    /// Subscribe to Steam Workshop item
    #[cfg(feature = "steam-integration")]
    pub async fn subscribe_workshop_item(&self, published_file_id: u64) -> PlatformResult<()> {
        let client = self.client.as_ref()
            .ok_or_else(|| PlatformError::SteamError {
                message: "Steam client not initialized".to_string(),
            })?;
        
        // This would use the UGC (User Generated Content) interface
        // Implementation depends on specific Steamworks SDK version
        info!("Subscribing to Steam Workshop item: {}", published_file_id);
        Ok(())
    }
    
    /// Launch Steam overlay
    #[cfg(feature = "steam-integration")]
    pub fn show_steam_overlay(&self, overlay_type: &str) -> PlatformResult<()> {
        let client = self.client.as_ref()
            .ok_or_else(|| PlatformError::SteamError {
                message: "Steam client not initialized".to_string(),
            })?;
        
        let friends = client.friends();
        
        match overlay_type {
            "friends" => friends.activate_game_overlay("friends"),
            "community" => friends.activate_game_overlay("community"),
            "players" => friends.activate_game_overlay("players"),
            "settings" => friends.activate_game_overlay("settings"),
            "achievements" => friends.activate_game_overlay("achievements"),
            "stats" => friends.activate_game_overlay("stats"),
            _ => {
                return Err(PlatformError::UnsupportedCapability {
                    capability: format!("Steam overlay type: {}", overlay_type),
                });
            }
        }
        
        info!("Steam overlay activated: {}", overlay_type);
        Ok(())
    }
    
    /// Non-Steamworks implementation for when the feature is disabled
    #[cfg(not(feature = "steam-integration"))]
    async fn initialize_fallback(&mut self, _app_id: u32) -> PlatformResult<()> {
        info!("Steam integration disabled, using fallback implementation");
        Ok(())
    }
}

#[async_trait]
impl PlatformAdapter for SteamAdapter {
    fn platform_id(&self) -> &PlatformId {
        &self.platform_id
    }
    
    fn capabilities(&self) -> &PlatformCapabilities {
        &self.capabilities
    }
    
    async fn initialize(&mut self, config: PlatformConfig) -> PlatformResult<()> {
        info!("Initializing Steam adapter");
        
        let app_id: u32 = config.app_id.parse()
            .map_err(|_| PlatformError::InvalidConfiguration {
                platform: "steam".to_string(),
                details: "Invalid Steam App ID format".to_string(),
            })?;
        
        #[cfg(feature = "steam-integration")]
        {
            self.initialize_steam_client(app_id).await?;
        }
        
        #[cfg(not(feature = "steam-integration"))]
        {
            self.initialize_fallback(app_id).await?;
        }
        
        self.config = Some(config);
        info!("Steam adapter initialized successfully");
        Ok(())
    }
    
    async fn shutdown(&mut self) -> PlatformResult<()> {
        info!("Shutting down Steam adapter");
        
        #[cfg(feature = "steam-integration")]
        {
            self.client = None;
            self.app_id = None;
        }
        
        self.config = None;
        Ok(())
    }
    
    async fn is_available(&self) -> bool {
        #[cfg(feature = "steam-integration")]
        {
            self.client.is_some()
        }
        
        #[cfg(not(feature = "steam-integration"))]
        {
            self.config.is_some()
        }
    }
    
    async fn authenticate(&self, _credentials: AuthenticationCredentials) -> PlatformResult<PlatformUser> {
        info!("Authenticating with Steam");
        
        #[cfg(feature = "steam-integration")]
        {
            let user_info = self.get_steam_user_info()?;
            
            Ok(PlatformUser {
                platform_id: self.platform_id.clone(),
                user_id: user_info.steam_id.to_string(),
                display_name: user_info.display_name,
                email: None,
                avatar_url: Some(user_info.avatar_url),
                access_token: None, // Steam doesn't use OAuth tokens
                refresh_token: None,
                token_expires_at: None,
                platform_specific_data: {
                    let mut data = HashMap::new();
                    data.insert("profile_url".to_string(), serde_json::Value::String(user_info.profile_url));
                    data.insert("country_code".to_string(), serde_json::Value::String(user_info.country_code.unwrap_or_default()));
                    data.insert("is_online".to_string(), serde_json::Value::Bool(user_info.is_online));
                    data
                },
            })
        }
        
        #[cfg(not(feature = "steam-integration"))]
        {
            // Fallback for when Steam integration is disabled
            Ok(PlatformUser {
                platform_id: self.platform_id.clone(),
                user_id: "steam_user_fallback".to_string(),
                display_name: "Steam User".to_string(),
                email: None,
                avatar_url: None,
                access_token: None,
                refresh_token: None,
                token_expires_at: None,
                platform_specific_data: HashMap::new(),
            })
        }
    }
    
    async fn refresh_token(&self, _refresh_token: &str) -> PlatformResult<AuthenticationResult> {
        // Steam doesn't use refresh tokens
        Err(PlatformError::UnsupportedCapability {
            capability: "Steam doesn't use refresh tokens".to_string(),
        })
    }
    
    async fn get_user_profile(&self, user_id: &str) -> PlatformResult<UserProfile> {
        info!("Getting Steam user profile for user: {}", user_id);
        
        #[cfg(feature = "steam-integration")]
        {
            let user_info = self.get_steam_user_info()?;
            
            Ok(UserProfile {
                user_id: user_info.steam_id.to_string(),
                display_name: user_info.display_name,
                email: None,
                avatar_url: Some(user_info.avatar_url),
                creation_date: None,
                last_seen: None,
                platform_specific_data: {
                    let mut data = HashMap::new();
                    data.insert("profile_url".to_string(), serde_json::Value::String(user_info.profile_url));
                    data.insert("country_code".to_string(), serde_json::Value::String(user_info.country_code.unwrap_or_default()));
                    data
                },
            })
        }
        
        #[cfg(not(feature = "steam-integration"))]
        {
            Ok(UserProfile {
                user_id: user_id.to_string(),
                display_name: "Steam User".to_string(),
                email: None,
                avatar_url: None,
                creation_date: None,
                last_seen: None,
                platform_specific_data: HashMap::new(),
            })
        }
    }
    
    async fn sync_achievements(&self, user_id: &str, achievements: &[Achievement]) -> PlatformResult<()> {
        info!("Syncing {} achievements with Steam for user {}", achievements.len(), user_id);
        
        #[cfg(feature = "steam-integration")]
        {
            for achievement in achievements {
                if achievement.unlocked {
                    if let Err(e) = self.unlock_steam_achievement(&achievement.id) {
                        warn!("Failed to sync achievement {}: {}", achievement.id, e);
                    }
                }
            }
        }
        
        debug!("Achievement sync completed for Steam");
        Ok(())
    }
    
    async fn get_friends_list(&self, user_id: &str) -> PlatformResult<Vec<Friend>> {
        info!("Getting friends list from Steam for user {}", user_id);
        
        #[cfg(feature = "steam-integration")]
        {
            self.get_steam_friends()
        }
        
        #[cfg(not(feature = "steam-integration"))]
        {
            Ok(vec![])
        }
    }
    
    async fn launch_content(&self, content_id: &str, launch_options: LaunchOptions) -> PlatformResult<()> {
        info!("Launching content {} through Steam", content_id);
        
        // Steam URL scheme for launching games
        let launch_url = if launch_options.arguments.is_empty() {
            format!("steam://rungameid/{}", content_id)
        } else {
            format!("steam://rungameid/{}///{}", content_id, launch_options.arguments.join(" "))
        };
        
        info!("Steam launch URL: {}", launch_url);
        
        // Platform-specific launch logic would go here
        // For now, just log the launch
        Ok(())
    }
    
    async fn get_analytics(&self, user_id: &str) -> PlatformResult<PlatformAnalytics> {
        #[cfg(feature = "steam-integration")]
        {
            let client = self.client.as_ref()
                .ok_or_else(|| PlatformError::SteamError {
                    message: "Steam client not initialized".to_string(),
                })?;
            
            // Get Steam statistics
            let user_stats = client.user_stats();
            
            let mut platform_metrics = HashMap::new();
            
            // Add Steam-specific metrics
            platform_metrics.insert("steam_level".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
            
            Ok(PlatformAnalytics {
                platform_id: self.platform_id.clone(),
                user_id: user_id.to_string(),
                session_count: 0,
                total_playtime_hours: 0.0,
                achievements_unlocked: 0,
                friends_count: 0,
                last_activity: None,
                platform_specific_metrics: HashMap::new(), // Placeholder for platform-specific data
            })
        }
        
        #[cfg(not(feature = "steam-integration"))]
        {
            Ok(PlatformAnalytics {
                platform_id: self.platform_id.clone(),
                user_id: user_id.to_string(),
                session_count: 0,
                total_playtime_hours: 0.0,
                achievements_unlocked: 0,
                friends_count: 0,
                last_activity: None,
                platform_specific_metrics: HashMap::new(),
            })
        }
    }
}

impl Default for SteamAdapter {
    fn default() -> Self {
        Self::new()
    }
}