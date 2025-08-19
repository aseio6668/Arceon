use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;
use std::sync::Arc;

/// Group management system for small party play (up to 8 players)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupManager {
    pub groups: HashMap<Uuid, Group>,
    pub player_groups: HashMap<Uuid, Uuid>, // player_id -> group_id
    pub group_invitations: HashMap<Uuid, Vec<GroupInvitation>>, // player_id -> invitations
    pub config: GroupConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub group_id: Uuid,
    pub name: String,
    pub leader_id: Uuid,
    pub members: Vec<GroupMember>,
    pub created_at: SystemTime,
    pub group_type: GroupType,
    pub status: GroupStatus,
    pub settings: GroupSettings,
    pub chat_channel: ChatChannel,
    pub activities: GroupActivities,
    pub permissions: GroupPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub player_id: Uuid,
    pub username: String,
    pub joined_at: SystemTime,
    pub role: GroupRole,
    pub status: MemberStatus,
    pub last_active: SystemTime,
    pub contributions: MemberContributions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupType {
    Adventure,     // General adventuring group
    Dungeon,       // Dungeon exploration
    Trading,       // Merchant trading group
    Crafting,      // Crafting collaboration
    Exploration,   // World exploration
    Social,        // Social gathering
    Combat,        // PvP/Combat focused
    Learning,      // Skill learning group
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupStatus {
    Forming,       // Accepting new members
    Active,        // Currently active
    Inactive,      // Temporarily inactive
    Disbanded,     // Group disbanded
    OnMission,     // Currently on a mission/activity
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupRole {
    Leader,        // Group leader (1 per group)
    Officer,       // Group officer (can invite/kick)
    Member,        // Regular member
    Trainee,       // New/learning member
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberStatus {
    Online,
    Offline,
    Away,
    Busy,
    InCombat,
    Trading,
    Crafting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSettings {
    pub is_public: bool,
    pub auto_accept_invites: bool,
    pub max_members: u8,
    pub min_level_requirement: Option<u32>,
    pub activity_focus: Vec<GroupType>,
    pub region_restriction: Option<String>,
    pub voice_chat_enabled: bool,
    pub loot_distribution: LootDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LootDistribution {
    LeaderDecides,
    RoundRobin,
    NeedRoll,
    Equal,
    ContributionBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannel {
    pub channel_id: Uuid,
    pub channel_name: String,
    pub messages: Vec<ChatMessage>,
    pub message_history_limit: usize,
    pub filters: ChatFilters,
    pub permissions: ChatPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: Uuid,
    pub sender_id: Uuid,
    pub sender_username: String,
    pub content: String,
    pub timestamp: SystemTime,
    pub message_type: MessageType,
    pub mentions: Vec<Uuid>,
    pub edited: bool,
    pub edited_at: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    System,
    Activity,
    Location,
    Trade,
    Combat,
    Achievement,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFilters {
    pub profanity_filter: bool,
    pub link_filter: bool,
    pub spam_protection: bool,
    pub mention_notifications: bool,
    pub blocked_users: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPermissions {
    pub who_can_send: ChatSendPermission,
    pub who_can_mention: ChatMentionPermission,
    pub message_cooldown: u64, // seconds
    pub max_message_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatSendPermission {
    Everyone,
    MembersOnly,
    OfficersAndAbove,
    LeaderOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatMentionPermission {
    Everyone,
    MembersOnly,
    OfficersOnly,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupActivities {
    pub current_activity: Option<GroupActivity>,
    pub activity_history: Vec<CompletedActivity>,
    pub scheduled_activities: Vec<ScheduledActivity>,
    pub activity_preferences: Vec<GroupType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupActivity {
    pub activity_id: Uuid,
    pub activity_type: GroupType,
    pub name: String,
    pub description: String,
    pub started_at: SystemTime,
    pub estimated_duration: Option<u64>, // minutes
    pub location: Option<String>,
    pub objectives: Vec<ActivityObjective>,
    pub participants: Vec<Uuid>,
    pub leader: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityObjective {
    pub objective_id: Uuid,
    pub description: String,
    pub completed: bool,
    pub completed_by: Option<Uuid>,
    pub completed_at: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedActivity {
    pub activity: GroupActivity,
    pub completed_at: SystemTime,
    pub success: bool,
    pub rewards: Vec<ActivityReward>,
    pub participants: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledActivity {
    pub activity: GroupActivity,
    pub scheduled_for: SystemTime,
    pub created_by: Uuid,
    pub confirmed_participants: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReward {
    pub reward_type: String,
    pub reward_value: String,
    pub recipient: Option<Uuid>, // None means for all participants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPermissions {
    pub can_invite: Vec<GroupRole>,
    pub can_kick: Vec<GroupRole>,
    pub can_promote: Vec<GroupRole>,
    pub can_change_settings: Vec<GroupRole>,
    pub can_start_activity: Vec<GroupRole>,
    pub can_manage_chat: Vec<GroupRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberContributions {
    pub activities_participated: u32,
    pub messages_sent: u32,
    pub time_in_group: u64, // seconds
    pub leadership_actions: u32,
    pub helpful_actions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInvitation {
    pub invitation_id: Uuid,
    pub group_id: Uuid,
    pub group_name: String,
    pub inviter_id: Uuid,
    pub inviter_username: String,
    pub invited_player_id: Uuid,
    pub message: Option<String>,
    pub sent_at: SystemTime,
    pub expires_at: SystemTime,
    pub status: InvitationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
    pub max_group_size: u8,
    pub max_groups_per_player: u8,
    pub invitation_timeout_hours: u64,
    pub max_chat_history: usize,
    pub auto_disband_inactive_days: u64,
    pub min_members_to_prevent_disband: u8,
    pub activity_timeout_hours: u64,
}

// Request/Response structures for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub group_type: GroupType,
    pub description: Option<String>,
    pub is_public: bool,
    pub max_members: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupResponse {
    pub success: bool,
    pub group_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitePlayerRequest {
    pub group_id: Uuid,
    pub player_username: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitePlayerResponse {
    pub success: bool,
    pub invitation_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGroupRequest {
    pub invitation_id: Uuid,
    pub accept: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGroupResponse {
    pub success: bool,
    pub group_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGroupMessageRequest {
    pub group_id: Uuid,
    pub content: String,
    pub message_type: Option<MessageType>,
    pub mentions: Option<Vec<String>>, // usernames to mention
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGroupMessageResponse {
    pub success: bool,
    pub message_id: Option<Uuid>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupStatusResponse {
    pub success: bool,
    pub groups: Vec<GroupSummary>,
    pub invitations: Vec<GroupInvitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSummary {
    pub group_id: Uuid,
    pub name: String,
    pub group_type: GroupType,
    pub member_count: u8,
    pub max_members: u8,
    pub status: GroupStatus,
    pub is_leader: bool,
    pub role: GroupRole,
    pub last_activity: SystemTime,
}

impl GroupManager {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            player_groups: HashMap::new(),
            group_invitations: HashMap::new(),
            config: GroupConfig::default(),
        }
    }

    /// Create a new group
    pub async fn create_group(&mut self, leader_id: Uuid, leader_username: String, request: CreateGroupRequest) -> Result<CreateGroupResponse> {
        info!("🎯 Creating group '{}' with leader: {}", request.name, leader_username);

        // Check if player is already in maximum groups
        let player_group_count = self.get_player_group_count(leader_id);
        if player_group_count >= self.config.max_groups_per_player {
            return Ok(CreateGroupResponse {
                success: false,
                group_id: None,
                message: format!("You can only be in {} groups at once", self.config.max_groups_per_player),
            });
        }

        // Validate group settings
        let max_members = request.max_members.unwrap_or(self.config.max_group_size).min(self.config.max_group_size);

        let group_id = Uuid::new_v4();
        let chat_channel_id = Uuid::new_v4();

        // Create group leader member
        let leader_member = GroupMember {
            player_id: leader_id,
            username: leader_username.clone(),
            joined_at: SystemTime::now(),
            role: GroupRole::Leader,
            status: MemberStatus::Online,
            last_active: SystemTime::now(),
            contributions: MemberContributions::default(),
        };

        // Create chat channel
        let chat_channel = ChatChannel {
            channel_id: chat_channel_id,
            channel_name: format!("{} Chat", request.name),
            messages: Vec::new(),
            message_history_limit: self.config.max_chat_history,
            filters: ChatFilters::default(),
            permissions: ChatPermissions::default(),
        };

        // Create group
        let group_type_clone = request.group_type.clone();
        let group = Group {
            group_id,
            name: request.name.clone(),
            leader_id,
            members: vec![leader_member],
            created_at: SystemTime::now(),
            group_type: request.group_type,
            status: GroupStatus::Forming,
            settings: GroupSettings {
                is_public: request.is_public,
                auto_accept_invites: false,
                max_members,
                min_level_requirement: None,
                activity_focus: vec![group_type_clone],
                region_restriction: None,
                voice_chat_enabled: false,
                loot_distribution: LootDistribution::LeaderDecides,
            },
            chat_channel,
            activities: GroupActivities::default(),
            permissions: GroupPermissions::default(),
        };

        // Store group and update mappings
        self.groups.insert(group_id, group);
        self.player_groups.insert(leader_id, group_id);

        // Send system message to group chat
        self.send_system_message(group_id, format!("Group '{}' created by {}", request.name, leader_username)).await?;

        info!("✅ Group created successfully: {} (ID: {})", request.name, group_id);

        Ok(CreateGroupResponse {
            success: true,
            group_id: Some(group_id),
            message: format!("Group '{}' created successfully!", request.name),
        })
    }

    /// Invite a player to a group
    pub async fn invite_player(&mut self, inviter_id: Uuid, player_id: Uuid, player_username: String, request: InvitePlayerRequest) -> Result<InvitePlayerResponse> {
        info!("📨 Inviting player {} to group {}", player_username, request.group_id);

        // Check if group exists and inviter has permission
        let group = match self.groups.get(&request.group_id) {
            Some(group) => group.clone(),
            None => return Ok(InvitePlayerResponse {
                success: false,
                invitation_id: None,
                message: "Group not found".to_string(),
            }),
        };

        // Check if inviter has permission to invite
        let inviter_member = group.members.iter().find(|m| m.player_id == inviter_id);
        match inviter_member {
            Some(member) => {
                if !group.permissions.can_invite.contains(&member.role) {
                    return Ok(InvitePlayerResponse {
                        success: false,
                        invitation_id: None,
                        message: "You don't have permission to invite players".to_string(),
                    });
                }
            },
            None => return Ok(InvitePlayerResponse {
                success: false,
                invitation_id: None,
                message: "You are not a member of this group".to_string(),
            }),
        }

        // Check if group is full
        if group.members.len() >= group.settings.max_members as usize {
            return Ok(InvitePlayerResponse {
                success: false,
                invitation_id: None,
                message: "Group is full".to_string(),
            });
        }

        // Check if player is already in group
        if group.members.iter().any(|m| m.player_id == player_id) {
            return Ok(InvitePlayerResponse {
                success: false,
                invitation_id: None,
                message: "Player is already in this group".to_string(),
            });
        }

        // Check if player already has pending invitation
        if let Some(invitations) = self.group_invitations.get(&player_id) {
            if invitations.iter().any(|inv| inv.group_id == request.group_id && inv.status == InvitationStatus::Pending) {
                return Ok(InvitePlayerResponse {
                    success: false,
                    invitation_id: None,
                    message: "Player already has a pending invitation to this group".to_string(),
                });
            }
        }

        // Create invitation
        let invitation_id = Uuid::new_v4();
        let invitation = GroupInvitation {
            invitation_id,
            group_id: request.group_id,
            group_name: group.name.clone(),
            inviter_id,
            inviter_username: inviter_member.unwrap().username.clone(),
            invited_player_id: player_id,
            message: request.message,
            sent_at: SystemTime::now(),
            expires_at: SystemTime::now() + std::time::Duration::from_secs(self.config.invitation_timeout_hours * 3600),
            status: InvitationStatus::Pending,
        };

        // Store invitation
        self.group_invitations.entry(player_id).or_insert_with(Vec::new).push(invitation);

        // Send notification to group chat
        self.send_system_message(request.group_id, format!("{} has been invited to the group", player_username)).await?;

        info!("✅ Invitation sent: {} -> {} (Group: {})", inviter_member.unwrap().username, player_username, group.name);

        Ok(InvitePlayerResponse {
            success: true,
            invitation_id: Some(invitation_id),
            message: format!("Invitation sent to {}", player_username),
        })
    }

    /// Accept or decline group invitation
    pub async fn respond_to_invitation(&mut self, player_id: Uuid, player_username: String, request: JoinGroupRequest) -> Result<JoinGroupResponse> {
        info!("📬 Player {} responding to invitation {}: {}", player_username, request.invitation_id, if request.accept { "ACCEPT" } else { "DECLINE" });

        // Find invitation
        let invitation = if let Some(invitations) = self.group_invitations.get_mut(&player_id) {
            if let Some(pos) = invitations.iter().position(|inv| inv.invitation_id == request.invitation_id) {
                invitations.remove(pos)
            } else {
                return Ok(JoinGroupResponse {
                    success: false,
                    group_id: None,
                    message: "Invitation not found".to_string(),
                });
            }
        } else {
            return Ok(JoinGroupResponse {
                success: false,
                group_id: None,
                message: "No invitations found".to_string(),
            });
        };

        // Check if invitation is still valid
        if invitation.status != InvitationStatus::Pending {
            return Ok(JoinGroupResponse {
                success: false,
                group_id: None,
                message: "Invitation is no longer valid".to_string(),
            });
        }

        if SystemTime::now() > invitation.expires_at {
            return Ok(JoinGroupResponse {
                success: false,
                group_id: None,
                message: "Invitation has expired".to_string(),
            });
        }

        if !request.accept {
            // Declined invitation
            self.send_system_message(invitation.group_id, format!("{} declined the group invitation", player_username)).await?;
            return Ok(JoinGroupResponse {
                success: true,
                group_id: None,
                message: "Invitation declined".to_string(),
            });
        }

        // Accept invitation - add player to group
        let group = match self.groups.get_mut(&invitation.group_id) {
            Some(group) => group,
            None => return Ok(JoinGroupResponse {
                success: false,
                group_id: None,
                message: "Group no longer exists".to_string(),
            }),
        };

        // Final checks
        if group.members.len() >= group.settings.max_members as usize {
            return Ok(JoinGroupResponse {
                success: false,
                group_id: None,
                message: "Group is now full".to_string(),
            });
        }

        if group.members.iter().any(|m| m.player_id == player_id) {
            return Ok(JoinGroupResponse {
                success: false,
                group_id: None,
                message: "You are already in this group".to_string(),
            });
        }

        // Add member to group
        let new_member = GroupMember {
            player_id,
            username: player_username.clone(),
            joined_at: SystemTime::now(),
            role: GroupRole::Member,
            status: MemberStatus::Online,
            last_active: SystemTime::now(),
            contributions: MemberContributions::default(),
        };

        group.members.push(new_member);
        self.player_groups.insert(player_id, invitation.group_id);

        // Update group status if needed
        if group.status == GroupStatus::Forming && group.members.len() >= 2 {
            group.status = GroupStatus::Active;
        }

        // Store group name before sending message
        let group_name = group.name.clone();
        let group_id = invitation.group_id;

        // Send welcome message
        self.send_system_message(group_id, format!("{} has joined the group! Welcome!", player_username)).await?;

        info!("✅ Player {} joined group: {}", player_username, group_name);

        Ok(JoinGroupResponse {
            success: true,
            group_id: Some(group_id),
            message: format!("Successfully joined group '{}'!", group_name),
        })
    }

    /// Send message to group chat
    pub async fn send_message(&mut self, sender_id: Uuid, request: SendGroupMessageRequest) -> Result<SendGroupMessageResponse> {
        // Check if group exists and sender is member
        let group = match self.groups.get_mut(&request.group_id) {
            Some(group) => group,
            None => return Ok(SendGroupMessageResponse {
                success: false,
                message_id: None,
                message: "Group not found".to_string(),
            }),
        };

        let sender_member = match group.members.iter().find(|m| m.player_id == sender_id) {
            Some(member) => member.clone(),
            None => return Ok(SendGroupMessageResponse {
                success: false,
                message_id: None,
                message: "You are not a member of this group".to_string(),
            }),
        };

        // Check chat permissions
        let can_send = match group.chat_channel.permissions.who_can_send {
            ChatSendPermission::Everyone => true,
            ChatSendPermission::MembersOnly => true,
            ChatSendPermission::OfficersAndAbove => matches!(sender_member.role, GroupRole::Leader | GroupRole::Officer),
            ChatSendPermission::LeaderOnly => matches!(sender_member.role, GroupRole::Leader),
        };

        if !can_send {
            return Ok(SendGroupMessageResponse {
                success: false,
                message_id: None,
                message: "You don't have permission to send messages".to_string(),
            });
        }

        // Validate message content
        if request.content.trim().is_empty() {
            return Ok(SendGroupMessageResponse {
                success: false,
                message_id: None,
                message: "Message cannot be empty".to_string(),
            });
        }

        if request.content.len() > group.chat_channel.permissions.max_message_length {
            return Ok(SendGroupMessageResponse {
                success: false,
                message_id: None,
                message: format!("Message too long (max {} characters)", group.chat_channel.permissions.max_message_length),
            });
        }

        // Process mentions
        let mut mentions = Vec::new();
        if let Some(mention_usernames) = request.mentions {
            for username in mention_usernames {
                if let Some(mentioned_member) = group.members.iter().find(|m| m.username.to_lowercase() == username.to_lowercase()) {
                    mentions.push(mentioned_member.player_id);
                }
            }
        }

        // Create message
        let message_id = Uuid::new_v4();
        let message = ChatMessage {
            message_id,
            sender_id,
            sender_username: sender_member.username.clone(),
            content: request.content.clone(),
            timestamp: SystemTime::now(),
            message_type: request.message_type.unwrap_or(MessageType::Text),
            mentions,
            edited: false,
            edited_at: None,
        };

        // Add to chat history
        group.chat_channel.messages.push(message);

        // Maintain message history limit
        if group.chat_channel.messages.len() > group.chat_channel.message_history_limit {
            group.chat_channel.messages.remove(0);
        }

        // Update sender's last active time and contribution
        if let Some(member) = group.members.iter_mut().find(|m| m.player_id == sender_id) {
            member.last_active = SystemTime::now();
            member.contributions.messages_sent += 1;
        }

        info!("💬 Group message sent: {} in {}: {}", sender_member.username, group.name, request.content);

        Ok(SendGroupMessageResponse {
            success: true,
            message_id: Some(message_id),
            message: "Message sent successfully".to_string(),
        })
    }

    /// Get player's group status and invitations
    pub async fn get_player_status(&self, player_id: Uuid) -> Result<GroupStatusResponse> {
        let mut groups = Vec::new();
        let mut invitations = Vec::new();

        // Get player's groups
        for (group_id, group) in &self.groups {
            if let Some(member) = group.members.iter().find(|m| m.player_id == player_id) {
                groups.push(GroupSummary {
                    group_id: *group_id,
                    name: group.name.clone(),
                    group_type: group.group_type.clone(),
                    member_count: group.members.len() as u8,
                    max_members: group.settings.max_members,
                    status: group.status.clone(),
                    is_leader: group.leader_id == player_id,
                    role: member.role.clone(),
                    last_activity: group.activities.current_activity.as_ref()
                        .map(|a| a.started_at)
                        .unwrap_or(group.created_at),
                });
            }
        }

        // Get player's invitations
        if let Some(player_invitations) = self.group_invitations.get(&player_id) {
            for invitation in player_invitations {
                if invitation.status == InvitationStatus::Pending && SystemTime::now() <= invitation.expires_at {
                    invitations.push(invitation.clone());
                }
            }
        }

        Ok(GroupStatusResponse {
            success: true,
            groups,
            invitations,
        })
    }

    /// Leave a group
    pub async fn leave_group(&mut self, player_id: Uuid, group_id: Uuid) -> Result<bool> {
        // First, get group data without mutable borrow
        let (leaving_member, is_leader, group_name, new_leader_info) = {
            let group = match self.groups.get(&group_id) {
                Some(group) => group,
                None => return Ok(false),
            };

            // Find member
            let member_pos = match group.members.iter().position(|m| m.player_id == player_id) {
                Some(pos) => pos,
                None => return Ok(false),
            };

            let leaving_member = group.members[member_pos].clone();
            let is_leader = group.leader_id == player_id;
            let group_name = group.name.clone();

            // If leader is leaving, find new leader
            let new_leader_info = if is_leader && group.members.len() > 1 {
                group.members.iter()
                    .find(|m| m.player_id != player_id && m.role == GroupRole::Officer)
                    .or_else(|| group.members.iter().find(|m| m.player_id != player_id))
                    .map(|m| (m.player_id, m.username.clone()))
            } else {
                None
            };

            (leaving_member, is_leader, group_name, new_leader_info)
        };

        // Now modify the group with mutable borrow
        let group = self.groups.get_mut(&group_id).unwrap();
        
        // Remove member
        let member_pos = group.members.iter().position(|m| m.player_id == player_id).unwrap();
        group.members.remove(member_pos);
        self.player_groups.remove(&player_id);

        // Handle leadership transfer and prepare messages
        let messages_to_send = if let Some((new_leader_id, new_leader_username)) = new_leader_info {
            group.leader_id = new_leader_id;
            if let Some(new_leader) = group.members.iter_mut().find(|m| m.player_id == new_leader_id) {
                new_leader.role = GroupRole::Leader;
            }

            vec![format!("{} has left the group. {} is now the leader.", leaving_member.username, new_leader_username)]
        } else if is_leader {
            vec![format!("{} has left the group.", leaving_member.username)]
        } else {
            vec![]
        };

        // Check if group should be disbanded
        let should_disband = group.members.len() < self.config.min_members_to_prevent_disband as usize;
        let has_members = !group.members.is_empty();

        let mut all_messages = messages_to_send;
        
        if should_disband {
            group.status = GroupStatus::Disbanded;
            if has_members {
                all_messages.push("Group has been disbanded due to insufficient members.".to_string());
            }
        }

        // Drop the mutable borrow before sending messages
        drop(group);

        // Send all messages
        for message in all_messages {
            self.send_system_message(group_id, message).await?;
        }

        info!("👋 Player {} left group: {}", leaving_member.username, group_name);
        Ok(true)
    }

    // Helper methods

    fn get_player_group_count(&self, player_id: Uuid) -> u8 {
        self.groups.values()
            .filter(|group| group.members.iter().any(|m| m.player_id == player_id))
            .count() as u8
    }

    async fn send_system_message(&mut self, group_id: Uuid, content: String) -> Result<()> {
        if let Some(group) = self.groups.get_mut(&group_id) {
            let message = ChatMessage {
                message_id: Uuid::new_v4(),
                sender_id: Uuid::nil(), // System messages use nil UUID
                sender_username: "System".to_string(),
                content,
                timestamp: SystemTime::now(),
                message_type: MessageType::System,
                mentions: Vec::new(),
                edited: false,
                edited_at: None,
            };

            group.chat_channel.messages.push(message);

            // Maintain history limit
            if group.chat_channel.messages.len() > group.chat_channel.message_history_limit {
                group.chat_channel.messages.remove(0);
            }
        }
        Ok(())
    }

    /// Cleanup expired invitations and inactive groups
    pub async fn cleanup(&mut self) -> Result<()> {
        let now = SystemTime::now();

        // Clean up expired invitations
        for invitations in self.group_invitations.values_mut() {
            invitations.retain(|inv| {
                if now > inv.expires_at && inv.status == InvitationStatus::Pending {
                    false // Remove expired invitations
                } else {
                    true
                }
            });
        }

        // Remove empty invitation lists
        self.group_invitations.retain(|_, invitations| !invitations.is_empty());

        // Clean up disbanded groups
        let disbanded_groups: Vec<Uuid> = self.groups.iter()
            .filter(|(_, group)| group.status == GroupStatus::Disbanded)
            .map(|(id, _)| *id)
            .collect();

        for group_id in disbanded_groups {
            if let Some(group) = self.groups.remove(&group_id) {
                // Remove all member mappings
                for member in group.members {
                    self.player_groups.remove(&member.player_id);
                }
                info!("🗑️ Cleaned up disbanded group: {}", group.name);
            }
        }

        Ok(())
    }
}

impl Default for GroupConfig {
    fn default() -> Self {
        Self {
            max_group_size: 8,
            max_groups_per_player: 3,
            invitation_timeout_hours: 24,
            max_chat_history: 100,
            auto_disband_inactive_days: 7,
            min_members_to_prevent_disband: 2,
            activity_timeout_hours: 4,
        }
    }
}

impl Default for GroupActivities {
    fn default() -> Self {
        Self {
            current_activity: None,
            activity_history: Vec::new(),
            scheduled_activities: Vec::new(),
            activity_preferences: Vec::new(),
        }
    }
}

impl Default for GroupPermissions {
    fn default() -> Self {
        Self {
            can_invite: vec![GroupRole::Leader, GroupRole::Officer],
            can_kick: vec![GroupRole::Leader, GroupRole::Officer],
            can_promote: vec![GroupRole::Leader],
            can_change_settings: vec![GroupRole::Leader],
            can_start_activity: vec![GroupRole::Leader, GroupRole::Officer, GroupRole::Member],
            can_manage_chat: vec![GroupRole::Leader, GroupRole::Officer],
        }
    }
}

impl Default for ChatFilters {
    fn default() -> Self {
        Self {
            profanity_filter: true,
            link_filter: true,
            spam_protection: true,
            mention_notifications: true,
            blocked_users: Vec::new(),
        }
    }
}

impl Default for ChatPermissions {
    fn default() -> Self {
        Self {
            who_can_send: ChatSendPermission::MembersOnly,
            who_can_mention: ChatMentionPermission::MembersOnly,
            message_cooldown: 1, // 1 second
            max_message_length: 500,
        }
    }
}

impl Default for MemberContributions {
    fn default() -> Self {
        Self {
            activities_participated: 0,
            messages_sent: 0,
            time_in_group: 0,
            leadership_actions: 0,
            helpful_actions: 0,
        }
    }
}

/// Thread-safe group manager
pub type SharedGroupManager = Arc<RwLock<GroupManager>>;

pub fn create_shared_group_manager() -> SharedGroupManager {
    Arc::new(RwLock::new(GroupManager::new()))
}