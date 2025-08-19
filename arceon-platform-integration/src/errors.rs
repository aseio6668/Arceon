/*!
# Platform Integration Error Types

Comprehensive error handling for platform integration operations.
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Platform not found: {platform}")]
    PlatformNotFound { platform: String },
    
    #[error("Authentication failed for platform {platform}: {reason}")]
    AuthenticationFailed { platform: String, reason: String },
    
    #[error("Platform API error: {message} (Status: {status_code})")]
    ApiError { message: String, status_code: u16 },
    
    #[error("Invalid configuration for platform {platform}: {details}")]
    InvalidConfiguration { platform: String, details: String },
    
    #[error("Platform capability not supported: {capability}")]
    UnsupportedCapability { capability: String },
    
    #[error("Token expired for platform {platform}")]
    TokenExpired { platform: String },
    
    #[error("Rate limit exceeded for platform {platform}. Retry after: {retry_after}s")]
    RateLimitExceeded { platform: String, retry_after: u64 },
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Windows API error: {message}")]
    WindowsApiError { message: String },
    
    #[error("Registry error: {message}")]
    RegistryError { message: String },
    
    #[error("Steam error: {message}")]
    SteamError { message: String },
    
    #[error("Epic Games error: {message}")]
    EpicGamesError { message: String },
    
    #[error("Launcher error: {message}")]
    LauncherError { message: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Generic platform error: {0}")]
    GenericError(#[from] anyhow::Error),
}