/*!
# Friendship Network

Social networking system for managing friendships, connections,
and social relationships between players.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::PlayerId;

/// Friendship network management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendshipNetwork {
    pub friendships: HashMap<PlayerId, Vec<Friendship>>,
    pub friend_requests: HashMap<PlayerId, Vec<FriendRequest>>,
    pub blocked_players: HashMap<PlayerId, Vec<PlayerId>>,
    pub social_circles: HashMap<Uuid, SocialCircle>,
    pub interaction_history: HashMap<PlayerId, Vec<SocialInteraction>>,
}

/// Individual friendship connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friendship {
    pub friend_id: PlayerId,
    pub friendship_level: FriendshipLevel,
    pub established_date: DateTime<Utc>,
    pub last_interaction: DateTime<Utc>,
    pub mutual_friends: Vec<PlayerId>,
    pub shared_activities: Vec<SharedActivity>,
    pub friendship_score: f64,
    pub status: FriendshipStatus,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FriendshipLevel {
    Acquaintance,
    Friend,
    GoodFriend,
    BestFriend,
    SoulMate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FriendshipStatus {
    Active,
    Inactive,
    Strained,
    Blocked,
    Favorite,
}

/// Friend request management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendRequest {
    pub request_id: Uuid,
    pub requester_id: PlayerId,
    pub target_id: PlayerId,
    pub message: Option<String>,
    pub sent_date: DateTime<Utc>,
    pub status: RequestStatus,
    pub mutual_connections: u32,
    pub compatibility_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
    Withdrawn,
}

/// Shared activities between friends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedActivity {
    pub activity_id: Uuid,
    pub activity_type: ActivityType,
    pub participants: Vec<PlayerId>,
    pub date: DateTime<Utc>,
    pub duration_minutes: u32,
    pub activity_rating: Option<f64>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    Gaming,
    Chatting,
    Questing,
    Trading,
    Crafting,
    Exploring,
    Events,
    PvP,
    Guild,
    Teaching,
}

/// Social circles and groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialCircle {
    pub circle_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<PlayerId>,
    pub creator_id: PlayerId,
    pub created_date: DateTime<Utc>,
    pub circle_type: CircleType,
    pub privacy_level: PrivacyLevel,
    pub activity_level: f64,
    pub common_interests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircleType {
    Gaming,
    Professional,
    Regional,
    Interest,
    Family,
    Casual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyLevel {
    Open,
    InviteOnly,
    Private,
    Secret,
}

/// Social interactions between players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInteraction {
    pub interaction_id: Uuid,
    pub other_player_id: PlayerId,
    pub interaction_type: InteractionType,
    pub timestamp: DateTime<Utc>,
    pub context: Option<String>,
    pub sentiment: InteractionSentiment,
    pub impact_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Message,
    Trade,
    Gift,
    Help,
    Collaboration,
    Conflict,
    Compliment,
    Invitation,
    Support,
    Achievement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionSentiment {
    VeryPositive,
    Positive,
    Neutral,
    Negative,
    VeryNegative,
}

/// Friendship recommendations and suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendSuggestion {
    pub suggested_player_id: PlayerId,
    pub suggestion_type: SuggestionType,
    pub confidence_score: f64,
    pub mutual_friends: Vec<PlayerId>,
    pub common_interests: Vec<String>,
    pub compatibility_factors: Vec<CompatibilityFactor>,
    pub suggestion_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    MutualFriends,
    SimilarInterests,
    CommonActivity,
    Geographic,
    Personality,
    GuildMate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityFactor {
    pub factor_type: String,
    pub score: f64,
    pub description: String,
}

impl FriendshipNetwork {
    /// Create new friendship network
    pub fn new() -> Self {
        Self {
            friendships: HashMap::new(),
            friend_requests: HashMap::new(),
            blocked_players: HashMap::new(),
            social_circles: HashMap::new(),
            interaction_history: HashMap::new(),
        }
    }

    /// Send friend request
    pub fn send_friend_request(&mut self, requester_id: PlayerId, target_id: PlayerId) -> Result<Uuid> {
        // Check if players are already friends
        if self.are_friends(requester_id, target_id) {
            return Err(anyhow::anyhow!("Players are already friends"));
        }

        // Check if requester is blocked
        if self.is_blocked(target_id, requester_id) {
            return Err(anyhow::anyhow!("You are blocked by this player"));
        }

        // Check for existing pending request
        if self.has_pending_request(requester_id, target_id) {
            return Err(anyhow::anyhow!("Friend request already sent"));
        }

        let request_id = Uuid::new_v4();
        let mutual_connections = self.count_mutual_friends(requester_id, target_id);
        let compatibility = self.calculate_compatibility(requester_id, target_id);

        let request = FriendRequest {
            request_id,
            requester_id,
            target_id,
            message: None,
            sent_date: Utc::now(),
            status: RequestStatus::Pending,
            mutual_connections,
            compatibility_score: compatibility,
        };

        self.friend_requests.entry(target_id)
            .or_insert_with(Vec::new)
            .push(request);

        // Record interaction
        self.record_interaction(requester_id, target_id, InteractionType::Invitation, InteractionSentiment::Positive, 1.0);

        Ok(request_id)
    }

    /// Accept friend request
    pub fn accept_friend_request(&mut self, player_id: PlayerId, request_id: Uuid) -> Result<()> {
        let requests = self.friend_requests.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("No friend requests found"))?;

        let request_index = requests.iter().position(|r| r.request_id == request_id)
            .ok_or_else(|| anyhow::anyhow!("Friend request not found"))?;

        let mut request = requests.remove(request_index);
        request.status = RequestStatus::Accepted;

        // Create friendship connections
        self.create_friendship(request.requester_id, player_id)?;
        self.create_friendship(player_id, request.requester_id)?;

        // Record positive interaction
        self.record_interaction(player_id, request.requester_id, InteractionType::Support, InteractionSentiment::VeryPositive, 5.0);

        Ok(())
    }

    /// Create friendship connection
    fn create_friendship(&mut self, player_id: PlayerId, friend_id: PlayerId) -> Result<()> {
        let mutual_friends = self.get_mutual_friends(player_id, friend_id);
        
        let friendship = Friendship {
            friend_id,
            friendship_level: FriendshipLevel::Friend,
            established_date: Utc::now(),
            last_interaction: Utc::now(),
            mutual_friends,
            shared_activities: vec![],
            friendship_score: 50.0, // Starting score
            status: FriendshipStatus::Active,
            tags: vec![],
        };

        self.friendships.entry(player_id)
            .or_insert_with(Vec::new)
            .push(friendship);

        Ok(())
    }

    /// Check if two players are friends
    pub fn are_friends(&self, player1_id: PlayerId, player2_id: PlayerId) -> bool {
        if let Some(friendships) = self.friendships.get(&player1_id) {
            friendships.iter().any(|f| f.friend_id == player2_id && matches!(f.status, FriendshipStatus::Active))
        } else {
            false
        }
    }

    /// Get friend count for a player
    pub fn get_friend_count(&self, player_id: PlayerId) -> u32 {
        self.friendships.get(&player_id)
            .map(|friendships| friendships.iter()
                .filter(|f| matches!(f.status, FriendshipStatus::Active))
                .count() as u32)
            .unwrap_or(0)
    }

    /// Get list of friends for a player
    pub fn get_friends(&self, player_id: PlayerId) -> Vec<&Friendship> {
        self.friendships.get(&player_id)
            .map(|friendships| friendships.iter()
                .filter(|f| matches!(f.status, FriendshipStatus::Active))
                .collect())
            .unwrap_or_default()
    }

    /// Get mutual friends between two players
    pub fn get_mutual_friends(&self, player1_id: PlayerId, player2_id: PlayerId) -> Vec<PlayerId> {
        let player1_friends: Vec<PlayerId> = self.get_friends(player1_id)
            .into_iter()
            .map(|f| f.friend_id)
            .collect();
        
        let player2_friends: Vec<PlayerId> = self.get_friends(player2_id)
            .into_iter()
            .map(|f| f.friend_id)
            .collect();

        player1_friends.into_iter()
            .filter(|id| player2_friends.contains(id))
            .collect()
    }

    /// Count mutual friends
    fn count_mutual_friends(&self, player1_id: PlayerId, player2_id: PlayerId) -> u32 {
        self.get_mutual_friends(player1_id, player2_id).len() as u32
    }

    /// Check if player is blocked
    pub fn is_blocked(&self, player_id: PlayerId, blocked_player_id: PlayerId) -> bool {
        self.blocked_players.get(&player_id)
            .map(|blocked| blocked.contains(&blocked_player_id))
            .unwrap_or(false)
    }

    /// Block a player
    pub fn block_player(&mut self, player_id: PlayerId, blocked_player_id: PlayerId) -> Result<()> {
        // Remove friendship if exists
        if let Some(friendships) = self.friendships.get_mut(&player_id) {
            friendships.retain(|f| f.friend_id != blocked_player_id);
        }

        // Add to blocked list
        self.blocked_players.entry(player_id)
            .or_insert_with(Vec::new)
            .push(blocked_player_id);

        // Record negative interaction
        self.record_interaction(player_id, blocked_player_id, InteractionType::Conflict, InteractionSentiment::VeryNegative, -10.0);

        Ok(())
    }

    /// Check for pending friend request
    fn has_pending_request(&self, requester_id: PlayerId, target_id: PlayerId) -> bool {
        self.friend_requests.get(&target_id)
            .map(|requests| requests.iter()
                .any(|r| r.requester_id == requester_id && matches!(r.status, RequestStatus::Pending)))
            .unwrap_or(false)
    }

    /// Calculate compatibility between two players
    fn calculate_compatibility(&self, player1_id: PlayerId, player2_id: PlayerId) -> f64 {
        let mut compatibility = 0.5; // Base compatibility

        // Mutual friends bonus
        let mutual_count = self.count_mutual_friends(player1_id, player2_id);
        compatibility += (mutual_count as f64 * 0.1).min(0.3);

        // Interaction history
        let positive_interactions = self.count_positive_interactions(player1_id, player2_id);
        compatibility += (positive_interactions as f64 * 0.05).min(0.2);

        compatibility.min(1.0).max(0.0)
    }

    /// Count positive interactions between players
    fn count_positive_interactions(&self, player1_id: PlayerId, player2_id: PlayerId) -> u32 {
        self.interaction_history.get(&player1_id)
            .map(|interactions| interactions.iter()
                .filter(|i| i.other_player_id == player2_id && 
                           matches!(i.sentiment, InteractionSentiment::Positive | InteractionSentiment::VeryPositive))
                .count() as u32)
            .unwrap_or(0)
    }

    /// Record social interaction
    pub fn record_interaction(&mut self, player1_id: PlayerId, player2_id: PlayerId, 
                             interaction_type: InteractionType, sentiment: InteractionSentiment, 
                             impact: f64) {
        let interaction = SocialInteraction {
            interaction_id: Uuid::new_v4(),
            other_player_id: player2_id,
            interaction_type,
            timestamp: Utc::now(),
            context: None,
            sentiment,
            impact_score: impact,
        };

        self.interaction_history.entry(player1_id)
            .or_insert_with(Vec::new)
            .push(interaction);

        // Update friendship score if they're friends
        self.update_friendship_score(player1_id, player2_id, impact);
    }

    /// Update friendship score
    fn update_friendship_score(&mut self, player_id: PlayerId, friend_id: PlayerId, impact: f64) {
        if let Some(friendships) = self.friendships.get_mut(&player_id) {
            if let Some(friendship) = friendships.iter_mut().find(|f| f.friend_id == friend_id) {
                friendship.friendship_score = (friendship.friendship_score + impact).max(0.0).min(100.0);
                friendship.last_interaction = Utc::now();
                
                // Update friendship level based on score
                friendship.friendship_level = match friendship.friendship_score {
                    90.0..=100.0 => FriendshipLevel::SoulMate,
                    75.0..90.0 => FriendshipLevel::BestFriend,
                    50.0..75.0 => FriendshipLevel::GoodFriend,
                    25.0..50.0 => FriendshipLevel::Friend,
                    _ => FriendshipLevel::Acquaintance,
                };
            }
        }
    }

    /// Create a social circle
    pub fn create_social_circle(&mut self, creator_id: PlayerId, name: String, 
                               circle_type: CircleType, privacy_level: PrivacyLevel) -> Result<Uuid> {
        let circle_id = Uuid::new_v4();
        
        let circle = SocialCircle {
            circle_id,
            name,
            description: None,
            members: vec![creator_id],
            creator_id,
            created_date: Utc::now(),
            circle_type,
            privacy_level,
            activity_level: 0.0,
            common_interests: vec![],
        };

        self.social_circles.insert(circle_id, circle);
        Ok(circle_id)
    }

    /// Add member to social circle
    pub fn add_to_circle(&mut self, circle_id: Uuid, player_id: PlayerId) -> Result<()> {
        let circle = self.social_circles.get_mut(&circle_id)
            .ok_or_else(|| anyhow::anyhow!("Social circle not found"))?;

        if !circle.members.contains(&player_id) {
            circle.members.push(player_id);
        }

        Ok(())
    }

    /// Generate friend suggestions
    pub fn generate_friend_suggestions(&self, player_id: PlayerId, limit: usize) -> Vec<FriendSuggestion> {
        let mut suggestions = vec![];
        let current_friends: Vec<PlayerId> = self.get_friends(player_id)
            .into_iter()
            .map(|f| f.friend_id)
            .collect();

        // Mutual friend suggestions
        for friend_id in &current_friends {
            let mutual_friends = self.get_friends(*friend_id);
            for mutual_friend in mutual_friends {
                if mutual_friend.friend_id != player_id && 
                   !current_friends.contains(&mutual_friend.friend_id) &&
                   !self.is_blocked(player_id, mutual_friend.friend_id) {
                    
                    let suggestion = FriendSuggestion {
                        suggested_player_id: mutual_friend.friend_id,
                        suggestion_type: SuggestionType::MutualFriends,
                        confidence_score: 0.8,
                        mutual_friends: vec![*friend_id],
                        common_interests: vec![], // Would be calculated from profiles
                        compatibility_factors: vec![
                            CompatibilityFactor {
                                factor_type: "Mutual Friends".to_string(),
                                score: 0.8,
                                description: "You have friends in common".to_string(),
                            }
                        ],
                        suggestion_reason: "Friend of your friend".to_string(),
                    };
                    suggestions.push(suggestion);
                }
            }
        }

        // Return limited suggestions
        suggestions.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap());
        suggestions.into_iter().take(limit).collect()
    }

    /// Record shared activity
    pub fn record_shared_activity(&mut self, participants: Vec<PlayerId>, activity_type: ActivityType, 
                                 duration_minutes: u32) -> Result<()> {
        let activity_id = Uuid::new_v4();
        let activity = SharedActivity {
            activity_id,
            activity_type: activity_type.clone(),
            participants: participants.clone(),
            date: Utc::now(),
            duration_minutes,
            activity_rating: None,
            location: None,
        };

        // Add activity to all participants' friendship records
        for i in 0..participants.len() {
            for j in (i+1)..participants.len() {
                let player1 = participants[i];
                let player2 = participants[j];
                
                // Add to both players' friendship records if they're friends
                if let Some(friendships) = self.friendships.get_mut(&player1) {
                    if let Some(friendship) = friendships.iter_mut().find(|f| f.friend_id == player2) {
                        friendship.shared_activities.push(activity.clone());
                    }
                }
                
                // Record positive interaction
                self.record_interaction(player1, player2, InteractionType::Collaboration, 
                                      InteractionSentiment::Positive, 2.0);
            }
        }

        Ok(())
    }

    /// Get friendship statistics
    pub fn get_friendship_stats(&self, player_id: PlayerId) -> FriendshipStats {
        let friends = self.get_friends(player_id);
        let total_friends = friends.len();
        
        let avg_friendship_score = if total_friends > 0 {
            friends.iter().map(|f| f.friendship_score).sum::<f64>() / total_friends as f64
        } else {
            0.0
        };

        let friendship_levels: HashMap<String, u32> = friends.iter()
            .map(|f| format!("{:?}", f.friendship_level))
            .fold(HashMap::new(), |mut acc, level| {
                *acc.entry(level).or_insert(0) += 1;
                acc
            });

        FriendshipStats {
            total_friends: total_friends as u32,
            average_friendship_score: avg_friendship_score,
            friendship_level_distribution: friendship_levels,
            total_interactions: self.interaction_history.get(&player_id)
                .map(|hist| hist.len() as u32).unwrap_or(0),
            social_circles_count: self.social_circles.values()
                .filter(|circle| circle.members.contains(&player_id))
                .count() as u32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendshipStats {
    pub total_friends: u32,
    pub average_friendship_score: f64,
    pub friendship_level_distribution: HashMap<String, u32>,
    pub total_interactions: u32,
    pub social_circles_count: u32,
}

impl Default for FriendshipNetwork {
    fn default() -> Self {
        Self::new()
    }
}