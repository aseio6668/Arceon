use anyhow::Result;
use arceon_core::config::BlockchainConfig;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::time::SystemTime;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use std::sync::Arc;
use sha2::{Sha256, Digest};

// Import our new blockchain modules
pub mod nft_system;
pub mod token_economy;

pub use nft_system::*;
pub use token_economy::*;

// Consensus types integrated into blockchain module
pub type BlockHash = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldChange {
    PlayerAction {
        player_id: Uuid,
        action_type: String,
        area_id: String,
        timestamp: SystemTime,
        data: serde_json::Value,
    },
    NPCAction {
        npc_id: Uuid,
        action_type: String,
        area_id: String,
        timestamp: SystemTime,
        data: serde_json::Value,
    },
    AreaUpdate {
        area_id: String,
        update_type: String,
        timestamp: SystemTime,
        data: serde_json::Value,
    },
    SkillEvolution {
        skill_name: String,
        evolution_type: String,
        timestamp: SystemTime,
        discoverer: Option<Uuid>,
        consensus_votes: u32,
    },
    WorldEvent {
        event_id: Uuid,
        event_type: String,
        timestamp: SystemTime,
        affected_areas: Vec<String>,
        data: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizedBlock {
    pub block_hash: BlockHash,
    pub epoch: u64,
    pub round: u32,
    pub proposer: Uuid,
    pub timestamp: SystemTime,
    pub world_changes: Vec<WorldChange>,
    pub validator_signatures: HashMap<Uuid, String>,
    pub merkle_root: String,
    pub previous_hash: Option<BlockHash>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMessage {
    Proposal(WorldStateProposal),
    Vote(Vote),
    ViewChange(ViewChangeVote),
    ValidatorJoin {
        node_id: Uuid,
        stake_amount: u64,
        timestamp: SystemTime,
    },
    ValidatorLeave {
        node_id: Uuid,
        timestamp: SystemTime,
    },
    SlashingEvidence {
        accused_node: Uuid,
        evidence_type: SlashingType,
        proof: Vec<u8>,
        timestamp: SystemTime,
    },
    SyncRequest {
        requester: Uuid,
        from_epoch: u64,
        to_epoch: Option<u64>,
    },
    SyncResponse {
        blocks: Vec<FinalizedBlock>,
        current_state: ConsensusState,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldStateProposal {
    pub proposal_id: Uuid,
    pub proposer: Uuid,
    pub epoch: u64,
    pub round: u32,
    pub timestamp: SystemTime,
    pub world_changes: Vec<WorldChange>,
    pub previous_block_hash: Option<BlockHash>,
    pub merkle_root: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter_id: Uuid,
    pub proposal_id: Uuid,
    pub vote_type: VoteType,
    pub epoch: u64,
    pub round: u32,
    pub timestamp: SystemTime,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Prevote(bool),   // true = vote for, false = vote against
    Precommit(bool), // true = commit, false = nil
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewChangeVote {
    pub voter_id: Uuid,
    pub new_round: u32,
    pub epoch: u64,
    pub timestamp: SystemTime,
    pub reason: ViewChangeReason,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewChangeReason {
    TimeoutPropose,
    TimeoutPrevote,
    TimeoutPrecommit,
    InvalidProposal,
    NetworkPartition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlashingType {
    DoubleVoting,
    InvalidProposal,
    Equivocation,
    Inactivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    pub current_epoch: u64,
    pub current_round: u32,
    pub current_step: ConsensusStep,
    pub last_finalized_block: Option<BlockHash>,
    pub active_proposal: Option<Uuid>,
    pub votes: HashMap<Uuid, Vote>,
    pub view_change_votes: HashMap<Uuid, ViewChangeVote>,
    pub total_stake: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsensusStep {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

/// Blockchain manager for world state persistence and decentralized consensus
pub struct BlockchainManager {
    config: BlockchainConfig,
    consensus_enabled: bool,
    blockchain_storage: Arc<RwLock<BlockchainStorage>>,
    world_state: Arc<RwLock<WorldState>>,
    message_sender: Option<mpsc::UnboundedSender<ConsensusMessage>>,
    message_receiver: Option<mpsc::UnboundedReceiver<ConsensusMessage>>,
    // Enhanced blockchain systems
    pub nft_system: Arc<RwLock<NFTSystem>>,
    pub token_economy: Arc<RwLock<TokenEconomySystem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStorage {
    pub blocks: BTreeMap<u64, FinalizedBlock>, // epoch -> block
    pub world_snapshots: HashMap<u64, WorldStateSnapshot>, // epoch -> snapshot
    pub pending_transactions: VecDeque<WorldTransaction>,
    pub last_finalized_epoch: u64,
    pub genesis_block: Option<FinalizedBlock>,
    pub blockchain_saves: Option<HashMap<String, BlockchainSaveSnapshot>>, // save_name -> snapshot
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub current_epoch: u64,
    pub players: HashMap<Uuid, PlayerState>,
    pub areas: HashMap<String, AreaState>,
    pub npcs: HashMap<Uuid, NPCState>,
    pub global_events: Vec<GlobalEvent>,
    pub skill_discoveries: HashMap<String, SkillDiscovery>,
    pub world_time: u64,
    pub last_update: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: Uuid,
    pub name: String,
    pub race: String,
    pub current_area: String,
    pub skills: HashMap<String, f64>,
    pub inventory: Vec<Item>,
    pub last_seen: SystemTime,
    pub online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaState {
    pub area_id: String,
    pub name: String,
    pub description: String,
    pub players: Vec<Uuid>,
    pub npcs: Vec<Uuid>,
    pub structures: Vec<Structure>,
    pub items: Vec<Item>,
    pub events: Vec<AreaEvent>,
    pub last_update: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCState {
    pub npc_id: Uuid,
    pub name: String,
    pub npc_type: String,
    pub current_area: String,
    pub personality: HashMap<String, f64>,
    pub skills: HashMap<String, f64>,
    pub inventory: Vec<Item>,
    pub relationships: HashMap<Uuid, f64>, // player/npc relationships
    pub last_action: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub structure_id: Uuid,
    pub name: String,
    pub structure_type: String,
    pub builder: Uuid,
    pub location: (f64, f64),
    pub interior_area_id: Option<String>,
    pub build_time: SystemTime,
    pub materials_used: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub item_id: Uuid,
    pub name: String,
    pub item_type: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub owner: Option<Uuid>,
    pub location: ItemLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemLocation {
    Area(String),
    Inventory(Uuid), // player or NPC ID
    Structure(Uuid), // structure ID
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub description: String,
    pub timestamp: SystemTime,
    pub participants: Vec<Uuid>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub description: String,
    pub timestamp: SystemTime,
    pub affected_areas: Vec<String>,
    pub global_impact: f64,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDiscovery {
    pub skill_name: String,
    pub discoverer: Uuid,
    pub discovery_method: String,
    pub discovery_time: SystemTime,
    pub validation_votes: HashMap<Uuid, bool>,
    pub confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldStateSnapshot {
    pub epoch: u64,
    pub world_state: WorldState,
    pub block_hash: BlockHash,
    pub timestamp: SystemTime,
    pub validator_signatures: HashMap<Uuid, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTransaction {
    pub transaction_id: Uuid,
    pub transaction_type: TransactionType,
    pub initiator: Uuid,
    pub timestamp: SystemTime,
    pub data: serde_json::Value,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    PlayerAction,
    NPCAction,
    AreaUpdate,
    SkillDiscovery,
    GlobalEvent,
    StructureConstruction,
    ItemTransfer,
}

impl BlockchainManager {
    pub async fn new(config: &BlockchainConfig) -> Result<Self> {
        let blockchain_storage = BlockchainStorage {
            blocks: BTreeMap::new(),
            world_snapshots: HashMap::new(),
            pending_transactions: VecDeque::new(),
            last_finalized_epoch: 0,
            genesis_block: None,
            blockchain_saves: None,
        };

        let world_state = WorldState {
            current_epoch: 0,
            players: HashMap::new(),
            areas: HashMap::new(),
            npcs: HashMap::new(),
            global_events: Vec::new(),
            skill_discoveries: HashMap::new(),
            world_time: 0,
            last_update: SystemTime::now(),
        };

        let (sender, receiver) = mpsc::unbounded_channel();

        // Initialize enhanced blockchain systems
        let nft_config = NFTConfig {
            minting_fee: 1000,
            max_supply_per_collection: Some(10000),
            royalty_percentage_cap: 10.0,
            metadata_ipfs_gateway: "https://ipfs.io/ipfs/".to_string(),
            marketplace_fee_percentage: 2.5,
            staking_min_duration_hours: 24,
            evolution_requirements_multiplier: 1.0,
            cross_chain_enabled: false,
            supported_chains: vec!["arceon".to_string()],
        };
        let nft_system = NFTSystem::new(nft_config).await?;
        let token_economy = TokenEconomySystem::new().await?;

        Ok(Self {
            config: config.clone(),
            consensus_enabled: false,
            blockchain_storage: Arc::new(RwLock::new(blockchain_storage)),
            world_state: Arc::new(RwLock::new(world_state)),
            message_sender: Some(sender),
            message_receiver: Some(receiver),
            nft_system: Arc::new(RwLock::new(nft_system)),
            token_economy: Arc::new(RwLock::new(token_economy)),
        })
    }
    
    /// Initialize blockchain with consensus manager
    pub async fn start(&mut self, node_id: Uuid, is_masternode: bool, stake_amount: u64) -> Result<()> {
        info!("🔗 Initializing blockchain system with consensus");

        // Enable consensus for decentralized mode
        self.consensus_enabled = true;

        // Create genesis block if this is the first node
        if is_masternode {
            self.create_genesis_block().await?;
        }

        // Start message processing loop
        self.start_message_processing().await?;

        info!("✅ Blockchain system initialized successfully");
        Ok(())
    }

    /// Create the genesis block
    async fn create_genesis_block(&mut self) -> Result<()> {
        info!("🌱 Creating genesis block");

        let genesis_world_state = self.world_state.read().await.clone();
        let genesis_changes = vec![
            WorldChange::WorldEvent {
                event_id: Uuid::new_v4(),
                event_type: "GENESIS".to_string(),
                timestamp: SystemTime::now(),
                affected_areas: vec!["Central Plains".to_string()],
                data: serde_json::json!({
                    "description": "The birth of Arceon's decentralized world",
                    "initial_areas": ["Central Plains", "Alderheart", "Silverleaf Enclave"],
                    "network_mode": "decentralized"
                }),
            }
        ];

        let genesis_block = FinalizedBlock {
            block_hash: self.calculate_genesis_hash()?,
            epoch: 0,
            round: 0,
            proposer: Uuid::new_v4(), // Genesis proposer
            timestamp: SystemTime::now(),
            world_changes: genesis_changes,
            validator_signatures: HashMap::new(),
            merkle_root: "genesis".to_string(),
            previous_hash: None,
        };

        // Store genesis block
        let mut storage = self.blockchain_storage.write().await;
        storage.blocks.insert(0, genesis_block.clone());
        storage.genesis_block = Some(genesis_block.clone());
        storage.last_finalized_epoch = 0;

        // Create genesis world state snapshot
        let genesis_snapshot = WorldStateSnapshot {
            epoch: 0,
            world_state: genesis_world_state,
            block_hash: genesis_block.block_hash,
            timestamp: SystemTime::now(),
            validator_signatures: HashMap::new(),
        };

        storage.world_snapshots.insert(0, genesis_snapshot);

        info!("✅ Genesis block created with hash: {:?}", genesis_block.block_hash);
        Ok(())
    }

    /// Calculate genesis block hash
    fn calculate_genesis_hash(&self) -> Result<BlockHash> {
        let mut hasher = Sha256::new();
        hasher.update(b"ARCEON_GENESIS_BLOCK");
        hasher.update(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs().to_be_bytes());
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Ok(hash)
    }

    /// Start message processing loop
    async fn start_message_processing(&mut self) -> Result<()> {
        if let Some(mut receiver) = self.message_receiver.take() {
            let blockchain_storage = self.blockchain_storage.clone();
            let world_state = self.world_state.clone();

            tokio::spawn(async move {
                while let Some(message) = receiver.recv().await {
                    if let Err(e) = Self::process_consensus_message(
                        message,
                        blockchain_storage.clone(),
                        world_state.clone(),
                    ).await {
                        error!("Error processing consensus message: {}", e);
                    }
                }
            });
        }

        Ok(())
    }

    /// Process incoming consensus messages
    async fn process_consensus_message(
        message: ConsensusMessage,
        _blockchain_storage: Arc<RwLock<BlockchainStorage>>,
        _world_state: Arc<RwLock<WorldState>>,
    ) -> Result<()> {
        debug!("🔗 Processing consensus message: {:?}", std::mem::discriminant(&message));
        
        // Simplified consensus processing for Phase 2
        // Full consensus implementation would go here
        match message {
            ConsensusMessage::Proposal(_) => {
                debug!("Received world state proposal");
            }
            ConsensusMessage::Vote(_) => {
                debug!("Received consensus vote");
            }
            ConsensusMessage::ViewChange(_) => {
                debug!("Received view change request");
            }
            _ => {
                debug!("Received other consensus message");
            }
        }
        Ok(())
    }

    /// Apply finalized block to world state
    pub async fn apply_finalized_block(&mut self, block: FinalizedBlock) -> Result<()> {
        info!("📝 Applying finalized block for epoch {}", block.epoch);

        // Update world state based on finalized changes
        let mut world_state = self.world_state.write().await;
        world_state.current_epoch = block.epoch;
        world_state.last_update = block.timestamp;

        for change in &block.world_changes {
            self.apply_world_change(&mut world_state, change).await?;
        }

        // Store block and create snapshot
        let mut storage = self.blockchain_storage.write().await;
        storage.blocks.insert(block.epoch, block.clone());
        storage.last_finalized_epoch = block.epoch;

        // Create world state snapshot
        let snapshot = WorldStateSnapshot {
            epoch: block.epoch,
            world_state: world_state.clone(),
            block_hash: block.block_hash,
            timestamp: block.timestamp,
            validator_signatures: block.validator_signatures.clone(),
        };

        storage.world_snapshots.insert(block.epoch, snapshot);

        // Clear processed transactions
        storage.pending_transactions.retain(|tx| {
            // Keep transactions that weren't included in this block
            !block.world_changes.iter().any(|change| {
                serde_json::to_value(change).map_or(false, |v| v == tx.data)
            })
        });

        info!("✅ Block {} applied successfully with {} changes", 
            block.epoch, block.world_changes.len());

        Ok(())
    }

    /// Apply individual world change to state
    async fn apply_world_change(&self, world_state: &mut WorldState, change: &WorldChange) -> Result<()> {
        match change {
            WorldChange::PlayerAction { player_id, action_type, area_id, timestamp, data } => {
                debug!("Applying player action: {} by {} in {}", action_type, player_id, area_id);
                
                // Update player state
                if let Some(player) = world_state.players.get_mut(player_id) {
                    player.current_area = area_id.clone();
                    player.last_seen = *timestamp;
                    player.online = true;
                }

                // Add to area events
                if let Some(area) = world_state.areas.get_mut(area_id) {
                    area.events.push(AreaEvent {
                        event_id: Uuid::new_v4(),
                        event_type: action_type.clone(),
                        description: format!("Player {} performed {}", player_id, action_type),
                        timestamp: *timestamp,
                        participants: vec![*player_id],
                        data: data.clone(),
                    });
                    area.last_update = *timestamp;
                }
            }

            WorldChange::NPCAction { npc_id, action_type, area_id, timestamp, data } => {
                debug!("Applying NPC action: {} by {} in {}", action_type, npc_id, area_id);
                
                // Update NPC state
                if let Some(npc) = world_state.npcs.get_mut(npc_id) {
                    npc.current_area = area_id.clone();
                    npc.last_action = *timestamp;
                }

                // Add to area events
                if let Some(area) = world_state.areas.get_mut(area_id) {
                    area.events.push(AreaEvent {
                        event_id: Uuid::new_v4(),
                        event_type: action_type.clone(),
                        description: format!("NPC {} performed {}", npc_id, action_type),
                        timestamp: *timestamp,
                        participants: vec![*npc_id],
                        data: data.clone(),
                    });
                    area.last_update = *timestamp;
                }
            }

            WorldChange::AreaUpdate { area_id, update_type, timestamp, data } => {
                debug!("Applying area update: {} in {}", update_type, area_id);
                
                if let Some(area) = world_state.areas.get_mut(area_id) {
                    area.events.push(AreaEvent {
                        event_id: Uuid::new_v4(),
                        event_type: update_type.clone(),
                        description: format!("Area update: {}", update_type),
                        timestamp: *timestamp,
                        participants: Vec::new(),
                        data: data.clone(),
                    });
                    area.last_update = *timestamp;
                }
            }

            WorldChange::SkillEvolution { skill_name, evolution_type, timestamp, discoverer, consensus_votes } => {
                debug!("Applying skill evolution: {} via {}", skill_name, evolution_type);
                
                let discovery = SkillDiscovery {
                    skill_name: skill_name.clone(),
                    discoverer: discoverer.unwrap_or(Uuid::new_v4()),
                    discovery_method: evolution_type.clone(),
                    discovery_time: *timestamp,
                    validation_votes: HashMap::new(),
                    confirmed: *consensus_votes >= 5, // Require 5+ votes for confirmation
                };

                world_state.skill_discoveries.insert(skill_name.clone(), discovery);
            }

            WorldChange::WorldEvent { event_id, event_type, timestamp, affected_areas, data } => {
                debug!("Applying world event: {} affecting {} areas", event_type, affected_areas.len());
                
                let global_event = GlobalEvent {
                    event_id: *event_id,
                    event_type: event_type.clone(),
                    description: format!("Global event: {}", event_type),
                    timestamp: *timestamp,
                    affected_areas: affected_areas.clone(),
                    global_impact: 1.0,
                    data: data.clone(),
                };

                world_state.global_events.push(global_event);
            }
        }

        Ok(())
    }

    /// Add a world change to pending transactions
    pub async fn submit_world_change(&mut self, change: WorldChange) -> Result<()> {
        if self.consensus_enabled {
            // Add to pending transactions for consensus processing
            let mut storage = self.blockchain_storage.write().await;
            let transaction = WorldTransaction {
                transaction_id: Uuid::new_v4(),
                transaction_type: match change {
                    WorldChange::PlayerAction { .. } => TransactionType::PlayerAction,
                    WorldChange::NPCAction { .. } => TransactionType::NPCAction,
                    WorldChange::AreaUpdate { .. } => TransactionType::AreaUpdate,
                    WorldChange::SkillEvolution { .. } => TransactionType::SkillDiscovery,
                    WorldChange::WorldEvent { .. } => TransactionType::GlobalEvent,
                },
                initiator: Uuid::new_v4(), // Would be actual initiator
                timestamp: SystemTime::now(),
                data: serde_json::to_value(&change)?,
                signature: None,
            };
            storage.pending_transactions.push_back(transaction);
        }
        Ok(())
    }

    /// Get current world state
    pub async fn get_world_state(&self) -> WorldState {
        self.world_state.read().await.clone()
    }

    /// Get blockchain statistics
    pub async fn get_blockchain_stats(&self) -> BlockchainStats {
        let storage = self.blockchain_storage.read().await;
        let world_state = self.world_state.read().await;
        
        BlockchainStats {
            total_blocks: storage.blocks.len(),
            last_finalized_epoch: storage.last_finalized_epoch,
            pending_transactions: storage.pending_transactions.len(),
            total_players: world_state.players.len(),
            total_areas: world_state.areas.len(),
            total_npcs: world_state.npcs.len(),
            total_events: world_state.global_events.len(),
            world_time: world_state.world_time,
        }
    }

    /// Generate a new wallet address for a player
    pub fn generate_wallet_address(&self) -> String {
        format!("ARC{}", uuid::Uuid::new_v4().to_string().replace("-", "")[..16].to_uppercase())
    }

    /// Get block by epoch
    pub async fn get_block(&self, epoch: u64) -> Option<FinalizedBlock> {
        let storage = self.blockchain_storage.read().await;
        storage.blocks.get(&epoch).cloned()
    }

    /// Get world state snapshot by epoch
    pub async fn get_world_snapshot(&self, epoch: u64) -> Option<WorldStateSnapshot> {
        let storage = self.blockchain_storage.read().await;
        storage.world_snapshots.get(&epoch).cloned()
    }

    /// Sync with other nodes (request missing blocks)
    pub async fn sync_with_network(&mut self, from_epoch: u64) -> Result<()> {
        if self.consensus_enabled {
            info!("🔄 Initiating blockchain sync from epoch {}", from_epoch);
            
            // Create sync request message
            let sync_request = ConsensusMessage::SyncRequest {
                requester: Uuid::new_v4(), // Would be actual node ID
                from_epoch,
                to_epoch: None,
            };
            
            // Send sync request through message channel
            if let Some(sender) = &self.message_sender {
                let _ = sender.send(sync_request);
            }
        }
        Ok(())
    }

    /// Save complete world state to blockchain storage (decentralized mode)
    pub async fn save_world_state_to_blockchain(&mut self, save_name: String) -> Result<String> {
        info!("💾 Saving world state '{}' to blockchain", save_name);

        if !self.consensus_enabled {
            return Err(anyhow::anyhow!("Blockchain save requires consensus mode"));
        }

        let world_state = self.world_state.read().await.clone();
        let storage = self.blockchain_storage.read().await;
        
        // Create comprehensive save data
        let save_data = BlockchainSaveData {
            save_id: Uuid::new_v4(),
            save_name: save_name.clone(),
            timestamp: SystemTime::now(),
            world_state: world_state.clone(),
            blockchain_metadata: BlockchainMetadata {
                total_blocks: storage.blocks.len(),
                last_finalized_epoch: storage.last_finalized_epoch,
                genesis_hash: storage.genesis_block.as_ref().map(|b| b.block_hash),
                validator_count: 0, // Will be filled by consensus layer
                network_id: self.calculate_network_id()?,
            },
            integrity_hash: String::new(), // Will be calculated
        };
        drop(storage);

        // Calculate integrity hash
        let mut save_data_with_hash = save_data.clone();
        save_data_with_hash.integrity_hash = self.calculate_save_integrity_hash(&save_data)?;

        // Store save data values before moving
        let save_id = save_data_with_hash.save_id;
        let integrity_hash = save_data_with_hash.integrity_hash.clone();

        // Create blockchain save snapshot
        let save_snapshot = BlockchainSaveSnapshot {
            save_data: save_data_with_hash,
            block_range: (0, world_state.current_epoch),
            compressed_blocks: self.compress_blockchain_data().await?,
            world_state_merkle_proof: self.generate_world_state_merkle_proof(&world_state).await?,
        };

        // Store in blockchain storage
        {
            let mut storage = self.blockchain_storage.write().await;
            if storage.blockchain_saves.is_none() {
                storage.blockchain_saves = Some(HashMap::new());
            }
            
            storage.blockchain_saves.as_mut().unwrap()
                .insert(save_name.clone(), save_snapshot);
        } // Drop storage lock here

        // Create world change to record the save
        let save_change = WorldChange::WorldEvent {
            event_id: Uuid::new_v4(),
            event_type: "BLOCKCHAIN_SAVE".to_string(),
            timestamp: SystemTime::now(),
            affected_areas: vec!["GLOBAL".to_string()],
            data: serde_json::json!({
                "save_name": save_name,
                "save_id": save_id,
                "epoch": world_state.current_epoch,
                "total_players": world_state.players.len(),
                "total_areas": world_state.areas.len(),
                "integrity_hash": integrity_hash
            }),
        };

        // Submit to consensus for network-wide recording
        self.submit_world_change(save_change).await?;

        info!("✅ World state '{}' saved to blockchain with integrity hash: {}", 
            save_name, integrity_hash);

        Ok(integrity_hash)
    }

    /// Load world state from blockchain storage (decentralized mode)
    pub async fn load_world_state_from_blockchain(&mut self, save_name: String) -> Result<()> {
        info!("📂 Loading world state '{}' from blockchain", save_name);

        if !self.consensus_enabled {
            return Err(anyhow::anyhow!("Blockchain load requires consensus mode"));
        }

        let storage = self.blockchain_storage.read().await;
        let save_snapshot = storage.blockchain_saves.as_ref()
            .and_then(|saves| saves.get(&save_name))
            .ok_or_else(|| anyhow::anyhow!("Save '{}' not found in blockchain", save_name))?
            .clone();
        drop(storage);

        // Verify integrity
        let calculated_hash = self.calculate_save_integrity_hash(&save_snapshot.save_data)?;
        if calculated_hash != save_snapshot.save_data.integrity_hash {
            return Err(anyhow::anyhow!("Save integrity verification failed for '{}'", save_name));
        }

        // Verify world state merkle proof
        if !self.verify_world_state_merkle_proof(&save_snapshot.save_data.world_state, &save_snapshot.world_state_merkle_proof).await? {
            return Err(anyhow::anyhow!("World state merkle proof verification failed"));
        }

        // Decompress and restore blockchain data
        self.restore_blockchain_data(save_snapshot.compressed_blocks).await?;

        // Restore world state
        let mut world_state = self.world_state.write().await;
        *world_state = save_snapshot.save_data.world_state;

        // Update blockchain storage metadata
        let mut storage = self.blockchain_storage.write().await;
        storage.last_finalized_epoch = save_snapshot.save_data.blockchain_metadata.last_finalized_epoch;

        // Create world change to record the load
        let load_change = WorldChange::WorldEvent {
            event_id: Uuid::new_v4(),
            event_type: "BLOCKCHAIN_LOAD".to_string(),
            timestamp: SystemTime::now(),
            affected_areas: vec!["GLOBAL".to_string()],
            data: serde_json::json!({
                "save_name": save_name,
                "restored_epoch": world_state.current_epoch,
                "total_players": world_state.players.len(),
                "total_areas": world_state.areas.len(),
                "integrity_verified": true
            }),
        };
        drop(storage);
        drop(world_state);

        // Submit to consensus for network-wide recording
        self.submit_world_change(load_change).await?;

        info!("✅ World state '{}' loaded successfully from blockchain", save_name);
        Ok(())
    }

    /// List all available blockchain saves
    pub async fn list_blockchain_saves(&self) -> Result<Vec<BlockchainSaveInfo>> {
        let storage = self.blockchain_storage.read().await;
        
        if let Some(saves) = &storage.blockchain_saves {
            let save_info: Vec<_> = saves.iter().map(|(name, snapshot)| {
                BlockchainSaveInfo {
                    save_name: name.clone(),
                    save_id: snapshot.save_data.save_id,
                    timestamp: snapshot.save_data.timestamp,
                    epoch: snapshot.save_data.world_state.current_epoch,
                    player_count: snapshot.save_data.world_state.players.len(),
                    area_count: snapshot.save_data.world_state.areas.len(),
                    npc_count: snapshot.save_data.world_state.npcs.len(),
                    integrity_hash: snapshot.save_data.integrity_hash.clone(),
                    block_count: snapshot.save_data.blockchain_metadata.total_blocks,
                }
            }).collect();
            
            Ok(save_info)
        } else {
            Ok(Vec::new())
        }
    }

    /// Export world state for backup (creates portable save file)
    pub async fn export_world_state(&self, save_name: String, export_path: String) -> Result<()> {
        info!("📤 Exporting world state '{}' to {}", save_name, export_path);

        let storage = self.blockchain_storage.read().await;
        let save_snapshot = storage.blockchain_saves.as_ref()
            .and_then(|saves| saves.get(&save_name))
            .ok_or_else(|| anyhow::anyhow!("Save '{}' not found", save_name))?
            .clone();
        drop(storage);

        // Create portable export data
        let export_data = PortableWorldExport {
            version: "1.0".to_string(),
            export_timestamp: SystemTime::now(),
            original_save: save_snapshot,
            network_compatibility: NetworkCompatibility {
                network_id: self.calculate_network_id()?,
                consensus_version: "1.0".to_string(),
                required_validators: 1,
                min_stake: 1000,
            },
        };

        // Serialize and write to file
        let serialized = serde_json::to_string_pretty(&export_data)?;
        std::fs::write(&export_path, serialized)?;

        info!("✅ World state exported to: {}", export_path);
        Ok(())
    }

    /// Import world state from backup file
    pub async fn import_world_state(&mut self, import_path: String, save_name: String) -> Result<()> {
        info!("📥 Importing world state from {} as '{}'", import_path, save_name);

        // Read and deserialize export file
        let file_content = std::fs::read_to_string(&import_path)?;
        let export_data: PortableWorldExport = serde_json::from_str(&file_content)?;

        // Verify network compatibility
        let current_network_id = self.calculate_network_id()?;
        if export_data.network_compatibility.network_id != current_network_id {
            warn!("⚠️ Network ID mismatch - import may cause issues");
        }

        // Verify save integrity
        let calculated_hash = self.calculate_save_integrity_hash(&export_data.original_save.save_data)?;
        if calculated_hash != export_data.original_save.save_data.integrity_hash {
            return Err(anyhow::anyhow!("Import integrity verification failed"));
        }

        // Store imported save
        let mut storage = self.blockchain_storage.write().await;
        if storage.blockchain_saves.is_none() {
            storage.blockchain_saves = Some(HashMap::new());
        }
        
        storage.blockchain_saves.as_mut().unwrap()
            .insert(save_name.clone(), export_data.original_save);

        info!("✅ World state imported successfully as '{}'", save_name);
        Ok(())
    }

    /// Create automatic checkpoint of current world state
    pub async fn create_automatic_checkpoint(&mut self) -> Result<String> {
        let checkpoint_name = format!("auto_checkpoint_{}", 
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs());
        
        info!("🔄 Creating automatic checkpoint: {}", checkpoint_name);
        self.save_world_state_to_blockchain(checkpoint_name.clone()).await?;
        
        // Cleanup old checkpoints (keep last 10)
        self.cleanup_old_checkpoints().await?;
        
        Ok(checkpoint_name)
    }

    /// Cleanup old automatic checkpoints
    async fn cleanup_old_checkpoints(&mut self) -> Result<()> {
        let mut storage = self.blockchain_storage.write().await;
        
        if let Some(saves) = &mut storage.blockchain_saves {
            let mut auto_checkpoints: Vec<_> = saves.iter()
                .filter(|(name, _)| name.starts_with("auto_checkpoint_"))
                .map(|(name, snapshot)| (name.clone(), snapshot.save_data.timestamp))
                .collect();
            
            // Sort by timestamp (newest first)
            auto_checkpoints.sort_by(|a, b| b.1.cmp(&a.1));
            
            // Remove old checkpoints (keep last 10)
            for (name, _) in auto_checkpoints.into_iter().skip(10) {
                saves.remove(&name);
                debug!("🗑️ Removed old checkpoint: {}", name);
            }
        }
        
        Ok(())
    }

    /// Calculate network ID for compatibility checking
    fn calculate_network_id(&self) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(b"ARCEON_NETWORK_V1");
        hasher.update(self.config.network_name.as_bytes());
        Ok(format!("{:x}", hasher.finalize())[..16].to_string())
    }

    /// Calculate integrity hash for save data
    fn calculate_save_integrity_hash(&self, save_data: &BlockchainSaveData) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(save_data.save_id.to_string().as_bytes());
        hasher.update(save_data.save_name.as_bytes());
        hasher.update(save_data.timestamp.duration_since(SystemTime::UNIX_EPOCH)?.as_secs().to_be_bytes());
        
        // Hash world state components
        hasher.update(save_data.world_state.current_epoch.to_be_bytes());
        hasher.update(save_data.world_state.world_time.to_be_bytes());
        hasher.update(save_data.world_state.players.len().to_be_bytes());
        hasher.update(save_data.world_state.areas.len().to_be_bytes());
        hasher.update(save_data.world_state.npcs.len().to_be_bytes());
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Compress blockchain data for storage
    async fn compress_blockchain_data(&self) -> Result<Vec<u8>> {
        let storage = self.blockchain_storage.read().await;
        let serialized = serde_json::to_vec(&*storage)?;
        
        // Simple compression (in production, would use proper compression)
        Ok(serialized)
    }

    /// Restore blockchain data from compressed format
    async fn restore_blockchain_data(&mut self, compressed_data: Vec<u8>) -> Result<()> {
        // Decompress and deserialize
        let restored_storage: BlockchainStorage = serde_json::from_slice(&compressed_data)?;
        
        let mut storage = self.blockchain_storage.write().await;
        storage.blocks = restored_storage.blocks;
        storage.world_snapshots = restored_storage.world_snapshots;
        storage.last_finalized_epoch = restored_storage.last_finalized_epoch;
        storage.genesis_block = restored_storage.genesis_block;
        
        Ok(())
    }

    /// Generate merkle proof for world state
    async fn generate_world_state_merkle_proof(&self, world_state: &WorldState) -> Result<MerkleProof> {
        let mut hasher = Sha256::new();
        
        // Hash all world state components
        let players_hash = {
            let mut h = Sha256::new();
            for (id, player) in &world_state.players {
                h.update(id.to_string().as_bytes());
                h.update(player.name.as_bytes());
                h.update(player.race.as_bytes());
            }
            format!("{:x}", h.finalize())
        };
        
        let areas_hash = {
            let mut h = Sha256::new();
            for (id, area) in &world_state.areas {
                h.update(id.as_bytes());
                h.update(area.name.as_bytes());
            }
            format!("{:x}", h.finalize())
        };
        
        let npcs_hash = {
            let mut h = Sha256::new();
            for (id, npc) in &world_state.npcs {
                h.update(id.to_string().as_bytes());
                h.update(npc.name.as_bytes());
            }
            format!("{:x}", h.finalize())
        };
        
        hasher.update(players_hash.as_bytes());
        hasher.update(areas_hash.as_bytes());
        hasher.update(npcs_hash.as_bytes());
        hasher.update(world_state.world_time.to_be_bytes());
        
        Ok(MerkleProof {
            root_hash: format!("{:x}", hasher.finalize()),
            proof_elements: vec![players_hash, areas_hash, npcs_hash],
            leaf_index: 0,
        })
    }

    /// Verify world state merkle proof
    async fn verify_world_state_merkle_proof(&self, world_state: &WorldState, proof: &MerkleProof) -> Result<bool> {
        let generated_proof = self.generate_world_state_merkle_proof(world_state).await?;
        Ok(generated_proof.root_hash == proof.root_hash)
    }

    /// Calculate merkle root for world changes
    fn calculate_merkle_root(&self, changes: &[WorldChange]) -> Result<String> {
        if changes.is_empty() {
            return Ok("0".repeat(64));
        }

        let mut hasher = Sha256::new();
        for change in changes {
            let serialized = serde_json::to_string(change)?;
            hasher.update(serialized.as_bytes());
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Conflict resolution for concurrent world modifications
    pub async fn resolve_world_state_conflicts(&mut self, conflicting_proposals: Vec<WorldStateProposal>) -> Result<WorldStateProposal> {
        info!("🔧 Resolving {} conflicting world state proposals", conflicting_proposals.len());

        if conflicting_proposals.is_empty() {
            return Err(anyhow::anyhow!("No proposals to resolve"));
        }

        if conflicting_proposals.len() == 1 {
            return Ok(conflicting_proposals.into_iter().next().unwrap());
        }

        // Group proposals by priority and timestamp
        let mut prioritized_proposals = self.prioritize_proposals(&conflicting_proposals).await?;
        
        // Sort by priority (highest first), then by timestamp (earliest first)
        prioritized_proposals.sort_by(|a, b| {
            b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.proposal.timestamp.cmp(&b.proposal.timestamp))
        });

        // Start with the highest priority proposal as base
        let mut resolved_proposal = prioritized_proposals[0].proposal.clone();
        let mut resolved_changes = resolved_proposal.world_changes.clone();

        // Merge compatible changes from other proposals
        for prioritized in &prioritized_proposals[1..] {
            let compatible_changes = self.find_compatible_changes(
                &resolved_changes,
                &prioritized.proposal.world_changes
            ).await?;

            for change in compatible_changes {
                if !self.conflicts_with_existing(&resolved_changes, &change).await? {
                    resolved_changes.push(change);
                }
            }
        }

        // Apply conflict resolution rules
        resolved_changes = self.apply_conflict_resolution_rules(resolved_changes).await?;

        // Update the resolved proposal
        resolved_proposal.world_changes = resolved_changes;
        resolved_proposal.proposal_id = Uuid::new_v4(); // New ID for resolved proposal
        resolved_proposal.merkle_root = self.calculate_merkle_root(&resolved_proposal.world_changes)?;
        resolved_proposal.timestamp = SystemTime::now();

        info!("✅ Resolved conflicts into single proposal with {} changes", 
            resolved_proposal.world_changes.len());

        Ok(resolved_proposal)
    }

    /// Prioritize proposals based on various factors
    async fn prioritize_proposals(&self, proposals: &[WorldStateProposal]) -> Result<Vec<PrioritizedProposal>> {
        let mut prioritized = Vec::new();

        for proposal in proposals {
            let mut priority = 0.0;

            // Priority based on proposer reputation (if validator)
            if let Some(validator_info) = self.get_validator_info(proposal.proposer).await {
                priority += validator_info.reputation_score * 0.3;
                priority += (validator_info.voting_power * 100.0) * 0.2;
            }

            // Priority based on proposal timestamp (earlier = higher priority)
            let age_seconds = SystemTime::now()
                .duration_since(proposal.timestamp)
                .unwrap_or_default()
                .as_secs();
            priority += (100.0 - (age_seconds as f64).min(100.0)) * 0.1;

            // Priority based on change types (critical changes get higher priority)
            priority += self.calculate_change_priority(&proposal.world_changes) * 0.4;

            prioritized.push(PrioritizedProposal {
                proposal: proposal.clone(),
                priority,
            });
        }

        Ok(prioritized)
    }

    /// Calculate priority based on change types
    fn calculate_change_priority(&self, changes: &[WorldChange]) -> f64 {
        let mut priority = 0.0;
        
        for change in changes {
            match change {
                WorldChange::PlayerAction { .. } => priority += 1.0,
                WorldChange::NPCAction { .. } => priority += 0.8,
                WorldChange::AreaUpdate { .. } => priority += 1.2,
                WorldChange::SkillEvolution { .. } => priority += 2.0, // High priority
                WorldChange::WorldEvent { .. } => priority += 1.5,
            }
        }

        priority / changes.len() as f64
    }

    /// Find changes that don't conflict with existing changes
    async fn find_compatible_changes(&self, existing: &[WorldChange], new: &[WorldChange]) -> Result<Vec<WorldChange>> {
        let mut compatible = Vec::new();

        for new_change in new {
            let mut is_compatible = true;

            for existing_change in existing {
                if self.changes_conflict(existing_change, new_change).await? {
                    is_compatible = false;
                    break;
                }
            }

            if is_compatible {
                compatible.push(new_change.clone());
            }
        }

        Ok(compatible)
    }

    /// Check if a new change conflicts with existing changes
    async fn conflicts_with_existing(&self, existing: &[WorldChange], new_change: &WorldChange) -> Result<bool> {
        for existing_change in existing {
            if self.changes_conflict(existing_change, new_change).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Determine if two world changes conflict with each other
    async fn changes_conflict(&self, change1: &WorldChange, change2: &WorldChange) -> Result<bool> {
        match (change1, change2) {
            // Player actions in the same area at similar times may conflict
            (WorldChange::PlayerAction { player_id: id1, area_id: area1, timestamp: t1, .. },
             WorldChange::PlayerAction { player_id: id2, area_id: area2, timestamp: t2, .. }) => {
                if id1 == id2 {
                    // Same player can't do two things simultaneously
                    let time_diff = t1.duration_since(*t2).unwrap_or_else(|_| t2.duration_since(*t1).unwrap_or_default());
                    return Ok(time_diff.as_secs() < 1); // Conflict if within 1 second
                }
                if area1 == area2 {
                    // Check for area-specific conflicts (e.g., both trying to pick up same item)
                    return Ok(self.check_area_specific_conflicts(change1, change2).await?);
                }
                Ok(false)
            }

            // Area updates in the same area may conflict
            (WorldChange::AreaUpdate { area_id: area1, .. },
             WorldChange::AreaUpdate { area_id: area2, .. }) => {
                Ok(area1 == area2)
            }

            // Skill evolutions with the same skill name conflict
            (WorldChange::SkillEvolution { skill_name: skill1, .. },
             WorldChange::SkillEvolution { skill_name: skill2, .. }) => {
                Ok(skill1 == skill2)
            }

            // World events may conflict if they affect the same areas
            (WorldChange::WorldEvent { affected_areas: areas1, .. },
             WorldChange::WorldEvent { affected_areas: areas2, .. }) => {
                Ok(areas1.iter().any(|area| areas2.contains(area)))
            }

            // Different types generally don't conflict
            _ => Ok(false),
        }
    }

    /// Check for area-specific conflicts between player actions
    async fn check_area_specific_conflicts(&self, change1: &WorldChange, change2: &WorldChange) -> Result<bool> {
        if let (WorldChange::PlayerAction { action_type: action1, data: data1, .. },
                WorldChange::PlayerAction { action_type: action2, data: data2, .. }) = (change1, change2) {
            
            // Check for item pickup conflicts
            if action1 == "pickup" && action2 == "pickup" {
                if let (Ok(item1), Ok(item2)) = (
                    serde_json::from_value::<String>(data1["item_id"].clone()),
                    serde_json::from_value::<String>(data2["item_id"].clone())
                ) {
                    return Ok(item1 == item2); // Conflict if picking up same item
                }
            }

            // Check for movement conflicts (same destination)
            if action1 == "move" && action2 == "move" {
                if let (Ok(pos1), Ok(pos2)) = (
                    serde_json::from_value::<(f64, f64)>(data1["position"].clone()),
                    serde_json::from_value::<(f64, f64)>(data2["position"].clone())
                ) {
                    let distance = ((pos1.0 - pos2.0).powi(2) + (pos1.1 - pos2.1).powi(2)).sqrt();
                    return Ok(distance < 1.0); // Conflict if moving to very close positions
                }
            }

            // Check for crafting conflicts (same resource usage)
            if action1 == "craft" && action2 == "craft" {
                if let (Ok(resource1), Ok(resource2)) = (
                    serde_json::from_value::<String>(data1["resource_id"].clone()),
                    serde_json::from_value::<String>(data2["resource_id"].clone())
                ) {
                    return Ok(resource1 == resource2); // Conflict if using same resource
                }
            }
        }

        Ok(false)
    }

    /// Apply conflict resolution rules to a set of changes
    async fn apply_conflict_resolution_rules(&self, mut changes: Vec<WorldChange>) -> Result<Vec<WorldChange>> {
        // Sort changes by timestamp to process them in order
        changes.sort_by(|a, b| self.get_change_timestamp(a).cmp(&self.get_change_timestamp(b)));

        let mut resolved_changes = Vec::new();
        let mut processed_conflicts = std::collections::HashSet::new();

        for (i, change) in changes.iter().enumerate() {
            if processed_conflicts.contains(&i) {
                continue; // Skip already processed conflicts
            }

            let mut conflicting_indices = vec![i];
            
            // Find all changes that conflict with this one
            for (j, other_change) in changes.iter().enumerate().skip(i + 1) {
                if processed_conflicts.contains(&j) {
                    continue;
                }
                
                if self.changes_conflict(change, other_change).await? {
                    conflicting_indices.push(j);
                }
            }

            if conflicting_indices.len() == 1 {
                // No conflicts, add the change as-is
                resolved_changes.push(change.clone());
            } else {
                // Resolve the conflict using priority rules
                let resolved_change = self.resolve_single_conflict(
                    &conflicting_indices.iter().map(|&idx| &changes[idx]).collect::<Vec<_>>()
                ).await?;
                
                if let Some(resolved) = resolved_change {
                    resolved_changes.push(resolved);
                }
                
                // Mark all conflicting indices as processed
                for &idx in &conflicting_indices {
                    processed_conflicts.insert(idx);
                }
            }
        }

        Ok(resolved_changes)
    }

    /// Resolve a single conflict between multiple changes
    async fn resolve_single_conflict(&self, conflicting_changes: &[&WorldChange]) -> Result<Option<WorldChange>> {
        if conflicting_changes.is_empty() {
            return Ok(None);
        }

        if conflicting_changes.len() == 1 {
            return Ok(Some((*conflicting_changes[0]).clone()));
        }

        // Use earliest timestamp as the winner (first-come-first-served)
        let earliest = conflicting_changes.iter()
            .min_by_key(|change| self.get_change_timestamp(change))
            .unwrap();

        debug!("🔧 Conflict resolved: selected earliest change at {:?}", 
            self.get_change_timestamp(earliest));

        Ok(Some((*earliest).clone()))
    }

    /// Get timestamp from a world change
    fn get_change_timestamp(&self, change: &WorldChange) -> SystemTime {
        match change {
            WorldChange::PlayerAction { timestamp, .. } => *timestamp,
            WorldChange::NPCAction { timestamp, .. } => *timestamp,
            WorldChange::AreaUpdate { timestamp, .. } => *timestamp,
            WorldChange::SkillEvolution { timestamp, .. } => *timestamp,
            WorldChange::WorldEvent { timestamp, .. } => *timestamp,
        }
    }

    /// Get validator information for a node
    async fn get_validator_info(&self, node_id: Uuid) -> Option<ValidatorInfo> {
        // This would interface with the consensus layer
        // For now, return default validator info
        Some(ValidatorInfo {
            node_id,
            stake_amount: 1000,
            voting_power: 0.1,
            is_active: true,
            last_activity: SystemTime::now(),
            reputation_score: 100.0,
            blocks_produced: 0,
            slashing_count: 0,
        })
    }

    /// Create conflict resolution report
    pub async fn create_conflict_resolution_report(&self, original_proposals: &[WorldStateProposal], resolved_proposal: &WorldStateProposal) -> ConflictResolutionReport {
        let total_original_changes: usize = original_proposals.iter().map(|p| p.world_changes.len()).sum();
        let resolved_changes = resolved_proposal.world_changes.len();
        let conflicts_resolved = total_original_changes.saturating_sub(resolved_changes);

        ConflictResolutionReport {
            resolution_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            original_proposal_count: original_proposals.len(),
            total_original_changes,
            resolved_changes,
            conflicts_resolved,
            resolution_method: "priority_based_timestamp_ordering".to_string(),
            success: true,
        }
    }

    /// Masternode validation and reward system
    pub async fn validate_masternode_eligibility(&self, node_id: Uuid, stake_amount: u64) -> Result<MasternodeEligibility> {
        info!("🏛️ Validating masternode eligibility for node: {}", node_id);

        let mut eligibility = MasternodeEligibility {
            node_id,
            is_eligible: false,
            current_stake: stake_amount,
            required_stake: self.config.reward_amount * 100, // 100x block reward as minimum stake
            reputation_score: 0.0,
            uptime_percentage: 0.0,
            block_production_history: Vec::new(),
            validation_errors: Vec::new(),
            estimated_rewards: MasternodeRewards::default(),
        };

        // Check minimum stake requirement
        if stake_amount < eligibility.required_stake {
            eligibility.validation_errors.push(format!(
                "Insufficient stake: {} < {} required", 
                stake_amount, 
                eligibility.required_stake
            ));
            return Ok(eligibility);
        }

        // Check reputation score
        if let Some(validator_info) = self.get_validator_info(node_id).await {
            eligibility.reputation_score = validator_info.reputation_score;
            
            if validator_info.reputation_score < 50.0 {
                eligibility.validation_errors.push(format!(
                    "Low reputation score: {:.1} < 50.0 required", 
                    validator_info.reputation_score
                ));
            }

            if validator_info.slashing_count > 5 {
                eligibility.validation_errors.push(format!(
                    "Too many slashing events: {} > 5 allowed", 
                    validator_info.slashing_count
                ));
            }

            // Calculate uptime based on recent activity
            eligibility.uptime_percentage = self.calculate_node_uptime(node_id).await?;
            if eligibility.uptime_percentage < 95.0 {
                eligibility.validation_errors.push(format!(
                    "Low uptime: {:.1}% < 95% required", 
                    eligibility.uptime_percentage
                ));
            }

            // Get block production history
            eligibility.block_production_history = self.get_block_production_history(node_id).await?;
        }

        // Calculate estimated rewards
        eligibility.estimated_rewards = self.calculate_masternode_rewards(
            stake_amount, 
            eligibility.reputation_score,
            eligibility.uptime_percentage
        ).await?;

        // Final eligibility check
        eligibility.is_eligible = eligibility.validation_errors.is_empty();

        if eligibility.is_eligible {
            info!("✅ Node {} is eligible for masternode status", node_id);
        } else {
            warn!("❌ Node {} failed masternode validation: {:?}", node_id, eligibility.validation_errors);
        }

        Ok(eligibility)
    }

    /// Calculate node uptime percentage
    async fn calculate_node_uptime(&self, node_id: Uuid) -> Result<f64> {
        // This would check the last 30 days of activity
        // For now, return a default based on validator info
        if let Some(validator) = self.get_validator_info(node_id).await {
            let time_since_activity = SystemTime::now()
                .duration_since(validator.last_activity)
                .unwrap_or_default()
                .as_secs();
            
            // Calculate uptime based on recent activity (simplified)
            let uptime = if time_since_activity < 300 { // 5 minutes
                100.0
            } else if time_since_activity < 3600 { // 1 hour
                95.0
            } else if time_since_activity < 86400 { // 1 day
                80.0
            } else {
                50.0
            };
            
            Ok(uptime)
        } else {
            Ok(0.0)
        }
    }

    /// Get block production history for a node
    async fn get_block_production_history(&self, node_id: Uuid) -> Result<Vec<BlockProductionRecord>> {
        let storage = self.blockchain_storage.read().await;
        let mut history = Vec::new();

        // Check last 100 blocks for this proposer
        let recent_blocks: Vec<_> = storage.blocks.values()
            .filter(|block| block.proposer == node_id)
            .rev()
            .take(100)
            .collect();

        for block in recent_blocks {
            history.push(BlockProductionRecord {
                block_hash: block.block_hash,
                epoch: block.epoch,
                timestamp: block.timestamp,
                world_changes_count: block.world_changes.len(),
                validation_time_ms: 0, // Would be measured in real implementation
                success: true,
            });
        }

        Ok(history)
    }

    /// Calculate masternode rewards
    async fn calculate_masternode_rewards(&self, stake_amount: u64, reputation_score: f64, uptime_percentage: f64) -> Result<MasternodeRewards> {
        let base_reward = self.config.reward_amount;
        
        // Stake multiplier (more stake = higher rewards, but with diminishing returns)
        let stake_multiplier = (stake_amount as f64 / (self.config.reward_amount * 100) as f64).sqrt().min(2.0);
        
        // Reputation multiplier
        let reputation_multiplier = (reputation_score / 100.0).max(0.1);
        
        // Uptime multiplier
        let uptime_multiplier = (uptime_percentage / 100.0).max(0.1);
        
        // Calculate various reward types
        let block_reward = (base_reward as f64 * stake_multiplier * reputation_multiplier * uptime_multiplier) as u64;
        let validation_reward = block_reward / 10; // 10% of block reward for validation work
        let consensus_reward = block_reward / 20; // 5% for consensus participation
        
        // Annual estimates (assuming ~52,560 blocks per year at 10-second intervals)
        let blocks_per_year = 525600 / 10;
        let estimated_blocks_per_node = blocks_per_year / self.get_active_masternode_count().await.max(1);
        
        Ok(MasternodeRewards {
            block_reward_per_block: block_reward,
            validation_reward_per_block: validation_reward,
            consensus_reward_per_block: consensus_reward,
            estimated_daily_rewards: (block_reward + validation_reward + consensus_reward) * (estimated_blocks_per_node / 365),
            estimated_monthly_rewards: (block_reward + validation_reward + consensus_reward) * (estimated_blocks_per_node / 12),
            estimated_annual_rewards: (block_reward + validation_reward + consensus_reward) * estimated_blocks_per_node,
            stake_multiplier,
            reputation_multiplier,
            uptime_multiplier,
        })
    }

    /// Get count of active masternodes
    async fn get_active_masternode_count(&self) -> u64 {
        // This would query the consensus layer for active masternode count
        // For now, return a default
        10
    }

    /// Process masternode reward distribution
    pub async fn distribute_masternode_rewards(&mut self, epoch: u64) -> Result<RewardDistribution> {
        info!("💰 Distributing masternode rewards for epoch {}", epoch);

        let (proposer, block_signatures) = {
            let storage = self.blockchain_storage.read().await;
            let block = storage.blocks.get(&epoch)
                .ok_or_else(|| anyhow::anyhow!("Block not found for epoch {}", epoch))?;
            
            (block.proposer, block.validator_signatures.clone())
        };

        // Validate that the proposer is an eligible masternode
        let proposer_stake = 10000; // Would get actual stake from consensus layer
        let eligibility = self.validate_masternode_eligibility(proposer, proposer_stake).await?;
        
        if !eligibility.is_eligible {
            return Err(anyhow::anyhow!("Block proposer {} is not an eligible masternode", proposer));
        }

        // Calculate rewards
        let mut distribution = RewardDistribution {
            epoch,
            total_rewards_distributed: 0,
            masternode_rewards: Vec::new(),
            validator_rewards: Vec::new(),
            treasury_allocation: 0,
            burn_amount: 0,
        };

        // Primary reward for block proposer
        let proposer_reward = MasternodeReward {
            node_id: proposer,
            reward_type: RewardType::BlockProduction,
            amount: eligibility.estimated_rewards.block_reward_per_block,
            epoch,
            timestamp: SystemTime::now(),
        };

        distribution.masternode_rewards.push(proposer_reward);
        distribution.total_rewards_distributed += eligibility.estimated_rewards.block_reward_per_block;

        // Validation rewards for all validators who signed the block
        for (validator_id, _signature) in &block_signatures {
            let validator_stake = 5000; // Would get actual stake
            let validator_eligibility = self.validate_masternode_eligibility(*validator_id, validator_stake).await?;
            
            if validator_eligibility.is_eligible {
                let validation_reward = MasternodeReward {
                    node_id: *validator_id,
                    reward_type: RewardType::Validation,
                    amount: validator_eligibility.estimated_rewards.validation_reward_per_block,
                    epoch,
                    timestamp: SystemTime::now(),
                };

                distribution.validator_rewards.push(validation_reward);
                distribution.total_rewards_distributed += validator_eligibility.estimated_rewards.validation_reward_per_block;
            }
        }

        // Treasury allocation (10% of total rewards)
        distribution.treasury_allocation = distribution.total_rewards_distributed / 10;

        // Apply rewards to world state (this would update actual balances)
        self.apply_reward_distribution(&distribution).await?;

        info!("✅ Distributed {} total rewards to {} masternodes and {} validators", 
            distribution.total_rewards_distributed,
            distribution.masternode_rewards.len(),
            distribution.validator_rewards.len());

        Ok(distribution)
    }

    /// Apply reward distribution to world state
    async fn apply_reward_distribution(&mut self, distribution: &RewardDistribution) -> Result<()> {
        // Create world changes for reward distribution
        let mut reward_changes = Vec::new();

        // Masternode rewards
        for reward in &distribution.masternode_rewards {
            reward_changes.push(WorldChange::WorldEvent {
                event_id: Uuid::new_v4(),
                event_type: "MASTERNODE_REWARD".to_string(),
                timestamp: reward.timestamp,
                affected_areas: vec!["GLOBAL".to_string()],
                data: serde_json::json!({
                    "node_id": reward.node_id,
                    "reward_type": format!("{:?}", reward.reward_type),
                    "amount": reward.amount,
                    "epoch": reward.epoch
                }),
            });
        }

        // Validator rewards
        for reward in &distribution.validator_rewards {
            reward_changes.push(WorldChange::WorldEvent {
                event_id: Uuid::new_v4(),
                event_type: "VALIDATOR_REWARD".to_string(),
                timestamp: reward.timestamp,
                affected_areas: vec!["GLOBAL".to_string()],
                data: serde_json::json!({
                    "node_id": reward.node_id,
                    "amount": reward.amount,
                    "epoch": reward.epoch
                }),
            });
        }

        // Treasury allocation
        if distribution.treasury_allocation > 0 {
            reward_changes.push(WorldChange::WorldEvent {
                event_id: Uuid::new_v4(),
                event_type: "TREASURY_ALLOCATION".to_string(),
                timestamp: SystemTime::now(),
                affected_areas: vec!["GLOBAL".to_string()],
                data: serde_json::json!({
                    "amount": distribution.treasury_allocation,
                    "epoch": distribution.epoch
                }),
            });
        }

        // Submit all reward changes for consensus
        for change in reward_changes {
            self.submit_world_change(change).await?;
        }

        Ok(())
    }

    /// Get masternode performance statistics
    pub async fn get_masternode_performance_stats(&self, node_id: Uuid) -> Result<MasternodePerformanceStats> {
        let history = self.get_block_production_history(node_id).await?;
        let total_blocks = history.len();
        
        let recent_performance = if total_blocks > 0 {
            let recent_blocks: Vec<_> = history.iter().take(20).collect(); // Last 20 blocks
            let avg_changes = recent_blocks.iter().map(|b| b.world_changes_count).sum::<usize>() as f64 / total_blocks.min(20) as f64;
            let avg_validation_time = recent_blocks.iter().map(|b| b.validation_time_ms).sum::<u64>() as f64 / total_blocks.min(20) as f64;
            
            (avg_changes, avg_validation_time)
        } else {
            (0.0, 0.0)
        };

        let uptime = self.calculate_node_uptime(node_id).await?;
        let validator_info = self.get_validator_info(node_id).await;

        Ok(MasternodePerformanceStats {
            node_id,
            total_blocks_produced: total_blocks as u64,
            average_block_changes: recent_performance.0,
            average_validation_time_ms: recent_performance.1,
            uptime_percentage: uptime,
            reputation_score: validator_info.map(|v| v.reputation_score).unwrap_or(0.0),
            total_rewards_earned: self.calculate_total_rewards_earned(node_id).await?,
            last_block_produced: history.first().map(|b| b.timestamp),
            performance_rating: self.calculate_performance_rating(uptime, recent_performance.0, recent_performance.1).await,
        })
    }

    /// Calculate total rewards earned by a masternode
    async fn calculate_total_rewards_earned(&self, node_id: Uuid) -> Result<u64> {
        // This would query historical reward records
        // For now, estimate based on block production
        let history = self.get_block_production_history(node_id).await?;
        let base_reward = self.config.reward_amount;
        Ok(history.len() as u64 * base_reward)
    }

    /// Calculate performance rating
    async fn calculate_performance_rating(&self, uptime: f64, avg_changes: f64, avg_validation_time: f64) -> PerformanceRating {
        let uptime_score = (uptime / 100.0) * 40.0; // 40% weight
        let efficiency_score = (avg_changes / 10.0).min(1.0) * 30.0; // 30% weight, max 10 changes
        let speed_score = ((1000.0 - avg_validation_time.min(1000.0)) / 1000.0) * 30.0; // 30% weight
        
        let total_score = uptime_score + efficiency_score + speed_score;
        
        if total_score >= 90.0 {
            PerformanceRating::Excellent
        } else if total_score >= 75.0 {
            PerformanceRating::Good
        } else if total_score >= 60.0 {
            PerformanceRating::Average
        } else if total_score >= 40.0 {
            PerformanceRating::Poor
        } else {
            PerformanceRating::Critical
        }
    }

    // Enhanced blockchain system integration methods

    /// Get access to the NFT system
    pub async fn get_nft_system(&self) -> Arc<RwLock<NFTSystem>> {
        self.nft_system.clone()
    }

    /// Get access to the token economy system
    pub async fn get_token_economy(&self) -> Arc<RwLock<TokenEconomySystem>> {
        self.token_economy.clone()
    }

    /// Create a blockchain-backed NFT (integrates with consensus)
    pub async fn create_blockchain_nft(&mut self, collection_id: String, owner_id: Uuid, attributes: Vec<NFTAttribute>) -> Result<Uuid> {
        info!("🎨 Creating blockchain-backed NFT in collection: {}", collection_id);

        let token_id = {
            let nft_system = self.nft_system.write().await;
            let mint_request = crate::nft_system::MintRequest {
                collection_id: collection_id.clone(),
                recipient_id: owner_id,
                creator_id: owner_id,
                name: "Blockchain NFT".to_string(),
                description: "NFT created via blockchain consensus".to_string(),
                image_url: "https://example.com/nft.jpg".to_string(),
                metadata_uri: "https://example.com/metadata.json".to_string(),
                external_url: None,
                attributes: attributes.clone(),
                utility_features: vec![],
            };
            nft_system.mint_nft(mint_request).await?
        };

        // Create blockchain record of NFT creation
        let nft_creation_change = WorldChange::WorldEvent {
            event_id: Uuid::new_v4(),
            event_type: "NFT_CREATION".to_string(),
            timestamp: SystemTime::now(),
            affected_areas: vec!["GLOBAL".to_string()],
            data: serde_json::json!({
                "token_id": token_id,
                "collection_id": collection_id,
                "owner_id": owner_id,
                "attributes": attributes,
                "creation_method": "blockchain_consensus"
            }),
        };

        // Submit to consensus for network-wide recording
        self.submit_world_change(nft_creation_change).await?;

        info!("✅ Blockchain NFT created with ID: {}", token_id);
        Ok(token_id)
    }

    /// Transfer tokens through blockchain consensus
    pub async fn transfer_tokens(&mut self, from: Uuid, to: Uuid, token_type: String, amount: u64) -> Result<String> {
        info!("💰 Processing blockchain token transfer: {} {} from {} to {}", amount, token_type, from, to);

        let token_economy = self.token_economy.read().await;
        let transaction_id = token_economy.transfer_tokens(from, to, token_type.clone(), amount).await?;

        let transaction_id_str = transaction_id.to_string();
        drop(token_economy);

        // Create blockchain record of token transfer
        let transfer_change = WorldChange::WorldEvent {
            event_id: Uuid::new_v4(),
            event_type: "TOKEN_TRANSFER".to_string(),
            timestamp: SystemTime::now(),
            affected_areas: vec!["GLOBAL".to_string()],
            data: serde_json::json!({
                "transaction_id": transaction_id_str,
                "from": from,
                "to": to,
                "token_type": token_type,
                "amount": amount,
                "consensus_verified": true
            }),
        };

        // Submit to consensus for network-wide recording
        self.submit_world_change(transfer_change).await?;

        info!("✅ Blockchain token transfer completed: {}", transaction_id_str);
        Ok(transaction_id_str)
    }

    /// Get comprehensive blockchain economy stats
    pub async fn get_economy_stats(&self) -> Result<BlockchainEconomyStats> {
        let nft_system = self.nft_system.read().await;
        let token_economy = self.token_economy.read().await;
        let blockchain_stats = self.get_blockchain_stats().await;

        let nft_stats = nft_system.get_system_stats().await?;
        let token_stats = token_economy.get_economy_stats().await?;

        Ok(BlockchainEconomyStats {
            blockchain_stats,
            total_nfts: nft_stats.total_tokens,
            total_collections: nft_stats.total_collections,
            nft_volume_24h: nft_stats.volume_24h,
            active_nft_traders: nft_stats.active_users_24h,
            total_token_supply: token_stats.total_supply,
            circulating_supply: token_stats.circulating_supply,
            total_staked: token_stats.total_staked,
            defi_tvl: token_stats.defi_tvl,
            governance_proposals_active: token_stats.active_proposals,
            cross_chain_volume_24h: token_stats.cross_chain_volume_24h,
        })
    }

    /// Process DeFi operations through blockchain consensus
    pub async fn process_defi_operation(&mut self, user_id: Uuid, operation: DeFiOperation) -> Result<String> {
        info!("🏦 Processing DeFi operation through blockchain consensus");

        let token_economy = self.token_economy.read().await;
        let result = match &operation {
            DeFiOperation::AddLiquidity { pool_id, token_a_amount, token_b_amount, .. } => {
                let pool_id_str = pool_id.to_string();
                token_economy.add_liquidity(user_id, pool_id_str, *token_a_amount, *token_b_amount).await?.to_string()
            }
            DeFiOperation::RemoveLiquidity { pool_id, liquidity_amount, .. } => {
                // For now, we'll simulate remove liquidity by swapping tokens
                let pool_id_str = pool_id.to_string();
                token_economy.swap_tokens(user_id, pool_id_str, "LP_TOKEN".to_string(), *liquidity_amount, 1).await?.to_string()
            }
            DeFiOperation::Stake { token_type, amount, duration_days: _ } => {
                // Simulate staking by creating a token transfer to a staking pool
                let staking_pool_id = Uuid::new_v4();
                token_economy.transfer_tokens(user_id, staking_pool_id, token_type.clone(), *amount).await?.to_string()
            }
            DeFiOperation::YieldFarm { farm_id: _, amount } => {
                // Simulate yield farming by minting reward tokens
                token_economy.mint_tokens("REWARD".to_string(), user_id, *amount / 10).await?.to_string()
            }
            DeFiOperation::Lend { token_type, amount, interest_rate: _ } => {
                // Simulate lending by transferring to lending pool
                let lending_pool_id = Uuid::new_v4();
                token_economy.transfer_tokens(user_id, lending_pool_id, token_type.clone(), *amount).await?.to_string()
            }
            DeFiOperation::Borrow { token_type, amount, collateral_type: _, collateral_amount: _ } => {
                // Simulate borrowing by minting borrowed tokens
                token_economy.mint_tokens(token_type.clone(), user_id, *amount).await?.to_string()
            }
        };

        drop(token_economy);

        // Create blockchain record of DeFi operation
        let defi_change = WorldChange::WorldEvent {
            event_id: Uuid::new_v4(),
            event_type: "DEFI_OPERATION".to_string(),
            timestamp: SystemTime::now(),
            affected_areas: vec!["GLOBAL".to_string()],
            data: serde_json::json!({
                "user_id": user_id,
                "operation": operation,
                "result": result,
                "consensus_verified": true
            }),
        };

        // Submit to consensus for network-wide recording
        self.submit_world_change(defi_change).await?;

        info!("✅ DeFi operation completed: {}", result);
        Ok(result)
    }

    /// Cross-chain bridge operation through blockchain consensus
    pub async fn initiate_cross_chain_bridge(&mut self, user_id: Uuid, source_chain: String, target_chain: String, token_type: String, amount: u64) -> Result<String> {
        info!("🌉 Initiating cross-chain bridge operation");

        // For now, simulate cross-chain bridge by transferring tokens to a bridge pool
        let bridge_pool_id = Uuid::new_v4();
        let token_economy = self.token_economy.read().await;
        let transfer_id = token_economy.transfer_tokens(user_id, bridge_pool_id, token_type.clone(), amount).await?;
        
        let bridge_id = format!("bridge_{}", transfer_id);
        drop(token_economy);

        // Create blockchain record of bridge operation
        let bridge_change = WorldChange::WorldEvent {
            event_id: Uuid::new_v4(),
            event_type: "CROSS_CHAIN_BRIDGE".to_string(),
            timestamp: SystemTime::now(),
            affected_areas: vec!["GLOBAL".to_string()],
            data: serde_json::json!({
                "bridge_id": bridge_id,
                "user_id": user_id,
                "source_chain": source_chain,
                "target_chain": target_chain,
                "token_type": token_type,
                "amount": amount,
                "status": "initiated"
            }),
        };

        // Submit to consensus for network-wide recording
        self.submit_world_change(bridge_change).await?;

        info!("✅ Cross-chain bridge initiated: {}", bridge_id);
        Ok(bridge_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainSaveData {
    pub save_id: Uuid,
    pub save_name: String,
    pub timestamp: SystemTime,
    pub world_state: WorldState,
    pub blockchain_metadata: BlockchainMetadata,
    pub integrity_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainMetadata {
    pub total_blocks: usize,
    pub last_finalized_epoch: u64,
    pub genesis_hash: Option<BlockHash>,
    pub validator_count: usize,
    pub network_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainSaveSnapshot {
    pub save_data: BlockchainSaveData,
    pub block_range: (u64, u64), // (start_epoch, end_epoch)
    pub compressed_blocks: Vec<u8>,
    pub world_state_merkle_proof: MerkleProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub root_hash: String,
    pub proof_elements: Vec<String>,
    pub leaf_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainSaveInfo {
    pub save_name: String,
    pub save_id: Uuid,
    pub timestamp: SystemTime,
    pub epoch: u64,
    pub player_count: usize,
    pub area_count: usize,
    pub npc_count: usize,
    pub integrity_hash: String,
    pub block_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortableWorldExport {
    pub version: String,
    pub export_timestamp: SystemTime,
    pub original_save: BlockchainSaveSnapshot,
    pub network_compatibility: NetworkCompatibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCompatibility {
    pub network_id: String,
    pub consensus_version: String,
    pub required_validators: usize,
    pub min_stake: u64,
}

#[derive(Debug, Clone)]
pub struct PrioritizedProposal {
    pub proposal: WorldStateProposal,
    pub priority: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub node_id: Uuid,
    pub stake_amount: u64,
    pub voting_power: f64,
    pub is_active: bool,
    pub last_activity: SystemTime,
    pub reputation_score: f64,
    pub blocks_produced: u64,
    pub slashing_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionReport {
    pub resolution_id: Uuid,
    pub timestamp: SystemTime,
    pub original_proposal_count: usize,
    pub total_original_changes: usize,
    pub resolved_changes: usize,
    pub conflicts_resolved: usize,
    pub resolution_method: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasternodeEligibility {
    pub node_id: Uuid,
    pub is_eligible: bool,
    pub current_stake: u64,
    pub required_stake: u64,
    pub reputation_score: f64,
    pub uptime_percentage: f64,
    pub block_production_history: Vec<BlockProductionRecord>,
    pub validation_errors: Vec<String>,
    pub estimated_rewards: MasternodeRewards,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProductionRecord {
    pub block_hash: BlockHash,
    pub epoch: u64,
    pub timestamp: SystemTime,
    pub world_changes_count: usize,
    pub validation_time_ms: u64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MasternodeRewards {
    pub block_reward_per_block: u64,
    pub validation_reward_per_block: u64,
    pub consensus_reward_per_block: u64,
    pub estimated_daily_rewards: u64,
    pub estimated_monthly_rewards: u64,
    pub estimated_annual_rewards: u64,
    pub stake_multiplier: f64,
    pub reputation_multiplier: f64,
    pub uptime_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub epoch: u64,
    pub total_rewards_distributed: u64,
    pub masternode_rewards: Vec<MasternodeReward>,
    pub validator_rewards: Vec<MasternodeReward>,
    pub treasury_allocation: u64,
    pub burn_amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasternodeReward {
    pub node_id: Uuid,
    pub reward_type: RewardType,
    pub amount: u64,
    pub epoch: u64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardType {
    BlockProduction,
    Validation,
    Consensus,
    Treasury,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasternodePerformanceStats {
    pub node_id: Uuid,
    pub total_blocks_produced: u64,
    pub average_block_changes: f64,
    pub average_validation_time_ms: f64,
    pub uptime_percentage: f64,
    pub reputation_score: f64,
    pub total_rewards_earned: u64,
    pub last_block_produced: Option<SystemTime>,
    pub performance_rating: PerformanceRating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceRating {
    Excellent,
    Good,
    Average,
    Poor,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub last_finalized_epoch: u64,
    pub pending_transactions: usize,
    pub total_players: usize,
    pub total_areas: usize,
    pub total_npcs: usize,
    pub total_events: usize,
    pub world_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainEconomyStats {
    pub blockchain_stats: BlockchainStats,
    pub total_nfts: usize,
    pub total_collections: usize,
    pub nft_volume_24h: u64,
    pub active_nft_traders: usize,
    pub total_token_supply: u64,
    pub circulating_supply: u64,
    pub total_staked: u64,
    pub defi_tvl: u64,
    pub governance_proposals_active: usize,
    pub cross_chain_volume_24h: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeFiOperation {
    AddLiquidity {
        pool_id: Uuid,
        token_a_type: String,
        token_b_type: String,
        token_a_amount: u64,
        token_b_amount: u64,
    },
    RemoveLiquidity {
        pool_id: Uuid,
        liquidity_amount: u64,
    },
    Stake {
        token_type: String,
        amount: u64,
        duration_days: u32,
    },
    YieldFarm {
        farm_id: Uuid,
        amount: u64,
    },
    Lend {
        token_type: String,
        amount: u64,
        interest_rate: f64,
    },
    Borrow {
        token_type: String,
        amount: u64,
        collateral_type: String,
        collateral_amount: u64,
    },
}
