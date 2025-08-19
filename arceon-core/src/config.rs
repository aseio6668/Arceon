use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub blockchain: BlockchainConfig,
    pub world: WorldConfig,
    pub ai: AiConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub listen_port: u16,
    pub bootstrap_nodes: Vec<String>,
    pub max_peers: usize,
    pub discovery_interval: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockchainConfig {
    pub data_dir: String,
    pub genesis_block: String,
    pub mining_difficulty: u32,
    pub block_time: u64,
    pub reward_amount: u64,
    pub network_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldConfig {
    pub seed: u64,
    pub continent_count: usize,
    pub city_count: usize,
    pub npc_population: usize,
    pub respawn_timers: HashMap<String, u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiConfig {
    pub npc_think_interval: u64,
    pub memory_size: usize,
    pub learning_rate: f32,
    pub personality_variance: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub data_dir: String,
    pub cache_size: usize,
    pub backup_interval: u64,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::with_prefix("ARCEON"))
            .build()?;
        
        Ok(settings.try_deserialize()?)
    }
    
    pub fn default() -> Self {
        Self {
            network: NetworkConfig {
                listen_port: 7777,
                bootstrap_nodes: vec![],
                max_peers: 50,
                discovery_interval: 30,
            },
            blockchain: BlockchainConfig {
                data_dir: "./data/blockchain".to_string(),
                genesis_block: "genesis".to_string(),
                mining_difficulty: 4,
                block_time: 10,
                reward_amount: 100,
                network_name: "arceon_mainnet".to_string(),
            },
            world: WorldConfig {
                seed: 12345,
                continent_count: 5,
                city_count: 20,
                npc_population: 1000,
                respawn_timers: {
                    let mut timers = HashMap::new();
                    timers.insert("npc".to_string(), 300);
                    timers.insert("player".to_string(), 120);
                    timers.insert("beast".to_string(), 180);
                    timers
                },
            },
            ai: AiConfig {
                npc_think_interval: 5,
                memory_size: 1000,
                learning_rate: 0.01,
                personality_variance: 0.3,
            },
            database: DatabaseConfig {
                data_dir: "./data/db".to_string(),
                cache_size: 100_000,
                backup_interval: 3600,
            },
        }
    }
}
