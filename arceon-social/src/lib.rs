/*!
# Arceon Social Systems

Advanced social interaction systems for the Arceon MMORPG including:
- Guild management and hierarchies
- Friendship networks and social connections
- Communication channels and messaging
- Community events and activities
- Social reputation and influence systems
- Player matching and recommendation systems

This module provides comprehensive social functionality to foster
community building and collaborative gameplay experiences.
*/

pub mod guild_system;
pub mod friendship_network;
pub mod communication;
pub mod community_events;
pub mod reputation_system;
pub mod social_matching;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;

/// Central social system coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSystem {
    pub guild_manager: guild_system::GuildManager,
    pub friendship_network: friendship_network::FriendshipNetwork,
    pub communication_hub: communication::CommunicationHub,
    pub event_coordinator: community_events::EventCoordinator,
    pub reputation_system: reputation_system::ReputationSystem,
    pub social_matching: social_matching::SocialMatching,
}

/// Social interaction types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialInteractionType {
    GuildInvitation,
    FriendRequest,
    MessageSent,
    EventParticipation,
    ReputationChange,
    TeamFormation,
    TradeOffer,
    AllianceProposal,
}

/// Social activity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialActivity {
    pub activity_id: Uuid,
    pub participant_ids: Vec<Uuid>,
    pub activity_type: SocialInteractionType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context: HashMap<String, String>,
    pub visibility: ActivityVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityVisibility {
    Public,
    Friends,
    Guild,
    Private,
}

/// Player social profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialProfile {
    pub player_id: Uuid,
    pub display_name: String,
    pub bio: Option<String>,
    pub social_preferences: SocialPreferences,
    pub activity_history: Vec<SocialActivity>,
    pub social_stats: SocialStatistics,
    pub badges: Vec<SocialBadge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPreferences {
    pub allow_friend_requests: bool,
    pub allow_guild_invitations: bool,
    pub show_online_status: bool,
    pub activity_visibility: ActivityVisibility,
    pub preferred_communication: Vec<String>,
    pub blocked_players: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialStatistics {
    pub friends_count: u32,
    pub guilds_joined: u32,
    pub events_attended: u32,
    pub messages_sent: u64,
    pub reputation_score: f64,
    pub influence_rating: f64,
    pub community_contributions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialBadge {
    pub badge_id: Uuid,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub earned_date: chrono::DateTime<chrono::Utc>,
    pub rarity: BadgeRarity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BadgeRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl SocialSystem {
    /// Create new social system
    pub fn new() -> Self {
        Self {
            guild_manager: guild_system::GuildManager::new(),
            friendship_network: friendship_network::FriendshipNetwork::new(),
            communication_hub: communication::CommunicationHub::new(),
            event_coordinator: community_events::EventCoordinator::new(),
            reputation_system: reputation_system::ReputationSystem::new(),
            social_matching: social_matching::SocialMatching::new(),
        }
    }

    /// Process social interaction
    pub fn process_interaction(&mut self, interaction: SocialInteractionType, participants: Vec<Uuid>) -> Result<()> {
        let activity = SocialActivity {
            activity_id: Uuid::new_v4(),
            participant_ids: participants.clone(),
            activity_type: interaction.clone(),
            timestamp: chrono::Utc::now(),
            context: HashMap::new(),
            visibility: ActivityVisibility::Public,
        };

        // Process through appropriate subsystem
        match interaction {
            SocialInteractionType::GuildInvitation => {
                if participants.len() >= 2 {
                    self.guild_manager.process_invitation(participants[0], participants[1])?;
                }
            },
            SocialInteractionType::FriendRequest => {
                if participants.len() >= 2 {
                    self.friendship_network.send_friend_request(participants[0], participants[1])?;
                }
            },
            SocialInteractionType::ReputationChange => {
                for participant in &participants {
                    self.reputation_system.update_reputation(*participant, 1.0)?;
                }
            },
            _ => {
                // Log other interaction types
                tracing::info!("Processing social interaction: {:?}", interaction);
            },
        }

        // Update social matching recommendations
        for participant in &participants {
            self.social_matching.update_player_activity(*participant, &activity)?;
        }

        Ok(())
    }

    /// Get player social profile
    pub fn get_social_profile(&self, player_id: Uuid) -> Option<SocialProfile> {
        // Aggregate data from all systems
        let social_stats = SocialStatistics {
            friends_count: self.friendship_network.get_friend_count(player_id),
            guilds_joined: self.guild_manager.get_guild_memberships(player_id).len() as u32,
            events_attended: self.event_coordinator.get_event_participation_count(player_id),
            messages_sent: self.communication_hub.get_message_count(player_id),
            reputation_score: self.reputation_system.get_reputation_score(player_id).unwrap_or(0.0),
            influence_rating: self.reputation_system.get_influence_rating(player_id).unwrap_or(0.0),
            community_contributions: 0, // Placeholder
        };

        Some(SocialProfile {
            player_id,
            display_name: format!("Player_{}", player_id.to_string()[..8].to_uppercase()),
            bio: None,
            social_preferences: SocialPreferences::default(),
            activity_history: vec![],
            social_stats,
            badges: vec![],
        })
    }

    /// Get social recommendations for player
    pub fn get_social_recommendations(&self, player_id: Uuid) -> Vec<SocialRecommendation> {
        let mut recommendations = vec![];

        // Friend recommendations
        let friend_suggestions = self.social_matching.get_friend_recommendations(player_id, 5);
        for suggestion in friend_suggestions {
            recommendations.push(SocialRecommendation {
                recommendation_type: RecommendationType::FriendSuggestion,
                target_id: suggestion,
                reason: "Based on mutual interests and activity patterns".to_string(),
                confidence: 0.8,
            });
        }

        // Guild recommendations
        let guild_suggestions = self.social_matching.get_guild_recommendations(player_id, 3);
        for suggestion in guild_suggestions {
            recommendations.push(SocialRecommendation {
                recommendation_type: RecommendationType::GuildSuggestion,
                target_id: suggestion,
                reason: "Guild matches your playstyle and interests".to_string(),
                confidence: 0.7,
            });
        }

        recommendations
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialRecommendation {
    pub recommendation_type: RecommendationType,
    pub target_id: Uuid,
    pub reason: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    FriendSuggestion,
    GuildSuggestion,
    EventRecommendation,
    TeamMatchmaking,
}

impl Default for SocialPreferences {
    fn default() -> Self {
        Self {
            allow_friend_requests: true,
            allow_guild_invitations: true,
            show_online_status: true,
            activity_visibility: ActivityVisibility::Friends,
            preferred_communication: vec!["text".to_string()],
            blocked_players: vec![],
        }
    }
}

impl Default for SocialSystem {
    fn default() -> Self {
        Self::new()
    }
}

// Type aliases for convenience
pub type PlayerId = Uuid;
pub type GuildId = Uuid;
pub type EventId = Uuid;
pub type MessageId = Uuid;