use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use std::sync::Arc;
use sha2::{Sha256, Digest};

/// Robust P2P consensus mechanism for Arceon's decentralized network
/// Implements a hybrid consensus combining Proof of Stake with Byzantine Fault Tolerance
pub struct ConsensusManager {
    pub node_id: Uuid,
    pub is_masternode: bool,
    pub stake_amount: u64,
    pub consensus_state: Arc<RwLock<ConsensusState>>,
    pub pending_proposals: Arc<RwLock<HashMap<Uuid, WorldStateProposal>>>,
    pub validators: Arc<RwLock<HashMap<Uuid, ValidatorInfo>>>,
    pub consensus_config: ConsensusConfig,
    pub message_sender: Option<mpsc::UnboundedSender<ConsensusMessage>>,
    pub block_producer: Arc<RwLock<BlockProducer>>,
    pub finality_tracker: Arc<RwLock<FinalityTracker>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub consensus_threshold: f64, // Percentage of stake needed for consensus (e.g., 0.67 for 67%)
    pub block_time: Duration,     // Target time between blocks
    pub max_validators: usize,    // Maximum number of validators
    pub min_stake: u64,          // Minimum stake to become validator
    pub validator_rotation_blocks: u64, // How often to rotate validator set
    pub finality_depth: u32,     // Number of blocks for finality
    pub timeout_propose: Duration,
    pub timeout_prevote: Duration,
    pub timeout_precommit: Duration,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum ConsensusStep {
    Propose,
    Prevote,
    Precommit,
    Commit,
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
pub enum SlashingType {
    DoubleVoting,
    InvalidProposal,
    Equivocation,
    Inactivity,
}

pub type BlockHash = [u8; 32];

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

#[derive(Debug, Clone)]
pub struct BlockProducer {
    pub current_proposal: Option<WorldStateProposal>,
    pub pending_changes: VecDeque<WorldChange>,
    pub last_block_time: SystemTime,
    pub is_proposer: bool,
}

#[derive(Debug, Clone)]
pub struct FinalityTracker {
    pub finalized_blocks: BTreeMap<u64, FinalizedBlock>,
    pub pending_blocks: HashMap<BlockHash, FinalizedBlock>,
    pub finality_votes: HashMap<BlockHash, HashMap<Uuid, bool>>,
    pub last_finalized_epoch: u64,
}

impl ConsensusManager {
    pub fn new(node_id: Uuid, is_masternode: bool, stake_amount: u64, config: ConsensusConfig) -> Self {
        let consensus_state = ConsensusState {
            current_epoch: 0,
            current_round: 0,
            current_step: ConsensusStep::Propose,
            last_finalized_block: None,
            active_proposal: None,
            votes: HashMap::new(),
            view_change_votes: HashMap::new(),
            total_stake: 0,
        };

        let block_producer = BlockProducer {
            current_proposal: None,
            pending_changes: VecDeque::new(),
            last_block_time: SystemTime::now(),
            is_proposer: false,
        };

        let finality_tracker = FinalityTracker {
            finalized_blocks: BTreeMap::new(),
            pending_blocks: HashMap::new(),
            finality_votes: HashMap::new(),
            last_finalized_epoch: 0,
        };

        Self {
            node_id,
            is_masternode,
            stake_amount,
            consensus_state: Arc::new(RwLock::new(consensus_state)),
            pending_proposals: Arc::new(RwLock::new(HashMap::new())),
            validators: Arc::new(RwLock::new(HashMap::new())),
            consensus_config: config,
            message_sender: None,
            block_producer: Arc::new(RwLock::new(block_producer)),
            finality_tracker: Arc::new(RwLock::new(finality_tracker)),
        }
    }

    /// Initialize the consensus system
    pub async fn initialize(&mut self, message_sender: mpsc::UnboundedSender<ConsensusMessage>) -> Result<()> {
        info!("🔧 Initializing consensus system for node: {}", self.node_id);
        
        self.message_sender = Some(message_sender);
        
        // Register as validator if we have sufficient stake
        if self.stake_amount >= self.consensus_config.min_stake {
            self.join_validator_set().await?;
        }
        
        info!("✅ Consensus system initialized successfully");
        Ok(())
    }

    /// Join the validator set
    pub async fn join_validator_set(&mut self) -> Result<()> {
        info!("📝 Joining validator set with stake: {}", self.stake_amount);
        
        let validator_info = ValidatorInfo {
            node_id: self.node_id,
            stake_amount: self.stake_amount,
            voting_power: self.calculate_voting_power().await,
            is_active: true,
            last_activity: SystemTime::now(),
            reputation_score: 100.0, // Start with perfect reputation
            blocks_produced: 0,
            slashing_count: 0,
        };

        let mut validators = self.validators.write().await;
        validators.insert(self.node_id, validator_info.clone());
        
        // Update total stake
        let mut state = self.consensus_state.write().await;
        state.total_stake += self.stake_amount;
        drop(state);
        drop(validators);

        // Broadcast join message
        if let Some(sender) = &self.message_sender {
            let _ = sender.send(ConsensusMessage::ValidatorJoin {
                node_id: self.node_id,
                stake_amount: self.stake_amount,
                timestamp: SystemTime::now(),
            });
        }

        info!("✅ Successfully joined validator set");
        Ok(())
    }

    /// Calculate voting power based on stake and reputation
    async fn calculate_voting_power(&self) -> f64 {
        let validators = self.validators.read().await;
        let total_stake: u64 = validators.values().map(|v| v.stake_amount).sum();
        
        if total_stake == 0 {
            return 0.0;
        }

        let base_power = (self.stake_amount as f64) / (total_stake as f64);
        
        // Apply reputation multiplier if we're already a validator
        if let Some(validator) = validators.get(&self.node_id) {
            let reputation_multiplier = (validator.reputation_score / 100.0).clamp(0.5, 1.5);
            base_power * reputation_multiplier
        } else {
            base_power
        }
    }

    /// Process incoming consensus messages
    pub async fn handle_consensus_message(&mut self, message: ConsensusMessage) -> Result<()> {
        debug!("📨 Processing consensus message: {:?}", std::mem::discriminant(&message));
        
        match message {
            ConsensusMessage::Proposal(proposal) => {
                self.handle_proposal(proposal).await?;
            }
            ConsensusMessage::Vote(vote) => {
                self.handle_vote(vote).await?;
            }
            ConsensusMessage::ViewChange(view_change) => {
                self.handle_view_change(view_change).await?;
            }
            ConsensusMessage::ValidatorJoin { node_id, stake_amount, timestamp } => {
                self.handle_validator_join(node_id, stake_amount, timestamp).await?;
            }
            ConsensusMessage::ValidatorLeave { node_id, timestamp } => {
                self.handle_validator_leave(node_id, timestamp).await?;
            }
            ConsensusMessage::SlashingEvidence { accused_node, evidence_type, proof, timestamp } => {
                self.handle_slashing_evidence(accused_node, evidence_type, proof, timestamp).await?;
            }
            ConsensusMessage::SyncRequest { requester, from_epoch, to_epoch } => {
                self.handle_sync_request(requester, from_epoch, to_epoch).await?;
            }
            ConsensusMessage::SyncResponse { blocks, current_state } => {
                self.handle_sync_response(blocks, current_state).await?;
            }
        }

        Ok(())
    }

    /// Handle incoming proposal
    async fn handle_proposal(&mut self, proposal: WorldStateProposal) -> Result<()> {
        info!("📋 Received proposal {} from {}", proposal.proposal_id, proposal.proposer);
        
        // Validate proposal
        if !self.validate_proposal(&proposal).await? {
            warn!("❌ Invalid proposal rejected: {}", proposal.proposal_id);
            return Ok(());
        }

        // Store proposal
        let mut proposals = self.pending_proposals.write().await;
        proposals.insert(proposal.proposal_id, proposal.clone());
        drop(proposals);

        // Update consensus state
        let mut state = self.consensus_state.write().await;
        state.active_proposal = Some(proposal.proposal_id);
        state.current_step = ConsensusStep::Prevote;
        state.votes.clear(); // Clear previous votes
        drop(state);

        // Cast prevote
        self.cast_prevote(proposal.proposal_id, true).await?;

        Ok(())
    }

    /// Validate a world state proposal
    async fn validate_proposal(&self, proposal: &WorldStateProposal) -> Result<bool> {
        // Check if proposer is valid validator
        let validators = self.validators.read().await;
        if !validators.contains_key(&proposal.proposer) {
            return Ok(false);
        }

        // Check epoch and round
        let state = self.consensus_state.read().await;
        if proposal.epoch < state.current_epoch {
            return Ok(false);
        }

        // Validate world changes
        for change in &proposal.world_changes {
            if !self.validate_world_change(change).await? {
                return Ok(false);
            }
        }

        // Verify merkle root
        let calculated_root = self.calculate_merkle_root(&proposal.world_changes)?;
        if calculated_root != proposal.merkle_root {
            return Ok(false);
        }

        Ok(true)
    }

    /// Validate individual world change
    async fn validate_world_change(&self, change: &WorldChange) -> Result<bool> {
        match change {
            WorldChange::PlayerAction { player_id, action_type, area_id, timestamp, data: _ } => {
                // Validate player exists and action is valid
                debug!("Validating player action: {} in {} by {}", action_type, area_id, player_id);
                // TODO: Check against game rules
                Ok(true)
            }
            WorldChange::NPCAction { npc_id, action_type, area_id, timestamp, data: _ } => {
                // Validate NPC exists and action is valid
                debug!("Validating NPC action: {} in {} by {}", action_type, area_id, npc_id);
                Ok(true)
            }
            WorldChange::AreaUpdate { area_id, update_type, timestamp, data: _ } => {
                // Validate area update
                debug!("Validating area update: {} in {}", update_type, area_id);
                Ok(true)
            }
            WorldChange::SkillEvolution { skill_name, evolution_type, consensus_votes, .. } => {
                // Validate skill evolution has sufficient consensus
                let required_votes = (self.get_active_validator_count().await as f64 * 0.6) as u32;
                Ok(*consensus_votes >= required_votes)
            }
            WorldChange::WorldEvent { event_id, event_type, affected_areas, .. } => {
                // Validate world event
                debug!("Validating world event: {} affecting {} areas", event_type, affected_areas.len());
                Ok(true)
            }
        }
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

    /// Cast a prevote
    async fn cast_prevote(&mut self, proposal_id: Uuid, support: bool) -> Result<()> {
        let state = self.consensus_state.read().await;
        let vote = Vote {
            voter_id: self.node_id,
            proposal_id,
            vote_type: VoteType::Prevote(support),
            epoch: state.current_epoch,
            round: state.current_round,
            timestamp: SystemTime::now(),
            signature: self.sign_vote(&proposal_id, &VoteType::Prevote(support))?,
        };
        drop(state);

        // Broadcast vote
        if let Some(sender) = &self.message_sender {
            let _ = sender.send(ConsensusMessage::Vote(vote));
        }

        info!("🗳️ Cast prevote for proposal {}: {}", proposal_id, support);
        Ok(())
    }

    /// Cast a precommit vote
    async fn cast_precommit(&mut self, proposal_id: Uuid, commit: bool) -> Result<()> {
        let state = self.consensus_state.read().await;
        let vote = Vote {
            voter_id: self.node_id,
            proposal_id,
            vote_type: VoteType::Precommit(commit),
            epoch: state.current_epoch,
            round: state.current_round,
            timestamp: SystemTime::now(),
            signature: self.sign_vote(&proposal_id, &VoteType::Precommit(commit))?,
        };
        drop(state);

        // Broadcast vote
        if let Some(sender) = &self.message_sender {
            let _ = sender.send(ConsensusMessage::Vote(vote));
        }

        info!("✅ Cast precommit for proposal {}: {}", proposal_id, commit);
        Ok(())
    }

    /// Handle incoming vote
    async fn handle_vote(&mut self, vote: Vote) -> Result<()> {
        debug!("🗳️ Processing vote from {} for proposal {}", vote.voter_id, vote.proposal_id);
        
        // Validate vote
        if !self.validate_vote(&vote).await? {
            warn!("❌ Invalid vote rejected from {}", vote.voter_id);
            return Ok(());
        }

        // Store vote
        let mut state = self.consensus_state.write().await;
        state.votes.insert(vote.voter_id, vote.clone());
        
        // Check if we have enough votes to progress
        match vote.vote_type {
            VoteType::Prevote(_) => {
                if self.check_prevote_threshold(&state).await? {
                    state.current_step = ConsensusStep::Precommit;
                    let proposal_id = vote.proposal_id;
                    drop(state);
                    self.cast_precommit(proposal_id, true).await?;
                }
            }
            VoteType::Precommit(_) => {
                if self.check_precommit_threshold(&state).await? {
                    state.current_step = ConsensusStep::Commit;
                    let proposal_id = vote.proposal_id;
                    drop(state);
                    self.finalize_proposal(proposal_id).await?;
                }
            }
        }

        Ok(())
    }

    /// Validate a vote
    async fn validate_vote(&self, vote: &Vote) -> Result<bool> {
        // Check if voter is valid validator
        let validators = self.validators.read().await;
        if !validators.contains_key(&vote.voter_id) {
            return Ok(false);
        }

        // Verify signature
        if !self.verify_vote_signature(vote)? {
            return Ok(false);
        }

        // Check epoch and round
        let state = self.consensus_state.read().await;
        if vote.epoch != state.current_epoch || vote.round != state.current_round {
            return Ok(false);
        }

        Ok(true)
    }

    /// Check if prevote threshold is reached
    async fn check_prevote_threshold(&self, state: &ConsensusState) -> Result<bool> {
        let prevotes: Vec<_> = state.votes.values()
            .filter(|v| matches!(v.vote_type, VoteType::Prevote(true)))
            .collect();

        let total_voting_power = self.calculate_total_voting_power().await;
        let prevote_power = self.calculate_vote_power(&prevotes).await;

        Ok(prevote_power / total_voting_power >= self.consensus_config.consensus_threshold)
    }

    /// Check if precommit threshold is reached
    async fn check_precommit_threshold(&self, state: &ConsensusState) -> Result<bool> {
        let precommits: Vec<_> = state.votes.values()
            .filter(|v| matches!(v.vote_type, VoteType::Precommit(true)))
            .collect();

        let total_voting_power = self.calculate_total_voting_power().await;
        let precommit_power = self.calculate_vote_power(&precommits).await;

        Ok(precommit_power / total_voting_power >= self.consensus_config.consensus_threshold)
    }

    /// Calculate total voting power of active validators
    async fn calculate_total_voting_power(&self) -> f64 {
        let validators = self.validators.read().await;
        validators.values()
            .filter(|v| v.is_active)
            .map(|v| v.voting_power)
            .sum()
    }

    /// Calculate voting power for a set of votes
    async fn calculate_vote_power(&self, votes: &[&Vote]) -> f64 {
        let validators = self.validators.read().await;
        votes.iter()
            .filter_map(|vote| validators.get(&vote.voter_id))
            .filter(|v| v.is_active)
            .map(|v| v.voting_power)
            .sum()
    }

    /// Finalize a proposal and create a block
    async fn finalize_proposal(&mut self, proposal_id: Uuid) -> Result<()> {
        info!("🎯 Finalizing proposal: {}", proposal_id);
        
        let proposals = self.pending_proposals.read().await;
        let proposal = proposals.get(&proposal_id)
            .ok_or_else(|| anyhow::anyhow!("Proposal not found"))?
            .clone();
        drop(proposals);

        // Create finalized block
        let block_hash = self.calculate_block_hash(&proposal)?;
        let mut state = self.consensus_state.write().await;
        
        // Collect validator signatures
        let validator_signatures: HashMap<Uuid, String> = state.votes.iter()
            .filter(|(_, vote)| matches!(vote.vote_type, VoteType::Precommit(true)))
            .map(|(id, vote)| (*id, vote.signature.clone()))
            .collect();

        let finalized_block = FinalizedBlock {
            block_hash,
            epoch: proposal.epoch,
            round: proposal.round,
            proposer: proposal.proposer,
            timestamp: proposal.timestamp,
            world_changes: proposal.world_changes,
            validator_signatures,
            merkle_root: proposal.merkle_root,
            previous_hash: state.last_finalized_block,
        };

        // Update state
        state.last_finalized_block = Some(block_hash);
        state.current_epoch += 1;
        state.current_round = 0;
        state.current_step = ConsensusStep::Propose;
        state.active_proposal = None;
        state.votes.clear();
        drop(state);

        // Store finalized block
        let mut finality_tracker = self.finality_tracker.write().await;
        finality_tracker.finalized_blocks.insert(proposal.epoch, finalized_block.clone());
        finality_tracker.last_finalized_epoch = proposal.epoch;
        drop(finality_tracker);

        info!("✅ Block finalized for epoch {} with {} changes", 
            proposal.epoch, finalized_block.world_changes.len());

        Ok(())
    }

    /// Calculate block hash
    fn calculate_block_hash(&self, proposal: &WorldStateProposal) -> Result<BlockHash> {
        let mut hasher = Sha256::new();
        hasher.update(proposal.proposal_id.to_string().as_bytes());
        hasher.update(proposal.merkle_root.as_bytes());
        hasher.update(proposal.epoch.to_be_bytes());
        hasher.update(proposal.round.to_be_bytes());
        
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Ok(hash)
    }

    /// Add a world change to pending changes
    pub async fn add_world_change(&mut self, change: WorldChange) -> Result<()> {
        let mut producer = self.block_producer.write().await;
        producer.pending_changes.push_back(change);
        
        // Check if we should propose a new block
        if producer.is_proposer && producer.pending_changes.len() >= 10 {
            drop(producer);
            self.propose_new_block().await?;
        }
        
        Ok(())
    }

    /// Propose a new block
    async fn propose_new_block(&mut self) -> Result<()> {
        info!("📝 Proposing new block");
        
        let mut producer = self.block_producer.write().await;
        let changes: Vec<_> = producer.pending_changes.drain(..).collect();
        drop(producer);

        if changes.is_empty() {
            return Ok(());
        }

        let state = self.consensus_state.read().await;
        let merkle_root = self.calculate_merkle_root(&changes)?;
        
        let proposal = WorldStateProposal {
            proposal_id: Uuid::new_v4(),
            proposer: self.node_id,
            epoch: state.current_epoch,
            round: state.current_round,
            timestamp: SystemTime::now(),
            world_changes: changes,
            previous_block_hash: state.last_finalized_block,
            merkle_root,
            signature: None, // TODO: Add cryptographic signature
        };
        drop(state);

        // Broadcast proposal
        if let Some(sender) = &self.message_sender {
            let _ = sender.send(ConsensusMessage::Proposal(proposal));
        }

        Ok(())
    }

    /// Sign a vote (placeholder implementation)
    fn sign_vote(&self, proposal_id: &Uuid, vote_type: &VoteType) -> Result<String> {
        // TODO: Implement proper cryptographic signature
        let mut hasher = Sha256::new();
        hasher.update(self.node_id.to_string().as_bytes());
        hasher.update(proposal_id.to_string().as_bytes());
        hasher.update(format!("{:?}", vote_type).as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Verify vote signature (placeholder implementation)
    fn verify_vote_signature(&self, vote: &Vote) -> Result<bool> {
        // TODO: Implement proper cryptographic signature verification
        let expected = self.sign_vote(&vote.proposal_id, &vote.vote_type)?;
        Ok(vote.signature == expected)
    }

    /// Handle view change
    async fn handle_view_change(&mut self, view_change: ViewChangeVote) -> Result<()> {
        info!("🔄 Processing view change from {} for round {}", 
            view_change.voter_id, view_change.new_round);
        
        let mut state = self.consensus_state.write().await;
        state.view_change_votes.insert(view_change.voter_id, view_change);
        
        // Check if we have enough view change votes
        let view_change_count = state.view_change_votes.len();
        let validator_count = self.get_active_validator_count().await;
        
        if view_change_count > validator_count * 2 / 3 {
            // Trigger view change
            state.current_round += 1;
            state.current_step = ConsensusStep::Propose;
            state.votes.clear();
            state.view_change_votes.clear();
            info!("🔄 View change triggered, new round: {}", state.current_round);
        }
        
        Ok(())
    }

    /// Handle validator join
    async fn handle_validator_join(&mut self, node_id: Uuid, stake_amount: u64, timestamp: SystemTime) -> Result<()> {
        info!("➕ Processing validator join: {} with stake {}", node_id, stake_amount);
        
        if stake_amount < self.consensus_config.min_stake {
            warn!("❌ Insufficient stake for validator: {}", node_id);
            return Ok(());
        }

        let validator_info = ValidatorInfo {
            node_id,
            stake_amount,
            voting_power: 0.0, // Will be calculated
            is_active: true,
            last_activity: timestamp,
            reputation_score: 100.0,
            blocks_produced: 0,
            slashing_count: 0,
        };

        let mut validators = self.validators.write().await;
        validators.insert(node_id, validator_info);
        
        // Recalculate voting powers
        let total_stake: u64 = validators.values().map(|v| v.stake_amount).sum();
        for validator in validators.values_mut() {
            validator.voting_power = (validator.stake_amount as f64) / (total_stake as f64);
        }
        
        let mut state = self.consensus_state.write().await;
        state.total_stake = total_stake;
        
        info!("✅ Validator {} joined with voting power: {:.3}", 
            node_id, validators.get(&node_id).unwrap().voting_power);
        
        Ok(())
    }

    /// Handle validator leave
    async fn handle_validator_leave(&mut self, node_id: Uuid, _timestamp: SystemTime) -> Result<()> {
        info!("➖ Processing validator leave: {}", node_id);
        
        let mut validators = self.validators.write().await;
        if let Some(validator) = validators.remove(&node_id) {
            let mut state = self.consensus_state.write().await;
            state.total_stake -= validator.stake_amount;
            
            // Recalculate voting powers
            let total_stake = state.total_stake;
            for validator in validators.values_mut() {
                validator.voting_power = (validator.stake_amount as f64) / (total_stake as f64);
            }
            
            info!("✅ Validator {} removed from set", node_id);
        }
        
        Ok(())
    }

    /// Handle slashing evidence
    async fn handle_slashing_evidence(&mut self, accused_node: Uuid, evidence_type: SlashingType, _proof: Vec<u8>, _timestamp: SystemTime) -> Result<()> {
        warn!("⚖️ Processing slashing evidence against {} for {:?}", accused_node, evidence_type);
        
        let mut validators = self.validators.write().await;
        if let Some(validator) = validators.get_mut(&accused_node) {
            validator.slashing_count += 1;
            validator.reputation_score = (validator.reputation_score * 0.8).max(0.0);
            
            // Severe slashing removes validator
            if validator.slashing_count >= 3 {
                validator.is_active = false;
                warn!("🚫 Validator {} deactivated due to repeated slashing", accused_node);
            }
        }
        
        Ok(())
    }

    /// Handle sync request
    async fn handle_sync_request(&mut self, requester: Uuid, from_epoch: u64, to_epoch: Option<u64>) -> Result<()> {
        info!("🔄 Processing sync request from {} for epochs {}+", requester, from_epoch);
        
        let finality_tracker = self.finality_tracker.read().await;
        let end_epoch = to_epoch.unwrap_or(finality_tracker.last_finalized_epoch);
        
        let blocks: Vec<_> = finality_tracker.finalized_blocks
            .range(from_epoch..=end_epoch)
            .map(|(_, block)| block.clone())
            .collect();
        
        let current_state = self.consensus_state.read().await.clone();
        drop(finality_tracker);
        
        if let Some(sender) = &self.message_sender {
            let _ = sender.send(ConsensusMessage::SyncResponse {
                blocks,
                current_state,
            });
        }
        
        Ok(())
    }

    /// Handle sync response
    async fn handle_sync_response(&mut self, blocks: Vec<FinalizedBlock>, current_state: ConsensusState) -> Result<()> {
        info!("📥 Processing sync response with {} blocks", blocks.len());
        
        let mut finality_tracker = self.finality_tracker.write().await;
        for block in blocks {
            finality_tracker.finalized_blocks.insert(block.epoch, block);
        }
        
        if let Some(latest_block) = finality_tracker.finalized_blocks.values().last() {
            finality_tracker.last_finalized_epoch = latest_block.epoch;
        }
        drop(finality_tracker);
        
        // Update our consensus state
        let mut state = self.consensus_state.write().await;
        *state = current_state;
        
        info!("✅ Sync completed, updated to epoch {}", state.current_epoch);
        Ok(())
    }

    /// Get active validator count
    async fn get_active_validator_count(&self) -> usize {
        let validators = self.validators.read().await;
        validators.values().filter(|v| v.is_active).count()
    }

    /// Get consensus statistics
    pub async fn get_consensus_stats(&self) -> ConsensusStats {
        let state = self.consensus_state.read().await;
        let validators = self.validators.read().await;
        let finality_tracker = self.finality_tracker.read().await;
        
        ConsensusStats {
            current_epoch: state.current_epoch,
            current_round: state.current_round,
            current_step: state.current_step.clone(),
            active_validators: validators.values().filter(|v| v.is_active).count(),
            total_stake: state.total_stake,
            finalized_blocks: finality_tracker.finalized_blocks.len(),
            last_finalized_epoch: finality_tracker.last_finalized_epoch,
            pending_proposals: self.pending_proposals.read().await.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub current_epoch: u64,
    pub current_round: u32,
    pub current_step: ConsensusStep,
    pub active_validators: usize,
    pub total_stake: u64,
    pub finalized_blocks: usize,
    pub last_finalized_epoch: u64,
    pub pending_proposals: usize,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            consensus_threshold: 0.67,
            block_time: Duration::from_secs(10),
            max_validators: 100,
            min_stake: 1000,
            validator_rotation_blocks: 100,
            finality_depth: 6,
            timeout_propose: Duration::from_secs(30),
            timeout_prevote: Duration::from_secs(10),
            timeout_precommit: Duration::from_secs(10),
        }
    }
}

/// Thread-safe consensus manager
pub type SharedConsensusManager = Arc<RwLock<ConsensusManager>>;

pub fn create_shared_consensus_manager(node_id: Uuid, is_masternode: bool, stake_amount: u64, config: ConsensusConfig) -> SharedConsensusManager {
    Arc::new(RwLock::new(ConsensusManager::new(node_id, is_masternode, stake_amount, config)))
}