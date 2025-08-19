use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber;
use std::io::{self, Write};

use arceon_core::{ArceonCore, Config, NetworkManager, BlockchainManager};
use arceon_core::entities::being::Race;
use arceon_world::WorldManager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Enable headless mode (server only)
    #[arg(long)]
    headless: bool,
    
    /// Network mode
    #[arg(long, default_value = "p2p")]
    network_mode: String,
    
    /// Enable masternode features
    #[arg(long)]
    masternode: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let cli = Cli::parse();
    
    info!("🌟 Starting Arceon - Decentralized Fantasy MMORPG");
    
    // Load configuration
    let config = Config::load(&cli.config).unwrap_or_else(|_| {
        info!("Could not load config file, using defaults");
        Config::default()
    });
    
    // Initialize core systems
    let mut core = ArceonCore::new(config.clone()).await?;
    
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
    
    // Create character selection
    info!("🎮 Welcome to Arceon!");
    println!("\n=== ARCEON - Fantasy MMORPG ===");
    println!("Character Creation");
    println!("==================");
    println!("Choose your race:");
    println!("1. Human - Versatile and adaptable, natural traders and diplomats");
    println!("2. Elf - Graceful and wise, masters of magic and archery");
    println!("3. Gnome - Small but clever, skilled in crafting and invention");
    println!("4. Halfling - Peaceful folk, exceptional at stealth and cooking");
    println!("5. Orc - Strong and proud, fierce warriors and shamans");
    println!("6. Dwarf - Hardy miners and smiths, guardians of ancient traditions");
    println!("7. Dragonborn - Descendants of dragons, powerful and noble");
    println!("8. Tiefling - Touched by infernal heritage, clever and charismatic");
    
    let race;
    loop {
        print!("Enter race (1-8): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        race = match input.trim() {
            "1" => Race::Human,
            "2" => Race::Elf,
            "3" => Race::Gnome,
            "4" => Race::Halfling,
            "5" => Race::Orc,
            "6" => Race::Dwarf,
            "7" => Race::Dragonborn,
            "8" => Race::Tiefling,
            _ => {
                println!("Invalid selection. Please choose 1-8.");
                continue;
            }
        };
        break;
    }
    
    print!("Enter your character name: ");
    io::stdout().flush()?;
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input)?;
    let player_name = name_input.trim();
    if player_name.is_empty() {
        println!("Using default name: Traveler");
    }
    let player_name = if player_name.is_empty() { "Traveler" } else { player_name };
    
    // Create player character
    println!("\nCreating your character...");
    let player_id = format!("player_{}", player_name.to_lowercase());
    let _being_id = core.create_player(player_id.clone(), player_name.to_string(), race).await?;
    
    println!("Welcome to the world of Espan, {}!", player_name);
    
    // Show initial look command
    let look_response = core.process_command(&player_id, "look").await?;
    println!("\n{}", look_response);
    println!("\nType 'help' for available commands, 'quit' to exit.\n");
    
    // Main game loop
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim();
        
        if command.is_empty() {
            continue;
        }
        
        if command.to_lowercase() == "quit" || command.to_lowercase() == "exit" {
            println!("Farewell, adventurer!");
            break;
        }
        
        // Process command through the core system
        match core.process_command(&player_id, command).await {
            Ok(response) => {
                if !response.is_empty() {
                    println!("{}", response);
                }
            },
            Err(e) => {
                println!("Error processing command: {}", e);
            }
        }
        
        // Update core systems
        core.update();
    }
    
    Ok(())
}
