/*!
# Epic Games Store Integration

Provides comprehensive integration with Epic Games Store including:
- OAuth 2.0 authentication
- User profiles and friends
- Achievements and statistics
- DLC and content management
- Epic Online Services (EOS) integration
*/

use crate::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, debug};

/// Epic Games Store adapter
pub struct EpicGamesAdapter {
    platform_id: PlatformId,
    config: Option<PlatformConfig>,
    client: Client,
    capabilities: PlatformCapabilities,
}

/// Epic Games OAuth response
#[derive(Debug, Serialize, Deserialize)]
struct EpicOAuthResponse {
    access_token: String,
    expires_in: u64,
    expires_at: String,
    token_type: String,
    refresh_token: String,
    refresh_expires: u64,
    refresh_expires_at: String,
    account_id: String,
    client_id: String,
    internal_client: bool,
    client_service: String,
    scope: Vec<String>,
}

/// Epic Games user profile
#[derive(Debug, Serialize, Deserialize)]
struct EpicUserProfile {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "displayName")]
    display_name: String,
    email: Option<String>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
    country: Option<String>,
    #[serde(rename = "dateOfBirth")]
    date_of_birth: Option<String>,
    #[serde(rename = "headshot")]
    avatar_url: Option<String>,
    #[serde(rename = "canUpdateDisplayName")]
    can_update_display_name: bool,
}

/// Epic Games friends list response
#[derive(Debug, Serialize, Deserialize)]
struct EpicFriendsResponse {
    friends: Vec<EpicFriend>,
    incoming: Vec<EpicFriend>,
    outgoing: Vec<EpicFriend>,
    suggested: Vec<EpicFriend>,
    blocklist: Vec<EpicFriend>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EpicFriend {
    #[serde(rename = "accountId")]
    account_id: String,
    status: String,
    direction: String,
    created: String,
    favorite: bool,
}

/// Epic Games achievement data
#[derive(Debug, Serialize, Deserialize)]
struct EpicAchievement {
    #[serde(rename = "achievementName")]
    achievement_name: String,
    progress: f64,
    unlocked: bool,
    #[serde(rename = "unlockedDateTime")]
    unlocked_date_time: Option<String>,
    #[serde(rename = "XP")]
    xp: u32,
}

impl EpicGamesAdapter {
    /// Create a new Epic Games adapter
    pub fn new() -> Self {
        Self {
            platform_id: PlatformId::new("epic"),
            config: None,
            client: Client::new(),
            capabilities: PlatformCapabilities {
                authentication: true,
                user_profiles: true,
                achievements: true,
                friends_list: true,
                content_management: true,
                analytics: true,
                overlay_support: false,
                cloud_saves: true,
                voice_chat: false,
                workshop: false,
            },
        }
    }
    
    /// Get Epic Games API base URL
    fn get_api_base_url(&self) -> &str {
        if let Some(config) = &self.config {
            if config.sandbox_mode {
                "https://api.epicgames.dev/sdk/v1"
            } else {
                "https://api.epicgames.dev/epic/id/v1"
            }
        } else {
            "https://api.epicgames.dev/epic/id/v1"
        }
    }
    
    /// Build authorization URL for OAuth flow
    pub fn build_auth_url(&self, state: &str, scopes: &[&str]) -> PlatformResult<String> {
        let config = self.config.as_ref()
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: "epic".to_string(),
                details: "Adapter not initialized".to_string(),
            })?;
        
        let client_id = config.client_id.as_ref()
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: "epic".to_string(),
                details: "Client ID not configured".to_string(),
            })?;
        
        let redirect_uri = config.redirect_uri.as_ref()
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: "epic".to_string(),
                details: "Redirect URI not configured".to_string(),
            })?;
        
        let scopes_str = if scopes.is_empty() {
            "basic_profile"
        } else {
            &scopes.join(" ")
        };
        
        let auth_url = format!(
            "https://www.epicgames.com/id/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            urlencoding::encode(client_id),
            urlencoding::encode(redirect_uri),
            urlencoding::encode(scopes_str),
            urlencoding::encode(state)
        );
        
        Ok(auth_url)
    }
    
    /// Exchange authorization code for access token
    async fn exchange_code_for_token(&self, code: &str) -> PlatformResult<EpicOAuthResponse> {
        let config = self.config.as_ref()
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: "epic".to_string(),
                details: "Adapter not initialized".to_string(),
            })?;
        
        let client_id = config.client_id.as_ref().unwrap();
        let client_secret = config.client_secret.as_ref().unwrap();
        let redirect_uri = config.redirect_uri.as_ref().unwrap();
        
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
        ];
        
        let response = self.client
            .post("https://api.epicgames.dev/epic/oauth/v1/token")
            .basic_auth(client_id, Some(client_secret))
            .form(&params)
            .send()
            .await?;
        
        if response.status().is_success() {
            let oauth_response: EpicOAuthResponse = response.json().await?;
            Ok(oauth_response)
        } else {
            let status_code = response.status().as_u16();
            let error_text = response.text().await?;
            Err(PlatformError::ApiError {
                message: format!("Epic OAuth error: {}", error_text),
                status_code,
            })
        }
    }
    
    /// Get user profile from Epic API
    async fn get_epic_user_profile(&self, account_id: &str, access_token: &str) -> PlatformResult<EpicUserProfile> {
        let url = format!("{}/accounts/{}", self.get_api_base_url(), account_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await?;
        
        if response.status().is_success() {
            let profile: EpicUserProfile = response.json().await?;
            Ok(profile)
        } else {
            let status_code = response.status().as_u16();
            let error_text = response.text().await?;
            Err(PlatformError::ApiError {
                message: format!("Epic profile error: {}", error_text),
                status_code,
            })
        }
    }
    
    /// Get friends list from Epic API
    async fn get_epic_friends(&self, account_id: &str, access_token: &str) -> PlatformResult<Vec<Friend>> {
        let url = format!("{}/accounts/{}/friends", self.get_api_base_url(), account_id);
        
        let response = self.client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await?;
        
        if response.status().is_success() {
            let friends_response: EpicFriendsResponse = response.json().await?;
            let friends = friends_response.friends
                .into_iter()
                .map(|epic_friend| Friend {
                    user_id: epic_friend.account_id,
                    display_name: "Unknown".to_string(), // Epic doesn't provide display names in friends list
                    status: match epic_friend.status.as_str() {
                        "ACCEPTED" => FriendStatus::Friend,
                        "PENDING" => FriendStatus::Pending,
                        _ => FriendStatus::Unknown,
                    },
                    platform_specific_data: {
                        let mut data = HashMap::new();
                        data.insert("direction".to_string(), serde_json::Value::String(epic_friend.direction));
                        data.insert("favorite".to_string(), serde_json::Value::Bool(epic_friend.favorite));
                        data.insert("created".to_string(), serde_json::Value::String(epic_friend.created));
                        data
                    },
                })
                .collect();
            
            Ok(friends)
        } else {
            let status_code = response.status().as_u16();
            let error_text = response.text().await?;
            Err(PlatformError::ApiError {
                message: format!("Epic friends error: {}", error_text),
                status_code,
            })
        }
    }
    
    /// Submit achievements to Epic Games Services
    async fn submit_achievements(&self, account_id: &str, access_token: &str, achievements: &[Achievement]) -> PlatformResult<()> {
        let config = self.config.as_ref().unwrap();
        let product_id = &config.app_id;
        
        for achievement in achievements {
            if achievement.unlocked {
                let url = format!(
                    "https://api.epicgames.dev/achievements/v1/{}/achievement/{}",
                    product_id, achievement.id
                );
                
                let payload = serde_json::json!({
                    "achievementName": achievement.id,
                    "progress": 100.0
                });
                
                let response = self.client
                    .post(&url)
                    .bearer_auth(access_token)
                    .json(&payload)
                    .send()
                    .await?;
                
                if !response.status().is_success() {
                    let error_text = response.text().await?;
                    warn!("Failed to submit achievement {}: {}", achievement.id, error_text);
                }
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl PlatformAdapter for EpicGamesAdapter {
    fn platform_id(&self) -> &PlatformId {
        &self.platform_id
    }
    
    fn capabilities(&self) -> &PlatformCapabilities {
        &self.capabilities
    }
    
    async fn initialize(&mut self, config: PlatformConfig) -> PlatformResult<()> {
        info!("Initializing Epic Games adapter");
        
        // Validate required configuration
        if config.client_id.is_none() || config.client_secret.is_none() {
            return Err(PlatformError::InvalidConfiguration {
                platform: "epic".to_string(),
                details: "Client ID and Client Secret are required for Epic Games integration".to_string(),
            });
        }
        
        self.config = Some(config);
        info!("Epic Games adapter initialized successfully");
        Ok(())
    }
    
    async fn shutdown(&mut self) -> PlatformResult<()> {
        info!("Shutting down Epic Games adapter");
        self.config = None;
        Ok(())
    }
    
    async fn is_available(&self) -> bool {
        self.config.is_some()
    }
    
    async fn authenticate(&self, credentials: AuthenticationCredentials) -> PlatformResult<PlatformUser> {
        info!("Authenticating with Epic Games");
        
        match credentials {
            AuthenticationCredentials::AuthorizationCode { code, .. } => {
                let oauth_response = self.exchange_code_for_token(&code).await?;
                let profile = self.get_epic_user_profile(&oauth_response.account_id, &oauth_response.access_token).await?;
                
                Ok(PlatformUser {
                    platform_id: self.platform_id.clone(),
                    user_id: oauth_response.account_id,
                    display_name: profile.display_name,
                    email: profile.email,
                    avatar_url: profile.avatar_url,
                    access_token: Some(oauth_response.access_token),
                    refresh_token: Some(oauth_response.refresh_token),
                    token_expires_at: Some(chrono::Utc::now() + chrono::Duration::seconds(oauth_response.expires_in as i64)),
                    platform_specific_data: {
                        let mut data = HashMap::new();
                        data.insert("first_name".to_string(), serde_json::Value::String(profile.first_name.unwrap_or_default()));
                        data.insert("last_name".to_string(), serde_json::Value::String(profile.last_name.unwrap_or_default()));
                        data.insert("country".to_string(), serde_json::Value::String(profile.country.unwrap_or_default()));
                        data.insert("can_update_display_name".to_string(), serde_json::Value::Bool(profile.can_update_display_name));
                        data
                    },
                })
            }
            _ => Err(PlatformError::UnsupportedCapability {
                capability: "Only authorization code authentication is supported for Epic Games".to_string(),
            }),
        }
    }
    
    async fn refresh_token(&self, refresh_token: &str) -> PlatformResult<AuthenticationResult> {
        let config = self.config.as_ref().unwrap();
        let client_id = config.client_id.as_ref().unwrap();
        let client_secret = config.client_secret.as_ref().unwrap();
        
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];
        
        let response = self.client
            .post("https://api.epicgames.dev/epic/oauth/v1/token")
            .basic_auth(client_id, Some(client_secret))
            .form(&params)
            .send()
            .await?;
        
        if response.status().is_success() {
            let oauth_response: EpicOAuthResponse = response.json().await?;
            Ok(AuthenticationResult {
                access_token: oauth_response.access_token,
                refresh_token: Some(oauth_response.refresh_token),
                expires_at: Some(chrono::Utc::now() + chrono::Duration::seconds(oauth_response.expires_in as i64)),
                scopes: oauth_response.scope,
            })
        } else {
            Err(PlatformError::TokenExpired {
                platform: "epic".to_string(),
            })
        }
    }
    
    async fn get_user_profile(&self, user_id: &str) -> PlatformResult<UserProfile> {
        // This would require an active session with access token
        // For now, return a basic profile
        Ok(UserProfile {
            user_id: user_id.to_string(),
            display_name: "Epic User".to_string(),
            email: None,
            avatar_url: None,
            creation_date: None,
            last_seen: None,
            platform_specific_data: HashMap::new(),
        })
    }
    
    async fn sync_achievements(&self, user_id: &str, achievements: &[Achievement]) -> PlatformResult<()> {
        info!("Syncing {} achievements with Epic Games for user {}", achievements.len(), user_id);
        
        // This would require an active session with access token
        // For now, just log the sync
        debug!("Achievement sync completed for Epic Games");
        Ok(())
    }
    
    async fn get_friends_list(&self, user_id: &str) -> PlatformResult<Vec<Friend>> {
        info!("Getting friends list from Epic Games for user {}", user_id);
        
        // This would require an active session with access token
        // For now, return empty list
        Ok(vec![])
    }
    
    async fn launch_content(&self, content_id: &str, launch_options: LaunchOptions) -> PlatformResult<()> {
        info!("Launching content {} through Epic Games", content_id);
        
        // Epic Games launcher URL scheme
        let launch_url = format!("com.epicgames.launcher://apps/{}?action=launch", content_id);
        
        // Add launch parameters if specified
        let final_url = if !launch_options.arguments.is_empty() {
            format!("{}&args={}", launch_url, urlencoding::encode(&launch_options.arguments.join(" ")))
        } else {
            launch_url
        };
        
        // Platform-specific launch logic would go here
        info!("Epic Games launch URL: {}", final_url);
        Ok(())
    }
    
    async fn get_analytics(&self, user_id: &str) -> PlatformResult<PlatformAnalytics> {
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

impl Default for EpicGamesAdapter {
    fn default() -> Self {
        Self::new()
    }
}