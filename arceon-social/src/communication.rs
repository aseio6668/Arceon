/*!
# Communication System

Advanced messaging and communication channels for social interaction
including text, voice, and multimedia messaging with moderation features.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{PlayerId, MessageId};

/// Communication hub management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationHub {
    pub channels: HashMap<Uuid, Channel>,
    pub private_conversations: HashMap<ConversationId, PrivateConversation>,
    pub message_history: HashMap<PlayerId, Vec<MessageRecord>>,
    pub moderation_system: ModerationSystem,
    pub notification_settings: HashMap<PlayerId, NotificationSettings>,
}

pub type ConversationId = Uuid;

/// Communication channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: Uuid,
    pub name: String,
    pub description: String,
    pub channel_type: ChannelType,
    pub created_by: PlayerId,
    pub created_date: DateTime<Utc>,
    pub members: Vec<ChannelMember>,
    pub messages: Vec<Message>,
    pub settings: ChannelSettings,
    pub moderation_rules: Vec<ModerationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    Global,
    Regional,
    Guild,
    Party,
    Trade,
    Help,
    Social,
    Roleplay,
    Voice,
    Custom,
}

/// Channel member with permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMember {
    pub player_id: PlayerId,
    pub join_date: DateTime<Utc>,
    pub role: ChannelRole,
    pub permissions: Vec<ChannelPermission>,
    pub mute_status: Option<MuteStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelRole {
    Owner,
    Moderator,
    VIP,
    Member,
    Guest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelPermission {
    SendMessages,
    DeleteMessages,
    MuteMembers,
    KickMembers,
    InviteMembers,
    ManageChannel,
    ViewHistory,
    SendFiles,
    UseEmojis,
    MentionEveryone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuteStatus {
    pub muted_until: DateTime<Utc>,
    pub reason: String,
    pub muted_by: PlayerId,
}

/// Individual message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: MessageId,
    pub sender_id: PlayerId,
    pub content: MessageContent,
    pub timestamp: DateTime<Utc>,
    pub edited: bool,
    pub edit_history: Vec<MessageEdit>,
    pub reactions: Vec<MessageReaction>,
    pub mentions: Vec<PlayerId>,
    pub reply_to: Option<MessageId>,
    pub moderation_status: ModerationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text(String),
    Image(ImageMessage),
    File(FileMessage),
    Voice(VoiceMessage),
    Embed(EmbedMessage),
    System(SystemMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMessage {
    pub url: String,
    pub caption: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMessage {
    pub filename: String,
    pub file_type: String,
    pub file_size: u64,
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceMessage {
    pub audio_url: String,
    pub duration_seconds: u32,
    pub file_size: u64,
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedMessage {
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail: Option<String>,
    pub fields: Vec<EmbedField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    pub message_type: SystemMessageType,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemMessageType {
    PlayerJoined,
    PlayerLeft,
    PlayerPromoted,
    ChannelCreated,
    ChannelUpdated,
    GameEvent,
    Achievement,
    Trade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEdit {
    pub edit_timestamp: DateTime<Utc>,
    pub previous_content: String,
    pub edit_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    pub emoji: String,
    pub players: Vec<PlayerId>,
    pub count: u32,
}

/// Private conversation between players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateConversation {
    pub conversation_id: ConversationId,
    pub participants: Vec<PlayerId>,
    pub messages: Vec<Message>,
    pub created_date: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub settings: ConversationSettings,
    pub encryption_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSettings {
    pub notifications_enabled: bool,
    pub read_receipts: bool,
    pub typing_indicators: bool,
    pub message_retention_days: Option<u32>,
}

/// Channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSettings {
    pub public: bool,
    pub password_protected: bool,
    pub member_limit: Option<u32>,
    pub message_retention_days: u32,
    pub slow_mode_seconds: Option<u32>,
    pub auto_moderation: bool,
    pub profanity_filter: bool,
    pub link_filter: bool,
    pub emoji_only: bool,
    pub require_membership: bool,
}

/// Message moderation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationSystem {
    pub auto_moderation_rules: Vec<AutoModerationRule>,
    pub word_filters: Vec<WordFilter>,
    pub spam_detection: SpamDetection,
    pub moderation_logs: Vec<ModerationLog>,
    pub appeals: HashMap<Uuid, ModerationAppeal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoModerationRule {
    pub rule_id: Uuid,
    pub name: String,
    pub description: String,
    pub rule_type: ModerationRuleType,
    pub action: ModerationAction,
    pub severity: ModerationSeverity,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModerationRuleType {
    Profanity,
    Spam,
    Links,
    Mentions,
    Caps,
    Repetition,
    PersonalInfo,
    Harassment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModerationAction {
    Warn,
    DeleteMessage,
    MuteUser,
    KickUser,
    BanUser,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModerationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordFilter {
    pub filter_id: Uuid,
    pub pattern: String,
    pub replacement: Option<String>,
    pub severity: ModerationSeverity,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpamDetection {
    pub rate_limits: HashMap<PlayerId, RateLimit>,
    pub suspicious_patterns: Vec<SpamPattern>,
    pub threshold_settings: SpamThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub messages_per_minute: u32,
    pub reset_time: DateTime<Utc>,
    pub violations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpamPattern {
    pub pattern: String,
    pub confidence_threshold: f64,
    pub action: ModerationAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpamThresholds {
    pub messages_per_minute: u32,
    pub duplicate_message_count: u32,
    pub caps_percentage: f64,
    pub link_count_per_message: u32,
    pub mention_count_per_message: u32,
}

/// Moderation logging and tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationLog {
    pub log_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub moderator_id: Option<PlayerId>,
    pub target_player_id: PlayerId,
    pub action: ModerationAction,
    pub reason: String,
    pub evidence: Vec<String>,
    pub duration: Option<u64>, // Duration in seconds
    pub channel_id: Option<Uuid>,
    pub message_id: Option<MessageId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationAppeal {
    pub appeal_id: Uuid,
    pub player_id: PlayerId,
    pub moderation_log_id: Uuid,
    pub appeal_reason: String,
    pub submitted_date: DateTime<Utc>,
    pub status: AppealStatus,
    pub reviewed_by: Option<PlayerId>,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppealStatus {
    Pending,
    Approved,
    Rejected,
    UnderReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModerationStatus {
    Approved,
    Pending,
    Flagged,
    Removed,
    Hidden,
}

/// Moderation rules for channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationRule {
    pub rule_id: Uuid,
    pub description: String,
    pub rule_type: ModerationRuleType,
    pub enabled: bool,
    pub custom_parameters: HashMap<String, String>,
}

/// Notification settings for players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub direct_messages: bool,
    pub channel_mentions: bool,
    pub friend_requests: bool,
    pub guild_messages: bool,
    pub system_messages: bool,
    pub sound_notifications: bool,
    pub push_notifications: bool,
    pub email_notifications: bool,
    pub quiet_hours: Option<QuietHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuietHours {
    pub start_hour: u8,
    pub end_hour: u8,
    pub timezone: String,
}

/// Message record for history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRecord {
    pub message_id: MessageId,
    pub channel_id: Option<Uuid>,
    pub conversation_id: Option<ConversationId>,
    pub timestamp: DateTime<Utc>,
    pub message_type: String,
    pub word_count: u32,
}

impl CommunicationHub {
    /// Create new communication hub
    pub fn new() -> Self {
        let mut hub = Self {
            channels: HashMap::new(),
            private_conversations: HashMap::new(),
            message_history: HashMap::new(),
            moderation_system: ModerationSystem::new(),
            notification_settings: HashMap::new(),
        };

        hub.create_default_channels();
        hub
    }

    /// Create default channels
    fn create_default_channels(&mut self) {
        // Global chat channel
        let global_channel = Channel {
            channel_id: Uuid::new_v4(),
            name: "Global".to_string(),
            description: "Global chat for all players".to_string(),
            channel_type: ChannelType::Global,
            created_by: Uuid::new_v4(), // System user
            created_date: Utc::now(),
            members: vec![],
            messages: vec![],
            settings: ChannelSettings {
                public: true,
                password_protected: false,
                member_limit: None,
                message_retention_days: 7,
                slow_mode_seconds: None,
                auto_moderation: true,
                profanity_filter: true,
                link_filter: true,
                emoji_only: false,
                require_membership: false,
            },
            moderation_rules: vec![],
        };

        // Trade channel
        let trade_channel = Channel {
            channel_id: Uuid::new_v4(),
            name: "Trade".to_string(),
            description: "Trading and marketplace discussions".to_string(),
            channel_type: ChannelType::Trade,
            created_by: Uuid::new_v4(),
            created_date: Utc::now(),
            members: vec![],
            messages: vec![],
            settings: ChannelSettings {
                public: true,
                password_protected: false,
                member_limit: None,
                message_retention_days: 3,
                slow_mode_seconds: Some(30),
                auto_moderation: true,
                profanity_filter: true,
                link_filter: false,
                emoji_only: false,
                require_membership: false,
            },
            moderation_rules: vec![],
        };

        // Help channel
        let help_channel = Channel {
            channel_id: Uuid::new_v4(),
            name: "Help".to_string(),
            description: "Get help from other players and moderators".to_string(),
            channel_type: ChannelType::Help,
            created_by: Uuid::new_v4(),
            created_date: Utc::now(),
            members: vec![],
            messages: vec![],
            settings: ChannelSettings {
                public: true,
                password_protected: false,
                member_limit: None,
                message_retention_days: 14,
                slow_mode_seconds: None,
                auto_moderation: false,
                profanity_filter: false,
                link_filter: false,
                emoji_only: false,
                require_membership: false,
            },
            moderation_rules: vec![],
        };

        self.channels.insert(global_channel.channel_id, global_channel);
        self.channels.insert(trade_channel.channel_id, trade_channel);
        self.channels.insert(help_channel.channel_id, help_channel);
    }

    /// Send message to channel
    pub fn send_channel_message(&mut self, channel_id: Uuid, sender_id: PlayerId, content: String) -> Result<MessageId> {
        // Check if user can send messages
        if !self.can_send_message(channel_id, sender_id)? {
            return Err(anyhow::anyhow!("No permission to send messages"));
        }

        // Check rate limits
        if self.is_rate_limited(sender_id) {
            return Err(anyhow::anyhow!("Rate limit exceeded"));
        }

        // Extract data needed before mutable borrow
        let mentions = self.extract_mentions(&content);
        
        let message_id = Uuid::new_v4();
        let mut message = Message {
            message_id,
            sender_id,
            content: MessageContent::Text(content.clone()),
            timestamp: Utc::now(),
            edited: false,
            edit_history: vec![],
            reactions: vec![],
            mentions,
            reply_to: None,
            moderation_status: ModerationStatus::Approved,
        };

        // Apply moderation
        message.moderation_status = self.moderate_message(&message)?;

        // Now get mutable reference to channel
        let channel = self.channels.get_mut(&channel_id)
            .ok_or_else(|| anyhow::anyhow!("Channel not found"))?;
        
        if matches!(message.moderation_status, ModerationStatus::Approved | ModerationStatus::Pending) {
            channel.messages.push(message);
            
            // Record message in history
            self.record_message(sender_id, message_id, Some(channel_id), None);
            
            // Update rate limits
            self.update_rate_limits(sender_id);
        }

        Ok(message_id)
    }

    /// Send private message
    pub fn send_private_message(&mut self, sender_id: PlayerId, recipient_id: PlayerId, content: String) -> Result<MessageId> {
        // Find or create private conversation
        let conversation_id = self.find_or_create_conversation(vec![sender_id, recipient_id])?;
        
        let message_id = Uuid::new_v4();
        let message = Message {
            message_id,
            sender_id,
            content: MessageContent::Text(content),
            timestamp: Utc::now(),
            edited: false,
            edit_history: vec![],
            reactions: vec![],
            mentions: vec![],
            reply_to: None,
            moderation_status: ModerationStatus::Approved, // Private messages have lighter moderation
        };

        let conversation = self.private_conversations.get_mut(&conversation_id)
            .ok_or_else(|| anyhow::anyhow!("Conversation not found"))?;
        
        conversation.messages.push(message);
        conversation.last_activity = Utc::now();
        
        // Record message in history
        self.record_message(sender_id, message_id, None, Some(conversation_id));

        Ok(message_id)
    }

    /// Find or create private conversation
    fn find_or_create_conversation(&mut self, mut participants: Vec<PlayerId>) -> Result<ConversationId> {
        participants.sort();
        
        // Look for existing conversation
        for (conv_id, conversation) in &self.private_conversations {
            let mut conv_participants = conversation.participants.clone();
            conv_participants.sort();
            if conv_participants == participants {
                return Ok(*conv_id);
            }
        }

        // Create new conversation
        let conversation_id = Uuid::new_v4();
        let conversation = PrivateConversation {
            conversation_id,
            participants,
            messages: vec![],
            created_date: Utc::now(),
            last_activity: Utc::now(),
            settings: ConversationSettings {
                notifications_enabled: true,
                read_receipts: true,
                typing_indicators: true,
                message_retention_days: None,
            },
            encryption_key: None,
        };

        self.private_conversations.insert(conversation_id, conversation);
        Ok(conversation_id)
    }

    /// Check if user can send message to channel
    fn can_send_message(&self, channel_id: Uuid, player_id: PlayerId) -> Result<bool> {
        let channel = self.channels.get(&channel_id)
            .ok_or_else(|| anyhow::anyhow!("Channel not found"))?;

        // Check if user is a member
        let member = channel.members.iter()
            .find(|m| m.player_id == player_id);

        if let Some(member) = member {
            // Check if user is muted
            if let Some(mute) = &member.mute_status {
                if mute.muted_until > Utc::now() {
                    return Ok(false);
                }
            }
            
            // Check permissions
            Ok(member.permissions.contains(&ChannelPermission::SendMessages) || 
               matches!(member.role, ChannelRole::Owner | ChannelRole::Moderator))
        } else {
            // Public channels allow non-members to send messages
            Ok(channel.settings.public && !channel.settings.require_membership)
        }
    }

    /// Check rate limits
    fn is_rate_limited(&self, player_id: PlayerId) -> bool {
        if let Some(rate_limit) = self.moderation_system.spam_detection.rate_limits.get(&player_id) {
            if rate_limit.reset_time > Utc::now() {
                return rate_limit.messages_per_minute >= 
                       self.moderation_system.spam_detection.threshold_settings.messages_per_minute;
            }
        }
        false
    }

    /// Update rate limits
    fn update_rate_limits(&mut self, player_id: PlayerId) {
        let threshold = self.moderation_system.spam_detection.threshold_settings.messages_per_minute;
        let rate_limit = self.moderation_system.spam_detection.rate_limits
            .entry(player_id)
            .or_insert_with(|| RateLimit {
                messages_per_minute: 0,
                reset_time: Utc::now() + chrono::Duration::minutes(1),
                violations: 0,
            });

        if rate_limit.reset_time <= Utc::now() {
            rate_limit.messages_per_minute = 1;
            rate_limit.reset_time = Utc::now() + chrono::Duration::minutes(1);
        } else {
            rate_limit.messages_per_minute += 1;
        }

        // Check for violations
        if rate_limit.messages_per_minute > threshold {
            rate_limit.violations += 1;
        }
    }

    /// Moderate message content
    fn moderate_message(&self, message: &Message) -> Result<ModerationStatus> {
        if let MessageContent::Text(content) = &message.content {
            // Check word filters
            for filter in &self.moderation_system.word_filters {
                if content.to_lowercase().contains(&filter.pattern.to_lowercase()) {
                    match filter.severity {
                        ModerationSeverity::Critical => return Ok(ModerationStatus::Removed),
                        ModerationSeverity::High => return Ok(ModerationStatus::Flagged),
                        _ => {},
                    }
                }
            }

            // Check for spam patterns
            for pattern in &self.moderation_system.spam_detection.suspicious_patterns {
                if content.contains(&pattern.pattern) {
                    return Ok(ModerationStatus::Flagged);
                }
            }
        }

        Ok(ModerationStatus::Approved)
    }

    /// Extract mentions from message content
    fn extract_mentions(&self, _content: &str) -> Vec<PlayerId> {
        // Simple mention extraction (in real implementation, would parse @username patterns)
        vec![] // Placeholder
    }

    /// Record message in player's history
    fn record_message(&mut self, player_id: PlayerId, message_id: MessageId, 
                     channel_id: Option<Uuid>, conversation_id: Option<ConversationId>) {
        let record = MessageRecord {
            message_id,
            channel_id,
            conversation_id,
            timestamp: Utc::now(),
            message_type: "text".to_string(),
            word_count: 10, // Would be calculated from actual content
        };

        self.message_history.entry(player_id)
            .or_insert_with(Vec::new)
            .push(record);
    }

    /// Get message count for player
    pub fn get_message_count(&self, player_id: PlayerId) -> u64 {
        self.message_history.get(&player_id)
            .map(|history| history.len() as u64)
            .unwrap_or(0)
    }

    /// Create custom channel
    pub fn create_channel(&mut self, creator_id: PlayerId, name: String, description: String, 
                         channel_type: ChannelType) -> Result<Uuid> {
        let channel_id = Uuid::new_v4();
        
        let creator_member = ChannelMember {
            player_id: creator_id,
            join_date: Utc::now(),
            role: ChannelRole::Owner,
            permissions: vec![
                ChannelPermission::SendMessages,
                ChannelPermission::DeleteMessages,
                ChannelPermission::MuteMembers,
                ChannelPermission::KickMembers,
                ChannelPermission::InviteMembers,
                ChannelPermission::ManageChannel,
            ],
            mute_status: None,
        };

        let channel = Channel {
            channel_id,
            name,
            description,
            channel_type,
            created_by: creator_id,
            created_date: Utc::now(),
            members: vec![creator_member],
            messages: vec![],
            settings: ChannelSettings {
                public: false,
                password_protected: false,
                member_limit: Some(50),
                message_retention_days: 30,
                slow_mode_seconds: None,
                auto_moderation: true,
                profanity_filter: true,
                link_filter: false,
                emoji_only: false,
                require_membership: true,
            },
            moderation_rules: vec![],
        };

        self.channels.insert(channel_id, channel);
        Ok(channel_id)
    }

    /// Join channel
    pub fn join_channel(&mut self, channel_id: Uuid, player_id: PlayerId) -> Result<()> {
        let channel = self.channels.get_mut(&channel_id)
            .ok_or_else(|| anyhow::anyhow!("Channel not found"))?;

        // Check if already a member
        if channel.members.iter().any(|m| m.player_id == player_id) {
            return Err(anyhow::anyhow!("Already a member of this channel"));
        }

        // Check member limit
        if let Some(limit) = channel.settings.member_limit {
            if channel.members.len() >= limit as usize {
                return Err(anyhow::anyhow!("Channel is full"));
            }
        }

        let member = ChannelMember {
            player_id,
            join_date: Utc::now(),
            role: ChannelRole::Member,
            permissions: vec![ChannelPermission::SendMessages, ChannelPermission::ViewHistory],
            mute_status: None,
        };

        channel.members.push(member);
        Ok(())
    }

    /// Leave channel
    pub fn leave_channel(&mut self, channel_id: Uuid, player_id: PlayerId) -> Result<()> {
        let channel = self.channels.get_mut(&channel_id)
            .ok_or_else(|| anyhow::anyhow!("Channel not found"))?;

        let member_index = channel.members.iter().position(|m| m.player_id == player_id)
            .ok_or_else(|| anyhow::anyhow!("Not a member of this channel"))?;

        // Don't allow owner to leave unless transferring ownership
        if matches!(channel.members[member_index].role, ChannelRole::Owner) {
            return Err(anyhow::anyhow!("Channel owner cannot leave without transferring ownership"));
        }

        channel.members.remove(member_index);
        Ok(())
    }

    /// Get player's notification settings
    pub fn get_notification_settings(&self, player_id: PlayerId) -> NotificationSettings {
        self.notification_settings.get(&player_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Update notification settings
    pub fn update_notification_settings(&mut self, player_id: PlayerId, settings: NotificationSettings) {
        self.notification_settings.insert(player_id, settings);
    }
}

impl ModerationSystem {
    /// Create new moderation system
    fn new() -> Self {
        Self {
            auto_moderation_rules: vec![],
            word_filters: Self::create_default_filters(),
            spam_detection: SpamDetection {
                rate_limits: HashMap::new(),
                suspicious_patterns: vec![],
                threshold_settings: SpamThresholds {
                    messages_per_minute: 10,
                    duplicate_message_count: 3,
                    caps_percentage: 0.8,
                    link_count_per_message: 3,
                    mention_count_per_message: 5,
                },
            },
            moderation_logs: vec![],
            appeals: HashMap::new(),
        }
    }

    /// Create default word filters
    fn create_default_filters() -> Vec<WordFilter> {
        vec![
            WordFilter {
                filter_id: Uuid::new_v4(),
                pattern: "spam".to_string(),
                replacement: Some("****".to_string()),
                severity: ModerationSeverity::Medium,
                category: "spam".to_string(),
            },
            // More filters would be added here in a real implementation
        ]
    }
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            direct_messages: true,
            channel_mentions: true,
            friend_requests: true,
            guild_messages: true,
            system_messages: true,
            sound_notifications: true,
            push_notifications: false,
            email_notifications: false,
            quiet_hours: None,
        }
    }
}

impl Default for CommunicationHub {
    fn default() -> Self {
        Self::new()
    }
}