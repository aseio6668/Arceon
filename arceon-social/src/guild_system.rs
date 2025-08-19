/*!
# Guild System

Comprehensive guild management system with hierarchies, permissions,
activities, and progression mechanics.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{PlayerId, GuildId};

/// Guild management system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildManager {
    pub guilds: HashMap<GuildId, Guild>,
    pub member_guilds: HashMap<PlayerId, Vec<GuildMembership>>,
    pub guild_templates: Vec<GuildTemplate>,
    pub alliance_network: AllianceNetwork,
}

/// Guild definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub guild_id: GuildId,
    pub name: String,
    pub description: String,
    pub guild_type: GuildType,
    pub founded_date: DateTime<Utc>,
    pub founder_id: PlayerId,
    pub members: HashMap<PlayerId, GuildMembership>,
    pub hierarchy: GuildHierarchy,
    pub treasury: GuildTreasury,
    pub facilities: Vec<GuildFacility>,
    pub achievements: Vec<GuildAchievement>,
    pub reputation: f64,
    pub level: u32,
    pub experience: u64,
    pub settings: GuildSettings,
}

/// Types of guilds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuildType {
    Combat,      // PvP/PvE focused
    Crafting,    // Production and trading
    Social,      // Community and events
    Exploration, // Discovery and adventure
    Economic,    // Trading and wealth
    Research,    // Knowledge and development
    Mercenary,   // Contract work
    Diplomatic,  // Inter-guild relations
}

/// Guild membership information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMembership {
    pub player_id: PlayerId,
    pub guild_id: GuildId,
    pub rank: GuildRank,
    pub join_date: DateTime<Utc>,
    pub contributions: GuildContributions,
    pub permissions: Vec<GuildPermission>,
    pub status: MembershipStatus,
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MembershipStatus {
    Active,
    Inactive,
    Leave,
    Kicked,
    Banned,
}

/// Guild hierarchy and ranks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildHierarchy {
    pub ranks: Vec<GuildRank>,
    pub promotion_requirements: HashMap<String, PromotionRequirement>,
    pub succession_rules: SuccessionRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRank {
    pub rank_id: Uuid,
    pub name: String,
    pub level: u32,
    pub permissions: Vec<GuildPermission>,
    pub salary: Option<u64>,
    pub member_limit: Option<u32>,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuildPermission {
    InviteMembers,
    KickMembers,
    PromoteMembers,
    DemoteMembers,
    ManageTreasury,
    ManageFacilities,
    DeclareWar,
    FormAlliances,
    SetMotd,
    ManageChannels,
    ViewTreasury,
    WithdrawFunds,
    ScheduleEvents,
    ManageRoles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionRequirement {
    pub target_rank: String,
    pub required_contributions: u64,
    pub required_time_days: u32,
    pub required_endorsements: u32,
    pub special_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessionRules {
    pub succession_type: SuccessionType,
    pub voting_requirements: Option<VotingRequirements>,
    pub automatic_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuccessionType {
    Hereditary,
    Democratic,
    Meritocratic,
    Appointed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingRequirements {
    pub eligible_ranks: Vec<String>,
    pub minimum_participation: f64,
    pub majority_threshold: f64,
}

/// Guild member contributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildContributions {
    pub total_points: u64,
    pub weekly_points: u64,
    pub contribution_types: HashMap<ContributionType, u64>,
    pub notable_achievements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ContributionType {
    ResourceDonation,
    EventParticipation,
    Recruitment,
    Combat,
    Crafting,
    Research,
    Diplomacy,
    Leadership,
}

/// Guild treasury management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTreasury {
    pub funds: HashMap<String, u64>, // Currency type -> amount
    pub resources: HashMap<String, u64>, // Resource type -> amount
    pub budget_allocations: Vec<BudgetAllocation>,
    pub transaction_history: Vec<TreasuryTransaction>,
    pub access_logs: Vec<TreasuryAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAllocation {
    pub category: String,
    pub allocated_amount: u64,
    pub spent_amount: u64,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryTransaction {
    pub transaction_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: u64,
    pub currency: String,
    pub member_id: Option<PlayerId>,
    pub timestamp: DateTime<Utc>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Salary,
    Purchase,
    Maintenance,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryAccess {
    pub member_id: PlayerId,
    pub access_type: String,
    pub timestamp: DateTime<Utc>,
    pub amount_accessed: Option<u64>,
}

/// Guild facilities and buildings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildFacility {
    pub facility_id: Uuid,
    pub facility_type: FacilityType,
    pub name: String,
    pub level: u32,
    pub capacity: u32,
    pub maintenance_cost: u64,
    pub benefits: Vec<FacilityBenefit>,
    pub upgrade_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FacilityType {
    GuildHall,
    Workshop,
    Storage,
    Training,
    Research,
    Diplomatic,
    Defense,
    Commerce,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityBenefit {
    pub benefit_type: String,
    pub magnitude: f64,
    pub description: String,
    pub conditions: Vec<String>,
}

/// Guild achievements and progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildAchievement {
    pub achievement_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub requirements: Vec<String>,
    pub rewards: Vec<GuildReward>,
    pub completion_date: DateTime<Utc>,
    pub contributors: Vec<PlayerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCategory {
    Growth,
    Combat,
    Economic,
    Social,
    Diplomatic,
    Research,
    Construction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildReward {
    pub reward_type: String,
    pub amount: u64,
    pub description: String,
}

/// Guild settings and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSettings {
    pub public_visibility: bool,
    pub recruitment_open: bool,
    pub member_limit: u32,
    pub minimum_level: u32,
    pub application_requirements: Vec<String>,
    pub motd: String,
    pub guild_colors: GuildColors,
    pub guild_emblem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildColors {
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
}

/// Guild template for quick setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub guild_type: GuildType,
    pub default_hierarchy: GuildHierarchy,
    pub starting_facilities: Vec<FacilityType>,
    pub recommended_settings: GuildSettings,
}

/// Alliance network for inter-guild relations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceNetwork {
    pub alliances: HashMap<Uuid, Alliance>,
    pub guild_relations: HashMap<GuildId, HashMap<GuildId, RelationType>>,
    pub diplomatic_history: Vec<DiplomaticEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alliance {
    pub alliance_id: Uuid,
    pub name: String,
    pub member_guilds: Vec<GuildId>,
    pub alliance_type: AllianceType,
    pub terms: AllianceTerms,
    pub created_date: DateTime<Utc>,
    pub leader_guild: GuildId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllianceType {
    Trade,
    Military,
    Research,
    Mutual,
    Temporary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceTerms {
    pub shared_resources: bool,
    pub mutual_defense: bool,
    pub trade_benefits: f64,
    pub joint_activities: bool,
    pub duration_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    Neutral,
    Friendly,
    Allied,
    Hostile,
    AtWar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticEvent {
    pub event_id: Uuid,
    pub guilds_involved: Vec<GuildId>,
    pub event_type: DiplomaticEventType,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiplomaticEventType {
    AllianceFormed,
    AllianceBroken,
    WarDeclared,
    PeaceTreaty,
    TradeAgreement,
    Betrayal,
}

impl GuildManager {
    /// Create new guild manager
    pub fn new() -> Self {
        Self {
            guilds: HashMap::new(),
            member_guilds: HashMap::new(),
            guild_templates: Self::create_default_templates(),
            alliance_network: AllianceNetwork {
                alliances: HashMap::new(),
                guild_relations: HashMap::new(),
                diplomatic_history: vec![],
            },
        }
    }

    /// Create default guild templates
    fn create_default_templates() -> Vec<GuildTemplate> {
        vec![
            GuildTemplate {
                template_id: Uuid::new_v4(),
                name: "Combat Guild".to_string(),
                guild_type: GuildType::Combat,
                default_hierarchy: GuildHierarchy::default_combat(),
                starting_facilities: vec![FacilityType::GuildHall, FacilityType::Training],
                recommended_settings: GuildSettings::default_combat(),
            },
            GuildTemplate {
                template_id: Uuid::new_v4(),
                name: "Crafting Guild".to_string(),
                guild_type: GuildType::Crafting,
                default_hierarchy: GuildHierarchy::default_crafting(),
                starting_facilities: vec![FacilityType::GuildHall, FacilityType::Workshop, FacilityType::Storage],
                recommended_settings: GuildSettings::default_crafting(),
            },
            GuildTemplate {
                template_id: Uuid::new_v4(),
                name: "Social Guild".to_string(),
                guild_type: GuildType::Social,
                default_hierarchy: GuildHierarchy::default_social(),
                starting_facilities: vec![FacilityType::GuildHall],
                recommended_settings: GuildSettings::default_social(),
            },
        ]
    }

    /// Create a new guild
    pub fn create_guild(&mut self, founder_id: PlayerId, name: String, description: String, guild_type: GuildType) -> Result<GuildId> {
        let guild_id = Uuid::new_v4();
        
        let guild = Guild {
            guild_id,
            name,
            description,
            guild_type: guild_type.clone(),
            founded_date: Utc::now(),
            founder_id,
            members: HashMap::new(),
            hierarchy: GuildHierarchy::default_for_type(&guild_type),
            treasury: GuildTreasury::new(),
            facilities: vec![GuildFacility::default_guild_hall()],
            achievements: vec![],
            reputation: 0.0,
            level: 1,
            experience: 0,
            settings: GuildSettings::default_for_type(&guild_type),
        };

        self.guilds.insert(guild_id, guild);
        
        // Add founder as guild leader
        self.add_member(guild_id, founder_id, true)?;
        
        Ok(guild_id)
    }

    /// Add member to guild
    pub fn add_member(&mut self, guild_id: GuildId, player_id: PlayerId, is_founder: bool) -> Result<()> {
        let guild = self.guilds.get_mut(&guild_id)
            .ok_or_else(|| anyhow::anyhow!("Guild not found"))?;

        let rank = if is_founder {
            guild.hierarchy.ranks.first().cloned()
                .unwrap_or_else(|| GuildRank::default_leader())
        } else {
            guild.hierarchy.ranks.last().cloned()
                .unwrap_or_else(|| GuildRank::default_member())
        };

        let membership = GuildMembership {
            player_id,
            guild_id,
            rank,
            join_date: Utc::now(),
            contributions: GuildContributions::new(),
            permissions: vec![], // Will be set based on rank
            status: MembershipStatus::Active,
            last_active: Utc::now(),
        };

        guild.members.insert(player_id, membership.clone());
        
        // Update player's guild list
        self.member_guilds.entry(player_id)
            .or_insert_with(Vec::new)
            .push(membership);

        Ok(())
    }

    /// Process guild invitation
    pub fn process_invitation(&mut self, inviter_id: PlayerId, invitee_id: PlayerId) -> Result<()> {
        // Find guilds where inviter has invite permissions
        let inviter_guilds = self.member_guilds.get(&inviter_id)
            .ok_or_else(|| anyhow::anyhow!("Inviter not in any guilds"))?;

        for membership in inviter_guilds {
            if membership.permissions.contains(&GuildPermission::InviteMembers) {
                // Send invitation (simplified implementation)
                tracing::info!("Guild invitation sent from {} to {} for guild {}", 
                             inviter_id, invitee_id, membership.guild_id);
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("No permission to invite members"))
    }

    /// Get guild memberships for a player
    pub fn get_guild_memberships(&self, player_id: PlayerId) -> Vec<GuildMembership> {
        self.member_guilds.get(&player_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get guild information
    pub fn get_guild(&self, guild_id: GuildId) -> Option<&Guild> {
        self.guilds.get(&guild_id)
    }

    /// Update guild experience and level
    pub fn add_guild_experience(&mut self, guild_id: GuildId, experience: u64) -> Result<()> {
        let guild = self.guilds.get_mut(&guild_id)
            .ok_or_else(|| anyhow::anyhow!("Guild not found"))?;

        guild.experience += experience;
        
        // Check for level up
        let required_exp = (guild.level as u64 + 1) * 1000;
        if guild.experience >= required_exp {
            guild.level += 1;
            guild.experience -= required_exp;
            
            // Grant level up rewards
            self.grant_level_rewards(guild_id, guild.level)?;
        }

        Ok(())
    }

    /// Grant rewards for guild level up
    fn grant_level_rewards(&mut self, guild_id: GuildId, level: u32) -> Result<()> {
        let guild = self.guilds.get_mut(&guild_id)
            .ok_or_else(|| anyhow::anyhow!("Guild not found"))?;

        // Increase member capacity
        guild.settings.member_limit += 5;
        
        // Add to treasury
        guild.treasury.funds.insert("gold".to_string(), 
            guild.treasury.funds.get("gold").unwrap_or(&0) + (level as u64 * 100));

        tracing::info!("Guild {} reached level {}", guild.name, level);
        Ok(())
    }
}

impl GuildHierarchy {
    /// Default hierarchy for combat guilds
    fn default_combat() -> Self {
        Self {
            ranks: vec![
                GuildRank {
                    rank_id: Uuid::new_v4(),
                    name: "Guild Master".to_string(),
                    level: 5,
                    permissions: vec![
                        GuildPermission::InviteMembers,
                        GuildPermission::KickMembers,
                        GuildPermission::PromoteMembers,
                        GuildPermission::DemoteMembers,
                        GuildPermission::ManageTreasury,
                        GuildPermission::DeclareWar,
                        GuildPermission::FormAlliances,
                    ],
                    salary: Some(500),
                    member_limit: Some(1),
                    requirements: vec!["Founder".to_string()],
                },
                GuildRank {
                    rank_id: Uuid::new_v4(),
                    name: "Officer".to_string(),
                    level: 4,
                    permissions: vec![
                        GuildPermission::InviteMembers,
                        GuildPermission::KickMembers,
                        GuildPermission::ScheduleEvents,
                    ],
                    salary: Some(200),
                    member_limit: Some(3),
                    requirements: vec!["100 contribution points".to_string()],
                },
                GuildRank {
                    rank_id: Uuid::new_v4(),
                    name: "Veteran".to_string(),
                    level: 3,
                    permissions: vec![GuildPermission::InviteMembers],
                    salary: Some(100),
                    member_limit: None,
                    requirements: vec!["50 contribution points".to_string()],
                },
                GuildRank {
                    rank_id: Uuid::new_v4(),
                    name: "Member".to_string(),
                    level: 2,
                    permissions: vec![GuildPermission::ViewTreasury],
                    salary: Some(50),
                    member_limit: None,
                    requirements: vec![],
                },
                GuildRank {
                    rank_id: Uuid::new_v4(),
                    name: "Recruit".to_string(),
                    level: 1,
                    permissions: vec![],
                    salary: None,
                    member_limit: None,
                    requirements: vec![],
                },
            ],
            promotion_requirements: HashMap::new(),
            succession_rules: SuccessionRules {
                succession_type: SuccessionType::Democratic,
                voting_requirements: Some(VotingRequirements {
                    eligible_ranks: vec!["Officer".to_string(), "Veteran".to_string()],
                    minimum_participation: 0.6,
                    majority_threshold: 0.51,
                }),
                automatic_conditions: vec!["Leader inactive for 30 days".to_string()],
            },
        }
    }

    /// Default hierarchy for crafting guilds
    fn default_crafting() -> Self {
        let mut hierarchy = Self::default_combat();
        hierarchy.ranks[1].name = "Master Artisan".to_string();
        hierarchy.ranks[2].name = "Journeyman".to_string();
        hierarchy.ranks[3].name = "Craftsman".to_string();
        hierarchy.ranks[4].name = "Apprentice".to_string();
        hierarchy
    }

    /// Default hierarchy for social guilds
    fn default_social() -> Self {
        let mut hierarchy = Self::default_combat();
        hierarchy.ranks[1].name = "Organizer".to_string();
        hierarchy.ranks[2].name = "Coordinator".to_string();
        hierarchy.ranks[3].name = "Active Member".to_string();
        hierarchy.ranks[4].name = "New Member".to_string();
        hierarchy
    }

    /// Default hierarchy for guild type
    fn default_for_type(guild_type: &GuildType) -> Self {
        match guild_type {
            GuildType::Combat => Self::default_combat(),
            GuildType::Crafting => Self::default_crafting(),
            GuildType::Social => Self::default_social(),
            _ => Self::default_combat(), // Fallback
        }
    }
}

impl GuildRank {
    /// Default leader rank
    fn default_leader() -> Self {
        Self {
            rank_id: Uuid::new_v4(),
            name: "Guild Master".to_string(),
            level: 5,
            permissions: vec![
                GuildPermission::InviteMembers,
                GuildPermission::KickMembers,
                GuildPermission::PromoteMembers,
                GuildPermission::DemoteMembers,
                GuildPermission::ManageTreasury,
            ],
            salary: Some(500),
            member_limit: Some(1),
            requirements: vec!["Founder".to_string()],
        }
    }

    /// Default member rank
    fn default_member() -> Self {
        Self {
            rank_id: Uuid::new_v4(),
            name: "Member".to_string(),
            level: 1,
            permissions: vec![],
            salary: None,
            member_limit: None,
            requirements: vec![],
        }
    }
}

impl GuildContributions {
    /// Create new contribution tracker
    fn new() -> Self {
        Self {
            total_points: 0,
            weekly_points: 0,
            contribution_types: HashMap::new(),
            notable_achievements: vec![],
        }
    }
}

impl GuildTreasury {
    /// Create new guild treasury
    fn new() -> Self {
        Self {
            funds: HashMap::new(),
            resources: HashMap::new(),
            budget_allocations: vec![],
            transaction_history: vec![],
            access_logs: vec![],
        }
    }
}

impl GuildFacility {
    /// Create default guild hall
    fn default_guild_hall() -> Self {
        Self {
            facility_id: Uuid::new_v4(),
            facility_type: FacilityType::GuildHall,
            name: "Guild Hall".to_string(),
            level: 1,
            capacity: 50,
            maintenance_cost: 100,
            benefits: vec![
                FacilityBenefit {
                    benefit_type: "Member Capacity".to_string(),
                    magnitude: 50.0,
                    description: "Allows 50 guild members".to_string(),
                    conditions: vec![],
                },
            ],
            upgrade_requirements: vec!["1000 gold".to_string(), "Level 5 guild".to_string()],
        }
    }
}

impl GuildSettings {
    /// Default settings for combat guilds
    fn default_combat() -> Self {
        Self {
            public_visibility: true,
            recruitment_open: true,
            member_limit: 50,
            minimum_level: 10,
            application_requirements: vec!["Combat experience required".to_string()],
            motd: "Welcome to our combat guild! Ready for battle!".to_string(),
            guild_colors: GuildColors {
                primary_color: "#FF0000".to_string(),
                secondary_color: "#800000".to_string(),
                accent_color: "#FFD700".to_string(),
            },
            guild_emblem: "crossed_swords.png".to_string(),
        }
    }

    /// Default settings for crafting guilds
    fn default_crafting() -> Self {
        let mut settings = Self::default_combat();
        settings.application_requirements = vec!["Crafting skills preferred".to_string()];
        settings.motd = "Master your craft with us!".to_string();
        settings.guild_colors.primary_color = "#8B4513".to_string();
        settings.guild_emblem = "anvil.png".to_string();
        settings
    }

    /// Default settings for social guilds
    fn default_social() -> Self {
        let mut settings = Self::default_combat();
        settings.minimum_level = 1;
        settings.application_requirements = vec!["Friendly attitude required".to_string()];
        settings.motd = "Making friends and having fun!".to_string();
        settings.guild_colors.primary_color = "#32CD32".to_string();
        settings.guild_emblem = "heart.png".to_string();
        settings
    }

    /// Default settings for guild type
    fn default_for_type(guild_type: &GuildType) -> Self {
        match guild_type {
            GuildType::Combat => Self::default_combat(),
            GuildType::Crafting => Self::default_crafting(),
            GuildType::Social => Self::default_social(),
            _ => Self::default_combat(), // Fallback
        }
    }
}

impl Default for GuildManager {
    fn default() -> Self {
        Self::new()
    }
}