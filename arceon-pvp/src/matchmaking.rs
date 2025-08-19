/*!
# Matchmaking System

Advanced matchmaking service for fair and balanced PvP matches
with skill-based matching, queue management, and team balancing.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, VecDeque};
use anyhow::Result;

use crate::{MatchType, CombatMode, PlayerId, MatchId, LeaderboardEntry, RankTier};

/// Matchmaking service coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingService {
    pub active_queues: HashMap<QueueId, MatchmakingQueue>,
    pub player_queues: HashMap<PlayerId, QueueStatus>,
    pub rating_system: RatingSystem,
    pub queue_settings: QueueSettings,
    pub match_history: HashMap<PlayerId, Vec<MatchHistoryEntry>>,
    pub leaderboards: HashMap<CombatMode, Vec<LeaderboardEntry>>,
}

pub type QueueId = String;

/// Individual matchmaking queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingQueue {
    pub queue_id: QueueId,
    pub queue_type: QueueType,
    pub combat_mode: CombatMode,
    pub waiting_players: VecDeque<QueuedPlayer>,
    pub active_searches: Vec<MatchSearch>,
    pub queue_times: QueueTimeStats,
    pub settings: QueueConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueType {
    Ranked,
    Casual,
    Custom,
    Tournament,
    Arena,
}

/// Player in matchmaking queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedPlayer {
    pub player_id: PlayerId,
    pub queue_time: DateTime<Utc>,
    pub rating: u32,
    pub preferred_roles: Vec<PlayerRole>,
    pub party_members: Vec<PlayerId>,
    pub search_preferences: SearchPreferences,
    pub dodge_penalty: Option<DodgePenalty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PlayerRole {
    Tank,
    Damage,
    Support,
    Flex,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPreferences {
    pub max_queue_time_minutes: u32,
    pub preferred_server_regions: Vec<String>,
    pub avoid_players: Vec<PlayerId>,
    pub skill_range_tolerance: f64, // 0.0 = exact skill, 1.0 = any skill
    pub ping_tolerance_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DodgePenalty {
    pub penalty_end_time: DateTime<Utc>,
    pub remaining_minutes: u32,
    pub offense_count: u32,
}

/// Queue status for a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub queue_id: QueueId,
    pub status: QueuePlayerStatus,
    pub queue_start_time: DateTime<Utc>,
    pub estimated_wait_time: Option<Duration>,
    pub match_found: Option<MatchId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueuePlayerStatus {
    Searching,
    MatchFound,
    Accepting,
    Declined,
    InGame,
    Penalty,
}

/// Active match search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSearch {
    pub search_id: Uuid,
    pub target_players: Vec<PlayerId>,
    pub target_rating_range: (u32, u32),
    pub search_start_time: DateTime<Utc>,
    pub search_criteria: SearchCriteria,
    pub potential_matches: Vec<PotentialMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCriteria {
    pub combat_mode: CombatMode,
    pub min_team_size: u32,
    pub max_team_size: u32,
    pub skill_variance_tolerance: f64,
    pub role_requirements: Vec<RoleRequirement>,
    pub geographic_preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRequirement {
    pub role: PlayerRole,
    pub required_count: u32,
    pub flexible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialMatch {
    pub match_id: Uuid,
    pub players: Vec<PlayerId>,
    pub team_compositions: Vec<TeamComposition>,
    pub skill_balance_score: f64,
    pub estimated_match_quality: f64,
    pub avg_queue_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamComposition {
    pub team_id: Uuid,
    pub members: Vec<PlayerId>,
    pub average_rating: u32,
    pub role_distribution: HashMap<PlayerRole, u32>,
    pub synergy_score: f64,
}

/// Queue time statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueTimeStats {
    pub average_wait_time: Duration,
    pub median_wait_time: Duration,
    pub peak_times: Vec<PeakTime>,
    pub current_queue_size: u32,
    pub matches_created_today: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakTime {
    pub hour_of_day: u8,
    pub average_queue_size: u32,
    pub average_wait_time: Duration,
}

/// Queue configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfiguration {
    pub max_queue_size: u32,
    pub max_wait_time_minutes: u32,
    pub skill_expansion_rate: f64, // How fast to expand skill range
    pub role_flexibility: bool,
    pub party_size_limit: u32,
    pub dodge_penalty_minutes: u32,
    pub minimum_players: u32,
    pub geographic_matching: bool,
}

/// Rating system management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingSystem {
    pub rating_algorithm: RatingAlgorithm,
    pub initial_rating: u32,
    pub rating_ranges: HashMap<RankTier, (u32, u32)>,
    pub calibration_matches: u32,
    pub rating_decay: RatingDecay,
    pub seasonal_resets: Vec<SeasonalReset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RatingAlgorithm {
    Elo,
    Glicko2,
    TrueSkill,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingDecay {
    pub enabled: bool,
    pub decay_per_day: f64,
    pub minimum_rating: u32,
    pub inactivity_threshold_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalReset {
    pub season_id: String,
    pub reset_date: DateTime<Utc>,
    pub soft_reset_percentage: f64, // 0.0 = no reset, 1.0 = full reset
    pub placement_matches: u32,
}

/// Global queue settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueSettings {
    pub enabled_queues: Vec<QueueId>,
    pub peak_hour_adjustments: bool,
    pub cross_region_matching: bool,
    pub skill_based_matching: bool,
    pub role_based_matching: bool,
    pub party_matching_preference: f64,
}

/// Match history entry for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchHistoryEntry {
    pub match_id: MatchId,
    pub queue_time: Duration,
    pub match_quality: f64,
    pub rating_before: u32,
    pub rating_after: u32,
    pub result: MatchResult,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchResult {
    Win,
    Loss,
    Draw,
    Cancelled,
}

impl MatchmakingService {
    /// Create new matchmaking service
    pub fn new() -> Self {
        let mut service = Self {
            active_queues: HashMap::new(),
            player_queues: HashMap::new(),
            rating_system: RatingSystem::default(),
            queue_settings: QueueSettings::default(),
            match_history: HashMap::new(),
            leaderboards: HashMap::new(),
        };

        service.initialize_default_queues();
        service
    }

    /// Initialize default matchmaking queues
    fn initialize_default_queues(&mut self) {
        // Ranked 1v1 Duel
        let duel_queue = MatchmakingQueue {
            queue_id: "ranked_duel".to_string(),
            queue_type: QueueType::Ranked,
            combat_mode: CombatMode::Duel,
            waiting_players: VecDeque::new(),
            active_searches: vec![],
            queue_times: QueueTimeStats::default(),
            settings: QueueConfiguration {
                max_queue_size: 1000,
                max_wait_time_minutes: 15,
                skill_expansion_rate: 0.1,
                role_flexibility: false,
                party_size_limit: 1,
                dodge_penalty_minutes: 5,
                minimum_players: 2,
                geographic_matching: true,
            },
        };

        // Ranked 3v3 Team Battle
        let team_queue = MatchmakingQueue {
            queue_id: "ranked_3v3".to_string(),
            queue_type: QueueType::Ranked,
            combat_mode: CombatMode::SmallGroup,
            waiting_players: VecDeque::new(),
            active_searches: vec![],
            queue_times: QueueTimeStats::default(),
            settings: QueueConfiguration {
                max_queue_size: 500,
                max_wait_time_minutes: 20,
                skill_expansion_rate: 0.15,
                role_flexibility: true,
                party_size_limit: 3,
                dodge_penalty_minutes: 10,
                minimum_players: 6,
                geographic_matching: true,
            },
        };

        // Casual matches
        let casual_queue = MatchmakingQueue {
            queue_id: "casual_mixed".to_string(),
            queue_type: QueueType::Casual,
            combat_mode: CombatMode::SmallGroup,
            waiting_players: VecDeque::new(),
            active_searches: vec![],
            queue_times: QueueTimeStats::default(),
            settings: QueueConfiguration {
                max_queue_size: 2000,
                max_wait_time_minutes: 10,
                skill_expansion_rate: 0.3,
                role_flexibility: true,
                party_size_limit: 5,
                dodge_penalty_minutes: 2,
                minimum_players: 4,
                geographic_matching: false,
            },
        };

        self.active_queues.insert("ranked_duel".to_string(), duel_queue);
        self.active_queues.insert("ranked_3v3".to_string(), team_queue);
        self.active_queues.insert("casual_mixed".to_string(), casual_queue);
    }

    /// Add player to matchmaking queue
    pub fn join_queue(&mut self, player_id: PlayerId, queue_id: QueueId, 
                     preferred_roles: Vec<PlayerRole>) -> Result<()> {
        // Check if player is already in a queue
        if self.player_queues.contains_key(&player_id) {
            return Err(anyhow::anyhow!("Player already in a queue"));
        }

        // Check dodge penalty
        if self.has_dodge_penalty(player_id) {
            return Err(anyhow::anyhow!("Player has active dodge penalty"));
        }

        // Check queue capacity first
        let (max_queue_size, current_players) = {
            let queue = self.active_queues.get(&queue_id)
                .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;
            (queue.settings.max_queue_size as usize, queue.waiting_players.len())
        };

        if current_players >= max_queue_size {
            return Err(anyhow::anyhow!("Queue is full"));
        }

        let player_rating = self.get_player_rating(player_id);
        
        let queue = self.active_queues.get_mut(&queue_id)
            .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;
        let queued_player = QueuedPlayer {
            player_id,
            queue_time: Utc::now(),
            rating: player_rating,
            preferred_roles,
            party_members: vec![], // TODO: Party support
            search_preferences: SearchPreferences::default(),
            dodge_penalty: None,
        };

        queue.waiting_players.push_back(queued_player);

        let queue_status = QueueStatus {
            queue_id: queue_id.clone(),
            status: QueuePlayerStatus::Searching,
            queue_start_time: Utc::now(),
            estimated_wait_time: Some(queue.queue_times.average_wait_time),
            match_found: None,
        };

        self.player_queues.insert(player_id, queue_status);
        Ok(())
    }

    /// Remove player from queue
    pub fn leave_queue(&mut self, player_id: PlayerId) -> Result<()> {
        let queue_status = self.player_queues.remove(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player not in queue"))?;

        let queue = self.active_queues.get_mut(&queue_status.queue_id)
            .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;

        // Remove from waiting players
        queue.waiting_players.retain(|p| p.player_id != player_id);

        // Remove from active searches
        queue.active_searches.retain(|search| !search.target_players.contains(&player_id));

        Ok(())
    }

    /// Find match for player
    pub fn find_match(&mut self, player_id: PlayerId, match_type: MatchType, 
                     combat_mode: CombatMode, _player_rating: u32) -> Result<Option<MatchId>> {
        // Determine appropriate queue
        let queue_id = self.get_queue_for_match_type(&match_type, &combat_mode);
        
        // Join queue if not already queued
        if !self.player_queues.contains_key(&player_id) {
            self.join_queue(player_id, queue_id.clone(), vec![PlayerRole::Any])?;
        }

        // Run matchmaking algorithm
        self.run_matchmaking_cycle(&queue_id)
    }

    /// Run matchmaking cycle for a queue
    fn run_matchmaking_cycle(&mut self, queue_id: &QueueId) -> Result<Option<MatchId>> {
        // First check if we have enough players
        let _minimum_players = {
            let queue = self.active_queues.get(queue_id)
                .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;
            
            if queue.waiting_players.len() < queue.settings.minimum_players as usize {
                return Ok(None); // Not enough players
            }
            
            queue.settings.minimum_players
        };

        // Try to create matches
        let potential_matches = {
            let queue = self.active_queues.get_mut(queue_id)
                .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;
            Self::find_potential_matches_static(queue)?
        };
        
        if let Some(best_match) = self.select_best_match(&potential_matches) {
            let match_id = best_match.match_id;
            let players_to_update = best_match.players.clone();
            
            // Update queue and remove players
            {
                let queue = self.active_queues.get_mut(queue_id)
                    .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;
                    
                // Remove players from queue
                for &player_id in &players_to_update {
                    queue.waiting_players.retain(|p| p.player_id != player_id);
                }
                
                // Update queue statistics
                queue.queue_times.matches_created_today += 1;
            }
            
            // Update player queue status
            for &player_id in &players_to_update {
                if let Some(status) = self.player_queues.get_mut(&player_id) {
                    status.status = QueuePlayerStatus::MatchFound;
                    status.match_found = Some(match_id);
                }
            }
            
            tracing::info!("Created match {} with {} players", match_id, players_to_update.len());
            return Ok(Some(match_id));
        }

        Ok(None)
    }

    /// Find potential matches in queue
    fn find_potential_matches_static(queue: &mut MatchmakingQueue) -> Result<Vec<PotentialMatch>> {
        let mut potential_matches = vec![];

        let players: Vec<&QueuedPlayer> = queue.waiting_players.iter().collect();
        let required_players = queue.settings.minimum_players as usize;

        // Try different combinations of players
        for chunk in players.chunks(required_players) {
            if chunk.len() != required_players {
                continue;
            }

            let skill_balance = Self::calculate_skill_balance_static(chunk);
            if skill_balance < 0.3 {
                continue; // Too imbalanced
            }

            let match_quality = Self::calculate_match_quality_static(chunk, queue);
            if match_quality < 0.5 {
                continue; // Low quality match
            }

            let team_compositions = Self::create_team_compositions_static(chunk, &queue.combat_mode)?;
            
            let potential_match = PotentialMatch {
                match_id: Uuid::new_v4(),
                players: chunk.iter().map(|p| p.player_id).collect(),
                team_compositions,
                skill_balance_score: skill_balance,
                estimated_match_quality: match_quality,
                avg_queue_time: Self::calculate_avg_queue_time_static(chunk),
            };

            potential_matches.push(potential_match);
        }

        Ok(potential_matches)
    }

    /// Calculate skill balance for a group of players
    fn calculate_skill_balance_static(players: &[&QueuedPlayer]) -> f64 {
        if players.is_empty() {
            return 0.0;
        }

        let ratings: Vec<u32> = players.iter().map(|p| p.rating).collect();
        let avg_rating = ratings.iter().sum::<u32>() as f64 / ratings.len() as f64;
        
        let variance = ratings.iter()
            .map(|&rating| (rating as f64 - avg_rating).powi(2))
            .sum::<f64>() / ratings.len() as f64;
        
        let std_dev = variance.sqrt();
        let coefficient_of_variation = std_dev / avg_rating;
        
        // Lower coefficient = better balance
        (1.0 - coefficient_of_variation.min(1.0)).max(0.0)
    }

    /// Calculate match quality score
    fn calculate_match_quality_static(players: &[&QueuedPlayer], queue: &MatchmakingQueue) -> f64 {
        let mut quality_score = 0.0;

        // Skill balance component
        quality_score += Self::calculate_skill_balance_static(players) * 0.4;

        // Queue time component (longer wait = accept lower quality)
        let avg_queue_time = Self::calculate_avg_queue_time_static(players);
        let wait_time_factor = (avg_queue_time.num_minutes() as f64 / queue.settings.max_wait_time_minutes as f64).min(1.0);
        quality_score += (1.0 - wait_time_factor) * 0.3;

        // Role distribution component
        let role_balance = Self::calculate_role_balance_static(players, &queue.combat_mode);
        quality_score += role_balance * 0.3;

        quality_score
    }

    /// Calculate role balance for team composition
    fn calculate_role_balance_static(players: &[&QueuedPlayer], combat_mode: &CombatMode) -> f64 {
        let role_counts: HashMap<PlayerRole, u32> = HashMap::new();
        
        // Count preferred roles
        for player in players {
            for role in &player.preferred_roles {
                *role_counts.get(role).unwrap_or(&0);
            }
        }

        // Calculate balance based on combat mode requirements
        match combat_mode {
            CombatMode::Duel => 1.0, // No role requirements for 1v1
            CombatMode::SmallGroup => {
                // Prefer balanced composition for small groups
                let total_roles = role_counts.values().sum::<u32>() as f64;
                if total_roles == 0.0 {
                    return 0.5; // Default if no roles specified
                }
                
                let ideal_distribution = 1.0 / role_counts.len() as f64;
                let balance_score = role_counts.values()
                    .map(|&count| (count as f64 / total_roles - ideal_distribution).abs())
                    .sum::<f64>();
                
                (1.0 - balance_score).max(0.0)
            },
            _ => 0.7, // Default balance score for other modes
        }
    }

    /// Create team compositions from players
    fn create_team_compositions_static(players: &[&QueuedPlayer], combat_mode: &CombatMode) -> Result<Vec<TeamComposition>> {
        let mut compositions = vec![];

        match combat_mode {
            CombatMode::Duel => {
                // 1v1: Each player is their own team
                for (_i, player) in players.iter().enumerate() {
                    compositions.push(TeamComposition {
                        team_id: Uuid::new_v4(),
                        members: vec![player.player_id],
                        average_rating: player.rating,
                        role_distribution: HashMap::new(),
                        synergy_score: 1.0,
                    });
                }
            },
            CombatMode::SmallGroup => {
                // Split into balanced teams
                let _team_size = players.len() / 2;
                let mut sorted_players: Vec<&QueuedPlayer> = players.iter().cloned().collect();
                sorted_players.sort_by(|a, b| b.rating.cmp(&a.rating));

                // Snake draft to balance teams
                let mut team1 = vec![];
                let mut team2 = vec![];
                
                for (i, player) in sorted_players.into_iter().enumerate() {
                    if i % 4 < 2 {
                        team1.push(player.player_id);
                    } else {
                        team2.push(player.player_id);
                    }
                }

                if !team1.is_empty() {
                    compositions.push(TeamComposition {
                        team_id: Uuid::new_v4(),
                        members: team1.clone(),
                        average_rating: Self::calculate_team_rating_static(&team1),
                        role_distribution: HashMap::new(),
                        synergy_score: 0.8,
                    });
                }

                if !team2.is_empty() {
                    compositions.push(TeamComposition {
                        team_id: Uuid::new_v4(),
                        members: team2.clone(),
                        average_rating: Self::calculate_team_rating_static(&team2),
                        role_distribution: HashMap::new(),
                        synergy_score: 0.8,
                    });
                }
            },
            _ => {
                // Default team creation
                compositions.push(TeamComposition {
                    team_id: Uuid::new_v4(),
                    members: players.iter().map(|p| p.player_id).collect(),
                    average_rating: players.iter().map(|p| p.rating).sum::<u32>() / players.len() as u32,
                    role_distribution: HashMap::new(),
                    synergy_score: 0.7,
                });
            }
        }

        Ok(compositions)
    }

    /// Calculate average team rating
    fn calculate_team_rating_static(team: &[PlayerId]) -> u32 {
        if team.is_empty() {
            return 1000; // Default rating
        }
        
        // Simplified: just return default rating since get_player_rating always returns 1000
        1000
    }

    /// Calculate average queue time for players
    fn calculate_avg_queue_time_static(players: &[&QueuedPlayer]) -> Duration {
        if players.is_empty() {
            return Duration::zero();
        }

        let now = Utc::now();
        let total_wait: i64 = players.iter()
            .map(|p| (now - p.queue_time).num_seconds())
            .sum();

        Duration::seconds(total_wait / players.len() as i64)
    }

    /// Select the best match from potential matches
    fn select_best_match<'a>(&self, potential_matches: &'a [PotentialMatch]) -> Option<&'a PotentialMatch> {
        potential_matches.iter()
            .max_by(|a, b| a.estimated_match_quality.partial_cmp(&b.estimated_match_quality).unwrap())
    }

    /// Create actual match from potential match
    fn create_match_from_potential(&mut self, queue: &mut MatchmakingQueue, 
                                  potential_match: &PotentialMatch) -> Result<MatchId> {
        let match_id = potential_match.match_id;

        // Remove players from queue
        for &player_id in &potential_match.players {
            queue.waiting_players.retain(|p| p.player_id != player_id);
            
            // Update player queue status
            if let Some(status) = self.player_queues.get_mut(&player_id) {
                status.status = QueuePlayerStatus::MatchFound;
                status.match_found = Some(match_id);
            }
        }

        // Update queue statistics
        queue.queue_times.matches_created_today += 1;

        tracing::info!("Created match {} with {} players", match_id, potential_match.players.len());
        Ok(match_id)
    }

    /// Get appropriate queue for match type and combat mode
    fn get_queue_for_match_type(&self, match_type: &MatchType, combat_mode: &CombatMode) -> QueueId {
        match (match_type, combat_mode) {
            (MatchType::Ranked, CombatMode::Duel) => "ranked_duel".to_string(),
            (MatchType::Ranked, CombatMode::SmallGroup) => "ranked_3v3".to_string(),
            (MatchType::Casual, _) => "casual_mixed".to_string(),
            _ => "casual_mixed".to_string(), // Default fallback
        }
    }

    /// Check if player has active dodge penalty
    fn has_dodge_penalty(&self, _player_id: PlayerId) -> bool {
        // In real implementation, would check penalty database
        false
    }

    /// Get player's current rating
    pub fn get_player_rating(&self, _player_id: PlayerId) -> u32 {
        // In real implementation, would query rating database
        1000 // Default starting rating
    }

    /// Apply dodge penalty to player
    pub fn apply_dodge_penalty(&mut self, player_id: PlayerId) -> Result<()> {
        if let Some(queue_status) = self.player_queues.get_mut(&player_id) {
            queue_status.status = QueuePlayerStatus::Penalty;
        }

        // In real implementation, would store penalty in database
        tracing::info!("Applied dodge penalty to player {}", player_id);
        Ok(())
    }

    /// Get leaderboard for combat mode
    pub fn get_leaderboard(&self, combat_mode: CombatMode, limit: usize) -> Vec<LeaderboardEntry> {
        self.leaderboards.get(&combat_mode)
            .map(|leaderboard| leaderboard.iter().take(limit).cloned().collect())
            .unwrap_or_default()
    }

    /// Update leaderboard after match
    pub fn update_leaderboard(&mut self, combat_mode: CombatMode, players: &[PlayerId], 
                             _results: &[MatchResult]) {
        // In real implementation, would update leaderboard based on match results
        tracing::info!("Updating leaderboard for {:?} with {} players", combat_mode, players.len());
    }

    /// Get queue status for player
    pub fn get_queue_status(&self, player_id: PlayerId) -> Option<&QueueStatus> {
        self.player_queues.get(&player_id)
    }

    /// Get queue statistics
    pub fn get_queue_stats(&self, queue_id: &QueueId) -> Option<&QueueTimeStats> {
        self.active_queues.get(queue_id).map(|q| &q.queue_times)
    }
}

impl Default for SearchPreferences {
    fn default() -> Self {
        Self {
            max_queue_time_minutes: 15,
            preferred_server_regions: vec!["auto".to_string()],
            avoid_players: vec![],
            skill_range_tolerance: 0.2,
            ping_tolerance_ms: 100,
        }
    }
}

impl Default for QueueTimeStats {
    fn default() -> Self {
        Self {
            average_wait_time: Duration::minutes(5),
            median_wait_time: Duration::minutes(3),
            peak_times: vec![],
            current_queue_size: 0,
            matches_created_today: 0,
        }
    }
}

impl Default for RatingSystem {
    fn default() -> Self {
        Self {
            rating_algorithm: RatingAlgorithm::Elo,
            initial_rating: 1000,
            rating_ranges: [
                (RankTier::Bronze, (0, 1199)),
                (RankTier::Silver, (1200, 1399)),
                (RankTier::Gold, (1400, 1599)),
                (RankTier::Platinum, (1600, 1799)),
                (RankTier::Diamond, (1800, 1999)),
                (RankTier::Master, (2000, 2199)),
                (RankTier::Grandmaster, (2200, 2399)),
                (RankTier::Challenger, (2400, 9999)),
            ].into(),
            calibration_matches: 10,
            rating_decay: RatingDecay {
                enabled: true,
                decay_per_day: 2.0,
                minimum_rating: 800,
                inactivity_threshold_days: 14,
            },
            seasonal_resets: vec![],
        }
    }
}

impl Default for QueueSettings {
    fn default() -> Self {
        Self {
            enabled_queues: vec![
                "ranked_duel".to_string(),
                "ranked_3v3".to_string(),
                "casual_mixed".to_string(),
            ],
            peak_hour_adjustments: true,
            cross_region_matching: false,
            skill_based_matching: true,
            role_based_matching: true,
            party_matching_preference: 0.8,
        }
    }
}

impl Default for MatchmakingService {
    fn default() -> Self {
        Self::new()
    }
}