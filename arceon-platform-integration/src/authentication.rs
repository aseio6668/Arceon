/*!
# Authentication Module

Unified authentication system supporting multiple authentication flows:
- OAuth 2.0 / OpenID Connect
- API Key authentication
- Username/Password
- Token-based authentication
- Platform-specific authentication methods
*/

use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication credentials for various platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthenticationCredentials {
    /// Username and password authentication
    UsernamePassword { username: String, password: String },
    /// Token-based authentication
    Token { token: String },
    /// API key authentication
    ApiKey { key: String },
    /// OAuth authorization code flow
    AuthorizationCode { 
        code: String, 
        state: Option<String>,
        code_verifier: Option<String> // for PKCE
    },
    /// Custom authentication data
    Custom { data: HashMap<String, serde_json::Value> },
}

/// Authentication result containing tokens and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub scopes: Vec<String>,
}

/// Platform user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformUser {
    pub platform_id: PlatformId,
    pub user_id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub platform_specific_data: HashMap<String, serde_json::Value>,
}

/// OAuth 2.0 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub use_pkce: bool,
}

/// Authentication manager for handling multi-platform authentication
pub struct AuthenticationManager {
    oauth_configs: HashMap<PlatformId, OAuthConfig>,
    stored_tokens: HashMap<String, StoredToken>,
}

/// Stored token information
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredToken {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
    platform_id: PlatformId,
    user_id: String,
}

impl AuthenticationManager {
    /// Create a new authentication manager
    pub fn new() -> Self {
        Self {
            oauth_configs: HashMap::new(),
            stored_tokens: HashMap::new(),
        }
    }
    
    /// Register OAuth configuration for a platform
    pub fn register_oauth_config(&mut self, platform_id: PlatformId, config: OAuthConfig) {
        self.oauth_configs.insert(platform_id, config);
    }
    
    /// Generate OAuth authorization URL with PKCE support
    pub fn generate_auth_url(&self, platform_id: &PlatformId, state: Option<&str>) -> PlatformResult<(String, Option<String>)> {
        let config = self.oauth_configs.get(platform_id)
            .ok_or_else(|| PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })?;
        
        // Prepare values first to avoid borrowing issues
        let scopes_str = if !config.scopes.is_empty() {
            Some(config.scopes.join(" "))
        } else {
            None
        };
        
        let (code_verifier, challenge) = if config.use_pkce {
            let verifier = generate_code_verifier();
            let challenge = generate_code_challenge(&verifier);
            (Some(verifier), Some(challenge))
        } else {
            (None, None)
        };

        let mut url_params = vec![
            ("client_id", config.client_id.as_str()),
            ("redirect_uri", config.redirect_uri.as_str()),
            ("response_type", "code"),
        ];
        
        if let Some(ref scopes) = scopes_str {
            url_params.push(("scope", scopes.as_str()));
        }
        
        if let Some(state_val) = state {
            url_params.push(("state", state_val));
        }
        
        if let Some(ref challenge_val) = challenge {
            url_params.push(("code_challenge", challenge_val.as_str()));
            url_params.push(("code_challenge_method", "S256"));
        }
        
        let query_string: String = url_params
            .iter()
            .map(|(key, value)| format!("{}={}", key, urlencoding::encode(value)))
            .collect::<Vec<_>>()
            .join("&");
        
        let auth_url = format!("{}?{}", config.authorization_endpoint, query_string);
        
        Ok((auth_url, code_verifier))
    }
    
    /// Exchange authorization code for access token
    pub async fn exchange_code_for_token(
        &self,
        platform_id: &PlatformId,
        code: &str,
        code_verifier: Option<&str>
    ) -> PlatformResult<AuthenticationResult> {
        let config = self.oauth_configs.get(platform_id)
            .ok_or_else(|| PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })?;
        
        let client = reqwest::Client::new();
        
        let mut form_params = vec![
            ("grant_type", "authorization_code"),
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
            ("code", code),
            ("redirect_uri", &config.redirect_uri),
        ];
        
        if let Some(verifier) = code_verifier {
            form_params.push(("code_verifier", verifier));
        }
        
        let response = client
            .post(&config.token_endpoint)
            .form(&form_params)
            .send()
            .await?;
        
        if response.status().is_success() {
            let token_response: serde_json::Value = response.json().await?;
            
            let access_token = token_response.get("access_token")
                .and_then(|v| v.as_str())
                .ok_or_else(|| PlatformError::ApiError {
                    message: "No access token in response".to_string(),
                    status_code: 200,
                })?
                .to_string();
            
            let refresh_token = token_response.get("refresh_token")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            let expires_in = token_response.get("expires_in")
                .and_then(|v| v.as_u64());
            
            let expires_at = expires_in.map(|seconds| {
                chrono::Utc::now() + chrono::Duration::seconds(seconds as i64)
            });
            
            let scopes = token_response.get("scope")
                .and_then(|v| v.as_str())
                .map(|s| s.split(' ').map(String::from).collect())
                .unwrap_or_else(Vec::new);
            
            Ok(AuthenticationResult {
                access_token,
                refresh_token,
                expires_at,
                scopes,
            })
        } else {
            let error_text = response.text().await?;
            Err(PlatformError::AuthenticationFailed {
                platform: platform_id.as_str().to_string(),
                reason: error_text,
            })
        }
    }
    
    /// Refresh access token using refresh token
    pub async fn refresh_access_token(
        &self,
        platform_id: &PlatformId,
        refresh_token: &str
    ) -> PlatformResult<AuthenticationResult> {
        let config = self.oauth_configs.get(platform_id)
            .ok_or_else(|| PlatformError::PlatformNotFound {
                platform: platform_id.as_str().to_string(),
            })?;
        
        let client = reqwest::Client::new();
        
        let form_params = vec![
            ("grant_type", "refresh_token"),
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
            ("refresh_token", refresh_token),
        ];
        
        let response = client
            .post(&config.token_endpoint)
            .form(&form_params)
            .send()
            .await?;
        
        if response.status().is_success() {
            let token_response: serde_json::Value = response.json().await?;
            
            let access_token = token_response.get("access_token")
                .and_then(|v| v.as_str())
                .ok_or_else(|| PlatformError::ApiError {
                    message: "No access token in response".to_string(),
                    status_code: 200,
                })?
                .to_string();
            
            let new_refresh_token = token_response.get("refresh_token")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| Some(refresh_token.to_string())); // Keep old refresh token if new one not provided
            
            let expires_in = token_response.get("expires_in")
                .and_then(|v| v.as_u64());
            
            let expires_at = expires_in.map(|seconds| {
                chrono::Utc::now() + chrono::Duration::seconds(seconds as i64)
            });
            
            let scopes = token_response.get("scope")
                .and_then(|v| v.as_str())
                .map(|s| s.split(' ').map(String::from).collect())
                .unwrap_or_else(Vec::new);
            
            Ok(AuthenticationResult {
                access_token,
                refresh_token: new_refresh_token,
                expires_at,
                scopes,
            })
        } else {
            Err(PlatformError::TokenExpired {
                platform: platform_id.as_str().to_string(),
            })
        }
    }
    
    /// Store authentication token securely
    pub fn store_token(&mut self, user_id: String, platform_id: PlatformId, auth_result: AuthenticationResult) {
        let stored_token = StoredToken {
            access_token: auth_result.access_token,
            refresh_token: auth_result.refresh_token,
            expires_at: auth_result.expires_at,
            platform_id,
            user_id: user_id.clone(),
        };
        
        self.stored_tokens.insert(user_id, stored_token);
    }
    
    /// Retrieve stored token for a user
    pub fn get_stored_token(&self, user_id: &str) -> Option<&StoredToken> {
        self.stored_tokens.get(user_id)
    }
    
    /// Check if token needs refresh
    pub fn token_needs_refresh(&self, user_id: &str) -> bool {
        if let Some(token) = self.stored_tokens.get(user_id) {
            if let Some(expires_at) = token.expires_at {
                // Refresh if token expires within 5 minutes
                let threshold = chrono::Utc::now() + chrono::Duration::minutes(5);
                expires_at <= threshold
            } else {
                false // No expiry information, assume it's still valid
            }
        } else {
            true // No token stored
        }
    }
    
    /// Auto-refresh token if needed
    pub async fn ensure_valid_token(&mut self, user_id: &str) -> PlatformResult<String> {
        if self.token_needs_refresh(user_id) {
            if let Some(stored_token) = self.stored_tokens.get(user_id).cloned() {
                if let Some(refresh_token) = &stored_token.refresh_token {
                    let new_auth_result = self.refresh_access_token(&stored_token.platform_id, refresh_token).await?;
                    let new_access_token = new_auth_result.access_token.clone();
                    
                    // Update stored token
                    self.store_token(user_id.to_string(), stored_token.platform_id, new_auth_result);
                    
                    return Ok(new_access_token);
                }
            }
            
            return Err(PlatformError::TokenExpired {
                platform: "unknown".to_string(),
            });
        }
        
        // Token is still valid
        if let Some(token) = self.stored_tokens.get(user_id) {
            Ok(token.access_token.clone())
        } else {
            Err(PlatformError::AuthenticationFailed {
                platform: "unknown".to_string(),
                reason: "No token available".to_string(),
            })
        }
    }
    
    /// Revoke stored token
    pub fn revoke_token(&mut self, user_id: &str) {
        self.stored_tokens.remove(user_id);
    }
    
    /// Clear all stored tokens
    pub fn clear_all_tokens(&mut self) {
        self.stored_tokens.clear();
    }
}

/// Generate a cryptographically secure code verifier for PKCE
fn generate_code_verifier() -> String {
    use base64::Engine;
    use rand::RngCore;
    
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Generate code challenge from verifier using SHA256
fn generate_code_challenge(verifier: &str) -> String {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

impl Default for AuthenticationManager {
    fn default() -> Self {
        Self::new()
    }
}