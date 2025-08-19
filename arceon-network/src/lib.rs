use anyhow::Result;
use arceon_core::config::NetworkConfig;
use libp2p::{
    swarm::SwarmEvent, 
    PeerId, Multiaddr, SwarmBuilder,
    gossipsub::{Behaviour as Gossipsub, Event as GossipsubEvent, MessageAuthenticity, ValidationMode, ConfigBuilder as GossipsubConfigBuilder},
    mdns::{tokio::Behaviour as Mdns, Event as MdnsEvent},
    tcp, yamux, noise,
    identify::{Behaviour as Identify, Event as IdentifyEvent},
    Swarm,
    futures::StreamExt,
};
use serde::{Serialize, Deserialize};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};
use tokio::{select, sync::{RwLock, mpsc}};
use tracing::{info, warn, debug};
use std::sync::Arc;

// Import blockchain types for integration
use arceon_blockchain::{
    ConsensusMessage, FinalizedBlock, WorldState, 
    BlockchainStats
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    // Player lifecycle
    PlayerJoin { player_id: String, player_name: String, race: String, area_id: String },
    PlayerLeave { player_id: String },
    PlayerMove { player_id: String, from_area: String, to_area: String },
    
    // Communication
    ChatMessage { player_id: String, area_id: String, message: String },
    Say { player_id: String, area_id: String, message: String },
    Emote { player_id: String, area_id: String, action: String },
    
    // Game state synchronization
    GameStateSync { world_time: u64, area_updates: Vec<AreaUpdate> },
    PlayerUpdate { player_id: String, being_data: Vec<u8> },
    SkillGain { player_id: String, skill_name: String, new_level: f64, experience: f64 },
    
    // World events
    NPCSpawn { npc_id: String, area_id: String, npc_data: Vec<u8> },
    NPCAction { npc_id: String, area_id: String, action: String },
    AreaEvent { area_id: String, event_type: String, description: String },
    
    // Skill evolution system
    SkillEvolution { skill_name: String, vote: String, voter_id: String },
    SkillDiscovery { discoverer_id: String, skill_name: String, method: String },
    
    // Cross-node synchronization messages
    ConsensusMessage(ConsensusMessage),
    WorldStateSync { requester: String, epoch_range: (u64, u64) },
    WorldStateResponse { blocks: Vec<FinalizedBlock>, current_state: WorldState },
    NodeHeartbeat { node_id: String, timestamp: SystemTime, blockchain_stats: BlockchainStats },
    PeerDiscoveryRequest { requesting_peer: String, known_peers: Vec<String> },
    PeerDiscoveryResponse { responding_peer: String, peer_list: Vec<(String, String)> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaUpdate {
    pub area_id: String,
    pub player_count: usize,
    pub recent_events: Vec<String>,
}

/// Enhanced network manager for P2P connectivity with cross-node synchronization
pub struct NetworkManager {
    config: NetworkConfig,
    is_masternode: bool,
    swarm: Option<Swarm<ArceonBehaviour>>,
    local_peer_id: Option<PeerId>,
    blockchain_enabled: bool,
    
    // Cross-node synchronization state
    known_peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,
    sync_state: Arc<RwLock<SyncState>>,
    _message_sender: Option<mpsc::UnboundedSender<NetworkMessage>>,
    message_receiver: Option<mpsc::UnboundedReceiver<NetworkMessage>>,
    
    // Heartbeat and discovery
    last_heartbeat: Arc<RwLock<SystemTime>>,
    discovery_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub multiaddr: Multiaddr,
    pub last_seen: SystemTime,
    pub blockchain_stats: Option<BlockchainStats>,
    pub is_validator: bool,
    pub stake_amount: u64,
    pub connection_quality: f64,
}

#[derive(Debug, Clone)]
pub struct SyncState {
    pub is_syncing: bool,
    pub sync_target_peer: Option<PeerId>,
    pub sync_start_epoch: u64,
    pub sync_current_epoch: u64,
    pub sync_target_epoch: u64,
    pub last_sync_request: SystemTime,
    pub pending_blocks: HashMap<u64, FinalizedBlock>,
}

#[derive(libp2p::swarm::NetworkBehaviour)]
#[behaviour(to_swarm = "ArceonEvent")]
struct ArceonBehaviour {
    gossipsub: Gossipsub,
    mdns: Mdns,
    identify: Identify,
}

#[derive(Debug)]
enum ArceonEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
    Identify(IdentifyEvent),
}

impl From<GossipsubEvent> for ArceonEvent {
    fn from(event: GossipsubEvent) -> Self {
        ArceonEvent::Gossipsub(event)
    }
}

impl From<MdnsEvent> for ArceonEvent {
    fn from(event: MdnsEvent) -> Self {
        ArceonEvent::Mdns(event)
    }
}

impl From<IdentifyEvent> for ArceonEvent {
    fn from(event: IdentifyEvent) -> Self {
        ArceonEvent::Identify(event)
    }
}


impl NetworkManager {
    pub async fn new(config: &NetworkConfig, is_masternode: bool) -> Result<Self> {
        let sync_state = SyncState {
            is_syncing: false,
            sync_target_peer: None,
            sync_start_epoch: 0,
            sync_current_epoch: 0,
            sync_target_epoch: 0,
            last_sync_request: SystemTime::now(),
            pending_blocks: HashMap::new(),
        };

        let (sender, receiver) = mpsc::unbounded_channel();

        Ok(Self {
            config: config.clone(),
            is_masternode,
            swarm: None,
            local_peer_id: None,
            blockchain_enabled: false,
            known_peers: Arc::new(RwLock::new(HashMap::new())),
            sync_state: Arc::new(RwLock::new(sync_state)),
            _message_sender: Some(sender),
            message_receiver: Some(receiver),
            last_heartbeat: Arc::new(RwLock::new(SystemTime::now())),
            discovery_interval: Duration::from_secs(30),
        })
    }
    
    /// Enable blockchain integration
    pub fn enable_blockchain(&mut self) {
        self.blockchain_enabled = true;
    }

    pub async fn start(&mut self) -> Result<()> {
        // Create a random PeerId
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        self.local_peer_id = Some(local_peer_id);
        
        info!("🌐 Local peer id: {local_peer_id}");
        
        // Set up gossipsub
        let message_id_fn = |message: &libp2p::gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            libp2p::gossipsub::MessageId::from(s.finish().to_string())
        };
        
        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .expect("Valid config");
            
        let mut gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        ).expect("Correct configuration");
        
        // Subscribe to enhanced topics for cross-node sync
        let game_topic = libp2p::gossipsub::IdentTopic::new("arceon-game");
        let chat_topic = libp2p::gossipsub::IdentTopic::new("arceon-chat");
        let world_topic = libp2p::gossipsub::IdentTopic::new("arceon-world");
        let consensus_topic = libp2p::gossipsub::IdentTopic::new("arceon-consensus");
        let sync_topic = libp2p::gossipsub::IdentTopic::new("arceon-sync");
        
        gossipsub.subscribe(&game_topic)?;
        gossipsub.subscribe(&chat_topic)?;
        gossipsub.subscribe(&world_topic)?;
        gossipsub.subscribe(&consensus_topic)?;
        gossipsub.subscribe(&sync_topic)?;
        
        // Create mDNS behavior for local discovery
        let mdns = Mdns::new(Default::default(), local_peer_id)?;
        
        // Create identify behavior
        let identify = Identify::new(libp2p::identify::Config::new(
            "/arceon/1.0.0".to_string(),
            local_key.public(),
        ).with_push_listen_addr_updates(true));

        // Create the enhanced network behavior
        let behaviour = ArceonBehaviour {
            gossipsub,
            mdns,
            identify,
        };
        
        // Create the swarm
        let mut swarm = SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| behaviour)?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();
        
        // Listen on the configured port
        let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", self.config.listen_port)
            .parse()?;
        swarm.listen_on(listen_addr.clone())?;
        
        info!("🎮 Enhanced Arceon P2P network starting on {}", listen_addr);
        
        // Connect to bootstrap nodes if configured
        for addr in &self.config.bootstrap_nodes {
            if let Ok(multiaddr) = addr.parse::<Multiaddr>() {
                if let Err(e) = swarm.dial(multiaddr.clone()) {
                    warn!("❌ Failed to dial bootstrap node {}: {}", multiaddr, e);
                } else {
                    info!("🔗 Connecting to bootstrap node: {}", multiaddr);
                }
            }
        }
        
        self.swarm = Some(swarm);

        // Start heartbeat and discovery loops
        self.start_heartbeat_loop().await?;
        self.start_peer_discovery_loop().await?;
        self.start_sync_loop().await?;

        Ok(())
    }
    
    /// Start heartbeat loop for peer health monitoring
    async fn start_heartbeat_loop(&mut self) -> Result<()> {
        let last_heartbeat = self.last_heartbeat.clone();
        let known_peers = self.known_peers.clone();
        let local_peer_id = self.local_peer_id.unwrap();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                
                let _blockchain_stats = BlockchainStats {
                    total_blocks: 0,
                    last_finalized_epoch: 0,
                    pending_transactions: 0,
                    total_players: 0,
                    total_areas: 0,
                    total_npcs: 0,
                    total_events: 0,
                    world_time: 0,
                };
                
                // Update last heartbeat time
                *last_heartbeat.write().await = SystemTime::now();
                
                // Cleanup stale peers
                Self::cleanup_stale_peers(known_peers.clone()).await;
                
                debug!("💓 Heartbeat sent from peer: {}", local_peer_id);
            }
        });
        
        Ok(())
    }
    
    /// Start peer discovery loop
    async fn start_peer_discovery_loop(&mut self) -> Result<()> {
        let known_peers = self.known_peers.clone();
        let _local_peer_id = self.local_peer_id.unwrap();
        let discovery_interval = self.discovery_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(discovery_interval);
            loop {
                interval.tick().await;
                
                // Initiate peer discovery
                let peer_count = known_peers.read().await.len();
                if peer_count < 10 { // Maintain at least 10 peers
                    debug!("🔍 Initiating peer discovery, current peers: {}", peer_count);
                    // Discovery would be triggered through the swarm
                }
            }
        });
        
        Ok(())
    }
    
    /// Start synchronization loop
    async fn start_sync_loop(&mut self) -> Result<()> {
        let sync_state = self.sync_state.clone();
        let known_peers = self.known_peers.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                
                let mut sync_state_guard = sync_state.write().await;
                if !sync_state_guard.is_syncing {
                    // Check if blockchain sync is needed
                    let peers = known_peers.read().await;
                    if let Some(target_peer) = peers.values().find(|peer| {
                        peer.blockchain_stats.as_ref()
                            .map_or(false, |s| s.last_finalized_epoch > 0)
                    }) {
                        sync_state_guard.is_syncing = true;
                        sync_state_guard.sync_target_peer = Some(target_peer.peer_id);
                        sync_state_guard.sync_start_epoch = 0;
                        sync_state_guard.sync_target_epoch = target_peer.blockchain_stats
                            .as_ref().unwrap().last_finalized_epoch;
                        sync_state_guard.last_sync_request = SystemTime::now();
                        
                        info!("🔄 Starting sync from epoch {} to {} with peer {}", 
                            sync_state_guard.sync_start_epoch,
                            sync_state_guard.sync_target_epoch,
                            target_peer.peer_id
                        );
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Cleanup stale peers
    async fn cleanup_stale_peers(known_peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>) {
        let mut peers = known_peers.write().await;
        let stale_threshold = SystemTime::now() - Duration::from_secs(300); // 5 minutes
        
        peers.retain(|peer_id, peer_info| {
            if peer_info.last_seen < stale_threshold {
                debug!("🗑️ Removing stale peer: {}", peer_id);
                false
            } else {
                true
            }
        });
    }

    pub async fn run_event_loop(&mut self) -> Result<()> {
        loop {
            if let Some(swarm) = &mut self.swarm {
                select! {
                    event = swarm.next() => {
                        if let Some(event) = event {
                            self.handle_swarm_event(event).await?;
                        }
                    }
                    message = self.message_receiver.as_mut().unwrap().recv() => {
                        if let Some(msg) = message {
                            self.handle_internal_message(msg).await?;
                        }
                    }
                }
            }
        }
    }
    
    async fn handle_swarm_event(&mut self, event: SwarmEvent<ArceonEvent>) -> Result<()> {
        match event {
            SwarmEvent::Behaviour(ArceonEvent::Mdns(MdnsEvent::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    info!("🔍 mDNS discovered peer: {} at {}", peer_id, multiaddr);
                    
                    // Add to known peers
                    let peer_info = PeerInfo {
                        peer_id,
                        multiaddr: multiaddr.clone(),
                        last_seen: SystemTime::now(),
                        blockchain_stats: None,
                        is_validator: false,
                        stake_amount: 0,
                        connection_quality: 0.5,
                    };
                    
                    self.known_peers.write().await.insert(peer_id, peer_info);
                    
                    // Attempt to connect
                    if let Some(swarm) = &mut self.swarm {
                        if let Err(e) = swarm.dial(multiaddr.clone()) {
                            warn!("❌ Failed to dial discovered peer {}: {}", peer_id, e);
                        } else {
                            info!("🔗 Connecting to discovered peer: {}", peer_id);
                        }
                    }
                }
            }
            SwarmEvent::Behaviour(ArceonEvent::Mdns(MdnsEvent::Expired(list))) => {
                for (peer_id, _multiaddr) in list {
                    info!("🕰️ mDNS peer expired: {}", peer_id);
                    // Mark peer as potentially stale
                    if let Some(peer_info) = self.known_peers.write().await.get_mut(&peer_id) {
                        peer_info.connection_quality *= 0.8;
                    }
                }
            }
            SwarmEvent::Behaviour(ArceonEvent::Gossipsub(GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: _,
                message,
            })) => {
                if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&message.data) {
                    self.handle_network_message(msg, Some(peer_id)).await?;
                }
            }
            SwarmEvent::Behaviour(ArceonEvent::Identify(IdentifyEvent::Received { peer_id, info })) => {
                info!("🆔 Identified peer: {} - Agent: {}", peer_id, info.agent_version);
                
                // Update peer info with identification data
                if let Some(peer_info) = self.known_peers.write().await.get_mut(&peer_id) {
                    peer_info.last_seen = SystemTime::now();
                    peer_info.connection_quality = 1.0;
                    
                    // Check if this is an Arceon node
                    if info.agent_version.contains("arceon") {
                        peer_info.is_validator = true;
                    }
                }
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("🎧 Listening on: {}", address);
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("🤝 Connected to peer: {}", peer_id);
                
                // Update peer connection quality
                if let Some(peer_info) = self.known_peers.write().await.get_mut(&peer_id) {
                    peer_info.last_seen = SystemTime::now();
                    peer_info.connection_quality = (peer_info.connection_quality + 0.2).min(1.0);
                }
                
                // Request peer discovery information
                self.request_peer_discovery(peer_id).await?;
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                info!("👋 Disconnected from peer: {}", peer_id);
                
                // Reduce connection quality but keep peer info
                if let Some(peer_info) = self.known_peers.write().await.get_mut(&peer_id) {
                    peer_info.connection_quality *= 0.5;
                }
            }
            
            _ => {}
        }
        Ok(())
    }
    
    /// Handle internal message processing
    async fn handle_internal_message(&mut self, message: NetworkMessage) -> Result<()> {
        // Forward internal messages to the network
        self.broadcast_message(message).await
    }
    
    /// Request peer discovery from a connected peer
    async fn request_peer_discovery(&mut self, _peer_id: PeerId) -> Result<()> {
        let known_peer_ids: Vec<_> = self.known_peers.read().await.keys().cloned().collect();
        
        let discovery_request = NetworkMessage::PeerDiscoveryRequest {
            requesting_peer: self.local_peer_id.unwrap().to_string(),
            known_peers: known_peer_ids.iter().map(|p| p.to_string()).collect(),
        };
        
        // Send discovery request directly to the peer
        if let Some(swarm) = &mut self.swarm {
            let data = serde_json::to_vec(&discovery_request)?;
            let topic = libp2p::gossipsub::IdentTopic::new("arceon-sync");
            swarm.behaviour_mut().gossipsub.publish(topic, data)?;
        }
        
        Ok(())
    }
    
    async fn handle_network_message(&mut self, message: NetworkMessage, _sender: Option<PeerId>) -> Result<()> {
        match message {
            // Existing game messages
            NetworkMessage::PlayerJoin { player_id, player_name, race, area_id } => {
                info!("👤 Player {} ({}, {}) joined area {}", player_name, race, player_id, area_id);
            }
            NetworkMessage::PlayerLeave { player_id } => {
                info!("👋 Player {} left the game", player_id);
            }
            NetworkMessage::PlayerMove { player_id, from_area, to_area } => {
                info!("🚶 Player {} moved from {} to {}", player_id, from_area, to_area);
            }
            NetworkMessage::ChatMessage { player_id, area_id, message } => {
                info!("💬 [{}] {}: {}", area_id, player_id, message);
            }
            NetworkMessage::Say { player_id, area_id, message } => {
                info!("🗣️ [{}] {} says: {}", area_id, player_id, message);
            }
            NetworkMessage::Emote { player_id, area_id, action } => {
                info!("🎭 [{}] {} {}", area_id, player_id, action);
            }
            NetworkMessage::GameStateSync { world_time, area_updates } => {
                info!("🔄 Game state sync at time {} with {} area updates", world_time, area_updates.len());
            }
            NetworkMessage::PlayerUpdate { player_id, being_data: _ } => {
                info!("📊 Player update for {}", player_id);
            }
            NetworkMessage::SkillGain { player_id, skill_name, new_level, experience } => {
                info!("⬆️ Player {} gained skill {} level {:.1} (XP: {:.1})", player_id, skill_name, new_level, experience);
            }
            NetworkMessage::NPCSpawn { npc_id, area_id, npc_data: _ } => {
                info!("🤖 NPC {} spawned in area {}", npc_id, area_id);
            }
            NetworkMessage::NPCAction { npc_id, area_id, action } => {
                info!("🎬 [{}] NPC {} performs: {}", area_id, npc_id, action);
            }
            NetworkMessage::AreaEvent { area_id, event_type, description } => {
                info!("🌟 [{}] {}: {}", area_id, event_type, description);
            }
            NetworkMessage::SkillEvolution { skill_name, vote, voter_id } => {
                info!("⚡ Player {} votes '{}' for skill evolution: {}", voter_id, vote, skill_name);
            }
            NetworkMessage::SkillDiscovery { discoverer_id, skill_name, method } => {
                info!("🔍 Player {} discovered skill '{}' via {}", discoverer_id, skill_name, method);
            }
            
            // Cross-node synchronization messages
            NetworkMessage::ConsensusMessage(_consensus_msg) => {
                if self.blockchain_enabled {
                    debug!("Processing consensus message");
                    // TODO: Forward to blockchain manager
                } else {
                    debug!("Received consensus message but blockchain not enabled");
                }
            }
            
            NetworkMessage::WorldStateSync { requester, epoch_range } => {
                info!("📥 World state sync requested by {} for epochs {}-{}", 
                    requester, epoch_range.0, epoch_range.1);
                if let Ok(peer_id) = requester.parse() {
                    self.handle_world_state_sync_request(peer_id, epoch_range).await?;
                }
            }
            
            NetworkMessage::WorldStateResponse { blocks, current_state } => {
                info!("📤 Received world state response with {} blocks", blocks.len());
                self.handle_world_state_sync_response(blocks, current_state).await?;
            }
            
            NetworkMessage::NodeHeartbeat { node_id, timestamp, blockchain_stats } => {
                debug!("💓 Heartbeat received from {}", node_id);
                if let Ok(peer_id) = node_id.parse() {
                    self.handle_node_heartbeat(peer_id, timestamp, blockchain_stats).await?;
                }
            }
            
            NetworkMessage::PeerDiscoveryRequest { requesting_peer, known_peers } => {
                info!("🔍 Peer discovery request from {} (knows {} peers)", 
                    requesting_peer, known_peers.len());
                if let Ok(peer_id) = requesting_peer.parse() {
                    let peer_ids: Vec<PeerId> = known_peers.iter().filter_map(|s| s.parse().ok()).collect();
                    self.handle_peer_discovery_request(peer_id, peer_ids).await?;
                }
            }
            
            NetworkMessage::PeerDiscoveryResponse { responding_peer, peer_list } => {
                info!("📋 Peer discovery response from {} with {} peers", 
                    responding_peer, peer_list.len());
                if let Ok(peer_id) = responding_peer.parse() {
                    let peers: Vec<(PeerId, Multiaddr)> = peer_list.iter()
                        .filter_map(|(pid, addr)| {
                            let peer_id = pid.parse().ok()?;
                            let multiaddr = addr.parse().ok()?;
                            Some((peer_id, multiaddr))
                        })
                        .collect();
                    self.handle_peer_discovery_response(peer_id, peers).await?;
                }
            }
        }
        Ok(())
    }
    
    /// Handle world state sync request
    async fn handle_world_state_sync_request(&mut self, _requester: PeerId, epoch_range: (u64, u64)) -> Result<()> {
        if self.blockchain_enabled {
            // This would typically fetch blocks from the blockchain manager
            // For now, we'll create a placeholder response
            let response = NetworkMessage::WorldStateResponse {
                blocks: Vec::new(), // Would fetch actual blocks here
                current_state: WorldState {
                    current_epoch: epoch_range.1,
                    players: HashMap::new(),
                    areas: HashMap::new(),
                    npcs: HashMap::new(),
                    global_events: Vec::new(),
                    skill_discoveries: HashMap::new(),
                    world_time: 0,
                    last_update: SystemTime::now(),
                },
            };
            
            // Send response
            if let Some(swarm) = &mut self.swarm {
                let data = serde_json::to_vec(&response)?;
                let topic = libp2p::gossipsub::IdentTopic::new("arceon-sync");
                swarm.behaviour_mut().gossipsub.publish(topic, data)?;
            }
        }
        Ok(())
    }
    
    /// Handle world state sync response
    async fn handle_world_state_sync_response(&mut self, blocks: Vec<FinalizedBlock>, _current_state: WorldState) -> Result<()> {
        let mut sync_state = self.sync_state.write().await;
        
        // Process received blocks
        for block in blocks {
            sync_state.pending_blocks.insert(block.epoch, block);
        }
        
        // Check if sync is complete
        if sync_state.pending_blocks.len() >= (sync_state.sync_target_epoch - sync_state.sync_start_epoch + 1) as usize {
            info!("✅ Blockchain sync completed! Processed {} blocks", sync_state.pending_blocks.len());
            sync_state.is_syncing = false;
            sync_state.pending_blocks.clear();
        }
        
        Ok(())
    }
    
    /// Handle node heartbeat
    async fn handle_node_heartbeat(&mut self, node_id: PeerId, timestamp: SystemTime, blockchain_stats: BlockchainStats) -> Result<()> {
        let mut peers = self.known_peers.write().await;
        
        if let Some(peer_info) = peers.get_mut(&node_id) {
            peer_info.last_seen = timestamp;
            peer_info.blockchain_stats = Some(blockchain_stats.clone());
            peer_info.connection_quality = (peer_info.connection_quality + 0.1).min(1.0);
            
            // Update validator status based on blockchain stats
            if blockchain_stats.total_blocks > 0 {
                peer_info.is_validator = true;
                peer_info.stake_amount = 1000; // Would get actual stake amount
            }
        } else {
            // Create new peer info from heartbeat
            let peer_info = PeerInfo {
                peer_id: node_id,
                multiaddr: "/ip4/0.0.0.0/tcp/0".parse().unwrap(), // Placeholder
                last_seen: timestamp,
                blockchain_stats: Some(blockchain_stats),
                is_validator: true,
                stake_amount: 1000,
                connection_quality: 0.8,
            };
            peers.insert(node_id, peer_info);
        }
        
        Ok(())
    }
    
    /// Handle peer discovery request
    async fn handle_peer_discovery_request(&mut self, requesting_peer: PeerId, known_peers: Vec<PeerId>) -> Result<()> {
        let peers = self.known_peers.read().await;
        
        // Build list of peers to share (excluding the requesting peer and ones they already know)
        let peer_list: Vec<(PeerId, Multiaddr)> = peers.iter()
            .filter_map(|(peer_id, peer_info)| {
                if *peer_id != requesting_peer && !known_peers.contains(peer_id) {
                    Some((*peer_id, peer_info.multiaddr.clone()))
                } else {
                    None
                }
            })
            .take(10) // Limit to 10 peers per response
            .collect();
        
        let peer_list_strings: Vec<(String, String)> = peer_list.iter()
            .map(|(pid, addr)| (pid.to_string(), addr.to_string()))
            .collect();
            
        let response = NetworkMessage::PeerDiscoveryResponse {
            responding_peer: self.local_peer_id.unwrap().to_string(),
            peer_list: peer_list_strings,
        };
        
        // Send response
        if let Some(swarm) = &mut self.swarm {
            let data = serde_json::to_vec(&response)?;
            let topic = libp2p::gossipsub::IdentTopic::new("arceon-sync");
            swarm.behaviour_mut().gossipsub.publish(topic, data)?;
        }
        
        Ok(())
    }
    
    /// Handle peer discovery response
    async fn handle_peer_discovery_response(&mut self, _responding_peer: PeerId, peer_list: Vec<(PeerId, Multiaddr)>) -> Result<()> {
        let mut peers = self.known_peers.write().await;
        
        let peer_count = peer_list.len();
        for (peer_id, multiaddr) in &peer_list {
            if !peers.contains_key(&peer_id) {
                let peer_info = PeerInfo {
                    peer_id: *peer_id,
                    multiaddr: multiaddr.clone(),
                    last_seen: SystemTime::now(),
                    blockchain_stats: None,
                    is_validator: false,
                    stake_amount: 0,
                    connection_quality: 0.5,
                };
                
                peers.insert(*peer_id, peer_info);
                
                // Attempt to connect to new peer
                if let Some(swarm) = &mut self.swarm {
                    if let Err(e) = swarm.dial(multiaddr.clone()) {
                        warn!("❌ Failed to dial discovered peer {}: {}", *peer_id, e);
                    } else {
                        info!("🔗 Connecting to discovered peer: {}", *peer_id);
                    }
                }
            }
        }
        
        info!("✅ Added {} new peers from discovery response", peer_count);
        Ok(())
    }
    
    pub async fn broadcast_message(&mut self, message: NetworkMessage) -> Result<()> {
        if let Some(swarm) = &mut self.swarm {
            let topic = match &message {
                // Game messages
                NetworkMessage::PlayerJoin { .. } => "arceon-players",
                NetworkMessage::PlayerLeave { .. } => "arceon-players",
                NetworkMessage::PlayerMove { .. } => "arceon-players", 
                NetworkMessage::ChatMessage { .. } => "arceon-chat",
                NetworkMessage::Say { .. } => "arceon-chat",
                NetworkMessage::Emote { .. } => "arceon-chat",
                NetworkMessage::GameStateSync { .. } => "arceon-world",
                NetworkMessage::PlayerUpdate { .. } => "arceon-players",
                NetworkMessage::SkillGain { .. } => "arceon-skills",
                NetworkMessage::NPCSpawn { .. } => "arceon-world",
                NetworkMessage::NPCAction { .. } => "arceon-world",
                NetworkMessage::AreaEvent { .. } => "arceon-world",
                NetworkMessage::SkillEvolution { .. } => "arceon-skills",
                NetworkMessage::SkillDiscovery { .. } => "arceon-skills",
                
                // Cross-node sync messages
                NetworkMessage::ConsensusMessage(_) => "arceon-consensus",
                NetworkMessage::WorldStateSync { .. } => "arceon-sync",
                NetworkMessage::WorldStateResponse { .. } => "arceon-sync",
                NetworkMessage::NodeHeartbeat { .. } => "arceon-heartbeat",
                NetworkMessage::PeerDiscoveryRequest { .. } => "arceon-discovery",
                NetworkMessage::PeerDiscoveryResponse { .. } => "arceon-discovery",
            };
            
            let data = serde_json::to_vec(&message)?;
            let topic = libp2p::gossipsub::IdentTopic::new(topic);
            
            swarm.behaviour_mut().gossipsub.publish(topic, data)?;
        }
        Ok(())
    }
    
    /// Request world state sync from the network
    pub async fn request_world_state_sync(&mut self, from_epoch: u64, to_epoch: u64) -> Result<()> {
        let sync_request = NetworkMessage::WorldStateSync {
            requester: self.local_peer_id.unwrap().to_string(),
            epoch_range: (from_epoch, to_epoch),
        };
        
        self.broadcast_message(sync_request).await?;
        
        // Update sync state
        let mut sync_state = self.sync_state.write().await;
        sync_state.is_syncing = true;
        sync_state.sync_start_epoch = from_epoch;
        sync_state.sync_target_epoch = to_epoch;
        sync_state.last_sync_request = SystemTime::now();
        
        info!("🔄 Requested world state sync for epochs {} to {}", from_epoch, to_epoch);
        Ok(())
    }
    
    /// Broadcast node heartbeat
    pub async fn broadcast_heartbeat(&mut self, blockchain_stats: BlockchainStats) -> Result<()> {
        let heartbeat = NetworkMessage::NodeHeartbeat {
            node_id: self.local_peer_id.unwrap().to_string(),
            timestamp: SystemTime::now(),
            blockchain_stats,
        };
        
        self.broadcast_message(heartbeat).await
    }
    
    /// Get current synchronization state
    pub async fn get_sync_state(&self) -> SyncState {
        self.sync_state.read().await.clone()
    }
    
    /// Get list of known peers
    pub async fn get_known_peers(&self) -> Vec<PeerInfo> {
        self.known_peers.read().await.values().cloned().collect()
    }
    
    /// Get peer count
    pub async fn get_peer_count(&self) -> usize {
        self.known_peers.read().await.len()
    }
    
    /// Check if currently syncing
    pub async fn is_syncing(&self) -> bool {
        self.sync_state.read().await.is_syncing
    }
    
    pub fn is_masternode(&self) -> bool {
        self.is_masternode
    }
    
    pub fn get_peer_id(&self) -> Option<PeerId> {
        self.local_peer_id
    }
    
    pub fn get_connected_peers(&self) -> Vec<PeerId> {
        if let Some(swarm) = &self.swarm {
            swarm.connected_peers().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get network statistics
    pub async fn get_network_stats(&self) -> NetworkStats {
        let peers = self.known_peers.read().await;
        let sync_state = self.sync_state.read().await;
        let connected_peers = self.get_connected_peers();
        
        NetworkStats {
            total_known_peers: peers.len(),
            connected_peers: connected_peers.len(),
            validator_peers: peers.values().filter(|p| p.is_validator).count(),
            is_syncing: sync_state.is_syncing,
            sync_progress: if sync_state.is_syncing {
                let total = sync_state.sync_target_epoch - sync_state.sync_start_epoch + 1;
                let current = sync_state.sync_current_epoch - sync_state.sync_start_epoch;
                (current as f64 / total as f64 * 100.0).min(100.0)
            } else {
                100.0
            },
            last_heartbeat: *self.last_heartbeat.read().await,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_known_peers: usize,
    pub connected_peers: usize,
    pub validator_peers: usize,
    pub is_syncing: bool,
    pub sync_progress: f64,
    pub last_heartbeat: SystemTime,
}
