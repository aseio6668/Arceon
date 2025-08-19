/*!
# Platform Integration Demo

Demonstrates comprehensive platform integration capabilities including:
- Epic Games Store integration with OAuth authentication
- Steam integration with Steamworks SDK
- Windows launcher with registry integration and auto-updates
- Generic platform adapter for custom third-party services
- Cross-platform achievement synchronization
- User profile management across platforms
- Analytics collection and reporting

## Features Demonstrated

### Epic Games Integration
- OAuth 2.0 authentication flow with PKCE
- User profile and friends list retrieval
- Achievement synchronization with Epic Games Services
- Content launching through Epic Games launcher

### Steam Integration
- Steamworks SDK integration for authentication
- Steam friends list and social features
- Steam Workshop integration for community content
- Steam overlay and achievement system

### Windows Launcher
- Registry-based application registration
- Desktop and Start Menu shortcut creation
- File association management
- Auto-update system with version checking
- Process management and monitoring

### Generic Platform Adapter
- RESTful API integration for custom platforms
- Configurable authentication methods (OAuth, API Key, Basic Auth)
- Flexible endpoint mapping and response parsing
- Webhook support for real-time events
- Custom handler system for specialized operations

## Usage Examples

```rust
use arceon_platform_integration::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize platform manager
    let mut platform_manager = PlatformManager::new();
    
    // Setup Epic Games integration
    let epic_adapter = EpicGamesAdapter::new();
    let epic_config = PlatformConfig::new(
        PlatformId::new("epic"),
        "your_epic_app_id".to_string()
    ).with_oauth(
        "epic_client_id".to_string(),
        "epic_client_secret".to_string(),
        "http://localhost:8080/auth/epic/callback".to_string()
    );
    
    platform_manager.register_platform(Box::new(epic_adapter)).await?;
    
    // Setup Steam integration
    let steam_adapter = SteamAdapter::new();
    let steam_config = PlatformConfig::new(
        PlatformId::new("steam"),
        "your_steam_app_id".to_string()
    );
    
    platform_manager.register_platform(Box::new(steam_adapter)).await?;
    
    // Auto-authenticate user across all platforms
    let user = platform_manager.authenticate_user_auto(None).await?;
    println!("Authenticated user: {}", user.display_name);
    
    Ok(())
}
```
*/

use arceon_platform_integration::*;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== ARCEON PLATFORM INTEGRATION DEMO ===\\n");

    // Demo 1: Platform Manager Setup
    println!("🎮 DEMO 1: Platform Manager Initialization");
    let mut platform_manager = PlatformManager::new();
    println!("✅ Platform manager created successfully\\n");

    // Demo 2: Epic Games Integration
    println!("🎯 DEMO 2: Epic Games Store Integration");
    println!("├─ Setting up Epic Games adapter with OAuth authentication");
    
    let mut epic_adapter = EpicGamesAdapter::new();
    let epic_config = PlatformConfig::new(
        PlatformId::new("epic"),
        "demo_epic_app_id".to_string()
    ).with_oauth(
        "demo_epic_client_id".to_string(),
        "demo_epic_client_secret".to_string(),
        "http://localhost:8080/auth/epic/callback".to_string()
    ).with_endpoint("https://api.epicgames.dev".to_string());
    
    epic_adapter.initialize(epic_config).await?;
    
    // Generate Epic Games OAuth URL
    let (auth_url, code_verifier) = epic_adapter.build_auth_url("demo_state", &["basic_profile", "friends_list"])?;
    println!("├─ Epic Games OAuth URL: {}", auth_url);
    println!("├─ Code verifier (PKCE): {:?}", code_verifier);
    
    platform_manager.register_platform(Box::new(epic_adapter)).await?;
    println!("├─ ✅ Epic Games adapter registered successfully");
    println!("└─ Epic Games integration ready\\n");

    // Demo 3: Steam Integration
    println!("🎮 DEMO 3: Steam Platform Integration");
    println!("├─ Setting up Steam adapter with Steamworks integration");
    
    let mut steam_adapter = SteamAdapter::new();
    let steam_config = PlatformConfig::new(
        PlatformId::new("steam"),
        "480".to_string() // Spacewar App ID for testing
    ).with_capabilities(PlatformCapabilities {
        authentication: true,
        user_profiles: true,
        achievements: true,
        friends_list: true,
        overlay_support: true,
        workshop: true,
        voice_chat: true,
        cloud_saves: true,
        ..Default::default()
    });
    
    steam_adapter.initialize(steam_config).await?;
    platform_manager.register_platform(Box::new(steam_adapter)).await?;
    
    println!("├─ ✅ Steam adapter registered successfully");
    println!("├─ Steam overlay support: enabled");
    println!("├─ Workshop integration: available");
    println!("└─ Steam integration ready\\n");

    // Demo 4: Windows Launcher Integration
    println!("🖥️  DEMO 4: Windows Launcher Integration");
    println!("├─ Setting up Windows launcher with registry integration");
    
    let mut windows_adapter = WindowsLauncherAdapter::new();
    let mut windows_config = PlatformConfig::new(
        PlatformId::new("windows"),
        "arceon_demo".to_string()
    );
    
    // Configure Windows launcher settings
    let launcher_config = WindowsLauncherConfig {
        app_name: "Arceon Demo".to_string(),
        app_version: "1.0.0".to_string(),
        publisher: "Arceon Team".to_string(),
        install_directory: std::env::current_dir()?,
        executable_name: "arceon.exe".to_string(),
        registry_root: "HKCU\\\\Software\\\\ArceonTeam\\\\ArceonDemo".to_string(),
        auto_start: false,
        create_desktop_shortcut: true,
        create_start_menu_shortcut: true,
        associate_file_types: vec![
            FileTypeAssociation {
                extension: "arceon".to_string(),
                description: "Arceon Game File".to_string(),
                icon_path: None,
            }
        ],
        required_dependencies: vec![
            Dependency {
                name: "Visual C++ Redistributable".to_string(),
                version: "14.0".to_string(),
                download_url: Some("https://aka.ms/vs/17/release/vc_redist.x64.exe".to_string()),
                installer_path: None,
            }
        ],
        update_server_url: Some("https://updates.arceon.game".to_string()),
    };
    
    windows_config.set_custom_setting("launcher_config", &launcher_config)?;
    windows_adapter.initialize(windows_config).await?;
    platform_manager.register_platform(Box::new(windows_adapter)).await?;
    
    println!("├─ ✅ Windows launcher registered successfully");
    println!("├─ Registry integration: enabled");
    println!("├─ Auto-update support: configured");
    println!("├─ File associations: .arceon files");
    println!("└─ Windows launcher ready\\n");

    // Demo 5: Generic Platform Adapter
    println!("🔧 DEMO 5: Generic Platform Adapter (Custom Integration)");
    println!("├─ Setting up generic adapter for custom third-party service");
    
    let mut generic_adapter = GenericPlatformAdapter::new("custom_platform");
    
    // Configure generic platform settings
    let generic_platform_config = GenericPlatformConfig {
        platform_name: "Custom Game Platform".to_string(),
        api_config: GenericApiConfig {
            base_url: "https://api.customgameplatform.com/v1".to_string(),
            authentication_type: AuthenticationType::ApiKey {
                header_name: "X-API-Key".to_string(),
            },
            headers: {
                let mut headers = HashMap::new();
                headers.insert("User-Agent".to_string(), "Arceon/1.0".to_string());
                headers.insert("Accept".to_string(), "application/json".to_string());
                headers
            },
            timeout_seconds: 30,
            retry_attempts: 3,
            rate_limit_per_minute: Some(60),
        },
        endpoints: {
            let mut endpoints = HashMap::new();
            endpoints.insert("authenticate".to_string(), EndpointConfig {
                path: "/auth/login".to_string(),
                method: "POST".to_string(),
                required_params: vec!["username".to_string(), "password".to_string()],
                optional_params: vec![],
                response_mapping: {
                    let mut mapping = HashMap::new();
                    mapping.insert("user_id".to_string(), "data.user.id".to_string());
                    mapping.insert("display_name".to_string(), "data.user.username".to_string());
                    mapping.insert("access_token".to_string(), "data.token.access_token".to_string());
                    mapping
                },
            });
            endpoints.insert("get_user_profile".to_string(), EndpointConfig {
                path: "/users/{user_id}".to_string(),
                method: "GET".to_string(),
                required_params: vec!["user_id".to_string()],
                optional_params: vec![],
                response_mapping: {
                    let mut mapping = HashMap::new();
                    mapping.insert("display_name".to_string(), "username".to_string());
                    mapping.insert("avatar_url".to_string(), "avatar.large".to_string());
                    mapping
                },
            });
            endpoints
        },
        custom_mappings: HashMap::new(),
        webhook_endpoints: vec![
            WebhookConfig {
                name: "achievement_unlocked".to_string(),
                url: "https://webhooks.arceon.game/achievement".to_string(),
                events: vec!["achievement.unlocked".to_string()],
                secret: Some("webhook_secret_key".to_string()),
                headers: HashMap::new(),
            }
        ],
    };
    
    // Register custom handler
    generic_adapter.register_custom_handler(
        "data_processor".to_string(),
        Box::new(SimpleCustomHandler)
    );
    
    let mut generic_config = PlatformConfig::new(
        PlatformId::new("custom_platform"),
        "custom_app_123".to_string()
    ).with_capabilities(PlatformCapabilities {
        authentication: true,
        user_profiles: true,
        achievements: true,
        analytics: true,
        ..Default::default()
    });
    
    generic_config.set_custom_setting("generic_config", &generic_platform_config)?;
    generic_adapter.initialize(generic_config).await?;
    platform_manager.register_platform(Box::new(generic_adapter)).await?;
    
    println!("├─ ✅ Generic adapter registered successfully");
    println!("├─ API integration: RESTful with custom endpoints");
    println!("├─ Authentication: API Key based");
    println!("├─ Webhooks: configured for real-time events");
    println!("├─ Custom handlers: data processing pipeline");
    println!("└─ Generic platform integration ready\\n");

    // Demo 6: Platform Capabilities Overview
    println!("📊 DEMO 6: Platform Capabilities Overview");
    let registered_platforms = platform_manager.get_registered_platforms().await;
    
    for platform_id in &registered_platforms {
        println!("🎯 Platform: {}", platform_id.as_str());
        if let Ok(capabilities) = platform_manager.get_platform_capabilities(platform_id).await {
            println!("   ├─ Authentication: {}", if capabilities.authentication { "✅" } else { "❌" });
            println!("   ├─ User Profiles: {}", if capabilities.user_profiles { "✅" } else { "❌" });
            println!("   ├─ Achievements: {}", if capabilities.achievements { "✅" } else { "❌" });
            println!("   ├─ Friends List: {}", if capabilities.friends_list { "✅" } else { "❌" });
            println!("   ├─ Content Management: {}", if capabilities.content_management { "✅" } else { "❌" });
            println!("   ├─ Analytics: {}", if capabilities.analytics { "✅" } else { "❌" });
            println!("   ├─ Overlay Support: {}", if capabilities.overlay_support { "✅" } else { "❌" });
            println!("   ├─ Cloud Saves: {}", if capabilities.cloud_saves { "✅" } else { "❌" });
            println!("   ├─ Voice Chat: {}", if capabilities.voice_chat { "✅" } else { "❌" });
            println!("   └─ Workshop: {}", if capabilities.workshop { "✅" } else { "❌" });
        }
    }
    println!();

    // Demo 7: Authentication Flow Simulation
    println!("🔐 DEMO 7: Multi-Platform Authentication");
    println!("├─ Simulating authentication flows for different platforms");
    
    // Simulate Epic Games authentication
    println!("├─ Epic Games OAuth Flow:");
    let epic_credentials = AuthenticationCredentials::AuthorizationCode {
        code: "demo_authorization_code".to_string(),
        state: Some("demo_state".to_string()),
        code_verifier: Some("demo_code_verifier".to_string()),
    };
    
    match platform_manager.authenticate_user(&PlatformId::new("epic"), epic_credentials).await {
        Ok(user) => {
            println!("│   ✅ Epic Games authentication successful");
            println!("│   └─ User: {} (ID: {})", user.display_name, user.user_id);
        }
        Err(e) => {
            println!("│   ❌ Epic Games authentication failed: {}", e);
        }
    }
    
    // Simulate Steam authentication (automatic)
    println!("├─ Steam Authentication:");
    match platform_manager.authenticate_user(&PlatformId::new("steam"), AuthenticationCredentials::Custom { data: HashMap::new() }).await {
        Ok(user) => {
            println!("│   ✅ Steam authentication successful");
            println!("│   └─ User: {} (ID: {})", user.display_name, user.user_id);
        }
        Err(e) => {
            println!("│   ❌ Steam authentication failed: {}", e);
        }
    }
    
    // Simulate custom platform authentication
    println!("└─ Custom Platform Authentication:");
    let custom_credentials = AuthenticationCredentials::UsernamePassword {
        username: "demo_user".to_string(),
        password: "demo_password".to_string(),
    };
    
    match platform_manager.authenticate_user(&PlatformId::new("custom_platform"), custom_credentials).await {
        Ok(user) => {
            println!("    ✅ Custom platform authentication successful");
            println!("    └─ User: {} (ID: {})", user.display_name, user.user_id);
        }
        Err(e) => {
            println!("    ❌ Custom platform authentication failed: {}", e);
        }
    }
    println!();

    // Demo 8: Achievement Synchronization
    println!("🏆 DEMO 8: Cross-Platform Achievement Synchronization");
    println!("├─ Creating sample achievements for synchronization");
    
    let sample_achievements = vec![
        Achievement {
            id: "first_login".to_string(),
            name: "Welcome to Arceon".to_string(),
            description: "Log into the game for the first time".to_string(),
            icon_url: Some("https://cdn.arceon.game/achievements/first_login.png".to_string()),
            unlocked: true,
            unlock_time: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
            progress: None,
            category: AchievementCategory::Miscellaneous,
            rarity: AchievementRarity::Common,
            platform_specific_data: HashMap::new(),
        },
        Achievement {
            id: "level_10".to_string(),
            name: "Rising Star".to_string(),
            description: "Reach level 10".to_string(),
            icon_url: Some("https://cdn.arceon.game/achievements/level_10.png".to_string()),
            unlocked: true,
            unlock_time: Some(chrono::Utc::now() - chrono::Duration::minutes(30)),
            progress: Some(AchievementProgress {
                current: 10,
                target: 10,
                unit: "levels".to_string(),
            }),
            category: AchievementCategory::Skill,
            rarity: AchievementRarity::Uncommon,
            platform_specific_data: HashMap::new(),
        },
        Achievement {
            id: "social_butterfly".to_string(),
            name: "Social Butterfly".to_string(),
            description: "Add 10 friends".to_string(),
            icon_url: Some("https://cdn.arceon.game/achievements/social_butterfly.png".to_string()),
            unlocked: false,
            unlock_time: None,
            progress: Some(AchievementProgress {
                current: 7,
                target: 10,
                unit: "friends".to_string(),
            }),
            category: AchievementCategory::Social,
            rarity: AchievementRarity::Rare,
            platform_specific_data: HashMap::new(),
        },
    ];
    
    println!("├─ Synchronizing {} achievements across all platforms", sample_achievements.len());
    match platform_manager.sync_achievements_all_platforms(&sample_achievements).await {
        Ok(()) => {
            println!("├─ ✅ Achievement synchronization completed successfully");
            for achievement in &sample_achievements {
                let status = if achievement.unlocked { "🏆 Unlocked" } else { "🔒 Locked" };
                let progress = if let Some(prog) = &achievement.progress {
                    format!(" ({}/{})", prog.current, prog.target)
                } else {
                    String::new()
                };
                println!("│   └─ {} {}{}", status, achievement.name, progress);
            }
        }
        Err(e) => {
            println!("├─ ❌ Achievement synchronization failed: {}", e);
        }
    }
    println!();

    // Demo 9: Content Launch Simulation
    println!("🚀 DEMO 9: Cross-Platform Content Launching");
    println!("├─ Testing content launch capabilities across platforms");
    
    let launch_options = LaunchOptions {
        arguments: vec!["--demo-mode".to_string(), "--platform-integration".to_string()],
        environment: {
            let mut env = HashMap::new();
            env.insert("ARCEON_DEMO".to_string(), "true".to_string());
            env
        },
        working_directory: None,
    };
    
    for platform_id in &registered_platforms {
        match platform_manager.launch_content(platform_id, "arceon_demo_content", launch_options.clone()).await {
            Ok(()) => {
                println!("├─ ✅ Content launched successfully on {}", platform_id.as_str());
            }
            Err(e) => {
                println!("├─ ❌ Content launch failed on {}: {}", platform_id.as_str(), e);
            }
        }
    }
    println!();

    // Demo 10: Analytics Collection
    println!("📈 DEMO 10: Cross-Platform Analytics Collection");
    println!("├─ Collecting analytics data from all integrated platforms");
    
    match platform_manager.collect_analytics().await {
        Ok(analytics_data) => {
            println!("├─ ✅ Analytics collection completed successfully");
            println!("├─ Collected data from {} platforms", analytics_data.len());
            
            for (platform_id, analytics) in &analytics_data {
                println!("│   🎯 Platform: {}", platform_id.as_str());
                println!("│   ├─ Session Count: {}", analytics.session_count);
                println!("│   ├─ Total Playtime: {:.2} hours", analytics.total_playtime_hours);
                println!("│   ├─ Achievements: {}", analytics.achievements_unlocked);
                println!("│   ├─ Friends Count: {}", analytics.friends_count);
                if !analytics.platform_specific_metrics.is_empty() {
                    println!("│   └─ Custom Metrics: {} items", analytics.platform_specific_metrics.len());
                }
            }
        }
        Err(e) => {
            println!("├─ ❌ Analytics collection failed: {}", e);
        }
    }
    println!();

    // Demo 11: Session Management
    println!("⚙️  DEMO 11: Platform Session Management");
    println!("├─ Demonstrating session cleanup and management");
    
    // Clean up expired sessions
    match platform_manager.cleanup_expired_sessions().await {
        Ok(()) => {
            println!("├─ ✅ Session cleanup completed successfully");
        }
        Err(e) => {
            println!("├─ ❌ Session cleanup failed: {}", e);
        }
    }
    
    // Get current configuration
    let config = platform_manager.get_config().await;
    println!("├─ Current Configuration:");
    println!("│   ├─ Auto-detect platforms: {}", config.auto_detect_platforms);
    println!("│   ├─ Fallback authentication: {}", config.fallback_authentication);
    println!("│   ├─ Cache user profiles: {}", config.cache_user_profiles);
    println!("│   ├─ Auto-sync achievements: {}", config.sync_achievements_automatically);
    println!("│   ├─ Analytics enabled: {}", config.analytics_enabled);
    println!("│   └─ Session timeout: {} minutes", config.session_timeout_minutes);
    println!();

    // Summary
    println!("✨ PLATFORM INTEGRATION FEATURES DEMONSTRATED:");
    println!("├─ 🎯 Epic Games Store: OAuth authentication, user profiles, achievements");
    println!("├─ 🎮 Steam Platform: Steamworks integration, overlay, workshop support");
    println!("├─ 🖥️  Windows Launcher: Registry integration, shortcuts, auto-updates");
    println!("├─ 🔧 Generic Adapter: RESTful APIs, webhooks, custom authentication");
    println!("├─ 🔐 Multi-Platform Auth: OAuth 2.0, PKCE, token management");
    println!("├─ 🏆 Achievement Sync: Cross-platform achievement synchronization");
    println!("├─ 👥 User Management: Profile sync, friends lists, social features");
    println!("├─ 🚀 Content Launching: Platform-specific launch mechanisms");
    println!("├─ 📈 Analytics Collection: Cross-platform usage analytics");
    println!("├─ ⚙️  Session Management: Token refresh, cleanup, configuration");
    println!("└─ 🔌 Extensible Architecture: Plugin system for new platforms");

    println!("\\n🎉 Platform integration demonstration complete!");
    println!("💡 The platform integration system provides:");
    println!("   • Seamless integration with major gaming platforms");
    println!("   • Unified authentication across multiple services");
    println!("   • Cross-platform achievement and profile synchronization");
    println!("   • Windows-native launcher capabilities");
    println!("   • Flexible architecture for custom platform integrations");
    println!("   • Comprehensive analytics and session management");
    println!("   • Production-ready security and error handling");

    Ok(())
}