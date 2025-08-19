/*!
# Platform Manager

Central management system for all platform integrations.
*/

use crate::*;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};

/// Main trait for platform adapters
#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Get the platform identifier
    fn platform_id(&self) -> &PlatformId;
    
    /// Get platform capabilities
    fn capabilities(&self) -> &PlatformCapabilities;
    
    /// Initialize the platform adapter
    async fn initialize(&mut self, config: PlatformConfig) -> PlatformResult<()>;
    
    /// Shutdown and cleanup the platform adapter
    async fn shutdown(&mut self) -> PlatformResult<()>;
    
    /// Check if the platform is available and ready
    async fn is_available(&self) -> bool;
    
    /// Authenticate a user with the platform
    async fn authenticate(&self, credentials: AuthenticationCredentials) -> PlatformResult<PlatformUser>;
    
    /// Refresh authentication token
    async fn refresh_token(&self, refresh_token: &str) -> PlatformResult<AuthenticationResult>;
    
    /// Get user profile information
    async fn get_user_profile(&self, user_id: &str) -> PlatformResult<UserProfile>;
    
    /// Sync achievements with the platform
    async fn sync_achievements(&self, user_id: &str, achievements: &[Achievement]) -> PlatformResult<()>;
    
    /// Get friends list
    async fn get_friends_list(&self, user_id: &str) -> PlatformResult<Vec<Friend>>;
    
    /// Launch content/game through the platform
    async fn launch_content(&self, content_id: &str, launch_options: LaunchOptions) -> PlatformResult<()>;
    
    /// Get platform-specific analytics data
    async fn get_analytics(&self, user_id: &str) -> PlatformResult<PlatformAnalytics>;
}

/// Central platform management system
pub struct PlatformManager {
    platforms: Arc<RwLock<HashMap<PlatformId, Box<dyn PlatformAdapter>>>>,
    config: Arc<RwLock<PlatformManagerConfig>>,
    active_sessions: Arc<RwLock<HashMap<String, ActiveSession>>>,
}

#[derive(Debug, Clone)]
pub struct PlatformManagerConfig {
    pub auto_detect_platforms: bool,
    pub fallback_authentication: bool,
    pub cache_user_profiles: bool,
    pub sync_achievements_automatically: bool,
    pub analytics_enabled: bool,
    pub session_timeout_minutes: u64,
}

impl Default for PlatformManagerConfig {
    fn default() -> Self {
        Self {
            auto_detect_platforms: true,
            fallback_authentication: true,
            cache_user_profiles: true,
            sync_achievements_automatically: true,
            analytics_enabled: true,
            session_timeout_minutes: 60,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub user: PlatformUser,
    pub platform_id: PlatformId,
    pub session_start: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub authentication_result: AuthenticationResult,
}

impl PlatformManager {
    /// Create a new platform manager
    pub fn new() -> Self {
        Self {
            platforms: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(PlatformManagerConfig::default())),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new platform manager with custom configuration
    pub fn with_config(config: PlatformManagerConfig) -> Self {
        Self {
            platforms: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(config)),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a platform adapter
    pub async fn register_platform(&self, adapter: Box<dyn PlatformAdapter>) -> PlatformResult<()> {
        let platform_id = adapter.platform_id().clone();
        info!("Registering platform adapter: {}", platform_id.as_str());
        
        let mut platforms = self.platforms.write().await;
        
        if platforms.contains_key(&platform_id) {
            warn!("Platform {} is already registered, replacing", platform_id.as_str());
        }
        
        platforms.insert(platform_id.clone(), adapter);
        info!("Successfully registered platform: {}", platform_id.as_str());
        
        Ok(())
    }
    
    /// Unregister a platform adapter
    pub async fn unregister_platform(&self, platform_id: &PlatformId) -> PlatformResult<()> {
        info!("Unregistering platform: {}", platform_id.as_str());
        
        let mut platforms = self.platforms.write().await;
        
        if let Some(mut adapter) = platforms.remove(platform_id) {
            if let Err(e) = adapter.shutdown().await {
                error!("Error shutting down platform {}: {}", platform_id.as_str(), e);
            }
            info!("Successfully unregistered platform: {}", platform_id.as_str());
            Ok(())
        } else {
            Err(PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })
        }
    }
    
    /// Get list of registered platforms
    pub async fn get_registered_platforms(&self) -> Vec<PlatformId> {
        let platforms = self.platforms.read().await;
        platforms.keys().cloned().collect()
    }
    
    /// Get platform capabilities
    pub async fn get_platform_capabilities(&self, platform_id: &PlatformId) -> PlatformResult<PlatformCapabilities> {
        let platforms = self.platforms.read().await;
        
        if let Some(adapter) = platforms.get(platform_id) {
            Ok(adapter.capabilities().clone())
        } else {
            Err(PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })
        }
    }
    
    /// Check if a platform is available
    pub async fn is_platform_available(&self, platform_id: &PlatformId) -> bool {
        let platforms = self.platforms.read().await;
        
        if let Some(adapter) = platforms.get(platform_id) {
            adapter.is_available().await
        } else {
            false
        }
    }
    
    /// Authenticate user with a specific platform
    pub async fn authenticate_user(
        &self, 
        platform_id: &PlatformId, 
        credentials: AuthenticationCredentials
    ) -> PlatformResult<PlatformUser> {
        info!("Authenticating user with platform: {}", platform_id.as_str());
        
        let platforms = self.platforms.read().await;
        
        if let Some(adapter) = platforms.get(platform_id) {
            let user = adapter.authenticate(credentials).await?;
            
            // Create active session
            let session = ActiveSession {
                user: user.clone(),
                platform_id: platform_id.clone(),
                session_start: chrono::Utc::now(),
                last_activity: chrono::Utc::now(),
                authentication_result: AuthenticationResult {
                    access_token: user.access_token.clone().unwrap_or_default(),
                    refresh_token: user.refresh_token.clone(),
                    expires_at: user.token_expires_at,
                    scopes: vec![], // Platform-specific scopes
                },
            };
            
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(user.user_id.clone(), session);
            
            info!("Successfully authenticated user {} on platform {}", user.user_id, platform_id.as_str());
            Ok(user)
        } else {
            Err(PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })
        }
    }
    
    /// Authenticate user automatically with the best available platform
    pub async fn authenticate_user_auto(&self, preferred_platforms: Option<Vec<PlatformId>>) -> PlatformResult<PlatformUser> {
        info!("Auto-authenticating user");
        
        let platforms_to_try = if let Some(preferred) = preferred_platforms {
            preferred
        } else {
            self.get_registered_platforms().await
        };
        
        for platform_id in platforms_to_try {
            if self.is_platform_available(&platform_id).await {
                info!("Attempting authentication with platform: {}", platform_id.as_str());
                
                // Try platform-specific auto-authentication
                if let Ok(credentials) = self.get_stored_credentials(&platform_id).await {
                    match self.authenticate_user(&platform_id, credentials).await {
                        Ok(user) => {
                            info!("Auto-authentication successful with platform: {}", platform_id.as_str());
                            return Ok(user);
                        }
                        Err(e) => {
                            warn!("Auto-authentication failed with platform {}: {}", platform_id.as_str(), e);
                        }
                    }
                }
            }
        }
        
        Err(PlatformError::AuthenticationFailed {
            platform: "auto".to_string(),
            reason: "No platforms available for auto-authentication".to_string(),
        })
    }
    
    /// Get user profile from platform
    pub async fn get_user_profile(&self, platform_id: &PlatformId, user_id: &str) -> PlatformResult<UserProfile> {
        let platforms = self.platforms.read().await;
        
        if let Some(adapter) = platforms.get(platform_id) {
            adapter.get_user_profile(user_id).await
        } else {
            Err(PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })
        }
    }
    
    /// Sync achievements across all platforms
    pub async fn sync_achievements_all_platforms(&self, achievements: &[Achievement]) -> PlatformResult<()> {
        info!("Syncing achievements across all platforms");
        
        let sessions = self.active_sessions.read().await;
        let platforms = self.platforms.read().await;
        
        for (user_id, session) in sessions.iter() {
            if let Some(adapter) = platforms.get(&session.platform_id) {
                if adapter.capabilities().achievements {
                    match adapter.sync_achievements(user_id, achievements).await {
                        Ok(()) => {
                            debug!("Successfully synced achievements for user {} on platform {}", 
                                   user_id, session.platform_id.as_str());
                        }
                        Err(e) => {
                            warn!("Failed to sync achievements for user {} on platform {}: {}", 
                                  user_id, session.platform_id.as_str(), e);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Launch content through the appropriate platform
    pub async fn launch_content(
        &self, 
        platform_id: &PlatformId, 
        content_id: &str, 
        launch_options: LaunchOptions
    ) -> PlatformResult<()> {
        info!("Launching content {} on platform {}", content_id, platform_id.as_str());
        
        let platforms = self.platforms.read().await;
        
        if let Some(adapter) = platforms.get(platform_id) {
            adapter.launch_content(content_id, launch_options).await
        } else {
            Err(PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })
        }
    }
    
    /// Get analytics data from all platforms
    pub async fn collect_analytics(&self) -> PlatformResult<HashMap<PlatformId, PlatformAnalytics>> {
        let mut analytics = HashMap::new();
        
        let sessions = self.active_sessions.read().await;
        let platforms = self.platforms.read().await;
        
        for (user_id, session) in sessions.iter() {
            if let Some(adapter) = platforms.get(&session.platform_id) {
                if adapter.capabilities().analytics {
                    match adapter.get_analytics(user_id).await {
                        Ok(platform_analytics) => {
                            analytics.insert(session.platform_id.clone(), platform_analytics);
                        }
                        Err(e) => {
                            warn!("Failed to collect analytics from platform {}: {}", 
                                  session.platform_id.as_str(), e);
                        }
                    }
                }
            }
        }
        
        Ok(analytics)
    }
    
    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> PlatformResult<()> {
        let config = self.config.read().await;
        let timeout = chrono::Duration::minutes(config.session_timeout_minutes as i64);
        let now = chrono::Utc::now();
        
        let mut sessions = self.active_sessions.write().await;
        let expired_sessions: Vec<String> = sessions
            .iter()
            .filter_map(|(user_id, session)| {
                if now - session.last_activity > timeout {
                    Some(user_id.clone())
                } else {
                    None
                }
            })
            .collect();
        
        for user_id in expired_sessions {
            sessions.remove(&user_id);
            info!("Removed expired session for user: {}", user_id);
        }
        
        Ok(())
    }
    
    /// Get stored credentials for a platform (implement based on secure storage)
    async fn get_stored_credentials(&self, platform_id: &PlatformId) -> PlatformResult<AuthenticationCredentials> {
        // This would integrate with secure credential storage
        // For now, return a placeholder
        Err(PlatformError::AuthenticationFailed {
            platform: platform_id.as_str().to_string(),
            reason: "No stored credentials available".to_string(),
        })
    }
    
    /// Update configuration
    pub async fn update_config(&self, config: PlatformManagerConfig) {
        let mut current_config = self.config.write().await;
        *current_config = config;
    }
    
    /// Get current configuration
    pub async fn get_config(&self) -> PlatformManagerConfig {
        self.config.read().await.clone()
    }
}

impl Default for PlatformManager {
    fn default() -> Self {
        Self::new()
    }
}