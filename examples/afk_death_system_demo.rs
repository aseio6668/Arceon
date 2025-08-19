use anyhow::Result;
use arceon_core::entities::{Being, Area, Vital};
use arceon_core::systems::{
    AFKSystem, AFKMode, AFKSafetySettings, AFKConfig, 
    DeathSystem, DeathSystemConfig, DeathCause, KillerType, 
    BindPointType, ResurrectionType
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};
use uuid::Uuid;
use std::collections::HashMap;

/// Comprehensive demo of AFK/Idle mode and Death/Resurrection systems
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::init();

    info!("🚀 Starting AFK and Death System Integration Demo");

    // Demo AFK System
    demo_afk_system().await?;

    // Demo Death and Resurrection System
    demo_death_resurrection_system().await?;

    // Demo AFK Death Integration
    demo_afk_death_integration().await?;

    info!("🎉 AFK and Death System Demo completed successfully!");
    Ok(())
}

/// Demonstrate AFK system functionality
async fn demo_afk_system() -> Result<()> {
    info!("\n=== 🤖 AFK System Demo ===");

    // Initialize AFK system
    let afk_config = AFKConfig::default();
    let afk_system = AFKSystem::new(afk_config).await?;

    let player_id = Uuid::new_v4();
    let character_id = Uuid::new_v4();

    // Demo Combat AFK Mode
    info!("🗡️ Starting Combat AFK Mode");
    let combat_mode = AFKMode::Combat {
        target_types: vec!["Goblin".to_string(), "Orc".to_string()],
        auto_loot: true,
        combat_radius: 50.0,
        retreat_on_low_health: true,
        use_consumables: true,
        skill_rotation: vec!["Slash".to_string(), "Block".to_string(), "Heal".to_string()],
        priority_targets: vec!["Orc Chief".to_string()],
    };

    let safety_settings = AFKSafetySettings {
        max_death_count: 3,
        min_health_threshold: 0.2,
        max_session_duration: Duration::from_hours(2),
        auto_pause_on_player_proximity: true,
        auto_pause_on_guild_member_proximity: false,
        emergency_logout_conditions: vec![],
        resource_protection: arceon_core::systems::ResourceProtectionSettings {
            protect_valuable_items: true,
            valuable_item_threshold: 1000,
            bank_items_when_full: true,
            auto_repair_equipment: true,
            max_repair_cost: 500,
        },
        anti_griefing_measures: true,
    };

    let combat_session_id = afk_system.start_afk_session(
        player_id,
        character_id,
        combat_mode,
        safety_settings.clone(),
    ).await?;

    info!("✅ Combat AFK session started: {}", combat_session_id);

    // Simulate some AFK time
    sleep(Duration::from_secs(2)).await;

    // Check session stats
    if let Some(session) = afk_system.get_active_session(combat_session_id).await {
        info!("📊 Combat AFK Stats: {} enemies defeated, {} XP gained",
            session.session_stats.enemies_defeated,
            session.session_stats.experience_gained.values().sum::<u64>()
        );
    }

    // Stop combat session
    let combat_stats = afk_system.stop_afk_session(combat_session_id, "Demo completed".to_string()).await?;
    info!("🛑 Combat session ended. Final stats: {} enemies defeated", combat_stats.enemies_defeated);

    // Demo Gathering AFK Mode
    info!("\n🪓 Starting Gathering AFK Mode");
    let gathering_mode = AFKMode::Gathering {
        resource_types: vec!["Iron Ore".to_string(), "Mithril".to_string()],
        gathering_radius: 30.0,
        auto_move_to_resources: true,
        ignore_dangerous_areas: true,
        preferred_tools: vec!["Enchanted Pickaxe".to_string()],
        min_resource_quality: 3,
    };

    let gathering_session_id = afk_system.start_afk_session(
        player_id,
        character_id,
        gathering_mode,
        safety_settings.clone(),
    ).await?;

    sleep(Duration::from_secs(1)).await;

    // Go offline with gathering session
    info!("🌙 Transitioning gathering to offline mode");
    afk_system.go_offline(gathering_session_id, Duration::from_hours(4)).await?;

    // Simulate player returning online
    sleep(Duration::from_millis(100)).await;
    info!("☀️ Player returning online after offline gathering");
    if let Some(offline_stats) = afk_system.return_online(player_id).await? {
        info!("📦 Offline rewards: {} resources gathered, {} total XP",
            offline_stats.resources_gathered,
            offline_stats.experience_gained.values().sum::<u64>()
        );
    }

    // Demo Crafting AFK Mode
    info!("\n🔨 Starting Crafting AFK Mode");
    let mut craft_queue = std::collections::VecDeque::new();
    craft_queue.push_back(arceon_core::systems::CraftingTask {
        task_id: Uuid::new_v4(),
        recipe_name: "Steel Sword".to_string(),
        quantity: 5,
        priority: 1,
        required_materials: {
            let mut materials = HashMap::new();
            materials.insert("Steel Ingot".to_string(), 10);
            materials.insert("Leather Wrap".to_string(), 2);
            materials
        },
        estimated_time: Duration::from_minutes(30),
        skill_requirements: {
            let mut skills = HashMap::new();
            skills.insert("Smithing".to_string(), 50);
            skills
        },
        auto_gather_materials: true,
    });

    let crafting_mode = AFKMode::Crafting {
        craft_queue,
        auto_restock_materials: true,
        crafting_station_id: Some(Uuid::new_v4()),
        skill_focus: Some("Smithing".to_string()),
        quality_threshold: 7,
        auto_salvage_failures: false,
    };

    let crafting_session_id = afk_system.start_afk_session(
        player_id,
        character_id,
        crafting_mode,
        safety_settings,
    ).await?;

    sleep(Duration::from_secs(1)).await;

    let crafting_stats = afk_system.stop_afk_session(crafting_session_id, "Demo completed".to_string()).await?;
    info!("🛑 Crafting session ended. Items crafted: {}", crafting_stats.items_crafted);

    // Show system stats
    let afk_stats = afk_system.get_system_stats().await;
    info!("📈 AFK System Stats: {} total sessions, {} active, {:.2} avg efficiency",
        afk_stats.total_sessions,
        afk_stats.active_sessions,
        afk_stats.efficiency_rating
    );

    Ok(())
}

/// Demonstrate death and resurrection system
async fn demo_death_resurrection_system() -> Result<()> {
    info!("\n=== ⚰️ Death and Resurrection System Demo ===");

    // Initialize death system
    let death_config = DeathSystemConfig::default();
    let death_system = DeathSystem::new(death_config).await?;

    let character_id = Uuid::new_v4();
    let area_id = "Dark Forest".to_string();

    // Create a mock being
    let mut being = Being {
        id: character_id,
        name: "Test Character".to_string(),
        race: "Human".to_string(),
        skills: HashMap::new(),
        vitals: HashMap::new(),
    };

    // Create a mock area
    let area = Area {
        id: area_id.clone(),
        name: "Dark Forest".to_string(),
        description: "A dangerous forest filled with monsters".to_string(),
        size: (1000.0, 1000.0),
        climate: "Temperate".to_string(),
        biome: "Forest".to_string(),
        danger_level: 5,
        resources: Vec::new(),
        structures: Vec::new(),
        npcs: Vec::new(),
        connections: Vec::new(),
        events: Vec::new(),
        lore: Vec::new(),
    };

    // Generate safe spots for the area
    info!("🛡️ Generating safe spots for {}", area_id);
    let safe_spots = death_system.generate_area_safe_spots(area_id.clone(), &area).await?;
    info!("✅ Generated {} safe spots", safe_spots.len());

    // Create bind points
    info!("🏠 Creating bind points for character");
    let home_bind_id = death_system.create_bind_point(
        character_id,
        BindPointType::PlayerHome,
        (100.0, 100.0),
        "Hometown".to_string(),
        "My Cozy Home".to_string(),
    ).await?;

    let guild_bind_id = death_system.create_bind_point(
        character_id,
        BindPointType::GuildHall,
        (500.0, 500.0),
        area_id.clone(),
        "Guild Hall".to_string(),
    ).await?;

    info!("✅ Created bind points: Home({}), Guild({})", home_bind_id, guild_bind_id);

    // Simulate character death
    info!("💀 Simulating character death");
    let death_location = (750.0, 300.0);
    let death_cause = DeathCause::Combat {
        killer_type: KillerType::Monster,
        killer_id: Some(Uuid::new_v4()),
        damage_type: "Claw Attack".to_string(),
        final_blow_skill: Some("Savage Strike".to_string()),
    };

    let death_record = death_system.process_death(
        character_id,
        death_cause,
        death_location,
        area_id.clone(),
        &being,
    ).await?;

    info!("⚰️ Character died at location: {:?}", death_location);
    info!("📋 Available resurrection options: {}", death_record.resurrection_options.len());

    for (i, option) in death_record.resurrection_options.iter().enumerate() {
        info!("   {}. {:?} at {} (Distance: {:.1}, Safety: {:.1})",
            i + 1,
            option.option_type,
            option.location_name,
            option.distance_from_death,
            option.safety_rating
        );
    }

    // Choose resurrection at home bind
    let home_resurrection_option = death_record.resurrection_options.iter()
        .find(|opt| matches!(opt.option_type, ResurrectionType::HomeBind))
        .cloned()
        .unwrap();

    info!("⛑️ Resurrecting at home bind point");
    let resurrection_result = death_system.resurrect_character(
        character_id,
        home_resurrection_option,
        &mut being,
    ).await?;

    info!("✅ Character resurrected at {} in area: {}", 
        resurrection_result.location_name, 
        resurrection_result.resurrection_area_id
    );
    info!("⏱️ Time dead: {:?}", resurrection_result.time_dead);
    info!("💊 Penalties: Health: {:.0}%, Mana: {:.0}%, Stamina: {:.0}%",
        resurrection_result.penalties_applied.health_percentage * 100.0,
        resurrection_result.penalties_applied.mana_percentage * 100.0,
        resurrection_result.penalties_applied.stamina_percentage * 100.0
    );

    // Show death system stats
    let death_stats = death_system.get_system_stats().await;
    info!("📊 Death System Stats: {} total deaths, {} resurrections by type",
        death_stats.total_deaths,
        death_stats.resurrections_by_type.len()
    );

    Ok(())
}

/// Demonstrate AFK and death system integration
async fn demo_afk_death_integration() -> Result<()> {
    info!("\n=== 🤖💀 AFK + Death System Integration Demo ===");

    // Initialize both systems
    let afk_config = AFKConfig::default();
    let afk_system = AFKSystem::new(afk_config).await?;
    
    let death_config = DeathSystemConfig::default();
    let death_system = DeathSystem::new(death_config).await?;

    let player_id = Uuid::new_v4();
    let character_id = Uuid::new_v4();
    let area_id = "Dangerous Dungeon".to_string();

    // Create a mock area
    let area = Area {
        id: area_id.clone(),
        name: "Dangerous Dungeon".to_string(),
        description: "A perilous dungeon with high-level monsters".to_string(),
        size: (500.0, 500.0),
        climate: "Underground".to_string(),
        biome: "Dungeon".to_string(),
        danger_level: 8,
        resources: Vec::new(),
        structures: Vec::new(),
        npcs: Vec::new(),
        connections: Vec::new(),
        events: Vec::new(),
        lore: Vec::new(),
    };

    // Generate safe spots including dungeon checkpoints
    let safe_spots = death_system.generate_area_safe_spots(area_id.clone(), &area).await?;
    info!("🛡️ Generated {} safe spots in dungeon", safe_spots.len());

    // Create bind point near dungeon entrance
    let dungeon_bind_id = death_system.create_bind_point(
        character_id,
        BindPointType::DungeonCheckpoint,
        (50.0, 50.0),
        area_id.clone(),
        "Dungeon Entrance".to_string(),
    ).await?;

    info!("🏰 Created dungeon checkpoint bind: {}", dungeon_bind_id);

    // Start risky AFK combat session in dangerous dungeon
    info!("⚔️ Starting risky AFK combat in dangerous dungeon");
    let risky_mode = AFKMode::Combat {
        target_types: vec!["Dragon".to_string(), "Demon".to_string()],
        auto_loot: true,
        combat_radius: 100.0,
        retreat_on_low_health: true,
        use_consumables: true,
        skill_rotation: vec!["Power Attack".to_string(), "Defend".to_string(), "Greater Heal".to_string()],
        priority_targets: vec!["Dungeon Boss".to_string()],
    };

    let risky_safety_settings = AFKSafetySettings {
        max_death_count: 2, // Lower death tolerance in dangerous area
        min_health_threshold: 0.3,
        max_session_duration: Duration::from_minutes(30),
        auto_pause_on_player_proximity: true,
        auto_pause_on_guild_member_proximity: true,
        emergency_logout_conditions: vec![
            arceon_core::systems::EmergencyCondition::HealthBelowPercent(0.15),
            arceon_core::systems::EmergencyCondition::TargetedByPlayer,
        ],
        resource_protection: arceon_core::systems::ResourceProtectionSettings {
            protect_valuable_items: true,
            valuable_item_threshold: 5000, // Higher threshold for dungeon
            bank_items_when_full: true,
            auto_repair_equipment: true,
            max_repair_cost: 2000,
        },
        anti_griefing_measures: true,
    };

    let risky_session_id = afk_system.start_afk_session(
        player_id,
        character_id,
        risky_mode,
        risky_safety_settings,
    ).await?;

    info!("✅ Risky AFK session started: {}", risky_session_id);

    // Simulate AFK combat for a bit
    sleep(Duration::from_millis(500)).await;

    // Simulate character death during AFK (this would normally trigger automatically)
    info!("💀 Simulating death during AFK session");
    let mut mock_being = Being {
        id: character_id,
        name: "AFK Fighter".to_string(),
        race: "Human".to_string(),
        skills: HashMap::new(),
        vitals: HashMap::new(),
    };

    let afk_death_cause = DeathCause::Combat {
        killer_type: KillerType::Boss,
        killer_id: Some(Uuid::new_v4()),
        damage_type: "Dragon Breath".to_string(),
        final_blow_skill: Some("Inferno".to_string()),
    };

    let afk_death_record = death_system.process_death(
        character_id,
        afk_death_cause,
        (400.0, 400.0), // Deep in dungeon
        area_id.clone(),
        &mock_being,
    ).await?;

    info!("⚰️ Character died during AFK session");
    info!("🤖 AFK session should pause due to death");

    // In a real implementation, this would automatically pause the AFK session
    let session_stats = afk_system.stop_afk_session(risky_session_id, "Character died".to_string()).await?;
    info!("🛑 AFK session paused due to death. Deaths: {}", session_stats.deaths);

    // Auto-resurrect at nearest safe spot (this would happen automatically)
    let safe_spot_option = afk_death_record.resurrection_options.iter()
        .find(|opt| matches!(opt.option_type, ResurrectionType::AreaSafeSpot))
        .cloned()
        .unwrap();

    info!("⛑️ Auto-resurrecting at safe spot: {}", safe_spot_option.location_name);
    let auto_resurrection = death_system.resurrect_character(
        character_id,
        safe_spot_option,
        &mut mock_being,
    ).await?;

    info!("✅ Auto-resurrection completed at safe spot");
    info!("💊 Low vitals after resurrection - character needs healing");

    // Character could resume AFK after healing, but with increased caution
    info!("🤖 AFK could resume after healing with more conservative settings");

    // Show combined system statistics
    let afk_final_stats = afk_system.get_system_stats().await;
    let death_final_stats = death_system.get_system_stats().await;

    info!("\n📊 Final Integration Stats:");
    info!("   AFK Sessions: {} total, {:.2} avg efficiency", 
        afk_final_stats.total_sessions, afk_final_stats.efficiency_rating);
    info!("   Deaths: {} total, {} in dangerous areas",
        death_final_stats.total_deaths,
        death_final_stats.most_dangerous_areas.get(&area_id).unwrap_or(&0)
    );
    info!("   Safety Features: AFK auto-pause on death, safe spot auto-resurrection");

    Ok(())
}

// Helper trait for duration creation
trait DurationExt {
    fn from_hours(hours: u64) -> Duration;
    fn from_minutes(minutes: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 3600)
    }

    fn from_minutes(minutes: u64) -> Duration {
        Duration::from_secs(minutes * 60)
    }
}