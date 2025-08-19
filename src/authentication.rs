use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use std::sync::Arc;

/// Comprehensive authentication system for Arceon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationManager {
    pub users: HashMap<String, UserAccount>,
    pub sessions: HashMap<Uuid, UserSession>,
    pub username_registry: HashMap<String, Uuid>, // lowercase username -> user_id
    pub authentication_config: AuthConfig,
    pub failed_attempts: HashMap<String, FailedAttemptTracker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub user_id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub email: Option<String>,
    pub wallet_address: Option<String>, // Optional decentralized wallet binding
    pub created_at: SystemTime,
    pub last_login: Option<SystemTime>,
    pub login_count: u64,
    pub account_status: AccountStatus,
    pub character_slots: Vec<CharacterSlot>,
    pub account_settings: AccountSettings,
    pub two_factor_enabled: bool,
    pub recovery_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSlot {
    pub slot_id: u8,
    pub character_id: Option<Uuid>,
    pub character_name: Option<String>,
    pub character_race: Option<String>,
    pub character_level: Option<u32>,
    pub last_played: Option<SystemTime>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    pub auto_save_interval: Duration,
    pub chat_filter_enabled: bool,
    pub privacy_mode: PrivacyMode,
    pub notification_preferences: NotificationSettings,
    pub ui_preferences: UiSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyMode {
    Public,     // Profile visible to all
    Friends,    // Profile visible to friends only
    Private,    // Profile hidden
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub guild_messages: bool,
    pub trade_requests: bool,
    pub friend_requests: bool,
    pub system_alerts: bool,
    pub email_notifications: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub theme: String,
    pub font_size: u8,
    pub sound_enabled: bool,
    pub music_enabled: bool,
    pub auto_target: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Suspended { until: SystemTime, reason: String },
    Banned { reason: String },
    PendingVerification,
    PasswordResetRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub created_at: SystemTime,
    pub last_activity: SystemTime,
    pub ip_address: Option<String>,
    pub character_id: Option<Uuid>,
    pub session_status: SessionStatus,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Idle,
    Expired,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    PlayGame,
    CreateCharacter,
    DeleteCharacter,
    AccessGuilds,
    AccessTrading,
    AccessMasterNode,
    AdminPanel,
    ModerateChat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub min_password_length: usize,
    pub max_password_length: usize,
    pub require_special_chars: bool,
    pub require_numbers: bool,
    pub require_uppercase: bool,
    pub max_login_attempts: u32,
    pub lockout_duration: Duration,
    pub session_timeout: Duration,
    pub password_reset_timeout: Duration,
    pub username_min_length: usize,
    pub username_max_length: usize,
    pub allow_username_reuse: bool,
    pub max_character_slots: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedAttemptTracker {
    pub attempts: u32,
    pub last_attempt: SystemTime,
    pub locked_until: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub ip_address: Option<String>,
    pub remember_me: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub session_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub message: String,
    pub account_status: Option<AccountStatus>,
    pub characters: Vec<CharacterSlot>,
    pub wallet_bound: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub wallet_address: Option<String>,
    pub initial_character_name: Option<String>,
    pub initial_character_race: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub user_id: Option<Uuid>,
    pub message: String,
    pub validation_errors: Vec<String>,
}

impl AuthenticationManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
            username_registry: HashMap::new(),
            authentication_config: AuthConfig::default(),
            failed_attempts: HashMap::new(),
        }
    }

    /// Register a new user account with comprehensive validation
    pub async fn register_user(&mut self, request: RegisterRequest) -> Result<RegisterResponse> {
        info!("🔐 Processing user registration for: {}", request.username);
        
        // Validate username and password
        let validation_errors = self.validate_registration(&request).await?;
        if !validation_errors.is_empty() {
            return Ok(RegisterResponse {
                success: false,
                user_id: None,
                message: "Registration validation failed".to_string(),
                validation_errors,
            });
        }

        // Check for duplicate username (case-insensitive)
        let username_lower = request.username.to_lowercase();
        if self.username_registry.contains_key(&username_lower) {
            return Ok(RegisterResponse {
                success: false,
                user_id: None,
                message: "Username already exists".to_string(),
                validation_errors: vec!["Username is already taken".to_string()],
            });
        }

        // Check for duplicate wallet address if provided
        if let Some(ref wallet) = request.wallet_address {
            if self.is_wallet_already_bound(wallet) {
                return Ok(RegisterResponse {
                    success: false,
                    user_id: None,
                    message: "Wallet address already bound to another account".to_string(),
                    validation_errors: vec!["This wallet address is already associated with another account".to_string()],
                });
            }
        }

        // Generate password hash with salt
        let salt = self.generate_salt();
        let password_hash = self.hash_password(&request.password, &salt)?;

        // Create new user account
        let user_id = Uuid::new_v4();
        let user_account = UserAccount {
            user_id,
            username: request.username.clone(),
            password_hash,
            salt,
            email: request.email,
            wallet_address: request.wallet_address,
            created_at: SystemTime::now(),
            last_login: None,
            login_count: 0,
            account_status: AccountStatus::Active,
            character_slots: self.initialize_character_slots(),
            account_settings: AccountSettings::default(),
            two_factor_enabled: false,
            recovery_key: Some(self.generate_recovery_key()),
        };

        // Store user account
        self.users.insert(user_account.user_id.to_string(), user_account.clone());
        self.username_registry.insert(username_lower, user_id);

        info!("✅ User registered successfully: {} (ID: {})", request.username, user_id);

        Ok(RegisterResponse {
            success: true,
            user_id: Some(user_id),
            message: format!("Account created successfully for '{}'", request.username),
            validation_errors: vec![],
        })
    }

    /// Authenticate user login with security measures
    pub async fn login_user(&mut self, request: LoginRequest) -> Result<LoginResponse> {
        info!("🔑 Processing login attempt for: {}", request.username);

        // Check for rate limiting
        if self.is_user_locked_out(&request.username) {
            let lockout_info = self.failed_attempts.get(&request.username).unwrap();
            let remaining = lockout_info.locked_until.unwrap().duration_since(SystemTime::now())
                .unwrap_or(Duration::from_secs(0));
            
            warn!("🔒 Login attempt for locked account: {}", request.username);
            return Ok(LoginResponse {
                success: false,
                session_id: None,
                user_id: None,
                username: None,
                message: format!("Account locked. Try again in {} seconds", remaining.as_secs()),
                account_status: None,
                characters: vec![],
                wallet_bound: false,
            });
        }

        // Find user by username (case-insensitive)
        let username_lower = request.username.to_lowercase();
        let user_id = match self.username_registry.get(&username_lower) {
            Some(id) => *id,
            None => {
                self.record_failed_attempt(&request.username);
                return Ok(LoginResponse {
                    success: false,
                    session_id: None,
                    user_id: None,
                    username: None,
                    message: "Invalid username or password".to_string(),
                    account_status: None,
                    characters: vec![],
                    wallet_bound: false,
                });
            }
        };

        let user = match self.users.get(&user_id.to_string()) {
            Some(user) => user.clone(),
            None => {
                error!("🚨 User ID found in registry but not in users: {}", user_id);
                return Ok(LoginResponse {
                    success: false,
                    session_id: None,
                    user_id: None,
                    username: None,
                    message: "Account error. Please contact support.".to_string(),
                    account_status: None,
                    characters: vec![],
                    wallet_bound: false,
                });
            }
        };

        // Check account status
        match &user.account_status {
            AccountStatus::Banned { reason } => {
                warn!("🚫 Login attempt for banned account: {} - {}", request.username, reason);
                return Ok(LoginResponse {
                    success: false,
                    session_id: None,
                    user_id: None,
                    username: None,
                    message: format!("Account banned: {}", reason),
                    account_status: Some(user.account_status.clone()),
                    characters: vec![],
                    wallet_bound: false,
                });
            }
            AccountStatus::Suspended { until, reason } => {
                if SystemTime::now() < *until {
                    warn!("⏸️ Login attempt for suspended account: {} - {}", request.username, reason);
                    return Ok(LoginResponse {
                        success: false,
                        session_id: None,
                        user_id: None,
                        username: None,
                        message: format!("Account suspended until {:?}: {}", until, reason),
                        account_status: Some(user.account_status.clone()),
                        characters: vec![],
                        wallet_bound: false,
                    });
                }
            }
            AccountStatus::PendingVerification => {
                return Ok(LoginResponse {
                    success: false,
                    session_id: None,
                    user_id: None,
                    username: None,
                    message: "Account pending verification. Please check your email.".to_string(),
                    account_status: Some(user.account_status.clone()),
                    characters: vec![],
                    wallet_bound: false,
                });
            }
            _ => {} // Continue with login
        }

        // Verify password
        if !self.verify_password(&request.password, &user.password_hash, &user.salt)? {
            self.record_failed_attempt(&request.username);
            warn!("❌ Invalid password for user: {}", request.username);
            return Ok(LoginResponse {
                success: false,
                session_id: None,
                user_id: None,
                username: None,
                message: "Invalid username or password".to_string(),
                account_status: None,
                characters: vec![],
                wallet_bound: false,
            });
        }

        // Clear any failed attempts
        self.failed_attempts.remove(&request.username);

        // Create session
        let session_id = Uuid::new_v4();
        let session = UserSession {
            session_id,
            user_id: user.user_id,
            username: user.username.clone(),
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            ip_address: request.ip_address,
            character_id: None,
            session_status: SessionStatus::Active,
            permissions: self.get_default_permissions(&user),
        };

        self.sessions.insert(session_id, session);

        // Update user login info
        if let Some(user) = self.users.get_mut(&user_id.to_string()) {
            user.last_login = Some(SystemTime::now());
            user.login_count += 1;
        }

        info!("✅ User logged in successfully: {} (Session: {})", request.username, session_id);

        Ok(LoginResponse {
            success: true,
            session_id: Some(session_id),
            user_id: Some(user.user_id),
            username: Some(user.username),
            message: "Login successful".to_string(),
            account_status: Some(user.account_status),
            characters: user.character_slots,
            wallet_bound: user.wallet_address.is_some(),
        })
    }

    /// Validate session and refresh activity
    pub async fn validate_session(&mut self, session_id: Uuid) -> Result<Option<UserSession>> {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            // Check if session has expired
            let now = SystemTime::now();
            if session.last_activity + self.authentication_config.session_timeout < now {
                session.session_status = SessionStatus::Expired;
                self.sessions.remove(&session_id);
                return Ok(None);
            }

            // Update last activity
            session.last_activity = now;
            session.session_status = SessionStatus::Active;
            
            Ok(Some(session.clone()))
        } else {
            Ok(None)
        }
    }

    /// Logout user and terminate session
    pub async fn logout_user(&mut self, session_id: Uuid) -> Result<bool> {
        if let Some(mut session) = self.sessions.remove(&session_id) {
            session.session_status = SessionStatus::Terminated;
            info!("👋 User logged out: {} (Session: {})", session.username, session_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Bind wallet address to existing account
    pub async fn bind_wallet(&mut self, user_id: Uuid, wallet_address: String) -> Result<bool> {
        // Check if wallet is already bound
        if self.is_wallet_already_bound(&wallet_address) {
            return Ok(false);
        }

        if let Some(user) = self.users.get_mut(&user_id.to_string()) {
            user.wallet_address = Some(wallet_address.clone());
            info!("🔗 Wallet bound to account: {} -> {}", user.username, wallet_address);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get user account by session
    pub async fn get_user_by_session(&self, session_id: Uuid) -> Option<UserAccount> {
        if let Some(session) = self.sessions.get(&session_id) {
            self.users.get(&session.user_id.to_string()).cloned()
        } else {
            None
        }
    }

    /// Change user password with verification
    pub async fn change_password(&mut self, user_id: Uuid, old_password: String, new_password: String) -> Result<bool> {
        let user = match self.users.get(&user_id.to_string()) {
            Some(user) => user.clone(),
            None => return Ok(false),
        };

        // Verify old password
        if !self.verify_password(&old_password, &user.password_hash, &user.salt)? {
            return Ok(false);
        }

        // Validate new password
        let validation_errors = self.validate_password(&new_password);
        if !validation_errors.is_empty() {
            return Ok(false);
        }

        // Update password
        let new_salt = self.generate_salt();
        let new_hash = self.hash_password(&new_password, &new_salt)?;

        if let Some(user) = self.users.get_mut(&user_id.to_string()) {
            user.password_hash = new_hash;
            user.salt = new_salt;
            info!("🔐 Password changed for user: {}", user.username);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Cleanup expired sessions
    pub async fn cleanup_expired_sessions(&mut self) {
        let now = SystemTime::now();
        let expired_sessions: Vec<Uuid> = self.sessions
            .iter()
            .filter(|(_, session)| {
                session.last_activity + self.authentication_config.session_timeout < now
            })
            .map(|(id, _)| *id)
            .collect();

        for session_id in expired_sessions {
            if let Some(session) = self.sessions.remove(&session_id) {
                info!("🧹 Expired session cleaned up: {} ({})", session.username, session_id);
            }
        }
    }

    // Private helper methods

    async fn validate_registration(&self, request: &RegisterRequest) -> Result<Vec<String>> {
        let mut errors = Vec::new();

        // Username validation
        if request.username.len() < self.authentication_config.username_min_length {
            errors.push(format!("Username must be at least {} characters", self.authentication_config.username_min_length));
        }
        if request.username.len() > self.authentication_config.username_max_length {
            errors.push(format!("Username cannot exceed {} characters", self.authentication_config.username_max_length));
        }
        if !request.username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            errors.push("Username can only contain letters, numbers, underscores, and hyphens".to_string());
        }

        // Password validation
        errors.extend(self.validate_password(&request.password));

        // Email validation (if provided)
        if let Some(ref email) = request.email {
            if !email.contains('@') || !email.contains('.') {
                errors.push("Invalid email format".to_string());
            }
        }

        Ok(errors)
    }

    fn validate_password(&self, password: &str) -> Vec<String> {
        let mut errors = Vec::new();

        if password.len() < self.authentication_config.min_password_length {
            errors.push(format!("Password must be at least {} characters", self.authentication_config.min_password_length));
        }
        if password.len() > self.authentication_config.max_password_length {
            errors.push(format!("Password cannot exceed {} characters", self.authentication_config.max_password_length));
        }

        if self.authentication_config.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }
        if self.authentication_config.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            errors.push("Password must contain at least one number".to_string());
        }
        if self.authentication_config.require_special_chars && !password.chars().any(|c| !c.is_alphanumeric()) {
            errors.push("Password must contain at least one special character".to_string());
        }

        errors
    }

    fn generate_salt(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32).map(|_| rng.gen::<u8>()).map(|b| format!("{:02x}", b)).collect()
    }

    fn hash_password(&self, password: &str, salt: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn verify_password(&self, password: &str, hash: &str, salt: &str) -> Result<bool> {
        let computed_hash = self.hash_password(password, salt)?;
        Ok(computed_hash == hash)
    }

    fn is_wallet_already_bound(&self, wallet_address: &str) -> bool {
        self.users.values().any(|user| {
            user.wallet_address.as_ref().map(|addr| addr.as_str()) == Some(wallet_address)
        })
    }

    fn initialize_character_slots(&self) -> Vec<CharacterSlot> {
        (0..self.authentication_config.max_character_slots)
            .map(|i| CharacterSlot {
                slot_id: i,
                character_id: None,
                character_name: None,
                character_race: None,
                character_level: None,
                last_played: None,
                is_active: false,
            })
            .collect()
    }

    fn generate_recovery_key(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..6).map(|_| format!("{:04}", rng.gen_range(0..10000))).collect::<Vec<_>>().join("-")
    }

    fn is_user_locked_out(&self, username: &str) -> bool {
        if let Some(tracker) = self.failed_attempts.get(username) {
            if let Some(locked_until) = tracker.locked_until {
                return SystemTime::now() < locked_until;
            }
        }
        false
    }

    fn record_failed_attempt(&mut self, username: &str) {
        let tracker = self.failed_attempts.entry(username.to_string()).or_insert(FailedAttemptTracker {
            attempts: 0,
            last_attempt: SystemTime::now(),
            locked_until: None,
        });

        tracker.attempts += 1;
        tracker.last_attempt = SystemTime::now();

        if tracker.attempts >= self.authentication_config.max_login_attempts {
            tracker.locked_until = Some(SystemTime::now() + self.authentication_config.lockout_duration);
            warn!("🔒 Account locked due to failed attempts: {} ({} attempts)", username, tracker.attempts);
        }
    }

    fn get_default_permissions(&self, user: &UserAccount) -> Vec<Permission> {
        let mut permissions = vec![
            Permission::PlayGame,
            Permission::CreateCharacter,
            Permission::AccessTrading,
        ];

        if user.wallet_address.is_some() {
            permissions.push(Permission::AccessMasterNode);
        }

        permissions
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            min_password_length: 8,
            max_password_length: 128,
            require_special_chars: true,
            require_numbers: true,
            require_uppercase: true,
            max_login_attempts: 5,
            lockout_duration: Duration::from_secs(900), // 15 minutes
            session_timeout: Duration::from_secs(3600 * 24), // 24 hours
            password_reset_timeout: Duration::from_secs(900), // 15 minutes
            username_min_length: 3,
            username_max_length: 20,
            allow_username_reuse: false,
            max_character_slots: 4,
        }
    }
}

impl Default for AccountSettings {
    fn default() -> Self {
        Self {
            auto_save_interval: Duration::from_secs(300), // 5 minutes
            chat_filter_enabled: false,
            privacy_mode: PrivacyMode::Public,
            notification_preferences: NotificationSettings::default(),
            ui_preferences: UiSettings::default(),
        }
    }
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            guild_messages: true,
            trade_requests: true,
            friend_requests: true,
            system_alerts: true,
            email_notifications: false,
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_size: 12,
            sound_enabled: true,
            music_enabled: true,
            auto_target: false,
        }
    }
}

// Authentication manager wrapper for thread-safety
pub type SharedAuthManager = Arc<RwLock<AuthenticationManager>>;

pub fn create_shared_auth_manager() -> SharedAuthManager {
    Arc::new(RwLock::new(AuthenticationManager::new()))
}