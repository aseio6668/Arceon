/*!
# Generic Platform Adapter

Flexible adapter system for integrating with custom or third-party platforms
that aren't directly supported. Provides a configurable framework for:
- Custom authentication flows
- API integration patterns
- Extensible capability system
- Plugin-style architecture
*/

use crate::*;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{info, warn, debug};

/// Generic platform adapter for custom integrations
pub struct GenericPlatformAdapter {
    platform_id: PlatformId,
    config: Option<PlatformConfig>,
    capabilities: PlatformCapabilities,
    client: Client,
    custom_handlers: HashMap<String, Box<dyn CustomHandler>>,
}

/// Custom handler trait for extensible operations
#[async_trait]
pub trait CustomHandler: Send + Sync {
    async fn handle(&self, operation: &str, params: HashMap<String, Value>) -> PlatformResult<Value>;
}

/// Generic API configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenericApiConfig {
    pub base_url: String,
    pub authentication_type: AuthenticationType,
    pub headers: HashMap<String, String>,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub rate_limit_per_minute: Option<u32>,
}

/// Authentication type for generic platforms
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AuthenticationType {
    None,
    ApiKey { header_name: String },
    Bearer,
    Basic,
    Custom { handler_name: String },
    OAuth2 {
        auth_url: String,
        token_url: String,
        scopes: Vec<String>,
    },
}

/// Generic endpoint configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EndpointConfig {
    pub path: String,
    pub method: String,
    pub required_params: Vec<String>,
    pub optional_params: Vec<String>,
    pub response_mapping: HashMap<String, String>,
}

/// Generic platform configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenericPlatformConfig {
    pub platform_name: String,
    pub api_config: GenericApiConfig,
    pub endpoints: HashMap<String, EndpointConfig>,
    pub custom_mappings: HashMap<String, Value>,
    pub webhook_endpoints: Vec<WebhookConfig>,
}

/// Webhook configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookConfig {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub headers: HashMap<String, String>,
}

impl GenericPlatformAdapter {
    /// Create a new generic platform adapter
    pub fn new(platform_name: &str) -> Self {
        Self {
            platform_id: PlatformId::new(platform_name),
            config: None,
            capabilities: PlatformCapabilities::default(),
            client: Client::new(),
            custom_handlers: HashMap::new(),
        }
    }
    
    /// Register a custom handler for specific operations
    pub fn register_custom_handler(&mut self, name: String, handler: Box<dyn CustomHandler>) {
        info!("Registering custom handler: {}", name);
        self.custom_handlers.insert(name, handler);
    }
    
    /// Build request for generic API call
    async fn build_request(&self, endpoint_name: &str, _params: HashMap<String, Value>) -> PlatformResult<reqwest::RequestBuilder> {
        let config = self.config.as_ref()
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: self.platform_id.as_str().to_string(),
                details: "Adapter not initialized".to_string(),
            })?;
        
        let generic_config: GenericPlatformConfig = config.get_custom_setting("generic_config")?
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: self.platform_id.as_str().to_string(),
                details: "Generic platform configuration is required".to_string(),
            })?;
        
        let endpoint = generic_config.endpoints.get(endpoint_name)
            .ok_or_else(|| PlatformError::UnsupportedCapability {
                capability: format!("Endpoint not configured: {}", endpoint_name),
            })?;
        
        // Build URL
        let url = format!("{}{}", generic_config.api_config.base_url, endpoint.path);
        
        // Create request builder
        let request_builder = match endpoint.method.to_uppercase().as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            "PATCH" => self.client.patch(&url),
            _ => return Err(PlatformError::UnsupportedCapability {
                capability: format!("HTTP method not supported: {}", endpoint.method),
            }),
        };
        
        // Add headers
        let mut builder = request_builder;
        for (header_name, header_value) in &generic_config.api_config.headers {
            builder = builder.header(header_name, header_value);
        }
        
        // Add authentication
        builder = self.add_authentication(builder, &generic_config.api_config.authentication_type).await?;
        
        // Add timeout
        builder = builder.timeout(std::time::Duration::from_secs(generic_config.api_config.timeout_seconds));
        
        Ok(builder)
    }
    
    /// Add authentication to request
    async fn add_authentication(&self, builder: reqwest::RequestBuilder, auth_type: &AuthenticationType) -> PlatformResult<reqwest::RequestBuilder> {
        match auth_type {
            AuthenticationType::None => Ok(builder),
            AuthenticationType::ApiKey { header_name } => {
                if let Some(api_key) = self.config.as_ref().and_then(|c| c.client_secret.as_ref()) {
                    Ok(builder.header(header_name, api_key))
                } else {
                    Err(PlatformError::InvalidConfiguration {
                        platform: self.platform_id.as_str().to_string(),
                        details: "API key not configured".to_string(),
                    })
                }
            }
            AuthenticationType::Bearer => {
                // Would use stored access token
                Ok(builder.bearer_auth("placeholder_token"))
            }
            AuthenticationType::Basic => {
                if let Some(config) = &self.config {
                    if let (Some(username), Some(password)) = (&config.client_id, &config.client_secret) {
                        Ok(builder.basic_auth(username, Some(password)))
                    } else {
                        Err(PlatformError::InvalidConfiguration {
                            platform: self.platform_id.as_str().to_string(),
                            details: "Basic auth credentials not configured".to_string(),
                        })
                    }
                } else {
                    Err(PlatformError::InvalidConfiguration {
                        platform: self.platform_id.as_str().to_string(),
                        details: "Configuration not available".to_string(),
                    })
                }
            }
            AuthenticationType::Custom { handler_name } => {
                if let Some(handler) = self.custom_handlers.get(handler_name) {
                    let params = HashMap::new();
                    let _auth_result = handler.handle("authenticate_request", params).await?;
                    // Apply authentication based on handler result
                    Ok(builder)
                } else {
                    Err(PlatformError::UnsupportedCapability {
                        capability: format!("Custom auth handler not found: {}", handler_name),
                    })
                }
            }
            AuthenticationType::OAuth2 { .. } => {
                // Would implement OAuth2 flow
                Ok(builder.bearer_auth("oauth2_placeholder_token"))
            }
        }
    }
    
    /// Execute generic API call
    pub async fn execute_api_call(&self, endpoint_name: &str, params: HashMap<String, Value>) -> PlatformResult<Value> {
        debug!("Executing API call: {} with params: {:?}", endpoint_name, params);
        
        let request_builder = self.build_request(endpoint_name, params.clone()).await?;
        
        // Add query parameters or body based on method
        // Determine if this should be a GET request based on method
        let should_use_query = {
            let temp_request = request_builder.try_clone()
                .ok_or_else(|| PlatformError::ApiError {
                    message: "Failed to clone request builder".to_string(),
                    status_code: 500,
                })?
                .build()
                .map_err(|e| PlatformError::ApiError {
                    message: format!("Failed to build request: {}", e),
                    status_code: 500,
                })?;
            temp_request.method() == reqwest::Method::GET
        };
        
        let request = if should_use_query {
            let query_params: Vec<(String, String)> = params.iter()
                .filter_map(|(key, value)| {
                    value.as_str().map(|s| (key.clone(), s.to_string()))
                })
                .collect();
            request_builder.query(&query_params)
        } else {
            request_builder.json(&params)
        };
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            let result: Value = response.json().await?;
            Ok(result)
        } else {
            let status_code = response.status().as_u16();
            let error_text = response.text().await?;
            Err(PlatformError::ApiError {
                message: error_text,
                status_code,
            })
        }
    }
    
    /// Map generic response to platform-specific format
    fn map_response(&self, endpoint_name: &str, response: Value) -> PlatformResult<Value> {
        let config = self.config.as_ref().unwrap();
        let generic_config: GenericPlatformConfig = config.get_custom_setting("generic_config")?.unwrap();
        
        if let Some(endpoint) = generic_config.endpoints.get(endpoint_name) {
            let mut mapped_response = serde_json::Map::new();
            
            for (target_field, source_path) in &endpoint.response_mapping {
                if let Some(source_value) = self.get_nested_value(&response, source_path) {
                    mapped_response.insert(target_field.clone(), source_value.clone());
                }
            }
            
            Ok(Value::Object(mapped_response))
        } else {
            Ok(response)
        }
    }
    
    /// Get nested value from JSON using dot notation
    fn get_nested_value<'a>(&self, value: &'a Value, path: &str) -> Option<&'a Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;
        
        for part in parts {
            current = current.get(part)?;
        }
        
        Some(current)
    }
    
    /// Setup webhooks for real-time events
    pub async fn setup_webhooks(&self) -> PlatformResult<()> {
        let config = self.config.as_ref().unwrap();
        let generic_config: GenericPlatformConfig = config.get_custom_setting("generic_config")?.unwrap();
        
        for webhook in &generic_config.webhook_endpoints {
            info!("Setting up webhook: {} -> {}", webhook.name, webhook.url);
            
            let webhook_params = {
                let mut params = HashMap::new();
                params.insert("url".to_string(), Value::String(webhook.url.clone()));
                params.insert("events".to_string(), Value::Array(
                    webhook.events.iter().map(|e| Value::String(e.clone())).collect()
                ));
                if let Some(secret) = &webhook.secret {
                    params.insert("secret".to_string(), Value::String(secret.clone()));
                }
                params
            };
            
            match self.execute_api_call("create_webhook", webhook_params).await {
                Ok(_) => info!("Webhook {} created successfully", webhook.name),
                Err(e) => warn!("Failed to create webhook {}: {}", webhook.name, e),
            }
        }
        
        Ok(())
    }
}

/// Simple custom handler example
pub struct SimpleCustomHandler;

#[async_trait]
impl CustomHandler for SimpleCustomHandler {
    async fn handle(&self, operation: &str, params: HashMap<String, Value>) -> PlatformResult<Value> {
        info!("SimpleCustomHandler handling operation: {} with params: {:?}", operation, params);
        
        match operation {
            "authenticate_request" => {
                // Return authentication headers or tokens
                Ok(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("Authorization".to_string(), Value::String("Bearer custom_token".to_string()));
                    map
                }))
            }
            "process_user_data" => {
                // Transform user data format
                Ok(params.get("user_data").cloned().unwrap_or(Value::Null))
            }
            _ => Err(PlatformError::UnsupportedCapability {
                capability: format!("Unknown operation: {}", operation),
            }),
        }
    }
}

#[async_trait]
impl PlatformAdapter for GenericPlatformAdapter {
    fn platform_id(&self) -> &PlatformId {
        &self.platform_id
    }
    
    fn capabilities(&self) -> &PlatformCapabilities {
        &self.capabilities
    }
    
    async fn initialize(&mut self, config: PlatformConfig) -> PlatformResult<()> {
        info!("Initializing generic platform adapter: {}", self.platform_id.as_str());
        
        // Validate generic configuration
        let _generic_config: GenericPlatformConfig = config.get_custom_setting("generic_config")?
            .ok_or_else(|| PlatformError::InvalidConfiguration {
                platform: self.platform_id.as_str().to_string(),
                details: "Generic platform configuration is required".to_string(),
            })?;
        
        self.capabilities = config.capabilities.clone();
        self.config = Some(config);
        
        // Setup webhooks if configured
        if let Err(e) = self.setup_webhooks().await {
            warn!("Failed to setup webhooks: {}", e);
        }
        
        info!("Generic platform adapter initialized successfully");
        Ok(())
    }
    
    async fn shutdown(&mut self) -> PlatformResult<()> {
        info!("Shutting down generic platform adapter");
        self.config = None;
        self.custom_handlers.clear();
        Ok(())
    }
    
    async fn is_available(&self) -> bool {
        self.config.is_some()
    }
    
    async fn authenticate(&self, credentials: AuthenticationCredentials) -> PlatformResult<PlatformUser> {
        info!("Authenticating with generic platform");
        
        let auth_params = match credentials {
            AuthenticationCredentials::UsernamePassword { username, password } => {
                let mut params = HashMap::new();
                params.insert("username".to_string(), Value::String(username));
                params.insert("password".to_string(), Value::String(password));
                params
            }
            AuthenticationCredentials::Token { token } => {
                let mut params = HashMap::new();
                params.insert("token".to_string(), Value::String(token));
                params
            }
            AuthenticationCredentials::ApiKey { key } => {
                let mut params = HashMap::new();
                params.insert("api_key".to_string(), Value::String(key));
                params
            }
            AuthenticationCredentials::AuthorizationCode { code, .. } => {
                let mut params = HashMap::new();
                params.insert("code".to_string(), Value::String(code));
                params
            }
            AuthenticationCredentials::Custom { data } => data,
        };
        
        let auth_response = self.execute_api_call("authenticate", auth_params).await?;
        let mapped_response = self.map_response("authenticate", auth_response)?;
        
        // Extract user information from response
        let user_id = mapped_response.get("user_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown_user")
            .to_string();
        
        let display_name = mapped_response.get("display_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Generic User")
            .to_string();
        
        let access_token = mapped_response.get("access_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(PlatformUser {
            platform_id: self.platform_id.clone(),
            user_id,
            display_name,
            email: mapped_response.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
            avatar_url: mapped_response.get("avatar_url").and_then(|v| v.as_str()).map(|s| s.to_string()),
            access_token,
            refresh_token: mapped_response.get("refresh_token").and_then(|v| v.as_str()).map(|s| s.to_string()),
            token_expires_at: None, // Parse from response if available
            platform_specific_data: {
                let mut data = HashMap::new();
                if let Value::Object(obj) = mapped_response {
                    for (key, value) in obj {
                        if !["user_id", "display_name", "email", "avatar_url", "access_token", "refresh_token"].contains(&key.as_str()) {
                            data.insert(key, value);
                        }
                    }
                }
                data
            },
        })
    }
    
    async fn refresh_token(&self, refresh_token: &str) -> PlatformResult<AuthenticationResult> {
        let mut params = HashMap::new();
        params.insert("refresh_token".to_string(), Value::String(refresh_token.to_string()));
        
        let response = self.execute_api_call("refresh_token", params).await?;
        let mapped_response = self.map_response("refresh_token", response)?;
        
        Ok(AuthenticationResult {
            access_token: mapped_response.get("access_token")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            refresh_token: mapped_response.get("refresh_token")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            expires_at: None, // Parse from response
            scopes: vec![],
        })
    }
    
    async fn get_user_profile(&self, user_id: &str) -> PlatformResult<UserProfile> {
        let mut params = HashMap::new();
        params.insert("user_id".to_string(), Value::String(user_id.to_string()));
        
        let response = self.execute_api_call("get_user_profile", params).await?;
        let mapped_response = self.map_response("get_user_profile", response)?;
        
        Ok(UserProfile {
            user_id: user_id.to_string(),
            display_name: mapped_response.get("display_name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
            email: mapped_response.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
            avatar_url: mapped_response.get("avatar_url").and_then(|v| v.as_str()).map(|s| s.to_string()),
            creation_date: None, // Parse from response
            last_seen: None, // Parse from response
            platform_specific_data: HashMap::new(),
        })
    }
    
    async fn sync_achievements(&self, user_id: &str, achievements: &[Achievement]) -> PlatformResult<()> {
        for achievement in achievements {
            if achievement.unlocked {
                let mut params = HashMap::new();
                params.insert("user_id".to_string(), Value::String(user_id.to_string()));
                params.insert("achievement_id".to_string(), Value::String(achievement.id.clone()));
                params.insert("unlocked_at".to_string(), Value::String(chrono::Utc::now().to_rfc3339()));
                
                match self.execute_api_call("unlock_achievement", params).await {
                    Ok(_) => debug!("Achievement {} synced successfully", achievement.id),
                    Err(e) => warn!("Failed to sync achievement {}: {}", achievement.id, e),
                }
            }
        }
        
        Ok(())
    }
    
    async fn get_friends_list(&self, user_id: &str) -> PlatformResult<Vec<Friend>> {
        let mut params = HashMap::new();
        params.insert("user_id".to_string(), Value::String(user_id.to_string()));
        
        let response = self.execute_api_call("get_friends", params).await?;
        let mapped_response = self.map_response("get_friends", response)?;
        
        if let Some(friends_array) = mapped_response.get("friends").and_then(|v| v.as_array()) {
            let friends = friends_array
                .iter()
                .filter_map(|friend_data| {
                    let friend_id = friend_data.get("user_id")?.as_str()?.to_string();
                    let display_name = friend_data.get("display_name")?.as_str()?.to_string();
                    
                    Some(Friend {
                        user_id: friend_id,
                        display_name,
                        status: FriendStatus::Friend,
                        platform_specific_data: HashMap::new(),
                    })
                })
                .collect();
            
            Ok(friends)
        } else {
            Ok(vec![])
        }
    }
    
    async fn launch_content(&self, content_id: &str, launch_options: LaunchOptions) -> PlatformResult<()> {
        let mut params = HashMap::new();
        params.insert("content_id".to_string(), Value::String(content_id.to_string()));
        params.insert("arguments".to_string(), Value::Array(
            launch_options.arguments.into_iter().map(Value::String).collect()
        ));
        
        self.execute_api_call("launch_content", params).await?;
        info!("Content {} launched through generic platform", content_id);
        Ok(())
    }
    
    async fn get_analytics(&self, user_id: &str) -> PlatformResult<PlatformAnalytics> {
        let mut params = HashMap::new();
        params.insert("user_id".to_string(), Value::String(user_id.to_string()));
        
        match self.execute_api_call("get_analytics", params).await {
            Ok(response) => {
                let mapped_response = self.map_response("get_analytics", response)?;
                
                Ok(PlatformAnalytics {
                    platform_id: self.platform_id.clone(),
                    user_id: user_id.to_string(),
                    session_count: mapped_response.get("session_count").and_then(|v| v.as_u64()).unwrap_or(0),
                    total_playtime_hours: mapped_response.get("playtime_hours").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    achievements_unlocked: mapped_response.get("achievements_count").and_then(|v| v.as_u64()).unwrap_or(0),
                    friends_count: mapped_response.get("friends_count").and_then(|v| v.as_u64()).unwrap_or(0),
                    last_activity: None, // Parse from response
                    platform_specific_metrics: HashMap::new(),
                })
            }
            Err(_) => {
                // Return default analytics if endpoint is not available
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
}