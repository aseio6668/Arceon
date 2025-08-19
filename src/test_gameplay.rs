use arceon_core::{ArceonCore, Config};
use arceon_core::entities::being::Race;
use arceon_world::WorldManager;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌟 Testing Arceon Game Systems");
    println!("===============================");
    
    // Initialize the game core
    let config = Config::default();
    let mut core = ArceonCore::new(config).await?;
    
    // Generate world areas
    let mut world_manager = WorldManager::new();
    let areas = world_manager.generate_espan()?;
    core.add_areas(areas).await?;
    
    println!("✅ Game core initialized successfully");
    
    // Create a test player
    let player_name = "TestHero";
    let player_id = "test_player_1";
    
    let being_id = core.create_player(
        player_id.to_string(),
        player_name.to_string(),
        Race::Human,
    ).await?;
    
    println!("✅ Created player '{}' with ID: {}", player_name, being_id);
    
    // Test basic commands
    println!("\n🎮 Testing Game Commands:");
    println!("==========================");
    
    // Test look command
    println!("\n>>> look");
    let response = core.process_command(player_id, "look").await?;
    println!("{}", response);
    
    // Test stats command
    println!("\n>>> stats");
    let response = core.process_command(player_id, "stats").await?;
    println!("{}", response);
    
    // Test who command
    println!("\n>>> who");
    let response = core.process_command(player_id, "who").await?;
    println!("{}", response);
    
    // Test say command
    println!("\n>>> say Hello, world of Arceon!");
    let response = core.process_command(player_id, "say Hello, world of Arceon!").await?;
    println!("{}", response);
    
    // Test npcs command
    println!("\n>>> npcs");
    let response = core.process_command(player_id, "npcs").await?;
    println!("{}", response);
    
    // Test talk command
    println!("\n>>> talk guard");
    let response = core.process_command(player_id, "talk guard").await?;
    println!("{}", response);
    
    // Create a second player to test multiplayer
    let player2_name = "TestMage";
    let player2_id = "test_player_2";
    
    let being_id2 = core.create_player(
        player2_id.to_string(),
        player2_name.to_string(),
        Race::Elf,
    ).await?;
    
    println!("\n✅ Created second player '{}' with ID: {}", player2_name, being_id2);
    
    // Test multiplayer who command
    println!("\n>>> who (with multiple players)");
    let response = core.process_command(player_id, "who").await?;
    println!("{}", response);
    
    // Test movement
    println!("\n>>> move 1");
    let response = core.process_command(player_id, "move 1").await?;
    println!("{}", response);
    
    // Test look after movement
    println!("\n>>> look (after movement)");
    let response = core.process_command(player_id, "look").await?;
    println!("{}", response);
    
    println!("\n🎉 All tests completed successfully!");
    println!("The GUI is ready for gameplay!");
    
    Ok(())
}