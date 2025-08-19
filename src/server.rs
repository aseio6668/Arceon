use anyhow::Result;
use clap::Parser;
use tracing::{info, warn, error, Level};
use tracing_subscriber;
use std::time::Duration;
use std::sync::Arc;
use warp::Filter;
use serde_json::json;
use uuid::Uuid;

use arceon_core::{ArceonCore, Config, NetworkManager, BlockchainManager};
use arceon_world::WorldManager;

// Import our authentication, database, group, guild, and skill migration systems
mod authentication;
mod database;
mod group_system;
mod guild_system;
mod skill_migration;
use authentication::{AuthenticationManager, LoginRequest, RegisterRequest, create_shared_auth_manager, SharedAuthManager};
use database::{DatabaseManager, DatabaseConfig, create_shared_database_manager, SharedDatabaseManager};
use group_system::{GroupManager, CreateGroupRequest, CreateGroupResponse, InvitePlayerRequest, InvitePlayerResponse, JoinGroupRequest, JoinGroupResponse, SendGroupMessageRequest, SendGroupMessageResponse, GroupStatusResponse, create_shared_group_manager, SharedGroupManager};
use guild_system::{GuildManager, CreateGuildRequest, CreateGuildResponse, InviteToGuildRequest, InviteToGuildResponse, JoinGuildRequest, JoinGuildResponse, GuildStatusResponse, create_shared_guild_manager, SharedGuildManager};
use skill_migration::{SkillMigrationManager, SkillMigrationReport, create_shared_skill_migration_manager, SharedSkillMigrationManager};

#[derive(Parser)]
#[command(author, version, about = "Arceon Game Server")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Server port
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// Network mode
    #[arg(long, default_value = "p2p")]
    network_mode: String,
    
    /// Enable masternode features
    #[arg(long)]
    masternode: bool,
    
    /// Enable debug logging
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let level = if cli.debug { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    info!("🌟 Starting Arceon Game Server");
    info!("===============================");
    
    // Load configuration
    let mut config = Config::load(&cli.config).unwrap_or_else(|_| {
        info!("Could not load config file, using defaults");
        Config::default()
    });
    
    // Override port from CLI
    info!("🌐 Server will run on port: {}", cli.port);
    
    // Initialize core systems
    let mut core = ArceonCore::new_server(config.clone()).await?;
    
    // Initialize blockchain manager
    let blockchain = BlockchainManager;
    
    // Initialize network manager
    let network = NetworkManager;
    
    // Start core systems
    core.start(blockchain, network).await?;
    
    // Generate initial world areas
    let mut world_manager = WorldManager::new();
    let areas = world_manager.generate_espan()?;
    core.add_areas(areas).await?;
    
    info!("✅ Server initialization complete");
    info!("🎮 Players can now connect via GUI or terminal");
    info!("📍 World: Espan with multiple areas loaded");
    info!("🔧 Features: NPC AI, P2P networking, character progression");
    
    // Start server loop
    info!("🚀 Server is now running...");
    info!("🌐 HTTP API available on http://localhost:{}", cli.port);
    info!("Press Ctrl+C to shutdown");
    
    // Create shared core reference for HTTP handlers
    let core_state = core.get_state();
    
    // Initialize database manager
    let db_config = DatabaseConfig::default();
    let database_manager = create_shared_database_manager(db_config)?;
    
    // Initialize database schema
    {
        let mut db = database_manager.write().await;
        db.initialize().await?;
    }
    
    // Initialize authentication manager
    let auth_manager = create_shared_auth_manager();
    
    // Load existing authentication data from database
    {
        let db = database_manager.read().await;
        db.load_auth_manager(auth_manager.clone()).await?;
    }
    
    // Initialize group manager
    let group_manager = create_shared_group_manager();
    
    // Initialize guild manager
    let guild_manager = create_shared_guild_manager();
    
    // Initialize skill migration manager
    let skill_migration_manager = create_shared_skill_migration_manager(auth_manager.clone(), database_manager.clone());
    
    info!("✅ Database, authentication, group, guild, and skill migration systems initialized");
    
    // Create HTTP API routes
    let api = warp::path("api");
    
    // Health check endpoint
    let health = api
        .and(warp::path("health"))
        .and(warp::path::end())
        .map(|| {
            warp::reply::json(&json!({
                "status": "ok",
                "server": "arceon",
                "version": "0.1.0",
                "features": ["authentication", "player_management", "world_state", "groups", "guilds", "skill_migration"]
            }))
        });
    
    // Authentication endpoints
    let register = api
        .and(warp::path("auth"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_auth(auth_manager.clone()))
        .and(with_database(database_manager.clone()))
        .and_then(handle_register);
    
    let login = api
        .and(warp::path("auth"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_auth(auth_manager.clone()))
        .and(with_database(database_manager.clone()))
        .and_then(handle_login);
    
    let logout = api
        .and(warp::path("auth"))
        .and(warp::path("logout"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and_then(handle_logout);
    
    // Create player endpoint (now requires authentication)
    let create_player = api
        .and(warp::path("players"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_core(core_state.clone()))
        .and(with_auth(auth_manager.clone()))
        .and_then(handle_create_player);
    
    // Process command endpoint (now requires authentication)
    let process_command = api
        .and(warp::path("commands"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_core(core_state.clone()))
        .and(with_auth(auth_manager.clone()))
        .and_then(handle_process_command);
    
    // User profile endpoint
    let user_profile = api
        .and(warp::path("user"))
        .and(warp::path("profile"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and_then(handle_user_profile);
    
    // Wallet binding endpoints
    let bind_wallet = api
        .and(warp::path("wallet"))
        .and(warp::path("bind"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_database(database_manager.clone()))
        .and_then(handle_bind_wallet);
    
    let unbind_wallet = api
        .and(warp::path("wallet"))
        .and(warp::path("unbind"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_database(database_manager.clone()))
        .and_then(handle_unbind_wallet);
    
    let wallet_status = api
        .and(warp::path("wallet"))
        .and(warp::path("status"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and_then(handle_wallet_status);
    
    // Group management endpoints
    let create_group = api
        .and(warp::path("groups"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_group_manager(group_manager.clone()))
        .and_then(handle_create_group);
    
    let invite_to_group = api
        .and(warp::path("groups"))
        .and(warp::path("invite"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_group_manager(group_manager.clone()))
        .and_then(handle_invite_to_group);
    
    let join_group = api
        .and(warp::path("groups"))
        .and(warp::path("join"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_group_manager(group_manager.clone()))
        .and_then(handle_join_group);
    
    let leave_group = api
        .and(warp::path("groups"))
        .and(warp::path("leave"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_group_manager(group_manager.clone()))
        .and_then(handle_leave_group);
    
    let send_group_message = api
        .and(warp::path("groups"))
        .and(warp::path("message"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_group_manager(group_manager.clone()))
        .and_then(handle_send_group_message);
    
    let group_status = api
        .and(warp::path("groups"))
        .and(warp::path("status"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_group_manager(group_manager.clone()))
        .and_then(handle_group_status);
    
    // Guild management endpoints
    let create_guild = api
        .and(warp::path("guilds"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_guild_manager(guild_manager.clone()))
        .and_then(handle_create_guild);
    
    let invite_to_guild = api
        .and(warp::path("guilds"))
        .and(warp::path("invite"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_guild_manager(guild_manager.clone()))
        .and_then(handle_invite_to_guild);
    
    let join_guild = api
        .and(warp::path("guilds"))
        .and(warp::path("join"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_guild_manager(guild_manager.clone()))
        .and_then(handle_join_guild);
    
    let guild_status = api
        .and(warp::path("guilds"))
        .and(warp::path("status"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_guild_manager(guild_manager.clone()))
        .and_then(handle_guild_status);
    
    // Skill migration endpoint
    let migrate_skills = api
        .and(warp::path("admin"))
        .and(warp::path("migrate-skills"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::<String>("session-id"))
        .and(with_auth(auth_manager.clone()))
        .and(with_skill_migration_manager(skill_migration_manager.clone()))
        .and_then(handle_migrate_skills);
    
    // Combine routes
    let routes = health
        .or(register)
        .or(login)
        .or(logout)
        .or(create_player)
        .or(process_command)
        .or(user_profile)
        .or(bind_wallet)
        .or(unbind_wallet)
        .or(wallet_status)
        .or(create_group)
        .or(invite_to_group)
        .or(join_group)
        .or(leave_group)
        .or(send_group_message)
        .or(group_status)
        .or(create_guild)
        .or(invite_to_guild)
        .or(join_guild)
        .or(guild_status)
        .or(migrate_skills)
        .with(warp::cors().allow_any_origin().allow_headers(vec!["content-type", "session-id"]).allow_methods(vec!["GET", "POST"]));
    
    // Start HTTP server on a separate task
    let mut http_server = tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], cli.port)));
    
    // Main server loop
    let mut tick_interval = tokio::time::interval(Duration::from_millis(100));
    
    loop {
        tokio::select! {
            _ = tick_interval.tick() => {
                // Update core systems
                core.update();
                
                // Note: World state updates are handled internally
                // Full NPC behavior updates will be added in future versions
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Shutdown signal received");
                break;
            }
            _ = &mut http_server => {
                info!("HTTP server stopped");
                break;
            }
        }
    }
    
    info!("🛑 Shutting down Arceon server...");
    info!("👋 Goodbye!");
    
    Ok(())
}

// HTTP API types and handlers
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

#[derive(Deserialize)]
struct CreatePlayerRequest {
    character_name: String,
    race: String,
    starting_area: Option<String>,
}

#[derive(Serialize)]
struct CreatePlayerResponse {
    success: bool,
    character_id: Option<String>,
    message: String,
    character_slot: Option<u8>,
}

#[derive(Deserialize)]
struct ProcessCommandRequest {
    command: String,
    character_id: Option<String>,
}

#[derive(Serialize)]
struct ProcessCommandResponse {
    success: bool,
    response: String,
    game_state_update: Option<String>,
}

#[derive(Serialize)]
struct UserProfileResponse {
    success: bool,
    username: String,
    user_id: String,
    characters: Vec<CharacterSummary>,
    account_status: String,
    wallet_bound: bool,
}

#[derive(Deserialize)]
struct BindWalletRequest {
    wallet_address: String,
    signature: Option<String>, // Optional signature for verification
}

#[derive(Serialize)]
struct BindWalletResponse {
    success: bool,
    message: String,
    wallet_address: Option<String>,
}

#[derive(Deserialize)]
struct UnbindWalletRequest {
    confirm: bool,
    signature: Option<String>, // Optional signature for verification
}

#[derive(Serialize)]
struct UnbindWalletResponse {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct CharacterSummary {
    slot_id: u8,
    character_id: Option<String>,
    character_name: Option<String>,
    race: Option<String>,
    level: Option<u32>,
    last_played: Option<String>,
}

// Helper function to inject core state into handlers
fn with_core(
    core_state: Arc<RwLock<arceon_core::state::GameState>>
) -> impl Filter<Extract = (Arc<RwLock<arceon_core::state::GameState>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || core_state.clone())
}

// Helper function to inject auth manager into handlers
fn with_auth(
    auth_manager: SharedAuthManager
) -> impl Filter<Extract = (SharedAuthManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || auth_manager.clone())
}

// Helper function to inject database manager into handlers
fn with_database(
    database_manager: SharedDatabaseManager
) -> impl Filter<Extract = (SharedDatabaseManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || database_manager.clone())
}

// Helper function to inject group manager into handlers
fn with_group_manager(
    group_manager: SharedGroupManager
) -> impl Filter<Extract = (SharedGroupManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || group_manager.clone())
}

// Helper function to inject guild manager into handlers
fn with_guild_manager(
    guild_manager: SharedGuildManager
) -> impl Filter<Extract = (SharedGuildManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || guild_manager.clone())
}

// Helper function to inject skill migration manager into handlers
fn with_skill_migration_manager(
    skill_migration_manager: SharedSkillMigrationManager
) -> impl Filter<Extract = (SharedSkillMigrationManager,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || skill_migration_manager.clone())
}

// Authentication handlers

async fn handle_register(
    req: RegisterRequest,
    auth_manager: SharedAuthManager,
    database_manager: SharedDatabaseManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🔐 Registration attempt for username: {}", req.username);
    
    let mut auth = auth_manager.write().await;
    match auth.register_user(req).await {
        Ok(response) => {
            if response.success {
                info!("✅ User registration successful: {:?}", response.user_id);
                
                // Save user data to database
                if let Some(user_id) = response.user_id {
                    if let Some(user) = auth.users.get(&user_id.to_string()) {
                        let mut db = database_manager.write().await;
                        if let Err(e) = db.save_user_data(user).await {
                            error!("Failed to save user to database: {}", e);
                        } else {
                            info!("💾 User data saved to database");
                        }
                    }
                }
            } else {
                info!("❌ User registration failed: {}", response.message);
            }
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            error!("💥 Registration error: {}", e);
            Ok(warp::reply::json(&authentication::RegisterResponse {
                success: false,
                user_id: None,
                message: "Registration failed due to internal error".to_string(),
                validation_errors: vec![],
            }))
        }
    }
}

async fn handle_login(
    req: LoginRequest,
    auth_manager: SharedAuthManager,
    database_manager: SharedDatabaseManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🔑 Login attempt for username: {}", req.username);
    
    let mut auth = auth_manager.write().await;
    match auth.login_user(req).await {
        Ok(response) => {
            if response.success {
                info!("✅ User login successful: {}", response.username.as_ref().unwrap_or(&"unknown".to_string()));
                
                // Sync user data to database on successful login
                if let Some(user_id) = response.user_id {
                    if let Some(user) = auth.users.get(&user_id.to_string()) {
                        let mut db = database_manager.write().await;
                        if let Err(e) = db.save_user_data(user).await {
                            warn!("Failed to sync user data to database: {}", e);
                        }
                    }
                }
            } else {
                info!("❌ User login failed: {}", response.message);
            }
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            error!("💥 Login error: {}", e);
            Ok(warp::reply::json(&authentication::LoginResponse {
                success: false,
                session_id: None,
                user_id: None,
                username: None,
                message: "Login failed due to internal error".to_string(),
                account_status: None,
                characters: vec![],
                wallet_bound: false,
            }))
        }
    }
}

async fn handle_logout(
    session_id_str: String,
    auth_manager: SharedAuthManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("👋 Logout attempt for session: {}", session_id_str);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        match auth.logout_user(session_id).await {
            Ok(success) => {
                if success {
                    info!("✅ User logout successful");
                    Ok(warp::reply::json(&json!({
                        "success": true,
                        "message": "Logged out successfully"
                    })))
                } else {
                    info!("❌ Session not found for logout");
                    Ok(warp::reply::json(&json!({
                        "success": false,
                        "message": "Session not found"
                    })))
                }
            }
            Err(e) => {
                info!("💥 Logout error: {}", e);
                Ok(warp::reply::json(&json!({
                    "success": false,
                    "message": "Logout failed"
                })))
            }
        }
    } else {
        Ok(warp::reply::json(&json!({
            "success": false,
            "message": "Invalid session ID format"
        })))
    }
}

// Game handlers (now with authentication)

async fn handle_create_player(
    req: CreatePlayerRequest,
    session_id_str: String,
    _core_state: Arc<RwLock<arceon_core::state::GameState>>,
    auth_manager: SharedAuthManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🎮 Creating character: {} ({})", req.character_name, req.race);
    
    // Validate session
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            // Find available character slot
            if let Some(user) = auth.users.get_mut(&session.user_id.to_string()) {
                for slot in &mut user.character_slots {
                    if slot.character_id.is_none() {
                        // Create character
                        let character_id = Uuid::new_v4();
                        slot.character_id = Some(character_id);
                        slot.character_name = Some(req.character_name.clone());
                        slot.character_race = Some(req.race.clone());
                        slot.character_level = Some(1);
                        slot.last_played = Some(std::time::SystemTime::now());
                        slot.is_active = true;
                        
                        info!("✅ Character created successfully: {} (ID: {})", req.character_name, character_id);
                        
                        return Ok(warp::reply::json(&CreatePlayerResponse {
                            success: true,
                            character_id: Some(character_id.to_string()),
                            message: format!("Character '{}' created successfully", req.character_name),
                            character_slot: Some(slot.slot_id),
                        }));
                    }
                }
                
                // No available slots
                Ok(warp::reply::json(&CreatePlayerResponse {
                    success: false,
                    character_id: None,
                    message: "No available character slots".to_string(),
                    character_slot: None,
                }))
            } else {
                Ok(warp::reply::json(&CreatePlayerResponse {
                    success: false,
                    character_id: None,
                    message: "User account not found".to_string(),
                    character_slot: None,
                }))
            }
        } else {
            Ok(warp::reply::json(&CreatePlayerResponse {
                success: false,
                character_id: None,
                message: "Invalid or expired session".to_string(),
                character_slot: None,
            }))
        }
    } else {
        Ok(warp::reply::json(&CreatePlayerResponse {
            success: false,
            character_id: None,
            message: "Invalid session ID format".to_string(),
            character_slot: None,
        }))
    }
}

async fn handle_process_command(
    req: ProcessCommandRequest,
    session_id_str: String,
    core_state: Arc<RwLock<arceon_core::state::GameState>>,
    auth_manager: SharedAuthManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("⚡ Processing command: {}", req.command);
    
    // Validate session
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            // Process command with game state
            let response_text = match req.command.as_str() {
                "look" => {
                    let state = core_state.read().await;
                    if state.areas.is_empty() {
                        "World is still loading...".to_string()
                    } else {
                        let first_area = state.areas.values().next().unwrap();
                        format!("You are in {}.\n{}", first_area.name, first_area.description)
                    }
                },
                "who" => {
                    let state = core_state.read().await;
                    format!("Players online ({}): Connected via authenticated API", state.online_players.len())
                },
                "stats" => {
                    if let Some(user) = auth.users.get(&session.user_id.to_string()) {
                        let active_character = user.character_slots.iter()
                            .find(|slot| slot.is_active && slot.character_id.is_some())
                            .and_then(|slot| slot.character_name.as_ref())
                            .map(|name| name.as_str())
                            .unwrap_or("No active character");
                        
                        format!("=== {} ===\nCharacter: {}\nHealth: 100/100\nMana: 50/50\nLevel: 1\n\n✅ Authenticated session active!", 
                            session.username,
                            active_character)
                    } else {
                        "Character stats unavailable".to_string()
                    }
                },
                "profile" => {
                    if let Some(user) = auth.users.get(&session.user_id.to_string()) {
                        let character_count = user.character_slots.iter().filter(|slot| slot.character_id.is_some()).count();
                        format!("=== Profile: {} ===\nAccount ID: {}\nCharacters: {}\nWallet Bound: {}\nLast Login: {:?}",
                            user.username,
                            user.user_id,
                            character_count,
                            user.wallet_address.is_some(),
                            user.last_login)
                    } else {
                        "Profile unavailable".to_string()
                    }
                },
                "skills" => {
                    if let Some(user) = auth.users.get(&session.user_id.to_string()) {
                        let active_character = user.character_slots.iter()
                            .find(|slot| slot.is_active && slot.character_id.is_some());
                        
                        if let Some(character_slot) = active_character {
                            let character_name = character_slot.character_name.as_ref()
                                .map(|name| name.as_str())
                                .unwrap_or("Unknown");
                            
                            format!("=== Skills: {} ===\n\
                                🗡️  Combat Skills: Available\n\
                                🔨 Crafting Skills: Available\n\
                                🌿 Gathering Skills: Available\n\
                                💬 Social Skills: Available\n\
                                ✨ Magic Skills: Available\n\
                                🏃 Survival Skills: Available\n\n\
                                💡 Skill system updated with 24 new skills!\n\
                                📊 Use 'POST /api/admin/migrate-skills' to update all characters\n\
                                🎯 Skills include: Combat, Crafting, Magic, Social, and Survival categories\n\n\
                                ✅ Modern skill system with continuous progression!", character_name)
                        } else {
                            "No active character found. Create a character first!".to_string()
                        }
                    } else {
                        "Skills unavailable - user not found".to_string()
                    }
                },
                _ => format!("🎮 Command processed: {}\n✅ Authenticated server connection active!", req.command),
            };
            
            Ok(warp::reply::json(&ProcessCommandResponse {
                success: true,
                response: response_text,
                game_state_update: None,
            }))
        } else {
            Ok(warp::reply::json(&ProcessCommandResponse {
                success: false,
                response: "Session invalid or expired. Please log in again.".to_string(),
                game_state_update: None,
            }))
        }
    } else {
        Ok(warp::reply::json(&ProcessCommandResponse {
            success: false,
            response: "Invalid session ID format".to_string(),
            game_state_update: None,
        }))
    }
}

async fn handle_user_profile(
    session_id_str: String,
    auth_manager: SharedAuthManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("👤 User profile request for session: {}", session_id_str);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            if let Some(user) = auth.users.get(&session.user_id.to_string()) {
                let characters: Vec<CharacterSummary> = user.character_slots.iter()
                    .map(|slot| CharacterSummary {
                        slot_id: slot.slot_id,
                        character_id: slot.character_id.map(|id| id.to_string()),
                        character_name: slot.character_name.clone(),
                        race: slot.character_race.clone(),
                        level: slot.character_level,
                        last_played: slot.last_played.map(|time| format!("{:?}", time)),
                    })
                    .collect();
                
                let response = UserProfileResponse {
                    success: true,
                    username: user.username.clone(),
                    user_id: user.user_id.to_string(),
                    characters,
                    account_status: format!("{:?}", user.account_status),
                    wallet_bound: user.wallet_address.is_some(),
                };
                
                Ok(warp::reply::json(&response))
            } else {
                Ok(warp::reply::json(&UserProfileResponse {
                    success: false,
                    username: "Unknown".to_string(),
                    user_id: "Unknown".to_string(),
                    characters: vec![],
                    account_status: "Error".to_string(),
                    wallet_bound: false,
                }))
            }
        } else {
            Ok(warp::reply::json(&UserProfileResponse {
                success: false,
                username: "Invalid Session".to_string(),
                user_id: "Invalid Session".to_string(),
                characters: vec![],
                account_status: "Session Expired".to_string(),
                wallet_bound: false,
            }))
        }
    } else {
        Ok(warp::reply::json(&UserProfileResponse {
            success: false,
            username: "Invalid Request".to_string(),
            user_id: "Invalid Request".to_string(),
            characters: vec![],
            account_status: "Invalid Session ID".to_string(),
            wallet_bound: false,
        }))
    }
}

// Wallet binding handlers

async fn handle_bind_wallet(
    req: BindWalletRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    database_manager: SharedDatabaseManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🔗 Wallet binding request for address: {}", req.wallet_address);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            // Validate wallet address format (basic validation)
            if !is_valid_wallet_address(&req.wallet_address) {
                return Ok(warp::reply::json(&BindWalletResponse {
                    success: false,
                    message: "Invalid wallet address format".to_string(),
                    wallet_address: None,
                }));
            }
            
            // Check if user already has a wallet bound
            if let Some(user) = auth.users.get(&session.user_id.to_string()) {
                if user.wallet_address.is_some() {
                    return Ok(warp::reply::json(&BindWalletResponse {
                        success: false,
                        message: "Account already has a wallet bound. Unbind first to change.".to_string(),
                        wallet_address: user.wallet_address.clone(),
                    }));
                }
            }
            
            // Attempt to bind wallet
            match auth.bind_wallet(session.user_id, req.wallet_address.clone()).await {
                Ok(true) => {
                    // Save updated user data to database
                    if let Some(user) = auth.users.get(&session.user_id.to_string()) {
                        let mut db = database_manager.write().await;
                        if let Err(e) = db.save_user_data(user).await {
                            error!("Failed to save wallet binding to database: {}", e);
                        }
                    }
                    
                    info!("✅ Wallet bound successfully: {} -> {}", session.username, req.wallet_address);
                    Ok(warp::reply::json(&BindWalletResponse {
                        success: true,
                        message: "Wallet bound successfully! You now have access to decentralized features.".to_string(),
                        wallet_address: Some(req.wallet_address),
                    }))
                },
                Ok(false) => {
                    Ok(warp::reply::json(&BindWalletResponse {
                        success: false,
                        message: "Wallet address is already bound to another account".to_string(),
                        wallet_address: None,
                    }))
                },
                Err(e) => {
                    error!("Failed to bind wallet: {}", e);
                    Ok(warp::reply::json(&BindWalletResponse {
                        success: false,
                        message: "Failed to bind wallet due to internal error".to_string(),
                        wallet_address: None,
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&BindWalletResponse {
                success: false,
                message: "Invalid or expired session".to_string(),
                wallet_address: None,
            }))
        }
    } else {
        Ok(warp::reply::json(&BindWalletResponse {
            success: false,
            message: "Invalid session ID format".to_string(),
            wallet_address: None,
        }))
    }
}

async fn handle_unbind_wallet(
    req: UnbindWalletRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    database_manager: SharedDatabaseManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🔓 Wallet unbinding request");
    
    if !req.confirm {
        return Ok(warp::reply::json(&UnbindWalletResponse {
            success: false,
            message: "Confirmation required to unbind wallet".to_string(),
        }));
    }
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            if let Some(user) = auth.users.get_mut(&session.user_id.to_string()) {
                match &user.wallet_address {
                    Some(wallet_addr) => {
                        let old_wallet = wallet_addr.clone();
                        user.wallet_address = None;
                        
                        // Save updated user data to database
                        let mut db = database_manager.write().await;
                        if let Err(e) = db.save_user_data(user).await {
                            error!("Failed to save wallet unbinding to database: {}", e);
                        }
                        
                        info!("✅ Wallet unbound successfully: {} (was: {})", session.username, old_wallet);
                        Ok(warp::reply::json(&UnbindWalletResponse {
                            success: true,
                            message: "Wallet unbound successfully. Decentralized features are now disabled.".to_string(),
                        }))
                    },
                    None => {
                        Ok(warp::reply::json(&UnbindWalletResponse {
                            success: false,
                            message: "No wallet is currently bound to this account".to_string(),
                        }))
                    }
                }
            } else {
                Ok(warp::reply::json(&UnbindWalletResponse {
                    success: false,
                    message: "User account not found".to_string(),
                }))
            }
        } else {
            Ok(warp::reply::json(&UnbindWalletResponse {
                success: false,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&UnbindWalletResponse {
            success: false,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_wallet_status(
    session_id_str: String,
    auth_manager: SharedAuthManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("💳 Wallet status request");
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            if let Some(user) = auth.users.get(&session.user_id.to_string()) {
                let response = json!({
                    "success": true,
                    "wallet_bound": user.wallet_address.is_some(),
                    "wallet_address": user.wallet_address,
                    "decentralized_features_enabled": user.wallet_address.is_some(),
                    "message": if user.wallet_address.is_some() {
                        "Wallet is bound - decentralized features available"
                    } else {
                        "No wallet bound - only centralized features available"
                    }
                });
                Ok(warp::reply::json(&response))
            } else {
                Ok(warp::reply::json(&json!({
                    "success": false,
                    "message": "User account not found"
                })))
            }
        } else {
            Ok(warp::reply::json(&json!({
                "success": false,
                "message": "Invalid or expired session"
            })))
        }
    } else {
        Ok(warp::reply::json(&json!({
            "success": false,
            "message": "Invalid session ID format"
        })))
    }
}

// Helper function to validate wallet address format
fn is_valid_wallet_address(address: &str) -> bool {
    // Basic validation - in a real implementation, this would validate specific formats
    // for different blockchain networks (Bitcoin, Ethereum, etc.)
    if address.is_empty() || address.len() < 20 || address.len() > 100 {
        return false;
    }
    
    // Check if it contains only valid characters (alphanumeric)
    address.chars().all(|c| c.is_alphanumeric())
}

// Group management handlers

async fn handle_create_group(
    req: CreateGroupRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    group_manager: SharedGroupManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🎯 Group creation request: {}", req.name);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            let mut groups = group_manager.write().await;
            match groups.create_group(session.user_id, session.username.clone(), req).await {
                Ok(response) => {
                    if response.success {
                        info!("✅ Group created successfully by: {}", session.username);
                    }
                    Ok(warp::reply::json(&response))
                },
                Err(e) => {
                    error!("Failed to create group: {}", e);
                    Ok(warp::reply::json(&CreateGroupResponse {
                        success: false,
                        group_id: None,
                        message: "Failed to create group due to internal error".to_string(),
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&CreateGroupResponse {
                success: false,
                group_id: None,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&CreateGroupResponse {
            success: false,
            group_id: None,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_invite_to_group(
    req: InvitePlayerRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    group_manager: SharedGroupManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("📨 Group invitation request: {} to group {}", req.player_username, req.group_id);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            // Find the invited player by username
            let invited_player = auth.users.values()
                .find(|user| user.username.to_lowercase() == req.player_username.to_lowercase())
                .cloned();
            
            match invited_player {
                Some(player) => {
                    let mut groups = group_manager.write().await;
                    match groups.invite_player(session.user_id, player.user_id, player.username, req).await {
                        Ok(response) => Ok(warp::reply::json(&response)),
                        Err(e) => {
                            error!("Failed to send invitation: {}", e);
                            Ok(warp::reply::json(&InvitePlayerResponse {
                                success: false,
                                invitation_id: None,
                                message: "Failed to send invitation due to internal error".to_string(),
                            }))
                        }
                    }
                },
                None => {
                    Ok(warp::reply::json(&InvitePlayerResponse {
                        success: false,
                        invitation_id: None,
                        message: "Player not found".to_string(),
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&InvitePlayerResponse {
                success: false,
                invitation_id: None,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&InvitePlayerResponse {
            success: false,
            invitation_id: None,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_join_group(
    req: JoinGroupRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    group_manager: SharedGroupManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("📬 Group join response: {} (accept: {})", req.invitation_id, req.accept);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            let mut groups = group_manager.write().await;
            match groups.respond_to_invitation(session.user_id, session.username.clone(), req).await {
                Ok(response) => Ok(warp::reply::json(&response)),
                Err(e) => {
                    error!("Failed to respond to invitation: {}", e);
                    Ok(warp::reply::json(&JoinGroupResponse {
                        success: false,
                        group_id: None,
                        message: "Failed to respond to invitation due to internal error".to_string(),
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&JoinGroupResponse {
                success: false,
                group_id: None,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&JoinGroupResponse {
            success: false,
            group_id: None,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_leave_group(
    group_id_str: String,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    group_manager: SharedGroupManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("👋 Group leave request: {}", group_id_str);
    
    let group_id = match Uuid::parse_str(&group_id_str) {
        Ok(id) => id,
        Err(_) => return Ok(warp::reply::json(&json!({
            "success": false,
            "message": "Invalid group ID format"
        }))),
    };
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            let mut groups = group_manager.write().await;
            match groups.leave_group(session.user_id, group_id).await {
                Ok(true) => {
                    Ok(warp::reply::json(&json!({
                        "success": true,
                        "message": "Successfully left the group"
                    })))
                },
                Ok(false) => {
                    Ok(warp::reply::json(&json!({
                        "success": false,
                        "message": "You are not a member of this group"
                    })))
                },
                Err(e) => {
                    error!("Failed to leave group: {}", e);
                    Ok(warp::reply::json(&json!({
                        "success": false,
                        "message": "Failed to leave group due to internal error"
                    })))
                }
            }
        } else {
            Ok(warp::reply::json(&json!({
                "success": false,
                "message": "Invalid or expired session"
            })))
        }
    } else {
        Ok(warp::reply::json(&json!({
            "success": false,
            "message": "Invalid session ID format"
        })))
    }
}

async fn handle_send_group_message(
    req: SendGroupMessageRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    group_manager: SharedGroupManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("💬 Group message request: {} characters to group {}", req.content.len(), req.group_id);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            let mut groups = group_manager.write().await;
            match groups.send_message(session.user_id, req).await {
                Ok(response) => Ok(warp::reply::json(&response)),
                Err(e) => {
                    error!("Failed to send group message: {}", e);
                    Ok(warp::reply::json(&SendGroupMessageResponse {
                        success: false,
                        message_id: None,
                        message: "Failed to send message due to internal error".to_string(),
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&SendGroupMessageResponse {
                success: false,
                message_id: None,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&SendGroupMessageResponse {
            success: false,
            message_id: None,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_group_status(
    session_id_str: String,
    auth_manager: SharedAuthManager,
    group_manager: SharedGroupManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("📊 Group status request");
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            let groups = group_manager.read().await;
            match groups.get_player_status(session.user_id).await {
                Ok(response) => Ok(warp::reply::json(&response)),
                Err(e) => {
                    error!("Failed to get group status: {}", e);
                    Ok(warp::reply::json(&GroupStatusResponse {
                        success: false,
                        groups: vec![],
                        invitations: vec![],
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&GroupStatusResponse {
                success: false,
                groups: vec![],
                invitations: vec![],
            }))
        }
    } else {
        Ok(warp::reply::json(&GroupStatusResponse {
            success: false,
            groups: vec![],
            invitations: vec![],
        }))
    }
}

// Guild management handlers

async fn handle_create_guild(
    req: CreateGuildRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    guild_manager: SharedGuildManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🏰 Guild creation request: {} [{}]", req.name, req.tag);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            let mut guilds = guild_manager.write().await;
            match guilds.create_guild(session.user_id, session.username.clone(), req).await {
                Ok(response) => {
                    if response.success {
                        info!("✅ Guild created successfully by: {}", session.username);
                    }
                    Ok(warp::reply::json(&response))
                },
                Err(e) => {
                    error!("Failed to create guild: {}", e);
                    Ok(warp::reply::json(&CreateGuildResponse {
                        success: false,
                        guild_id: None,
                        message: "Failed to create guild due to internal error".to_string(),
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&CreateGuildResponse {
                success: false,
                guild_id: None,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&CreateGuildResponse {
            success: false,
            guild_id: None,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_invite_to_guild(
    req: InviteToGuildRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    guild_manager: SharedGuildManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("📨 Guild invitation request: {} to guild {}", req.player_username, req.guild_id);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            // Find the invited player by username
            let invited_player = auth.users.values()
                .find(|user| user.username.to_lowercase() == req.player_username.to_lowercase())
                .cloned();
            
            match invited_player {
                Some(player) => {
                    let mut guilds = guild_manager.write().await;
                    match guilds.invite_to_guild(session.user_id, player.user_id, player.username, req).await {
                        Ok(response) => Ok(warp::reply::json(&response)),
                        Err(e) => {
                            error!("Failed to send guild invitation: {}", e);
                            Ok(warp::reply::json(&InviteToGuildResponse {
                                success: false,
                                invitation_id: None,
                                message: "Failed to send invitation due to internal error".to_string(),
                            }))
                        }
                    }
                },
                None => {
                    Ok(warp::reply::json(&InviteToGuildResponse {
                        success: false,
                        invitation_id: None,
                        message: "Player not found".to_string(),
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&InviteToGuildResponse {
                success: false,
                invitation_id: None,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&InviteToGuildResponse {
            success: false,
            invitation_id: None,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_join_guild(
    req: JoinGuildRequest,
    session_id_str: String,
    auth_manager: SharedAuthManager,
    guild_manager: SharedGuildManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("📬 Guild join response: {} (accept: {})", req.invitation_id, req.accept);
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            // Implementation for responding to guild invitation would go here
            // For now, return a placeholder response
            Ok(warp::reply::json(&JoinGuildResponse {
                success: false,
                guild_id: None,
                message: "Guild invitation response system not yet implemented".to_string(),
            }))
        } else {
            Ok(warp::reply::json(&JoinGuildResponse {
                success: false,
                guild_id: None,
                message: "Invalid or expired session".to_string(),
            }))
        }
    } else {
        Ok(warp::reply::json(&JoinGuildResponse {
            success: false,
            guild_id: None,
            message: "Invalid session ID format".to_string(),
        }))
    }
}

async fn handle_guild_status(
    session_id_str: String,
    auth_manager: SharedAuthManager,
    guild_manager: SharedGuildManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("📊 Guild status request");
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            let guilds = guild_manager.read().await;
            match guilds.get_guild_status(session.user_id).await {
                Ok(response) => Ok(warp::reply::json(&response)),
                Err(e) => {
                    error!("Failed to get guild status: {}", e);
                    Ok(warp::reply::json(&GuildStatusResponse {
                        success: false,
                        guild: None,
                        invitations: vec![],
                        available_guilds: vec![],
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&GuildStatusResponse {
                success: false,
                guild: None,
                invitations: vec![],
                available_guilds: vec![],
            }))
        }
    } else {
        Ok(warp::reply::json(&GuildStatusResponse {
            success: false,
            guild: None,
            invitations: vec![],
            available_guilds: vec![],
        }))
    }
}

// Skill migration handler

async fn handle_migrate_skills(
    session_id_str: String,
    auth_manager: SharedAuthManager,
    skill_migration_manager: SharedSkillMigrationManager
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("🔄 Skill migration request initiated");
    
    if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
        let mut auth = auth_manager.write().await;
        if let Ok(Some(session)) = auth.validate_session(session_id).await {
            // Check if user has admin permissions (you can implement proper admin checks)
            // For now, any authenticated user can trigger migration
            drop(auth); // Release the lock before starting migration
            
            let migration_manager = skill_migration_manager.read().await;
            match migration_manager.migrate_all_players().await {
                Ok(report) => {
                    if report.success {
                        info!("✅ Skill migration completed successfully for user: {}", session.username);
                        info!("📊 Migration Report: {} players processed, {} updated", 
                            report.total_players_processed, report.players_updated);
                    } else {
                        warn!("⚠️ Skill migration completed with errors for user: {}", session.username);
                    }
                    Ok(warp::reply::json(&report))
                },
                Err(e) => {
                    error!("Failed to run skill migration: {}", e);
                    Ok(warp::reply::json(&SkillMigrationReport {
                        success: false,
                        total_players_processed: 0,
                        players_updated: 0,
                        new_skills_added: vec![],
                        errors: vec![format!("Migration failed: {}", e)],
                        processing_time_ms: 0,
                    }))
                }
            }
        } else {
            Ok(warp::reply::json(&SkillMigrationReport {
                success: false,
                total_players_processed: 0,
                players_updated: 0,
                new_skills_added: vec![],
                errors: vec!["Invalid or expired session".to_string()],
                processing_time_ms: 0,
            }))
        }
    } else {
        Ok(warp::reply::json(&SkillMigrationReport {
            success: false,
            total_players_processed: 0,
            players_updated: 0,
            new_skills_added: vec![],
            errors: vec!["Invalid session ID format".to_string()],
            processing_time_ms: 0,
        }))
    }
}