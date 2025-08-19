use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;
use std::sync::Arc;

/// Guild management system for large-scale organization (up to 40 players)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildManager {
    pub guilds: HashMap<Uuid, Guild>,
    pub player_guilds: HashMap<Uuid, Uuid>, // player_id -> guild_id
    pub guild_invitations: HashMap<Uuid, Vec<GuildInvitation>>, // player_id -> invitations
    pub guild_alliances: HashMap<Uuid, Vec<GuildAlliance>>, // guild_id -> alliances
    pub config: GuildConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub guild_id: Uuid,
    pub name: String,
    pub tag: String, // Short guild identifier (3-6 characters)
    pub description: String,
    pub guild_master_id: Uuid,
    pub members: Vec<GuildMember>,
    pub created_at: SystemTime,
    pub guild_type: GuildType,
    pub status: GuildStatus,
    pub settings: GuildSettings,
    pub hierarchy: GuildHierarchy,
    pub resources: GuildResources,
    pub activities: GuildActivities,
    pub diplomacy: GuildDiplomacy,
    pub channels: Vec<GuildChannel>,
    pub bank: GuildBank,
    pub territories: Vec<GuildTerritory>,
    pub reputation: GuildReputation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMember {
    pub player_id: Uuid,
    pub username: String,
    pub rank: GuildRank,
    pub joined_at: SystemTime,
    pub last_active: SystemTime,
    pub status: MemberStatus,
    pub contributions: GuildContributions,
    pub permissions: Vec<GuildPermission>,
    pub notes: String, // Officer notes about the member
    pub mentor_id: Option<Uuid>, // For new member mentoring
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GuildRank {
    GuildMaster,    // Rank 1 - Ultimate authority (1 person)
    Officer,        // Rank 2 - High-level management (up to 5)
    Lieutenant,     // Rank 3 - Mid-level leadership (up to 10)
    Veteran,        // Rank 4 - Experienced members
    Member,         // Rank 5 - Regular members
    Initiate,       // Rank 6 - New/trial members
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum GuildType {
    Military,       // Combat and warfare focused
    Trading,        // Economic and merchant activities
    Crafting,       // Production and manufacturing
    Exploration,    // Discovery and mapping
    Social,         // Community and events
    Research,       // Knowledge and development
    Mercenary,      // Hired services
    Religious,      // Spiritual and ceremonial
    Political,      // Governance and diplomacy
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GuildStatus {
    Active,         // Normal operations
    Recruiting,     // Actively seeking members
    Closed,         // Not accepting new members
    AtWar,          // Currently in conflict
    Disbanded,      // Guild dissolved
    Sanctioned,     // Under penalties
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MemberStatus {
    Online,
    Offline,
    Away,
    Busy,
    OnMission,
    InCombat,
    Trading,
    Crafting,
    Inactive, // Long-term absence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSettings {
    pub is_public: bool,
    pub recruitment_open: bool,
    pub auto_accept_invites: bool,
    pub max_members: u8,
    pub min_level_requirement: Option<u32>,
    pub application_required: bool,
    pub voice_chat_enabled: bool,
    pub timezone: String,
    pub primary_language: String,
    pub activity_focus: Vec<GuildType>,
    pub region_restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildHierarchy {
    pub ranks: HashMap<GuildRank, RankDefinition>,
    pub succession_plan: SuccessionPlan,
    pub leadership_council: Vec<Uuid>, // Officers involved in decisions
    pub rank_requirements: HashMap<GuildRank, RankRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankDefinition {
    pub rank: GuildRank,
    pub name: String, // Custom rank name
    pub description: String,
    pub permissions: Vec<GuildPermission>,
    pub max_count: Option<u8>, // Maximum members at this rank
    pub promotion_requirements: PromotionRequirements,
    pub insignia_color: String, // Visual identifier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessionPlan {
    pub succession_type: SuccessionType,
    pub designated_heir: Option<Uuid>,
    pub succession_council: Vec<Uuid>, // Who decides succession
    pub emergency_contacts: Vec<Uuid>,
    pub inactive_threshold_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuccessionType {
    Designated,     // Guild master chooses heir
    Election,       // Officers vote
    Council,        // Leadership council decides
    Automatic,      // Highest ranking officer
    Application,    // Open applications with vote
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankRequirements {
    pub min_time_in_guild: u64, // days
    pub min_contribution_points: u64,
    pub required_skills: Vec<String>,
    pub endorsements_required: u8,
    pub trial_period_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionRequirements {
    pub min_time_in_rank: u64, // days
    pub contribution_threshold: u64,
    pub skill_requirements: Vec<String>,
    pub approval_required_from: Vec<GuildRank>,
    pub trial_period: Option<u32>, // days
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GuildPermission {
    // Management
    InviteMembers,
    KickMembers,
    PromoteMembers,
    DemoteMembers,
    EditGuildInfo,
    ManageRanks,
    
    // Communication
    UseGuildChat,
    UseOfficerChat,
    CreateChannels,
    ManageChannels,
    ModerateChat,
    MakeAnnouncements,
    
    // Resources
    AccessGuildBank,
    WithdrawFromBank,
    DepositToBank,
    ManageBankPermissions,
    ViewBankLogs,
    
    // Territory
    ClaimTerritory,
    ManageTerritory,
    DefendTerritory,
    UpgradeStructures,
    
    // Diplomacy
    ManageAlliances,
    DeclareWar,
    NegotiatePeace,
    TradeAgreements,
    
    // Events
    ScheduleEvents,
    CreateMissions,
    LeadRaids,
    OrganizeActivities,
    
    // Administration
    ViewMemberList,
    ViewActivityLogs,
    ManageApplications,
    SetMemberNotes,
    AccessRecruitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildResources {
    pub treasury: HashMap<String, f64>, // currency_type -> amount
    pub materials: HashMap<String, u32>, // material_type -> quantity
    pub influence_points: u64,
    pub research_points: u64,
    pub military_power: u64,
    pub territory_control: f64, // percentage of claimed territories under control
    pub weekly_income: HashMap<String, f64>,
    pub maintenance_costs: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildActivities {
    pub current_campaigns: Vec<GuildCampaign>,
    pub scheduled_events: Vec<GuildEvent>,
    pub completed_missions: Vec<CompletedMission>,
    pub active_projects: Vec<GuildProject>,
    pub activity_calendar: HashMap<String, Vec<CalendarEvent>>, // date -> events
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildCampaign {
    pub campaign_id: Uuid,
    pub name: String,
    pub description: String,
    pub campaign_type: CampaignType,
    pub leader_id: Uuid,
    pub participants: Vec<Uuid>,
    pub started_at: SystemTime,
    pub estimated_duration: u64, // hours
    pub objectives: Vec<CampaignObjective>,
    pub rewards: Vec<CampaignReward>,
    pub status: CampaignStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CampaignType {
    Military,       // Combat operations
    Economic,       // Trading ventures
    Exploration,    // Discovery missions
    Construction,   // Building projects
    Diplomatic,     // Negotiations
    Research,       // Knowledge gathering
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CampaignStatus {
    Planning,
    Active,
    OnHold,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignObjective {
    pub objective_id: Uuid,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub target_value: f64,
    pub current_progress: f64,
    pub assigned_to: Vec<Uuid>,
    pub deadline: Option<SystemTime>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    Capture,        // Territory or resource
    Defend,         // Protect something
    Collect,        // Gather materials/currency
    Eliminate,      // Remove threats
    Build,          // Construct something
    Research,       // Discover knowledge
    Negotiate,      // Diplomatic goals
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignReward {
    pub reward_type: String,
    pub reward_value: f64,
    pub distribution_method: RewardDistribution,
    pub eligibility_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardDistribution {
    EqualSplit,         // All participants get equal share
    ContributionBased,  // Based on participation level
    RankBased,          // Higher ranks get more
    ObjectiveBased,     // Based on objective completion
    LeaderDecision,     // Campaign leader decides
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEvent {
    pub event_id: Uuid,
    pub name: String,
    pub description: String,
    pub event_type: EventType,
    pub organizer_id: Uuid,
    pub scheduled_time: SystemTime,
    pub duration_hours: u32,
    pub participants: Vec<EventParticipant>,
    pub requirements: EventRequirements,
    pub rewards: Vec<EventReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Raid,           // Large-scale combat
    Tournament,     // Competitive event
    Training,       // Skill development
    Social,         // Community gathering
    Ceremony,       // Formal occasion
    Meeting,        // Planning session
    Festival,       // Celebration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParticipant {
    pub player_id: Uuid,
    pub signup_time: SystemTime,
    pub role: EventRole,
    pub attendance_confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventRole {
    Leader,
    CoLeader,
    Participant,
    Reserve,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRequirements {
    pub min_rank: Option<GuildRank>,
    pub min_level: Option<u32>,
    pub required_skills: Vec<String>,
    pub equipment_requirements: Vec<String>,
    pub max_participants: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReward {
    pub reward_type: String,
    pub reward_amount: f64,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedMission {
    pub mission_id: Uuid,
    pub name: String,
    pub completed_at: SystemTime,
    pub participants: Vec<Uuid>,
    pub success_level: SuccessLevel,
    pub rewards_distributed: Vec<MissionReward>,
    pub experience_gained: HashMap<Uuid, u64>, // player_id -> xp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuccessLevel {
    Failed,
    PartialSuccess,
    Success,
    GreatSuccess,
    CriticalSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionReward {
    pub recipient_id: Option<Uuid>, // None = guild-wide
    pub reward_type: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildProject {
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
    pub project_type: ProjectType,
    pub manager_id: Uuid,
    pub started_at: SystemTime,
    pub estimated_completion: SystemTime,
    pub progress_percentage: f64,
    pub required_resources: HashMap<String, f64>,
    pub contributed_resources: HashMap<String, f64>,
    pub contributors: HashMap<Uuid, ProjectContribution>,
    pub milestones: Vec<ProjectMilestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    Infrastructure,     // Guild hall upgrades
    Research,          // Technology development
    Military,          // Weapons/defenses
    Economic,          // Income generation
    Social,            // Community features
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContribution {
    pub contributor_id: Uuid,
    pub contributions: HashMap<String, f64>, // resource_type -> amount
    pub time_contributed: u64, // hours
    pub leadership_role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMilestone {
    pub milestone_id: Uuid,
    pub name: String,
    pub description: String,
    pub target_progress: f64,
    pub completed: bool,
    pub completed_at: Option<SystemTime>,
    pub rewards: Vec<MilestoneReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneReward {
    pub reward_type: String,
    pub reward_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub event_id: Uuid,
    pub name: String,
    pub time: SystemTime,
    pub event_type: EventType,
    pub organizer_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildDiplomacy {
    pub alliances: Vec<GuildAlliance>,
    pub wars: Vec<GuildWar>,
    pub trade_agreements: Vec<TradeAgreement>,
    pub reputation_standings: HashMap<Uuid, i32>, // guild_id -> reputation
    pub diplomatic_missions: Vec<DiplomaticMission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildAlliance {
    pub alliance_id: Uuid,
    pub partner_guild_id: Uuid,
    pub alliance_type: AllianceType,
    pub terms: AllianceTerms,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub status: AllianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllianceType {
    NonAggression,      // Won't attack each other
    Trade,              // Economic cooperation
    Military,           // Mutual defense
    Research,           // Knowledge sharing
    Full,               // Complete alliance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceTerms {
    pub mutual_defense: bool,
    pub resource_sharing: bool,
    pub information_sharing: bool,
    pub joint_operations: bool,
    pub trade_bonuses: bool,
    pub territory_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AllianceStatus {
    Proposed,
    Active,
    Suspended,
    Terminated,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildWar {
    pub war_id: Uuid,
    pub enemy_guild_id: Uuid,
    pub declared_at: SystemTime,
    pub war_type: WarType,
    pub objectives: Vec<WarObjective>,
    pub status: WarStatus,
    pub peace_terms: Option<PeaceTerms>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarType {
    Territorial,        // Over land/resources
    Economic,           // Trade war
    Ideological,        // Conflicting beliefs
    Revenge,            // Retaliation
    Total,              // Complete destruction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarStatus {
    Active,
    Ceasefire,
    Negotiating,
    Concluded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarObjective {
    pub objective_id: Uuid,
    pub description: String,
    pub target_guild_id: Uuid,
    pub completed: bool,
    pub completion_date: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeaceTerms {
    pub terms: Vec<String>,
    pub reparations: HashMap<String, f64>,
    pub territory_changes: Vec<TerritoryChange>,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryChange {
    pub territory_id: Uuid,
    pub from_guild_id: Uuid,
    pub to_guild_id: Uuid,
    pub transfer_type: TransferType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferType {
    Permanent,
    Temporary,
    Shared,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAgreement {
    pub agreement_id: Uuid,
    pub partner_guild_id: Uuid,
    pub terms: TradeTerms,
    pub duration: u64, // days
    pub signed_at: SystemTime,
    pub status: AgreementStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeTerms {
    pub tariff_reductions: HashMap<String, f64>, // item_type -> reduction %
    pub exclusive_access: Vec<String>,
    pub bulk_discounts: HashMap<String, f64>,
    pub shipping_benefits: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgreementStatus {
    Active,
    Suspended,
    Expired,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticMission {
    pub mission_id: Uuid,
    pub target_guild_id: Uuid,
    pub mission_type: DiplomaticMissionType,
    pub ambassador_id: Uuid,
    pub objectives: Vec<String>,
    pub started_at: SystemTime,
    pub status: MissionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiplomaticMissionType {
    Negotiation,
    Intelligence,
    TradeProposal,
    AllianceOffer,
    WarningDelivery,
    PeaceOffering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissionStatus {
    InProgress,
    Successful,
    Failed,
    Recalled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildChannel {
    pub channel_id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub description: String,
    pub access_level: AccessLevel,
    pub messages: Vec<GuildMessage>,
    pub message_limit: usize,
    pub created_by: Uuid,
    pub created_at: SystemTime,
    pub settings: ChannelSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    General,        // Open guild discussion
    Officers,       // Leadership only
    Announcements,  // Official communications
    Trading,        // Economic discussions
    Military,       // Combat planning
    Social,         // Casual chat
    Projects,       // Work coordination
    Diplomacy,      // External relations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessLevel {
    Everyone,       // All guild members
    Veterans,       // Veteran rank and above
    Leadership,     // Lieutenant rank and above
    Officers,       // Officer rank and above
    GuildMaster,    // Guild master only
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMessage {
    pub message_id: Uuid,
    pub sender_id: Uuid,
    pub sender_username: String,
    pub sender_rank: GuildRank,
    pub content: String,
    pub timestamp: SystemTime,
    pub message_type: MessageType,
    pub mentions: Vec<Uuid>,
    pub attachments: Vec<MessageAttachment>,
    pub edited: bool,
    pub edited_at: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    System,
    Announcement,
    Event,
    Mission,
    Promotion,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    pub attachment_id: Uuid,
    pub attachment_type: AttachmentType,
    pub filename: String,
    pub content_hash: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttachmentType {
    Image,
    Document,
    Map,
    Blueprint,
    Contract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSettings {
    pub slow_mode_seconds: Option<u32>,
    pub max_message_length: u32,
    pub allow_attachments: bool,
    pub auto_delete_after_days: Option<u32>,
    pub moderation_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildBank {
    pub treasury: HashMap<String, f64>, // currency -> amount
    pub vaults: Vec<BankVault>,
    pub transaction_log: Vec<BankTransaction>,
    pub access_permissions: HashMap<GuildRank, BankPermissions>,
    pub daily_limits: HashMap<Uuid, DailyLimit>, // player_id -> limits
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankVault {
    pub vault_id: Uuid,
    pub name: String,
    pub vault_type: VaultType,
    pub contents: HashMap<String, u32>, // item_id -> quantity
    pub access_level: AccessLevel,
    pub capacity: u32,
    pub security_level: u8, // 1-10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VaultType {
    Treasury,       // Currency storage
    Equipment,      // Weapons/armor
    Materials,      // Crafting resources
    Consumables,    // Potions/food
    Artifacts,      // Rare items
    Documents,      // Contracts/maps
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    pub transaction_id: Uuid,
    pub player_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub currency_or_item: String,
    pub timestamp: SystemTime,
    pub authorized_by: Uuid,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Payment,
    Fine,
    Reward,
    Fee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankPermissions {
    pub can_view_balance: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub can_transfer: bool,
    pub withdrawal_limit: Option<f64>,
    pub requires_approval: bool,
    pub can_approve_transactions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLimit {
    pub currency_limits: HashMap<String, f64>,
    pub reset_time: SystemTime,
    pub current_usage: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTerritory {
    pub territory_id: Uuid,
    pub name: String,
    pub territory_type: TerritoryType,
    pub coordinates: TerritoryCoordinates,
    pub size_hectares: f64,
    pub claimed_at: SystemTime,
    pub control_level: f64, // 0.0 - 1.0
    pub structures: Vec<TerritoryStructure>,
    pub resources: TerritoryResources,
    pub defense_level: u8,
    pub garrison: Vec<Uuid>, // player_ids of defenders
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerritoryType {
    GuildHall,      // Main base
    Outpost,        // Secondary base
    Mine,           // Resource extraction
    Farm,           // Food production
    Trading,        // Commercial hub
    Fortress,       // Military stronghold
    Research,       // Knowledge center
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryCoordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub world_region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryStructure {
    pub structure_id: Uuid,
    pub structure_type: StructureType,
    pub name: String,
    pub level: u8,
    pub health: f64,
    pub max_health: f64,
    pub built_at: SystemTime,
    pub last_upgraded: Option<SystemTime>,
    pub maintenance_cost: f64,
    pub benefits: Vec<StructureBenefit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureType {
    Hall,           // Main building
    Barracks,       // Military training
    Workshop,       // Crafting facility
    Warehouse,      // Storage
    Tower,          // Defense
    Market,         // Trading
    Library,        // Research
    Temple,         // Spiritual
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureBenefit {
    pub benefit_type: String,
    pub benefit_value: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryResources {
    pub resource_nodes: Vec<ResourceNode>,
    pub daily_production: HashMap<String, f64>,
    pub storage_capacity: HashMap<String, f64>,
    pub current_stockpile: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub node_id: Uuid,
    pub resource_type: String,
    pub quality: ResourceQuality,
    pub extraction_rate: f64,
    pub remaining_quantity: f64,
    pub requires_workers: u8,
    pub assigned_workers: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceQuality {
    Poor,
    Common,
    Good,
    Excellent,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildReputation {
    pub overall_reputation: i32,
    pub faction_standings: HashMap<String, i32>, // faction_name -> standing
    pub reputation_history: Vec<ReputationEvent>,
    pub achievements: Vec<GuildAchievement>,
    pub titles: Vec<GuildTitle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub event_id: Uuid,
    pub event_type: ReputationEventType,
    pub reputation_change: i32,
    pub description: String,
    pub timestamp: SystemTime,
    pub involved_players: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReputationEventType {
    MissionSuccess,
    MissionFailure,
    DiplomaticVictory,
    WarVictory,
    WarDefeat,
    TradeSuccess,
    CharitableAct,
    CriminalActivity,
    AllianceFormed,
    AllianceBetrayed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildAchievement {
    pub achievement_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub rarity: AchievementRarity,
    pub unlocked_at: SystemTime,
    pub contributors: Vec<Uuid>,
    pub rewards: Vec<AchievementReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCategory {
    Military,
    Economic,
    Social,
    Exploration,
    Diplomatic,
    Construction,
    Research,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementReward {
    pub reward_type: String,
    pub reward_value: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTitle {
    pub title_id: Uuid,
    pub name: String,
    pub description: String,
    pub earned_at: SystemTime,
    pub requirements_met: Vec<String>,
    pub benefits: Vec<TitleBenefit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleBenefit {
    pub benefit_type: String,
    pub benefit_value: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildContributions {
    pub total_contribution_points: u64,
    pub resource_contributions: HashMap<String, f64>,
    pub time_contributed: u64, // hours
    pub missions_completed: u32,
    pub events_organized: u32,
    pub members_recruited: u32,
    pub leadership_time: u64, // hours in leadership roles
    pub special_achievements: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildInvitation {
    pub invitation_id: Uuid,
    pub guild_id: Uuid,
    pub guild_name: String,
    pub guild_tag: String,
    pub inviter_id: Uuid,
    pub inviter_username: String,
    pub inviter_rank: GuildRank,
    pub invited_player_id: Uuid,
    pub message: Option<String>,
    pub sent_at: SystemTime,
    pub expires_at: SystemTime,
    pub status: InvitationStatus,
    pub proposed_rank: GuildRank,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
    Cancelled,
    ApplicationPending, // For application-based recruitment
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub max_guild_size: u8,
    pub max_guilds_per_player: u8,
    pub invitation_timeout_hours: u64,
    pub max_chat_history_per_channel: usize,
    pub auto_disband_inactive_days: u64,
    pub min_members_to_prevent_disband: u8,
    pub max_officers: u8,
    pub max_lieutenants: u8,
    pub succession_timeout_days: u32,
    pub territory_claim_cooldown_hours: u64,
}

// Request/Response structures for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGuildRequest {
    pub name: String,
    pub tag: String,
    pub description: String,
    pub guild_type: GuildType,
    pub is_public: bool,
    pub recruitment_open: bool,
    pub application_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGuildResponse {
    pub success: bool,
    pub guild_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteToGuildRequest {
    pub guild_id: Uuid,
    pub player_username: String,
    pub message: Option<String>,
    pub proposed_rank: Option<GuildRank>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteToGuildResponse {
    pub success: bool,
    pub invitation_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGuildRequest {
    pub invitation_id: Uuid,
    pub accept: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGuildResponse {
    pub success: bool,
    pub guild_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildStatusResponse {
    pub success: bool,
    pub guild: Option<GuildSummary>,
    pub invitations: Vec<GuildInvitation>,
    pub available_guilds: Vec<PublicGuildInfo>, // For browsing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSummary {
    pub guild_id: Uuid,
    pub name: String,
    pub tag: String,
    pub guild_type: GuildType,
    pub member_count: u8,
    pub max_members: u8,
    pub status: GuildStatus,
    pub player_rank: GuildRank,
    pub joined_at: SystemTime,
    pub last_activity: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicGuildInfo {
    pub guild_id: Uuid,
    pub name: String,
    pub tag: String,
    pub description: String,
    pub guild_type: GuildType,
    pub member_count: u8,
    pub max_members: u8,
    pub recruitment_open: bool,
    pub application_required: bool,
    pub reputation: i32,
    pub founded_date: SystemTime,
}

impl GuildManager {
    pub fn new() -> Self {
        Self {
            guilds: HashMap::new(),
            player_guilds: HashMap::new(),
            guild_invitations: HashMap::new(),
            guild_alliances: HashMap::new(),
            config: GuildConfig::default(),
        }
    }

    /// Create a new guild
    pub async fn create_guild(&mut self, founder_id: Uuid, founder_username: String, request: CreateGuildRequest) -> Result<CreateGuildResponse> {
        info!("🏰 Creating guild '{}' [{}] with founder: {}", request.name, request.tag, founder_username);

        // Check if player is already in a guild
        if self.player_guilds.contains_key(&founder_id) {
            return Ok(CreateGuildResponse {
                success: false,
                guild_id: None,
                message: "You are already a member of a guild".to_string(),
            });
        }

        // Validate guild tag uniqueness
        if self.is_guild_tag_taken(&request.tag) {
            return Ok(CreateGuildResponse {
                success: false,
                guild_id: None,
                message: "Guild tag is already taken".to_string(),
            });
        }

        // Validate guild name uniqueness
        if self.is_guild_name_taken(&request.name) {
            return Ok(CreateGuildResponse {
                success: false,
                guild_id: None,
                message: "Guild name is already taken".to_string(),
            });
        }

        let guild_id = Uuid::new_v4();

        // Create founder as guild master
        let founder_member = GuildMember {
            player_id: founder_id,
            username: founder_username.clone(),
            rank: GuildRank::GuildMaster,
            joined_at: SystemTime::now(),
            last_active: SystemTime::now(),
            status: MemberStatus::Online,
            contributions: GuildContributions::default(),
            permissions: self.get_rank_permissions(&GuildRank::GuildMaster),
            notes: "Guild Founder".to_string(),
            mentor_id: None,
        };

        // Create guild
        let guild = Guild {
            guild_id,
            name: request.name.clone(),
            tag: request.tag.clone(),
            description: request.description,
            guild_master_id: founder_id,
            members: vec![founder_member],
            created_at: SystemTime::now(),
            guild_type: request.guild_type,
            status: if request.recruitment_open { GuildStatus::Recruiting } else { GuildStatus::Active },
            settings: GuildSettings {
                is_public: request.is_public,
                recruitment_open: request.recruitment_open,
                auto_accept_invites: false,
                max_members: self.config.max_guild_size,
                min_level_requirement: None,
                application_required: request.application_required,
                voice_chat_enabled: false,
                timezone: "UTC".to_string(),
                primary_language: "English".to_string(),
                activity_focus: vec![request.guild_type.clone()],
                region_restrictions: vec![],
            },
            hierarchy: GuildHierarchy::default(),
            resources: GuildResources::default(),
            activities: GuildActivities::default(),
            diplomacy: GuildDiplomacy::default(),
            channels: self.create_default_channels(founder_id),
            bank: GuildBank::default(),
            territories: vec![],
            reputation: GuildReputation::default(),
        };

        // Store guild and update mappings
        self.guilds.insert(guild_id, guild);
        self.player_guilds.insert(founder_id, guild_id);

        info!("✅ Guild created successfully: {} [{}] (ID: {})", request.name, request.tag, guild_id);

        Ok(CreateGuildResponse {
            success: true,
            guild_id: Some(guild_id),
            message: format!("Guild '{}' [{}] created successfully!", request.name, request.tag),
        })
    }

    /// Invite a player to a guild
    pub async fn invite_to_guild(&mut self, inviter_id: Uuid, target_player_id: Uuid, target_username: String, request: InviteToGuildRequest) -> Result<InviteToGuildResponse> {
        info!("📨 Guild invitation: {} to guild {}", target_username, request.guild_id);

        // Check if guild exists and inviter has permission
        let guild = match self.guilds.get(&request.guild_id) {
            Some(guild) => guild.clone(),
            None => return Ok(InviteToGuildResponse {
                success: false,
                invitation_id: None,
                message: "Guild not found".to_string(),
            }),
        };

        // Check inviter permissions
        let inviter_member = guild.members.iter().find(|m| m.player_id == inviter_id);
        match inviter_member {
            Some(member) => {
                if !member.permissions.contains(&GuildPermission::InviteMembers) {
                    return Ok(InviteToGuildResponse {
                        success: false,
                        invitation_id: None,
                        message: "You don't have permission to invite members".to_string(),
                    });
                }
            },
            None => return Ok(InviteToGuildResponse {
                success: false,
                invitation_id: None,
                message: "You are not a member of this guild".to_string(),
            }),
        }

        // Check if guild is full
        if guild.members.len() >= guild.settings.max_members as usize {
            return Ok(InviteToGuildResponse {
                success: false,
                invitation_id: None,
                message: "Guild is full".to_string(),
            });
        }

        // Check if player is already in this guild
        if guild.members.iter().any(|m| m.player_id == target_player_id) {
            return Ok(InviteToGuildResponse {
                success: false,
                invitation_id: None,
                message: "Player is already in this guild".to_string(),
            });
        }

        // Check if player is already in another guild
        if self.player_guilds.contains_key(&target_player_id) {
            return Ok(InviteToGuildResponse {
                success: false,
                invitation_id: None,
                message: "Player is already in another guild".to_string(),
            });
        }

        // Check for existing pending invitation
        if let Some(invitations) = self.guild_invitations.get(&target_player_id) {
            if invitations.iter().any(|inv| inv.guild_id == request.guild_id && inv.status == InvitationStatus::Pending) {
                return Ok(InviteToGuildResponse {
                    success: false,
                    invitation_id: None,
                    message: "Player already has a pending invitation to this guild".to_string(),
                });
            }
        }

        // Create invitation
        let invitation_id = Uuid::new_v4();
        let proposed_rank = request.proposed_rank.unwrap_or(GuildRank::Initiate);
        let invitation = GuildInvitation {
            invitation_id,
            guild_id: request.guild_id,
            guild_name: guild.name.clone(),
            guild_tag: guild.tag.clone(),
            inviter_id,
            inviter_username: inviter_member.unwrap().username.clone(),
            inviter_rank: inviter_member.unwrap().rank.clone(),
            invited_player_id: target_player_id,
            message: request.message,
            sent_at: SystemTime::now(),
            expires_at: SystemTime::now() + std::time::Duration::from_secs(self.config.invitation_timeout_hours * 3600),
            status: InvitationStatus::Pending,
            proposed_rank,
        };

        // Store invitation
        self.guild_invitations.entry(target_player_id).or_insert_with(Vec::new).push(invitation);

        info!("✅ Guild invitation sent: {} -> {} (Guild: {})", inviter_member.unwrap().username, target_username, guild.name);

        Ok(InviteToGuildResponse {
            success: true,
            invitation_id: Some(invitation_id),
            message: format!("Invitation sent to {}", target_username),
        })
    }

    /// Get player's guild status and invitations
    pub async fn get_guild_status(&self, player_id: Uuid) -> Result<GuildStatusResponse> {
        let mut guild_summary = None;
        let mut invitations = Vec::new();
        let mut available_guilds = Vec::new();

        // Get player's guild
        if let Some(guild_id) = self.player_guilds.get(&player_id) {
            if let Some(guild) = self.guilds.get(guild_id) {
                if let Some(member) = guild.members.iter().find(|m| m.player_id == player_id) {
                    guild_summary = Some(GuildSummary {
                        guild_id: *guild_id,
                        name: guild.name.clone(),
                        tag: guild.tag.clone(),
                        guild_type: guild.guild_type.clone(),
                        member_count: guild.members.len() as u8,
                        max_members: guild.settings.max_members,
                        status: guild.status.clone(),
                        player_rank: member.rank.clone(),
                        joined_at: member.joined_at,
                        last_activity: guild.activities.scheduled_events.last()
                            .map(|e| e.scheduled_time)
                            .unwrap_or(guild.created_at),
                    });
                }
            }
        }

        // Get player's invitations
        if let Some(player_invitations) = self.guild_invitations.get(&player_id) {
            for invitation in player_invitations {
                if invitation.status == InvitationStatus::Pending && SystemTime::now() <= invitation.expires_at {
                    invitations.push(invitation.clone());
                }
            }
        }

        // Get public guilds for browsing (if player is not in a guild)
        if guild_summary.is_none() {
            for guild in self.guilds.values() {
                if guild.settings.is_public && guild.settings.recruitment_open {
                    available_guilds.push(PublicGuildInfo {
                        guild_id: guild.guild_id,
                        name: guild.name.clone(),
                        tag: guild.tag.clone(),
                        description: guild.description.clone(),
                        guild_type: guild.guild_type.clone(),
                        member_count: guild.members.len() as u8,
                        max_members: guild.settings.max_members,
                        recruitment_open: guild.settings.recruitment_open,
                        application_required: guild.settings.application_required,
                        reputation: guild.reputation.overall_reputation,
                        founded_date: guild.created_at,
                    });
                }
            }
        }

        Ok(GuildStatusResponse {
            success: true,
            guild: guild_summary,
            invitations,
            available_guilds,
        })
    }

    // Helper methods

    fn is_guild_tag_taken(&self, tag: &str) -> bool {
        self.guilds.values().any(|guild| guild.tag.to_lowercase() == tag.to_lowercase())
    }

    fn is_guild_name_taken(&self, name: &str) -> bool {
        self.guilds.values().any(|guild| guild.name.to_lowercase() == name.to_lowercase())
    }

    fn get_rank_permissions(&self, rank: &GuildRank) -> Vec<GuildPermission> {
        match rank {
            GuildRank::GuildMaster => vec![
                // All permissions for guild master
                GuildPermission::InviteMembers,
                GuildPermission::KickMembers,
                GuildPermission::PromoteMembers,
                GuildPermission::DemoteMembers,
                GuildPermission::EditGuildInfo,
                GuildPermission::ManageRanks,
                GuildPermission::UseGuildChat,
                GuildPermission::UseOfficerChat,
                GuildPermission::CreateChannels,
                GuildPermission::ManageChannels,
                GuildPermission::ModerateChat,
                GuildPermission::MakeAnnouncements,
                GuildPermission::AccessGuildBank,
                GuildPermission::WithdrawFromBank,
                GuildPermission::DepositToBank,
                GuildPermission::ManageBankPermissions,
                GuildPermission::ViewBankLogs,
                GuildPermission::ClaimTerritory,
                GuildPermission::ManageTerritory,
                GuildPermission::DefendTerritory,
                GuildPermission::UpgradeStructures,
                GuildPermission::ManageAlliances,
                GuildPermission::DeclareWar,
                GuildPermission::NegotiatePeace,
                GuildPermission::TradeAgreements,
                GuildPermission::ScheduleEvents,
                GuildPermission::CreateMissions,
                GuildPermission::LeadRaids,
                GuildPermission::OrganizeActivities,
                GuildPermission::ViewMemberList,
                GuildPermission::ViewActivityLogs,
                GuildPermission::ManageApplications,
                GuildPermission::SetMemberNotes,
                GuildPermission::AccessRecruitment,
            ],
            GuildRank::Officer => vec![
                GuildPermission::InviteMembers,
                GuildPermission::KickMembers,
                GuildPermission::PromoteMembers,
                GuildPermission::UseGuildChat,
                GuildPermission::UseOfficerChat,
                GuildPermission::ModerateChat,
                GuildPermission::MakeAnnouncements,
                GuildPermission::AccessGuildBank,
                GuildPermission::WithdrawFromBank,
                GuildPermission::DepositToBank,
                GuildPermission::ViewBankLogs,
                GuildPermission::DefendTerritory,
                GuildPermission::ScheduleEvents,
                GuildPermission::CreateMissions,
                GuildPermission::LeadRaids,
                GuildPermission::OrganizeActivities,
                GuildPermission::ViewMemberList,
                GuildPermission::ViewActivityLogs,
                GuildPermission::ManageApplications,
                GuildPermission::SetMemberNotes,
                GuildPermission::AccessRecruitment,
            ],
            GuildRank::Lieutenant => vec![
                GuildPermission::InviteMembers,
                GuildPermission::UseGuildChat,
                GuildPermission::UseOfficerChat,
                GuildPermission::AccessGuildBank,
                GuildPermission::DepositToBank,
                GuildPermission::DefendTerritory,
                GuildPermission::ScheduleEvents,
                GuildPermission::LeadRaids,
                GuildPermission::OrganizeActivities,
                GuildPermission::ViewMemberList,
                GuildPermission::ViewActivityLogs,
                GuildPermission::AccessRecruitment,
            ],
            GuildRank::Veteran => vec![
                GuildPermission::UseGuildChat,
                GuildPermission::AccessGuildBank,
                GuildPermission::DepositToBank,
                GuildPermission::DefendTerritory,
                GuildPermission::ViewMemberList,
            ],
            GuildRank::Member => vec![
                GuildPermission::UseGuildChat,
                GuildPermission::AccessGuildBank,
                GuildPermission::DepositToBank,
                GuildPermission::DefendTerritory,
            ],
            GuildRank::Initiate => vec![
                GuildPermission::UseGuildChat,
                GuildPermission::DepositToBank,
            ],
        }
    }

    fn create_default_channels(&self, creator_id: Uuid) -> Vec<GuildChannel> {
        vec![
            GuildChannel {
                channel_id: Uuid::new_v4(),
                name: "General".to_string(),
                channel_type: ChannelType::General,
                description: "General guild discussion".to_string(),
                access_level: AccessLevel::Everyone,
                messages: vec![],
                message_limit: self.config.max_chat_history_per_channel,
                created_by: creator_id,
                created_at: SystemTime::now(),
                settings: ChannelSettings::default(),
            },
            GuildChannel {
                channel_id: Uuid::new_v4(),
                name: "Officers".to_string(),
                channel_type: ChannelType::Officers,
                description: "Leadership discussion".to_string(),
                access_level: AccessLevel::Officers,
                messages: vec![],
                message_limit: self.config.max_chat_history_per_channel,
                created_by: creator_id,
                created_at: SystemTime::now(),
                settings: ChannelSettings::default(),
            },
            GuildChannel {
                channel_id: Uuid::new_v4(),
                name: "Announcements".to_string(),
                channel_type: ChannelType::Announcements,
                description: "Official guild announcements".to_string(),
                access_level: AccessLevel::Everyone,
                messages: vec![],
                message_limit: self.config.max_chat_history_per_channel,
                created_by: creator_id,
                created_at: SystemTime::now(),
                settings: ChannelSettings::default(),
            },
        ]
    }
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self {
            max_guild_size: 40,
            max_guilds_per_player: 1,
            invitation_timeout_hours: 72, // 3 days
            max_chat_history_per_channel: 500,
            auto_disband_inactive_days: 30,
            min_members_to_prevent_disband: 3,
            max_officers: 5,
            max_lieutenants: 10,
            succession_timeout_days: 14,
            territory_claim_cooldown_hours: 24,
        }
    }
}

impl Default for GuildHierarchy {
    fn default() -> Self {
        Self {
            ranks: HashMap::new(),
            succession_plan: SuccessionPlan {
                succession_type: SuccessionType::Automatic,
                designated_heir: None,
                succession_council: vec![],
                emergency_contacts: vec![],
                inactive_threshold_days: 14,
            },
            leadership_council: vec![],
            rank_requirements: HashMap::new(),
        }
    }
}

impl Default for GuildResources {
    fn default() -> Self {
        Self {
            treasury: HashMap::new(),
            materials: HashMap::new(),
            influence_points: 0,
            research_points: 0,
            military_power: 0,
            territory_control: 0.0,
            weekly_income: HashMap::new(),
            maintenance_costs: HashMap::new(),
        }
    }
}

impl Default for GuildActivities {
    fn default() -> Self {
        Self {
            current_campaigns: vec![],
            scheduled_events: vec![],
            completed_missions: vec![],
            active_projects: vec![],
            activity_calendar: HashMap::new(),
        }
    }
}

impl Default for GuildDiplomacy {
    fn default() -> Self {
        Self {
            alliances: vec![],
            wars: vec![],
            trade_agreements: vec![],
            reputation_standings: HashMap::new(),
            diplomatic_missions: vec![],
        }
    }
}

impl Default for GuildBank {
    fn default() -> Self {
        Self {
            treasury: HashMap::new(),
            vaults: vec![],
            transaction_log: vec![],
            access_permissions: HashMap::new(),
            daily_limits: HashMap::new(),
        }
    }
}

impl Default for GuildReputation {
    fn default() -> Self {
        Self {
            overall_reputation: 0,
            faction_standings: HashMap::new(),
            reputation_history: vec![],
            achievements: vec![],
            titles: vec![],
        }
    }
}

impl Default for GuildContributions {
    fn default() -> Self {
        Self {
            total_contribution_points: 0,
            resource_contributions: HashMap::new(),
            time_contributed: 0,
            missions_completed: 0,
            events_organized: 0,
            members_recruited: 0,
            leadership_time: 0,
            special_achievements: vec![],
        }
    }
}

impl Default for ChannelSettings {
    fn default() -> Self {
        Self {
            slow_mode_seconds: None,
            max_message_length: 1000,
            allow_attachments: true,
            auto_delete_after_days: None,
            moderation_enabled: false,
        }
    }
}

/// Thread-safe guild manager
pub type SharedGuildManager = Arc<RwLock<GuildManager>>;

pub fn create_shared_guild_manager() -> SharedGuildManager {
    Arc::new(RwLock::new(GuildManager::new()))
}