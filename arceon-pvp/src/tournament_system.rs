/*!
# Tournament System

Comprehensive tournament organization and management system
for competitive PvP events with brackets, scheduling, and prizes.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use anyhow::Result;

use crate::{TournamentId, PlayerId, MatchId, ArenaId, CombatMode};

/// Tournament coordination system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentCoordinator {
    pub tournaments: HashMap<TournamentId, Tournament>,
    pub tournament_templates: Vec<TournamentTemplate>,
    pub player_registrations: HashMap<PlayerId, Vec<TournamentRegistration>>,
    pub tournament_calendar: TournamentCalendar,
    pub ranking_system: TournamentRankingSystem,
}

/// Tournament definition and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub tournament_id: TournamentId,
    pub name: String,
    pub description: String,
    pub tournament_type: TournamentType,
    pub format: TournamentFormat,
    pub organizer_id: PlayerId,
    pub co_organizers: Vec<PlayerId>,
    pub prize_pool: PrizePool,
    pub registration: RegistrationInfo,
    pub schedule: TournamentSchedule,
    pub participants: HashMap<PlayerId, ParticipantInfo>,
    pub brackets: HashMap<String, TournamentBracket>,
    pub matches: HashMap<MatchId, TournamentMatch>,
    pub rules: TournamentRules,
    pub settings: TournamentSettings,
    pub status: TournamentStatus,
    pub statistics: TournamentStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TournamentType {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    Swiss,
    Ladder,
    KingOfTheHill,
    BattleRoyale,
    TeamBased,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TournamentFormat {
    Individual,
    Team,
    Mixed,
    GuildVsGuild,
}

/// Tournament prize pool and rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrizePool {
    pub total_value: u64,
    pub currency_type: String,
    pub distribution: Vec<PrizeDistribution>,
    pub additional_rewards: Vec<AdditionalReward>,
    pub sponsor_contributions: Vec<SponsorContribution>,
    pub entry_fee_pool: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrizeDistribution {
    pub placement: PlacementRange,
    pub amount: u64,
    pub percentage: f64,
    pub additional_items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlacementRange {
    Exact(u32),
    Range { start: u32, end: u32 },
    Top(u32),
    Bottom(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalReward {
    pub reward_type: String,
    pub description: String,
    pub eligibility: Vec<String>,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SponsorContribution {
    pub sponsor_id: String,
    pub sponsor_name: String,
    pub contribution_amount: u64,
    pub sponsored_prizes: Vec<String>,
    pub visibility_requirements: Vec<String>,
}

/// Registration information and requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationInfo {
    pub registration_open: DateTime<Utc>,
    pub registration_close: DateTime<Utc>,
    pub entry_fee: Option<u64>,
    pub requirements: Vec<RegistrationRequirement>,
    pub capacity: TournamentCapacity,
    pub approval_required: bool,
    pub team_registration: bool,
    pub substitution_policy: SubstitutionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequirement {
    pub requirement_type: RequirementType,
    pub description: String,
    pub mandatory: bool,
    pub verification_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequirementType {
    MinimumRating(u32),
    MaximumRating(u32),
    RatingRange { min: u32, max: u32 },
    AccountAge(u32), // Days
    RegionRestriction(Vec<String>),
    GuildMembership(String),
    AchievementRequired(String),
    PreviousTournamentParticipation,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentCapacity {
    pub minimum_participants: u32,
    pub maximum_participants: u32,
    pub optimal_participants: u32,
    pub waiting_list_enabled: bool,
    pub expansion_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstitutionPolicy {
    pub allowed: bool,
    pub deadline: Option<DateTime<Utc>>,
    pub restrictions: Vec<String>,
    pub approval_required: bool,
}

/// Tournament schedule and timing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentSchedule {
    pub start_time: DateTime<Utc>,
    pub estimated_duration: Duration,
    pub check_in_period: Duration,
    pub match_scheduling: MatchSchedulingType,
    pub break_periods: Vec<BreakPeriod>,
    pub timezone: String,
    pub flexible_timing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchSchedulingType {
    Fixed,      // All matches at predetermined times
    Rolling,    // Matches start as soon as ready
    Staggered,  // Matches spread over time
    PlayerChoice, // Players schedule within windows
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakPeriod {
    pub break_type: BreakType,
    pub duration: Duration,
    pub scheduled_after: String, // Round, match, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakType {
    Intermission,
    Meal,
    Technical,
    Awards,
    Custom(String),
}

/// Participant information and status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantInfo {
    pub player_id: PlayerId,
    pub registration_date: DateTime<Utc>,
    pub team_id: Option<Uuid>,
    pub seed: Option<u32>,
    pub check_in_status: CheckInStatus,
    pub current_bracket: Option<String>,
    pub match_history: Vec<MatchId>,
    pub performance: ParticipantPerformance,
    pub status: ParticipantStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckInStatus {
    NotCheckedIn,
    CheckedIn,
    Ready,
    NoShow,
    Disqualified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantPerformance {
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
    pub total_score: u32,
    pub average_performance: f64,
    pub best_performance: f64,
    pub consistency_rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantStatus {
    Active,
    Eliminated,
    Withdrawn,
    Disqualified,
    Champion,
    RunnerUp,
    Semifinalist,
}

/// Tournament bracket system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentBracket {
    pub bracket_id: String,
    pub bracket_type: BracketType,
    pub rounds: Vec<TournamentRound>,
    pub participants: Vec<PlayerId>,
    pub seeding_method: SeedingMethod,
    pub advancement_rules: AdvancementRules,
    pub current_round: u32,
    pub bracket_status: BracketStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BracketType {
    Main,
    Losers,
    Winners,
    Consolation,
    Qualifying,
    Group(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentRound {
    pub round_number: u32,
    pub round_name: String,
    pub matches: Vec<MatchId>,
    pub scheduled_start: DateTime<Utc>,
    pub estimated_duration: Duration,
    pub status: RoundStatus,
    pub advancement_count: u32, // How many advance from this round
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoundStatus {
    Scheduled,
    InProgress,
    Completed,
    Delayed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeedingMethod {
    Random,
    Rating,
    Registration,
    Manual,
    Swiss,
    BalancedGroups,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancementRules {
    pub top_finishers: u32,
    pub win_requirement: Option<u32>,
    pub score_threshold: Option<u32>,
    pub tiebreaker_method: TiebreakerMethod,
    pub wildcard_slots: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TiebreakerMethod {
    HeadToHead,
    OverallRecord,
    ScoreDifferential,
    Random,
    PlayoffMatch,
    Multiple(Vec<TiebreakerMethod>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BracketStatus {
    NotStarted,
    Active,
    Completed,
    Paused,
    Cancelled,
}

/// Individual tournament match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentMatch {
    pub match_id: MatchId,
    pub tournament_id: TournamentId,
    pub bracket_id: String,
    pub round_number: u32,
    pub match_number: u32,
    pub participants: Vec<PlayerId>,
    pub scheduled_time: DateTime<Utc>,
    pub arena_id: Option<ArenaId>,
    pub match_format: MatchFormat,
    pub status: TournamentMatchStatus,
    pub result: Option<TournamentMatchResult>,
    pub admin_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchFormat {
    pub combat_mode: CombatMode,
    pub best_of: u32, // Best of X games
    pub time_limit: Option<u32>,
    pub special_rules: Vec<String>,
    pub map_selection: MapSelectionMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapSelectionMethod {
    Fixed(ArenaId),
    Random,
    PlayerChoice,
    BanPick,
    Tournament,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TournamentMatchStatus {
    Scheduled,
    Ready,
    InProgress,
    Completed,
    Disputed,
    Cancelled,
    Forfeited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentMatchResult {
    pub winner: Option<PlayerId>,
    pub scores: HashMap<PlayerId, u32>,
    pub individual_game_results: Vec<GameResult>,
    pub duration: Duration,
    pub mvp: Option<PlayerId>,
    pub performance_ratings: HashMap<PlayerId, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub game_number: u32,
    pub winner: Option<PlayerId>,
    pub scores: HashMap<PlayerId, u32>,
    pub duration: Duration,
}

/// Tournament rules and regulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentRules {
    pub ruleset_version: String,
    pub general_rules: Vec<String>,
    pub conduct_rules: Vec<String>,
    pub technical_rules: Vec<String>,
    pub dispute_resolution: DisputeResolution,
    pub penalties: Vec<PenaltyRule>,
    pub equipment_restrictions: Vec<String>,
    pub external_software_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeResolution {
    pub dispute_window: Duration,
    pub escalation_process: Vec<EscalationLevel>,
    pub evidence_requirements: Vec<String>,
    pub admin_authority: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationLevel {
    pub level: u32,
    pub authority: String,
    pub timeframe: Duration,
    pub resolution_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyRule {
    pub violation_type: String,
    pub penalty_type: PenaltyType,
    pub severity: PenaltySeverity,
    pub repeat_offender_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PenaltyType {
    Warning,
    MatchLoss,
    RoundLoss,
    Disqualification,
    Suspension,
    Ban,
    Fine(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PenaltySeverity {
    Minor,
    Major,
    Severe,
    Critical,
}

/// Tournament settings and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentSettings {
    pub public_visibility: bool,
    pub spectating_allowed: bool,
    pub live_streaming: bool,
    pub recording_enabled: bool,
    pub chat_enabled: bool,
    pub pause_allowed: bool,
    pub coaching_allowed: bool,
    pub substitution_allowed: bool,
    pub rematch_policy: RematchPolicy,
    pub technical_pause_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RematchPolicy {
    pub allowed: bool,
    pub conditions: Vec<String>,
    pub time_limit: Option<Duration>,
    pub approval_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TournamentStatus {
    Draft,
    RegistrationOpen,
    RegistrationClosed,
    CheckInPeriod,
    InProgress,
    Paused,
    Completed,
    Cancelled,
    Postponed,
}

/// Tournament statistics and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentStatistics {
    pub total_participants: u32,
    pub total_matches: u32,
    pub completed_matches: u32,
    pub average_match_duration: Duration,
    pub viewership_stats: ViewershipStats,
    pub engagement_metrics: EngagementMetrics,
    pub prize_distribution_status: PrizeDistributionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewershipStats {
    pub peak_concurrent_viewers: u32,
    pub total_view_time: Duration,
    pub average_viewers: f64,
    pub viewer_retention_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub chat_messages: u64,
    pub reactions: u64,
    pub social_shares: u64,
    pub participant_satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrizeDistributionStatus {
    pub distributed_amount: u64,
    pub pending_amount: u64,
    pub distribution_timeline: Vec<PayoutSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutSchedule {
    pub recipient: PlayerId,
    pub amount: u64,
    pub scheduled_date: DateTime<Utc>,
    pub status: PayoutStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayoutStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Disputed,
}

/// Tournament template for easy creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub tournament_type: TournamentType,
    pub format: TournamentFormat,
    pub default_settings: TournamentSettings,
    pub suggested_prize_structure: Vec<PrizeDistribution>,
    pub recommended_rules: TournamentRules,
    pub capacity_recommendations: TournamentCapacity,
    pub popularity_score: f64,
}

/// Player tournament registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentRegistration {
    pub registration_id: Uuid,
    pub tournament_id: TournamentId,
    pub player_id: PlayerId,
    pub team_id: Option<Uuid>,
    pub registration_date: DateTime<Utc>,
    pub status: RegistrationStatus,
    pub payment_status: Option<PaymentStatus>,
    pub requirements_met: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationStatus {
    Applied,
    Approved,
    Rejected,
    Waitlisted,
    Withdrawn,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Refunded,
    Failed,
    Waived,
}

/// Tournament calendar and scheduling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentCalendar {
    pub upcoming_tournaments: Vec<TournamentId>,
    pub featured_tournaments: Vec<TournamentId>,
    pub seasonal_tournaments: HashMap<String, Vec<TournamentId>>,
    pub recurring_tournaments: HashMap<Uuid, RecurringTournament>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringTournament {
    pub recurring_id: Uuid,
    pub base_template: TournamentTemplate,
    pub recurrence_pattern: RecurrencePattern,
    pub next_tournament: DateTime<Utc>,
    pub auto_generation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrencePattern {
    Daily,
    Weekly,
    Monthly,
    Seasonal,
    Custom(Duration),
}

/// Tournament ranking and rating system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentRankingSystem {
    pub global_rankings: HashMap<CombatMode, Vec<PlayerTournamentRating>>,
    pub seasonal_rankings: HashMap<String, HashMap<CombatMode, Vec<PlayerTournamentRating>>>,
    pub tournament_history: HashMap<PlayerId, Vec<TournamentResult>>,
    pub rating_calculations: RatingCalculationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTournamentRating {
    pub player_id: PlayerId,
    pub rating: u32,
    pub tournament_points: u64,
    pub tournaments_played: u32,
    pub best_placement: u32,
    pub consistency_score: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentResult {
    pub tournament_id: TournamentId,
    pub placement: u32,
    pub total_participants: u32,
    pub performance_score: f64,
    pub prize_earned: u64,
    pub rating_change: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingCalculationRules {
    pub base_rating: u32,
    pub placement_multipliers: HashMap<u32, f64>,
    pub tournament_size_bonus: f64,
    pub consistency_weight: f64,
    pub decay_rate: f64,
    pub seasonal_reset_percentage: f64,
}

impl TournamentCoordinator {
    /// Create new tournament coordinator
    pub fn new() -> Self {
        Self {
            tournaments: HashMap::new(),
            tournament_templates: Self::create_default_templates(),
            player_registrations: HashMap::new(),
            tournament_calendar: TournamentCalendar::default(),
            ranking_system: TournamentRankingSystem::default(),
        }
    }

    /// Create default tournament templates
    fn create_default_templates() -> Vec<TournamentTemplate> {
        vec![
            TournamentTemplate {
                template_id: Uuid::new_v4(),
                name: "Weekly Duel Championship".to_string(),
                tournament_type: TournamentType::SingleElimination,
                format: TournamentFormat::Individual,
                default_settings: TournamentSettings::default_competitive(),
                suggested_prize_structure: vec![
                    PrizeDistribution {
                        placement: PlacementRange::Exact(1),
                        amount: 5000,
                        percentage: 50.0,
                        additional_items: vec!["Champion Title".to_string()],
                    },
                    PrizeDistribution {
                        placement: PlacementRange::Exact(2),
                        amount: 2500,
                        percentage: 25.0,
                        additional_items: vec!["Runner-up Title".to_string()],
                    },
                    PrizeDistribution {
                        placement: PlacementRange::Range { start: 3, end: 4 },
                        amount: 1250,
                        percentage: 12.5,
                        additional_items: vec!["Semifinalist Badge".to_string()],
                    },
                ],
                recommended_rules: TournamentRules::default_competitive(),
                capacity_recommendations: TournamentCapacity {
                    minimum_participants: 8,
                    maximum_participants: 64,
                    optimal_participants: 32,
                    waiting_list_enabled: true,
                    expansion_allowed: false,
                },
                popularity_score: 0.8,
            },
            TournamentTemplate {
                template_id: Uuid::new_v4(),
                name: "Team Battle Championship".to_string(),
                tournament_type: TournamentType::DoubleElimination,
                format: TournamentFormat::Team,
                default_settings: TournamentSettings::default_team(),
                suggested_prize_structure: vec![
                    PrizeDistribution {
                        placement: PlacementRange::Exact(1),
                        amount: 15000,
                        percentage: 60.0,
                        additional_items: vec!["Team Championship Trophy".to_string()],
                    },
                    PrizeDistribution {
                        placement: PlacementRange::Exact(2),
                        amount: 6000,
                        percentage: 24.0,
                        additional_items: vec!["Silver Medal".to_string()],
                    },
                ],
                recommended_rules: TournamentRules::default_team(),
                capacity_recommendations: TournamentCapacity {
                    minimum_participants: 6, // 6 teams minimum
                    maximum_participants: 16,
                    optimal_participants: 8,
                    waiting_list_enabled: true,
                    expansion_allowed: true,
                },
                popularity_score: 0.9,
            },
        ]
    }

    /// Create new tournament
    pub fn create_tournament(&mut self, organizer_id: PlayerId, template_id: Option<Uuid>, 
                           custom_config: Option<TournamentConfig>) -> Result<TournamentId> {
        let tournament_id = Uuid::new_v4();
        
        let tournament = if let Some(template_id) = template_id {
            self.create_from_template(tournament_id, organizer_id, template_id, custom_config)?
        } else {
            self.create_custom_tournament(tournament_id, organizer_id, custom_config)?
        };

        self.tournaments.insert(tournament_id, tournament);
        Ok(tournament_id)
    }

    /// Create tournament from template
    fn create_from_template(&self, tournament_id: TournamentId, organizer_id: PlayerId, 
                           template_id: Uuid, custom_config: Option<TournamentConfig>) -> Result<Tournament> {
        let template = self.tournament_templates.iter()
            .find(|t| t.template_id == template_id)
            .ok_or_else(|| anyhow::anyhow!("Tournament template not found"))?;

        let config = custom_config.unwrap_or_default();
        
        Ok(Tournament {
            tournament_id,
            name: config.name.unwrap_or_else(|| template.name.clone()),
            description: config.description.unwrap_or_default(),
            tournament_type: template.tournament_type.clone(),
            format: template.format.clone(),
            organizer_id,
            co_organizers: vec![],
            prize_pool: PrizePool {
                total_value: config.prize_pool.unwrap_or(10000),
                currency_type: "gold".to_string(),
                distribution: template.suggested_prize_structure.clone(),
                additional_rewards: vec![],
                sponsor_contributions: vec![],
                entry_fee_pool: 0,
            },
            registration: RegistrationInfo {
                registration_open: Utc::now(),
                registration_close: Utc::now() + Duration::days(7),
                entry_fee: config.entry_fee,
                requirements: vec![],
                capacity: template.capacity_recommendations.clone(),
                approval_required: false,
                team_registration: matches!(template.format, TournamentFormat::Team | TournamentFormat::GuildVsGuild),
                substitution_policy: SubstitutionPolicy {
                    allowed: true,
                    deadline: None,
                    restrictions: vec![],
                    approval_required: false,
                },
            },
            schedule: TournamentSchedule {
                start_time: Utc::now() + Duration::days(7),
                estimated_duration: Duration::hours(4),
                check_in_period: Duration::minutes(30),
                match_scheduling: MatchSchedulingType::Rolling,
                break_periods: vec![],
                timezone: "UTC".to_string(),
                flexible_timing: false,
            },
            participants: HashMap::new(),
            brackets: HashMap::new(),
            matches: HashMap::new(),
            rules: template.recommended_rules.clone(),
            settings: template.default_settings.clone(),
            status: TournamentStatus::Draft,
            statistics: TournamentStatistics::default(),
        })
    }

    /// Create custom tournament
    fn create_custom_tournament(&self, tournament_id: TournamentId, organizer_id: PlayerId, 
                               config: Option<TournamentConfig>) -> Result<Tournament> {
        let config = config.unwrap_or_default();
        
        Ok(Tournament {
            tournament_id,
            name: config.name.unwrap_or_else(|| "Custom Tournament".to_string()),
            description: config.description.unwrap_or_default(),
            tournament_type: TournamentType::SingleElimination,
            format: TournamentFormat::Individual,
            organizer_id,
            co_organizers: vec![],
            prize_pool: PrizePool {
                total_value: config.prize_pool.unwrap_or(1000),
                currency_type: "gold".to_string(),
                distribution: vec![],
                additional_rewards: vec![],
                sponsor_contributions: vec![],
                entry_fee_pool: 0,
            },
            registration: RegistrationInfo {
                registration_open: Utc::now(),
                registration_close: Utc::now() + Duration::days(3),
                entry_fee: config.entry_fee,
                requirements: vec![],
                capacity: TournamentCapacity {
                    minimum_participants: 4,
                    maximum_participants: 32,
                    optimal_participants: 16,
                    waiting_list_enabled: true,
                    expansion_allowed: true,
                },
                approval_required: false,
                team_registration: false,
                substitution_policy: SubstitutionPolicy {
                    allowed: false,
                    deadline: None,
                    restrictions: vec![],
                    approval_required: false,
                },
            },
            schedule: TournamentSchedule {
                start_time: Utc::now() + Duration::days(3),
                estimated_duration: Duration::hours(2),
                check_in_period: Duration::minutes(15),
                match_scheduling: MatchSchedulingType::Rolling,
                break_periods: vec![],
                timezone: "UTC".to_string(),
                flexible_timing: true,
            },
            participants: HashMap::new(),
            brackets: HashMap::new(),
            matches: HashMap::new(),
            rules: TournamentRules::default_casual(),
            settings: TournamentSettings::default_casual(),
            status: TournamentStatus::Draft,
            statistics: TournamentStatistics::default(),
        })
    }

    /// Register player for tournament
    pub fn register_player(&mut self, tournament_id: TournamentId, player_id: PlayerId, 
                          team_id: Option<Uuid>) -> Result<Uuid> {
        let tournament = self.tournaments.get_mut(&tournament_id)
            .ok_or_else(|| anyhow::anyhow!("Tournament not found"))?;

        // Check if registration is open
        let now = Utc::now();
        if now < tournament.registration.registration_open || now > tournament.registration.registration_close {
            return Err(anyhow::anyhow!("Registration is not currently open"));
        }

        // Check capacity
        if tournament.participants.len() >= tournament.registration.capacity.maximum_participants as usize {
            return Err(anyhow::anyhow!("Tournament is at maximum capacity"));
        }

        // Check if already registered
        if tournament.participants.contains_key(&player_id) {
            return Err(anyhow::anyhow!("Player already registered"));
        }

        // Validate requirements
        for requirement in &tournament.registration.requirements {
            if requirement.mandatory && !self.check_requirement(player_id, requirement)? {
                return Err(anyhow::anyhow!("Player does not meet requirement: {}", requirement.description));
            }
        }

        let registration_id = Uuid::new_v4();
        
        // Create participant info
        let participant_info = ParticipantInfo {
            player_id,
            registration_date: now,
            team_id,
            seed: None,
            check_in_status: CheckInStatus::NotCheckedIn,
            current_bracket: None,
            match_history: vec![],
            performance: ParticipantPerformance::default(),
            status: ParticipantStatus::Active,
        };

        tournament.participants.insert(player_id, participant_info);

        // Create registration record
        let registration = TournamentRegistration {
            registration_id,
            tournament_id,
            player_id,
            team_id,
            registration_date: now,
            status: if tournament.registration.approval_required {
                RegistrationStatus::Applied
            } else {
                RegistrationStatus::Approved
            },
            payment_status: tournament.registration.entry_fee.map(|_| PaymentStatus::Pending),
            requirements_met: HashMap::new(),
        };

        self.player_registrations.entry(player_id)
            .or_insert_with(Vec::new)
            .push(registration);

        tracing::info!("Player {} registered for tournament {}", player_id, tournament_id);
        Ok(registration_id)
    }

    /// Check if player meets requirement
    fn check_requirement(&self, _player_id: PlayerId, _requirement: &RegistrationRequirement) -> Result<bool> {
        // In real implementation, would validate against player profile
        Ok(true) // Simplified for now
    }

    /// Start tournament
    pub fn start_tournament(&mut self, tournament_id: TournamentId) -> Result<()> {
        let tournament = self.tournaments.get_mut(&tournament_id)
            .ok_or_else(|| anyhow::anyhow!("Tournament not found"))?;

        // Check if tournament can start
        if tournament.participants.len() < tournament.registration.capacity.minimum_participants as usize {
            return Err(anyhow::anyhow!("Not enough participants to start tournament"));
        }

        // Check all participants are checked in
        let checked_in_count = tournament.participants.values()
            .filter(|p| matches!(p.check_in_status, CheckInStatus::CheckedIn | CheckInStatus::Ready))
            .count();

        if checked_in_count != tournament.participants.len() {
            return Err(anyhow::anyhow!("Not all participants are checked in"));
        }

        tournament.status = TournamentStatus::InProgress;

        // Generate brackets
        self.generate_brackets(tournament_id)?;

        // Schedule first round
        self.schedule_next_round(tournament_id)?;

        tracing::info!("Started tournament {} with {} participants", tournament_id, tournament.participants.len());
        Ok(())
    }

    /// Generate tournament brackets
    fn generate_brackets(&mut self, tournament_id: TournamentId) -> Result<()> {
        let tournament = self.tournaments.get_mut(&tournament_id)
            .ok_or_else(|| anyhow::anyhow!("Tournament not found"))?;

        match tournament.tournament_type {
            TournamentType::SingleElimination => {
                self.generate_single_elimination_bracket(tournament)?;
            },
            TournamentType::DoubleElimination => {
                self.generate_double_elimination_bracket(tournament)?;
            },
            TournamentType::RoundRobin => {
                self.generate_round_robin_bracket(tournament)?;
            },
            _ => {
                return Err(anyhow::anyhow!("Tournament type not yet implemented"));
            }
        }

        Ok(())
    }

    /// Generate single elimination bracket
    fn generate_single_elimination_bracket(&mut self, tournament: &mut Tournament) -> Result<()> {
        let participants: Vec<PlayerId> = tournament.participants.keys().cloned().collect();
        let participant_count = participants.len();
        
        // Calculate rounds needed
        let rounds_needed = (participant_count as f64).log2().ceil() as u32;
        
        let mut bracket = TournamentBracket {
            bracket_id: "main".to_string(),
            bracket_type: BracketType::Main,
            rounds: vec![],
            participants: participants.clone(),
            seeding_method: SeedingMethod::Rating,
            advancement_rules: AdvancementRules {
                top_finishers: 1,
                win_requirement: Some(1),
                score_threshold: None,
                tiebreaker_method: TiebreakerMethod::HeadToHead,
                wildcard_slots: 0,
            },
            current_round: 1,
            bracket_status: BracketStatus::NotStarted,
        };

        // Create rounds
        for round_number in 1..=rounds_needed {
            let matches_in_round = (participant_count >> (round_number - 1)).max(1);
            let round = TournamentRound {
                round_number,
                round_name: match round_number {
                    1 if rounds_needed == 1 => "Finals".to_string(),
                    n if n == rounds_needed => "Finals".to_string(),
                    n if n == rounds_needed - 1 => "Semifinals".to_string(),
                    n if n == rounds_needed - 2 => "Quarterfinals".to_string(),
                    n => format!("Round {}", n),
                },
                matches: vec![], // Will be populated with actual match IDs
                scheduled_start: tournament.schedule.start_time + Duration::hours((round_number - 1) as i64),
                estimated_duration: Duration::hours(1),
                status: RoundStatus::Scheduled,
                advancement_count: matches_in_round as u32 / 2,
            };
            bracket.rounds.push(round);
        }

        tournament.brackets.insert("main".to_string(), bracket);
        Ok(())
    }

    /// Generate double elimination bracket
    fn generate_double_elimination_bracket(&mut self, tournament: &mut Tournament) -> Result<()> {
        // Winner's bracket
        self.generate_single_elimination_bracket(tournament)?;
        
        // Losers bracket (simplified)
        let participants: Vec<PlayerId> = tournament.participants.keys().cloned().collect();
        let losers_bracket = TournamentBracket {
            bracket_id: "losers".to_string(),
            bracket_type: BracketType::Losers,
            rounds: vec![],
            participants: vec![], // Populated as players are eliminated
            seeding_method: SeedingMethod::Rating,
            advancement_rules: AdvancementRules {
                top_finishers: 1,
                win_requirement: Some(1),
                score_threshold: None,
                tiebreaker_method: TiebreakerMethod::HeadToHead,
                wildcard_slots: 0,
            },
            current_round: 1,
            bracket_status: BracketStatus::NotStarted,
        };

        tournament.brackets.insert("losers".to_string(), losers_bracket);
        Ok(())
    }

    /// Generate round robin bracket
    fn generate_round_robin_bracket(&mut self, tournament: &mut Tournament) -> Result<()> {
        let participants: Vec<PlayerId> = tournament.participants.keys().cloned().collect();
        let participant_count = participants.len();
        
        // In round robin, each player plays every other player once
        let total_rounds = participant_count - 1;
        let matches_per_round = participant_count / 2;

        let bracket = TournamentBracket {
            bracket_id: "round_robin".to_string(),
            bracket_type: BracketType::Main,
            rounds: (1..=total_rounds).map(|round_number| {
                TournamentRound {
                    round_number: round_number as u32,
                    round_name: format!("Round {}", round_number),
                    matches: vec![],
                    scheduled_start: tournament.schedule.start_time + Duration::hours((round_number - 1) as i64),
                    estimated_duration: Duration::hours(1),
                    status: RoundStatus::Scheduled,
                    advancement_count: 0, // All advance in round robin
                }
            }).collect(),
            participants,
            seeding_method: SeedingMethod::Registration,
            advancement_rules: AdvancementRules {
                top_finishers: 4, // Top 4 might advance to playoffs
                win_requirement: None,
                score_threshold: None,
                tiebreaker_method: TiebreakerMethod::Multiple(vec![
                    TiebreakerMethod::HeadToHead,
                    TiebreakerMethod::OverallRecord,
                    TiebreakerMethod::ScoreDifferential,
                ]),
                wildcard_slots: 0,
            },
            current_round: 1,
            bracket_status: BracketStatus::NotStarted,
        };

        tournament.brackets.insert("round_robin".to_string(), bracket);
        Ok(())
    }

    /// Schedule next round of matches
    fn schedule_next_round(&mut self, tournament_id: TournamentId) -> Result<()> {
        // In real implementation, would create actual match instances
        tracing::info!("Scheduling next round for tournament {}", tournament_id);
        Ok(())
    }

    /// Complete tournament match
    pub fn complete_match(&mut self, tournament_id: TournamentId, match_id: MatchId, 
                         result: TournamentMatchResult) -> Result<()> {
        let tournament = self.tournaments.get_mut(&tournament_id)
            .ok_or_else(|| anyhow::anyhow!("Tournament not found"))?;

        if let Some(tournament_match) = tournament.matches.get_mut(&match_id) {
            tournament_match.status = TournamentMatchStatus::Completed;
            tournament_match.result = Some(result.clone());

            // Update participant performance
            if let Some(winner) = result.winner {
                if let Some(participant) = tournament.participants.get_mut(&winner) {
                    participant.performance.wins += 1;
                    participant.match_history.push(match_id);
                }
            }

            // Check if round is complete and advance bracket
            self.check_round_completion(tournament_id, tournament_match.bracket_id.clone())?;
        }

        Ok(())
    }

    /// Check if round is complete and advance
    fn check_round_completion(&mut self, tournament_id: TournamentId, bracket_id: String) -> Result<()> {
        // In real implementation, would check if all matches in round are complete
        // and advance winners to next round
        tracing::info!("Checking round completion for tournament {} bracket {}", tournament_id, bracket_id);
        Ok(())
    }

    /// Get tournament by ID
    pub fn get_tournament(&self, tournament_id: TournamentId) -> Option<&Tournament> {
        self.tournaments.get(&tournament_id)
    }

    /// Get upcoming tournaments
    pub fn get_upcoming_tournaments(&self, limit: Option<usize>) -> Vec<&Tournament> {
        let mut upcoming: Vec<&Tournament> = self.tournaments.values()
            .filter(|t| t.schedule.start_time > Utc::now())
            .collect();
        
        upcoming.sort_by(|a, b| a.schedule.start_time.cmp(&b.schedule.start_time));
        
        if let Some(limit) = limit {
            upcoming.into_iter().take(limit).collect()
        } else {
            upcoming
        }
    }

    /// Get player's tournament history
    pub fn get_player_tournament_history(&self, player_id: PlayerId) -> Vec<&TournamentResult> {
        self.ranking_system.tournament_history.get(&player_id)
            .map(|results| results.iter().collect())
            .unwrap_or_default()
    }
}

// Configuration struct for tournament creation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TournamentConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub prize_pool: Option<u64>,
    pub entry_fee: Option<u64>,
    pub max_participants: Option<u32>,
    pub start_time: Option<DateTime<Utc>>,
}

impl Default for ParticipantPerformance {
    fn default() -> Self {
        Self {
            wins: 0,
            losses: 0,
            draws: 0,
            total_score: 0,
            average_performance: 0.0,
            best_performance: 0.0,
            consistency_rating: 0.0,
        }
    }
}

impl TournamentSettings {
    fn default_competitive() -> Self {
        Self {
            public_visibility: true,
            spectating_allowed: true,
            live_streaming: true,
            recording_enabled: true,
            chat_enabled: true,
            pause_allowed: true,
            coaching_allowed: false,
            substitution_allowed: false,
            rematch_policy: RematchPolicy {
                allowed: false,
                conditions: vec![],
                time_limit: None,
                approval_required: false,
            },
            technical_pause_limit: 2,
        }
    }

    fn default_team() -> Self {
        let mut settings = Self::default_competitive();
        settings.coaching_allowed = true;
        settings.substitution_allowed = true;
        settings
    }

    fn default_casual() -> Self {
        Self {
            public_visibility: true,
            spectating_allowed: true,
            live_streaming: false,
            recording_enabled: false,
            chat_enabled: true,
            pause_allowed: true,
            coaching_allowed: true,
            substitution_allowed: true,
            rematch_policy: RematchPolicy {
                allowed: true,
                conditions: vec!["Technical issues".to_string()],
                time_limit: Some(Duration::minutes(10)),
                approval_required: true,
            },
            technical_pause_limit: 1,
        }
    }
}

impl TournamentRules {
    fn default_competitive() -> Self {
        Self {
            ruleset_version: "1.0".to_string(),
            general_rules: vec![
                "No cheating or exploitation of game mechanics".to_string(),
                "Respectful conduct required at all times".to_string(),
                "Players must compete to the best of their ability".to_string(),
            ],
            conduct_rules: vec![
                "No harassment or discrimination".to_string(),
                "No unsporting behavior".to_string(),
                "Follow all tournament official instructions".to_string(),
            ],
            technical_rules: vec![
                "Stable internet connection required".to_string(),
                "No third-party software that provides unfair advantage".to_string(),
                "Report technical issues immediately".to_string(),
            ],
            dispute_resolution: DisputeResolution {
                dispute_window: Duration::minutes(30),
                escalation_process: vec![],
                evidence_requirements: vec!["Screenshots or video proof required".to_string()],
                admin_authority: vec!["Tournament admin decisions are final".to_string()],
            },
            penalties: vec![],
            equipment_restrictions: vec![],
            external_software_policy: "Prohibited unless explicitly allowed".to_string(),
        }
    }

    fn default_team() -> Self {
        let mut rules = Self::default_competitive();
        rules.general_rules.push("Team captains are responsible for their team's conduct".to_string());
        rules
    }

    fn default_casual() -> Self {
        let mut rules = Self::default_competitive();
        rules.general_rules = vec![
            "Have fun and enjoy the competition".to_string(),
            "Be respectful to all participants".to_string(),
        ];
        rules
    }
}

impl Default for TournamentStatistics {
    fn default() -> Self {
        Self {
            total_participants: 0,
            total_matches: 0,
            completed_matches: 0,
            average_match_duration: Duration::minutes(15),
            viewership_stats: ViewershipStats {
                peak_concurrent_viewers: 0,
                total_view_time: Duration::zero(),
                average_viewers: 0.0,
                viewer_retention_rate: 0.0,
            },
            engagement_metrics: EngagementMetrics {
                chat_messages: 0,
                reactions: 0,
                social_shares: 0,
                participant_satisfaction: 0.0,
            },
            prize_distribution_status: PrizeDistributionStatus {
                distributed_amount: 0,
                pending_amount: 0,
                distribution_timeline: vec![],
            },
        }
    }
}

impl Default for TournamentCalendar {
    fn default() -> Self {
        Self {
            upcoming_tournaments: vec![],
            featured_tournaments: vec![],
            seasonal_tournaments: HashMap::new(),
            recurring_tournaments: HashMap::new(),
        }
    }
}

impl Default for TournamentRankingSystem {
    fn default() -> Self {
        Self {
            global_rankings: HashMap::new(),
            seasonal_rankings: HashMap::new(),
            tournament_history: HashMap::new(),
            rating_calculations: RatingCalculationRules {
                base_rating: 1000,
                placement_multipliers: [
                    (1, 2.0),
                    (2, 1.5),
                    (3, 1.2),
                    (4, 1.0),
                ].into(),
                tournament_size_bonus: 0.1,
                consistency_weight: 0.3,
                decay_rate: 0.02,
                seasonal_reset_percentage: 0.2,
            },
        }
    }
}

impl Default for TournamentCoordinator {
    fn default() -> Self {
        Self::new()
    }
}