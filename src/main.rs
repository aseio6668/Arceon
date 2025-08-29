use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber;

use arceon_core::{ArceonCore, Config, NetworkManager, BlockchainManager};
use arceon_gui::ArceonGui;
use arceon_world::WorldManager;

mod contribution;
mod master_node;
mod reward_engine;
mod voting_system;
mod currency_market;
mod crafting_system;
mod non_combat_roles;
mod land_ownership;
mod authentication;
mod database;
mod group_system;
mod guild_system;
mod skill_migration;
mod work_classification;
mod work_based_deployment;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Enable headless mode (server only)
    #[arg(long)]
    headless: bool,
    
    /// Launch mode: genesis (decentralized), solo (local), or centralized (server)
    #[arg(long, default_value = "genesis")]
    mode: LaunchMode,
    
    /// Network mode
    #[arg(long, default_value = "p2p")]
    network_mode: String,
    
    /// Enable masternode features
    #[arg(long)]
    masternode: bool,
    
    /// Add peer nodes for decentralized mode (can be used multiple times)
    #[arg(long = "addnode")]
    add_nodes: Vec<String>,
    
    /// Peer connections config file for decentralized mode
    #[arg(long, default_value = "peers.ini")]
    peers_config: String,
    
    /// Server IP for centralized mode
    #[arg(long, default_value = "localhost:8080")]
    server: String,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
enum LaunchMode {
    /// Decentralized genesis mode - full P2P network with world persistence
    Genesis,
    /// Solo mode - local gameplay with local storage
    Solo, 
    /// Centralized mode - connect to traditional server
    Centralized,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let cli = Cli::parse();
    
    info!("🌟 Starting Arceon - Decentralized Fantasy MMORPG");
    info!("🚀 Launch Mode: {:?}", cli.mode);
    
    // Load configuration
    let config = Config::load(&cli.config).unwrap_or_else(|_| {
        info!("Could not load config file, using defaults");
        Config::default()
    });
    
    // Handle different launch modes
    match cli.mode {
        LaunchMode::Genesis => {
            info!("🌐 Genesis Mode: Decentralized P2P Network");
            run_genesis_mode(cli, config).await?;
        },
        LaunchMode::Solo => {
            info!("🏠 Solo Mode: Local Gameplay");
            run_solo_mode(cli, config).await?;
        },
        LaunchMode::Centralized => {
            info!("🖥️ Centralized Mode: Traditional Server Connection");
            run_centralized_mode(cli, config).await?;
        },
    }
    
    Ok(())
}

/// Genesis Mode: Decentralized P2P network with full world persistence
async fn run_genesis_mode(cli: Cli, config: Config) -> Result<()> {
    info!("⚡ Initializing Genesis P2P Network...");
    
    // Load peer connections from config file and CLI args
    let peers = load_peer_connections(&cli).await?;
    info!("🔗 Discovered {} peer nodes", peers.len());
    
    // Initialize core systems with P2P networking
    let mut core = ArceonCore::new(config.clone()).await?;
    
    // Initialize blockchain for world state persistence
    let blockchain = BlockchainManager;
    
    // Initialize P2P network manager with peer list
    let network = NetworkManager; // TODO: Pass peer list to network manager
    
    // Start core systems
    core.start(blockchain, network).await?;
    info!("🌍 P2P Network active - World state synchronized");
    
    // Generate or load world areas from network
    let mut world_manager = WorldManager::new();
    let areas = world_manager.generate_espan()?;
    core.add_areas(areas).await?;
    
    if cli.headless {
        info!("🖥️ Running as P2P node (headless)");
        info!("📡 Broadcasting world availability to network...");
        // Keep the P2P node running
        tokio::signal::ctrl_c().await?;
        info!("Shutting down P2P node...");
    } else {
        info!("🎮 Starting P2P client with GUI");
        let gui = ArceonGui::new(core);
        gui.run().await?;
    }
    
    Ok(())
}

/// Solo Mode: Local gameplay with local file storage
async fn run_solo_mode(cli: Cli, config: Config) -> Result<()> {
    info!("🏠 Initializing Solo Mode...");
    info!("💾 Using local storage for world persistence");
    
    // Initialize core systems without networking
    let mut core = ArceonCore::new(config.clone()).await?;
    
    // No blockchain or network needed for solo mode
    let blockchain = BlockchainManager; // Configured for local storage
    let network = NetworkManager; // Disabled for solo mode
    
    // Start core systems in offline mode
    core.start(blockchain, network).await?;
    
    // Load or generate world areas locally
    let mut world_manager = WorldManager::new();
    let areas = world_manager.generate_espan()?;
    core.add_areas(areas).await?;
    
    if cli.headless {
        info!("🖥️ Running solo world simulation (headless)");
        // Keep the world simulation running
        tokio::signal::ctrl_c().await?;
        info!("Shutting down solo mode...");
    } else {
        info!("🎮 Starting solo gameplay with GUI");
        let gui = ArceonGui::new(core);
        gui.run().await?;
    }
    
    Ok(())
}

/// Centralized Mode: Traditional client-server architecture
async fn run_centralized_mode(cli: Cli, config: Config) -> Result<()> {
    info!("🖥️ Initializing Centralized Mode...");
    info!("🔌 Connecting to server: {}", cli.server);
    
    // Initialize core systems for client mode
    let mut core = ArceonCore::new(config.clone()).await?;
    
    // No local blockchain needed - server manages state
    let blockchain = BlockchainManager; // Client mode
    let network = NetworkManager; // Client-server networking
    
    // Start core systems in client mode
    core.start(blockchain, network).await?;
    
    // World areas will be received from server
    let mut world_manager = WorldManager::new();
    let areas = world_manager.generate_espan()?; // TODO: Load from server
    core.add_areas(areas).await?;
    
    if cli.headless {
        info!("🖥️ Running as headless client");
        // Keep the client connection running
        tokio::signal::ctrl_c().await?;
        info!("Disconnecting from server...");
    } else {
        info!("🎮 Starting client GUI");
        let gui = ArceonGui::new(core);
        gui.run().await?;
    }
    
    Ok(())
}

/// Load peer connections from INI file and CLI arguments
async fn load_peer_connections(cli: &Cli) -> Result<Vec<String>> {
    let mut peers = Vec::new();
    
    // Add peers from CLI arguments
    peers.extend(cli.add_nodes.iter().cloned());
    
    // Load peers from INI config file
    if std::path::Path::new(&cli.peers_config).exists() {
        let ini_content = std::fs::read_to_string(&cli.peers_config)?;
        let parsed_peers = parse_peers_ini(&ini_content)?;
        let peer_count = parsed_peers.len();
        peers.extend(parsed_peers);
        info!("📄 Loaded {} peers from {}", peer_count, cli.peers_config);
    } else {
        info!("📄 Creating default peers config: {}", cli.peers_config);
        create_default_peers_config(&cli.peers_config).await?;
    }
    
    // Remove duplicates
    peers.sort();
    peers.dedup();
    
    Ok(peers)
}

/// Parse peers from INI format
fn parse_peers_ini(content: &str) -> Result<Vec<String>> {
    let mut peers = Vec::new();
    let mut in_peers_section = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with('[') && line.ends_with(']') {
            in_peers_section = line == "[peers]";
            continue;
        }
        
        if in_peers_section && !line.is_empty() && !line.starts_with('#') {
            if let Some((key, value)) = line.split_once('=') {
                if key.trim().starts_with("node") {
                    peers.push(value.trim().to_string());
                }
            }
        }
    }
    
    Ok(peers)
}

/// Create default peers configuration file
async fn create_default_peers_config(path: &str) -> Result<()> {
    let default_config = r#"# Arceon P2P Peers Configuration
# Add peer nodes in the format: nodeN = ip:port

[peers]
# Example peer nodes (replace with actual IPs)
node1 = 127.0.0.1:7777
node2 = 192.168.1.100:7777
# node3 = peer.arceon.world:7777

[settings]
# Maximum number of peer connections
max_peers = 50

# Connection timeout in seconds  
timeout = 30

# Enable automatic peer discovery
auto_discovery = true
"#;
    
    std::fs::write(path, default_config)?;
    info!("📝 Created default peers config at: {}", path);
    
    Ok(())
}
