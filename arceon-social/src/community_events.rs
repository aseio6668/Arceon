/*!
# Community Events

Event coordination and management system for social activities,
competitions, and community gatherings.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use anyhow::Result;

use crate::{PlayerId, EventId, GuildId};

/// Event coordination system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCoordinator {
    pub events: HashMap<EventId, Event>,
    pub event_templates: Vec<EventTemplate>,
    pub participant_history: HashMap<PlayerId, Vec<EventParticipation>>,
    pub event_calendar: EventCalendar,
    pub recurring_events: HashMap<Uuid, RecurringEvent>,
}

/// Community event definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: EventId,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub organizer_id: PlayerId,
    pub co_organizers: Vec<PlayerId>,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub location: EventLocation,
    pub participants: HashMap<PlayerId, ParticipantStatus>,
    pub requirements: Vec<EventRequirement>,
    pub rewards: Vec<EventReward>,
    pub rules: Vec<String>,
    pub status: EventStatus,
    pub capacity: EventCapacity,
    pub registration: RegistrationSettings,
    pub visibility: EventVisibility,
    pub tags: Vec<String>,
    pub created_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Social,
    Competition,
    Tournament,
    Workshop,
    Quest,
    Raid,
    PvP,
    Building,
    Trading,
    Celebration,
    Educational,
    Roleplay,
    Festival,
    Race,
    Auction,
    Meeting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventLocation {
    InGame(GameLocation),
    Virtual(VirtualLocation),
    External(ExternalLocation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameLocation {
    pub world_id: Uuid,
    pub coordinates: (f64, f64, f64),
    pub area_name: String,
    pub instance_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualLocation {
    pub platform: String,
    pub room_id: String,
    pub access_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalLocation {
    pub platform: String,
    pub details: String,
    pub contact_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantStatus {
    Registered,
    Confirmed,
    Attended,
    NoShow,
    Cancelled,
    Waitlisted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRequirement {
    pub requirement_type: RequirementType,
    pub description: String,
    pub mandatory: bool,
    pub verification_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequirementType {
    Level(u32),
    Skill(String),
    Item(String),
    Achievement(String),
    GuildMembership(GuildId),
    FriendConnection(PlayerId),
    Experience(u64),
    Reputation(f64),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReward {
    pub reward_type: RewardType,
    pub amount: u64,
    pub description: String,
    pub distribution_method: DistributionMethod,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardType {
    Experience,
    Currency,
    Items,
    Titles,
    Badges,
    Reputation,
    SkillPoints,
    Access,
    Recognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionMethod {
    All,
    TopPerformers(u32),
    Random(u32),
    Graduated,
    Participation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    Planned,
    Registration,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
    Postponed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCapacity {
    pub minimum_participants: u32,
    pub maximum_participants: Option<u32>,
    pub optimal_participants: Option<u32>,
    pub waiting_list_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationSettings {
    pub registration_start: DateTime<Utc>,
    pub registration_end: DateTime<Utc>,
    pub approval_required: bool,
    pub fee_required: Option<RegistrationFee>,
    pub cancellation_policy: CancellationPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationFee {
    pub amount: u64,
    pub currency: String,
    pub refundable: bool,
    pub refund_deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancellationPolicy {
    pub cancellation_deadline: Option<DateTime<Utc>>,
    pub penalty_percentage: f64,
    pub automatic_replacement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventVisibility {
    Public,
    FriendsOnly,
    GuildOnly(GuildId),
    InviteOnly,
    Private,
}

/// Event template for quick event creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub description: String,
    pub event_type: EventType,
    pub default_duration_hours: u32,
    pub default_capacity: EventCapacity,
    pub suggested_requirements: Vec<EventRequirement>,
    pub suggested_rewards: Vec<EventReward>,
    pub default_rules: Vec<String>,
    pub category: String,
}

/// Event participation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParticipation {
    pub event_id: EventId,
    pub participation_status: ParticipantStatus,
    pub registration_date: DateTime<Utc>,
    pub attendance_duration: Option<u64>, // Duration in minutes
    pub performance_score: Option<f64>,
    pub feedback_given: bool,
    pub rewards_received: Vec<EventReward>,
}

/// Event calendar management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCalendar {
    pub calendar_view: CalendarView,
    pub time_zones: Vec<String>,
    pub filtered_categories: Vec<EventType>,
    pub subscribed_organizers: Vec<PlayerId>,
    pub personal_schedule: HashMap<PlayerId, Vec<EventId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalendarView {
    Daily,
    Weekly,
    Monthly,
    Custom(u32), // Number of days
}

/// Recurring event configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringEvent {
    pub recurring_id: Uuid,
    pub base_event_template: EventTemplate,
    pub recurrence_pattern: RecurrencePattern,
    pub next_occurrence: DateTime<Utc>,
    pub end_recurrence: Option<DateTime<Utc>>,
    pub max_occurrences: Option<u32>,
    pub generated_events: Vec<EventId>,
    pub auto_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurrencePattern {
    Daily,
    Weekly { day_of_week: u32 },
    Monthly { day_of_month: u32 },
    Custom { interval_days: u32 },
}

/// Event feedback and rating system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFeedback {
    pub feedback_id: Uuid,
    pub event_id: EventId,
    pub participant_id: PlayerId,
    pub overall_rating: f64,
    pub organization_rating: f64,
    pub content_rating: f64,
    pub difficulty_rating: f64,
    pub duration_rating: f64,
    pub comments: Option<String>,
    pub suggestions: Option<String>,
    pub would_attend_again: bool,
    pub submitted_date: DateTime<Utc>,
}

/// Event metrics and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetrics {
    pub event_id: EventId,
    pub total_registrations: u32,
    pub actual_attendance: u32,
    pub no_show_rate: f64,
    pub average_duration: u64,
    pub completion_rate: f64,
    pub satisfaction_score: f64,
    pub engagement_metrics: EngagementMetrics,
    pub financial_metrics: Option<FinancialMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub active_participation_rate: f64,
    pub interaction_count: u32,
    pub social_sharing_count: u32,
    pub return_participant_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMetrics {
    pub total_revenue: u64,
    pub total_costs: u64,
    pub profit_margin: f64,
    pub cost_per_participant: f64,
}

impl EventCoordinator {
    /// Create new event coordinator
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            event_templates: Self::create_default_templates(),
            participant_history: HashMap::new(),
            event_calendar: EventCalendar {
                calendar_view: CalendarView::Weekly,
                time_zones: vec!["UTC".to_string()],
                filtered_categories: vec![],
                subscribed_organizers: vec![],
                personal_schedule: HashMap::new(),
            },
            recurring_events: HashMap::new(),
        }
    }

    /// Create default event templates
    fn create_default_templates() -> Vec<EventTemplate> {
        vec![
            EventTemplate {
                template_id: Uuid::new_v4(),
                name: "Social Gathering".to_string(),
                description: "Casual social event for community building".to_string(),
                event_type: EventType::Social,
                default_duration_hours: 2,
                default_capacity: EventCapacity {
                    minimum_participants: 3,
                    maximum_participants: Some(20),
                    optimal_participants: Some(10),
                    waiting_list_enabled: true,
                },
                suggested_requirements: vec![],
                suggested_rewards: vec![
                    EventReward {
                        reward_type: RewardType::Experience,
                        amount: 100,
                        description: "Social participation bonus".to_string(),
                        distribution_method: DistributionMethod::All,
                        conditions: vec!["Attend full event".to_string()],
                    }
                ],
                default_rules: vec![
                    "Be respectful to all participants".to_string(),
                    "Follow community guidelines".to_string(),
                ],
                category: "Social".to_string(),
            },
            EventTemplate {
                template_id: Uuid::new_v4(),
                name: "PvP Tournament".to_string(),
                description: "Competitive player vs player tournament".to_string(),
                event_type: EventType::Tournament,
                default_duration_hours: 4,
                default_capacity: EventCapacity {
                    minimum_participants: 8,
                    maximum_participants: Some(32),
                    optimal_participants: Some(16),
                    waiting_list_enabled: true,
                },
                suggested_requirements: vec![
                    EventRequirement {
                        requirement_type: RequirementType::Level(25),
                        description: "Minimum level 25".to_string(),
                        mandatory: true,
                        verification_method: "automatic".to_string(),
                    }
                ],
                suggested_rewards: vec![
                    EventReward {
                        reward_type: RewardType::Currency,
                        amount: 1000,
                        description: "Winner's prize".to_string(),
                        distribution_method: DistributionMethod::TopPerformers(1),
                        conditions: vec!["First place".to_string()],
                    },
                    EventReward {
                        reward_type: RewardType::Titles,
                        amount: 1,
                        description: "Champion title".to_string(),
                        distribution_method: DistributionMethod::TopPerformers(1),
                        conditions: vec!["Tournament winner".to_string()],
                    }
                ],
                default_rules: vec![
                    "No cheating or exploits".to_string(),
                    "Tournament bracket will be randomized".to_string(),
                    "Disconnections result in forfeit".to_string(),
                ],
                category: "Competition".to_string(),
            },
            EventTemplate {
                template_id: Uuid::new_v4(),
                name: "Crafting Workshop".to_string(),
                description: "Educational workshop for learning crafting techniques".to_string(),
                event_type: EventType::Workshop,
                default_duration_hours: 3,
                default_capacity: EventCapacity {
                    minimum_participants: 5,
                    maximum_participants: Some(15),
                    optimal_participants: Some(10),
                    waiting_list_enabled: false,
                },
                suggested_requirements: vec![
                    EventRequirement {
                        requirement_type: RequirementType::Item("Basic Tools".to_string()),
                        description: "Must have basic crafting tools".to_string(),
                        mandatory: true,
                        verification_method: "inventory_check".to_string(),
                    }
                ],
                suggested_rewards: vec![
                    EventReward {
                        reward_type: RewardType::SkillPoints,
                        amount: 5,
                        description: "Crafting skill bonus".to_string(),
                        distribution_method: DistributionMethod::All,
                        conditions: vec!["Complete workshop".to_string()],
                    }
                ],
                default_rules: vec![
                    "Bring required materials".to_string(),
                    "Ask questions freely".to_string(),
                    "Share knowledge with others".to_string(),
                ],
                category: "Educational".to_string(),
            },
        ]
    }

    /// Create new event
    pub fn create_event(&mut self, organizer_id: PlayerId, title: String, description: String,
                       event_type: EventType, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<EventId> {
        let event_id = Uuid::new_v4();
        
        let event = Event {
            event_id,
            title,
            description,
            event_type,
            organizer_id,
            co_organizers: vec![],
            scheduled_start: start_time,
            scheduled_end: end_time,
            actual_start: None,
            actual_end: None,
            location: EventLocation::InGame(GameLocation {
                world_id: Uuid::new_v4(),
                coordinates: (0.0, 0.0, 0.0),
                area_name: "Main Hub".to_string(),
                instance_id: None,
            }),
            participants: HashMap::new(),
            requirements: vec![],
            rewards: vec![],
            rules: vec![],
            status: EventStatus::Planned,
            capacity: EventCapacity {
                minimum_participants: 1,
                maximum_participants: None,
                optimal_participants: None,
                waiting_list_enabled: false,
            },
            registration: RegistrationSettings {
                registration_start: Utc::now(),
                registration_end: start_time - Duration::hours(1),
                approval_required: false,
                fee_required: None,
                cancellation_policy: CancellationPolicy {
                    cancellation_deadline: Some(start_time - Duration::hours(2)),
                    penalty_percentage: 0.0,
                    automatic_replacement: true,
                },
            },
            visibility: EventVisibility::Public,
            tags: vec![],
            created_date: Utc::now(),
            last_updated: Utc::now(),
        };

        self.events.insert(event_id, event);
        Ok(event_id)
    }

    /// Register player for event
    pub fn register_for_event(&mut self, event_id: EventId, player_id: PlayerId) -> Result<()> {
        let event = self.events.get_mut(&event_id)
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        // Check if registration is open
        let now = Utc::now();
        if now < event.registration.registration_start || now > event.registration.registration_end {
            return Err(anyhow::anyhow!("Registration is not currently open"));
        }

        // Check if already registered
        if event.participants.contains_key(&player_id) {
            return Err(anyhow::anyhow!("Already registered for this event"));
        }

        // Check capacity
        if let Some(max_capacity) = event.capacity.maximum_participants {
            let current_registered = event.participants.len() as u32;
            if current_registered >= max_capacity {
                if event.capacity.waiting_list_enabled {
                    event.participants.insert(player_id, ParticipantStatus::Waitlisted);
                    return Ok(());
                } else {
                    return Err(anyhow::anyhow!("Event is at full capacity"));
                }
            }
        }

        // Check requirements
        for requirement in &event.requirements {
            if requirement.mandatory && !self.check_requirement(player_id, requirement)? {
                return Err(anyhow::anyhow!("Does not meet requirement: {}", requirement.description));
            }
        }

        // Register participant
        event.participants.insert(player_id, ParticipantStatus::Registered);
        
        // Update participant history
        let participation = EventParticipation {
            event_id,
            participation_status: ParticipantStatus::Registered,
            registration_date: Utc::now(),
            attendance_duration: None,
            performance_score: None,
            feedback_given: false,
            rewards_received: vec![],
        };

        self.participant_history.entry(player_id)
            .or_insert_with(Vec::new)
            .push(participation);

        Ok(())
    }

    /// Check if player meets requirement
    fn check_requirement(&self, _player_id: PlayerId, _requirement: &EventRequirement) -> Result<bool> {
        // Simplified requirement checking - in real implementation would check player stats
        Ok(true)
    }

    /// Cancel event registration
    pub fn cancel_registration(&mut self, event_id: EventId, player_id: PlayerId) -> Result<()> {
        let event = self.events.get_mut(&event_id)
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        // Check if registered
        if !event.participants.contains_key(&player_id) {
            return Err(anyhow::anyhow!("Not registered for this event"));
        }

        // Check cancellation policy
        if let Some(deadline) = event.registration.cancellation_policy.cancellation_deadline {
            if Utc::now() > deadline {
                return Err(anyhow::anyhow!("Cancellation deadline has passed"));
            }
        }

        // Remove from participants
        event.participants.remove(&player_id);
        
        // Promote waiting list member if applicable
        if event.capacity.waiting_list_enabled {
            if let Some((waitlisted_player, _)) = event.participants.iter()
                .find(|(_, status)| matches!(status, ParticipantStatus::Waitlisted))
                .map(|(id, status)| (*id, status.clone())) {
                event.participants.insert(waitlisted_player, ParticipantStatus::Registered);
            }
        }

        // Update participant history
        if let Some(history) = self.participant_history.get_mut(&player_id) {
            if let Some(participation) = history.iter_mut().find(|p| p.event_id == event_id) {
                participation.participation_status = ParticipantStatus::Cancelled;
            }
        }

        Ok(())
    }

    /// Start event
    pub fn start_event(&mut self, event_id: EventId) -> Result<()> {
        let event = self.events.get_mut(&event_id)
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        // Check if event can start
        let registered_count = event.participants.values()
            .filter(|status| matches!(status, ParticipantStatus::Registered | ParticipantStatus::Confirmed))
            .count() as u32;

        if registered_count < event.capacity.minimum_participants {
            return Err(anyhow::anyhow!("Not enough participants to start event"));
        }

        event.status = EventStatus::InProgress;
        event.actual_start = Some(Utc::now());

        // Mark registered participants as confirmed
        for (_, status) in &mut event.participants {
            if matches!(status, ParticipantStatus::Registered) {
                *status = ParticipantStatus::Confirmed;
            }
        }

        Ok(())
    }

    /// End event and distribute rewards
    pub fn end_event(&mut self, event_id: EventId) -> Result<()> {
        let event = self.events.get_mut(&event_id)
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        event.status = EventStatus::Completed;
        event.actual_end = Some(Utc::now());

        // Mark confirmed participants as attended
        let mut attendees = vec![];
        for (player_id, status) in &mut event.participants {
            if matches!(status, ParticipantStatus::Confirmed) {
                *status = ParticipantStatus::Attended;
                attendees.push(*player_id);
            }
        }

        // Distribute rewards
        for reward in &event.rewards {
            self.distribute_reward(event_id, reward, &attendees)?;
        }

        Ok(())
    }

    /// Distribute reward to eligible participants
    fn distribute_reward(&mut self, event_id: EventId, reward: &EventReward, attendees: &[PlayerId]) -> Result<()> {
        let recipients = match &reward.distribution_method {
            DistributionMethod::All => attendees.to_vec(),
            DistributionMethod::TopPerformers(count) => {
                // In real implementation, would rank by performance
                attendees.iter().take(*count as usize).cloned().collect()
            },
            DistributionMethod::Random(count) => {
                // In real implementation, would randomly select
                attendees.iter().take(*count as usize).cloned().collect()
            },
            _ => attendees.to_vec(),
        };

        // Update participant history with rewards
        for recipient in recipients {
            if let Some(history) = self.participant_history.get_mut(&recipient) {
                if let Some(participation) = history.iter_mut().find(|p| p.event_id == event_id) {
                    participation.rewards_received.push(reward.clone());
                }
            }
        }

        Ok(())
    }

    /// Get events for player (registered or eligible)
    pub fn get_player_events(&self, player_id: PlayerId, status_filter: Option<EventStatus>) -> Vec<&Event> {
        self.events.values()
            .filter(|event| {
                // Check if player is registered or event is public
                let is_participant = event.participants.contains_key(&player_id);
                let is_visible = matches!(event.visibility, EventVisibility::Public);
                
                let status_matches = status_filter.as_ref()
                    .map(|filter| &event.status == filter)
                    .unwrap_or(true);
                
                (is_participant || is_visible) && status_matches
            })
            .collect()
    }

    /// Get upcoming events
    pub fn get_upcoming_events(&self, limit: Option<usize>) -> Vec<&Event> {
        let mut events: Vec<&Event> = self.events.values()
            .filter(|event| event.scheduled_start > Utc::now() && 
                           matches!(event.visibility, EventVisibility::Public))
            .collect();
        
        events.sort_by(|a, b| a.scheduled_start.cmp(&b.scheduled_start));
        
        if let Some(limit) = limit {
            events.into_iter().take(limit).collect()
        } else {
            events
        }
    }

    /// Get event participation count for player
    pub fn get_event_participation_count(&self, player_id: PlayerId) -> u32 {
        self.participant_history.get(&player_id)
            .map(|history| history.iter()
                .filter(|p| matches!(p.participation_status, ParticipantStatus::Attended))
                .count() as u32)
            .unwrap_or(0)
    }

    /// Create recurring event
    pub fn create_recurring_event(&mut self, template: EventTemplate, pattern: RecurrencePattern, 
                                 start_date: DateTime<Utc>) -> Result<Uuid> {
        let recurring_id = Uuid::new_v4();
        
        let recurring_event = RecurringEvent {
            recurring_id,
            base_event_template: template,
            recurrence_pattern: pattern,
            next_occurrence: start_date,
            end_recurrence: None,
            max_occurrences: None,
            generated_events: vec![],
            auto_registration: false,
        };

        self.recurring_events.insert(recurring_id, recurring_event);
        Ok(recurring_id)
    }

    /// Generate next occurrence of recurring event
    pub fn generate_next_occurrence(&mut self, recurring_id: Uuid) -> Result<EventId> {
        let recurring_event = self.recurring_events.get_mut(&recurring_id)
            .ok_or_else(|| anyhow::anyhow!("Recurring event not found"))?;

        let next_start = recurring_event.next_occurrence;
        let next_end = next_start + Duration::hours(recurring_event.base_event_template.default_duration_hours as i64);

        // Create event from template
        let event_id = self.create_event(
            Uuid::new_v4(), // System organizer
            recurring_event.base_event_template.name.clone(),
            recurring_event.base_event_template.description.clone(),
            recurring_event.base_event_template.event_type.clone(),
            next_start,
            next_end,
        )?;

        // Apply template settings to the event
        if let Some(event) = self.events.get_mut(&event_id) {
            event.capacity = recurring_event.base_event_template.default_capacity.clone();
            event.requirements = recurring_event.base_event_template.suggested_requirements.clone();
            event.rewards = recurring_event.base_event_template.suggested_rewards.clone();
            event.rules = recurring_event.base_event_template.default_rules.clone();
        }

        // Update next occurrence time
        recurring_event.next_occurrence = match &recurring_event.recurrence_pattern {
            RecurrencePattern::Daily => next_start + Duration::days(1),
            RecurrencePattern::Weekly { .. } => next_start + Duration::weeks(1),
            RecurrencePattern::Monthly { .. } => next_start + Duration::days(30), // Approximate
            RecurrencePattern::Custom { interval_days } => next_start + Duration::days(*interval_days as i64),
        };

        recurring_event.generated_events.push(event_id);
        Ok(event_id)
    }

    /// Get event by ID
    pub fn get_event(&self, event_id: EventId) -> Option<&Event> {
        self.events.get(&event_id)
    }

    /// Update event status
    pub fn update_event_status(&mut self, event_id: EventId, status: EventStatus) -> Result<()> {
        let event = self.events.get_mut(&event_id)
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;
        
        event.status = status;
        event.last_updated = Utc::now();
        Ok(())
    }
}

impl Default for EventCoordinator {
    fn default() -> Self {
        Self::new()
    }
}