use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::entities::being::{EvolutionRequest, RequestStatus, EvolutionEvidence, Skill};

/// Global skill evolution manager - handles NPC requests for new skills
/// This represents the "consensus system" where NPCs can petition for new abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEvolutionSystem {
    pub pending_requests: Vec<EvolutionRequest>,
    pub approved_skills: HashMap<String, ApprovedSkill>,
    pub rejected_requests: Vec<RejectedRequest>,
    pub community_votes: HashMap<String, CommunityVote>, // skill_name -> vote data
    pub developer_queue: Vec<DeveloperReview>,
    pub implementation_log: Vec<ImplementationRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovedSkill {
    pub skill: Skill,
    pub approval_date: DateTime<Utc>,
    pub requesting_beings: Vec<Uuid>,
    pub community_support: f64, // Percentage of community that voted yes
    pub developer_notes: String,
    pub implementation_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedRequest {
    pub original_request: EvolutionRequest,
    pub rejection_reason: String,
    pub rejection_date: DateTime<Utc>,
    pub rejection_authority: RejectionAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RejectionAuthority {
    Community,           // Community vote rejected it
    Developer,          // Developer review rejected it
    GameBalance,        // Rejected for balance reasons
    TechnicalLimitations, // Cannot be implemented
    Duplicate,          // Skill already exists
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityVote {
    pub skill_name: String,
    pub total_votes: u32,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub abstain_votes: u32,
    pub voting_period_end: DateTime<Utc>,
    pub voter_ids: Vec<Uuid>, // Track who voted to prevent double voting
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperReview {
    pub request: EvolutionRequest,
    pub assigned_developer: String,
    pub review_status: DeveloperReviewStatus,
    pub technical_assessment: TechnicalAssessment,
    pub balance_assessment: BalanceAssessment,
    pub implementation_estimate: ImplementationEstimate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeveloperReviewStatus {
    Assigned,
    InProgress,
    TechnicalReview,
    BalanceReview,
    AwaitingImplementation,
    ReadyForImplementation,
    Rejected,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalAssessment {
    pub feasible: bool,
    pub complexity_score: f64, // 1-10 scale
    pub required_systems: Vec<String>, // What systems need to be modified
    pub potential_conflicts: Vec<String>, // Existing skills that might conflict
    pub technical_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceAssessment {
    pub balanced: bool,
    pub power_level: f64, // 1-10 scale relative to existing skills
    pub synergy_concerns: Vec<String>, // Overpowered combinations
    pub progression_fit: bool, // Fits into existing progression
    pub balance_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationEstimate {
    pub estimated_hours: f64,
    pub required_milestones: Vec<String>,
    pub testing_requirements: Vec<String>,
    pub release_target: Option<String>, // Version or date
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationRecord {
    pub skill_name: String,
    pub implemented_date: DateTime<Utc>,
    pub implementation_version: String,
    pub original_request_id: Uuid,
    pub final_skill_definition: Skill,
    pub changelog: Vec<String>, // What changed from original request
    pub testing_results: Vec<String>,
}

/// Represents a "letter to the gods" - NPCs formally requesting new skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterToGods {
    pub id: Uuid,
    pub sender_being: Uuid,
    pub sender_faction: Option<String>,
    pub subject: String,
    pub message: String,
    pub requested_skill: String,
    pub justification: String,
    pub supporting_lore: Vec<String>, // In-game books, observations, etc.
    pub co_signers: Vec<Uuid>,       // Other beings supporting this request
    pub timestamp: DateTime<Utc>,
    pub response_status: LetterResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LetterResponse {
    Pending,
    Acknowledged,
    UnderConsideration,
    CommunityVoteInitiated,
    DeveloperReview,
    Approved,
    Rejected(String),
    Implemented,
}

impl SkillEvolutionSystem {
    pub fn new() -> Self {
        Self {
            pending_requests: Vec::new(),
            approved_skills: HashMap::new(),
            rejected_requests: Vec::new(),
            community_votes: HashMap::new(),
            developer_queue: Vec::new(),
            implementation_log: Vec::new(),
        }
    }

    /// Submit a new skill evolution request
    pub fn submit_request(&mut self, request: EvolutionRequest) -> Result<(), String> {
        // Validate request
        if self.skill_exists(&request.requested_skill) {
            return Err("Skill already exists".to_string());
        }

        if self.has_pending_request(&request.requested_skill) {
            return Err("Request for this skill already pending".to_string());
        }

        // Add to pending requests
        self.pending_requests.push(request);
        Ok(())
    }

    /// Submit a "letter to the gods" - immersive way for NPCs to request skills
    pub fn submit_letter_to_gods(&mut self, letter: LetterToGods) -> Result<Uuid, String> {
        // Convert letter to evolution request
        let mut evidence = vec![
            EvolutionEvidence::ObservedPhenomena(letter.justification.clone())
        ];

        // Add lore as evidence
        for lore in &letter.supporting_lore {
            evidence.push(EvolutionEvidence::RunestoneKnowledge(lore.clone()));
        }

        let request = EvolutionRequest {
            requested_skill: letter.requested_skill.clone(),
            requesting_being: letter.sender_being,
            justification: format!("Letter to Gods: {}", letter.message),
            proposed_traits: self.generate_default_traits(&letter.requested_skill),
            prerequisite_skills: Vec::new(), // Could be derived from letter content
            supporting_evidence: evidence,
            timestamp: letter.timestamp,
            status: RequestStatus::Submitted,
        };

        self.submit_request(request)?;
        Ok(letter.id)
    }

    /// Generate default skill traits for a requested skill
    fn generate_default_traits(&self, skill_name: &str) -> (crate::entities::being::PassiveTrait, Option<crate::entities::being::ActiveTrait>) {
        use crate::entities::being::{PassiveTrait, ActiveTrait, PassiveEffect, ActiveEffect, ActivationCost, SkillCooldown, SkillFormula};

        let passive = PassiveTrait {
            name: format!("{} Mastery", skill_name),
            description: format!("Understanding and proficiency with {}", skill_name),
            effects: vec![PassiveEffect::IncreaseDamage(1.0)], // Generic improvement
            formula: SkillFormula::Linear(0.01),
        };

        let active = Some(ActiveTrait {
            name: skill_name.to_string(),
            description: format!("Active use of {}", skill_name),
            activation_cost: ActivationCost::Energy(25.0), // Default energy cost
            cooldown: SkillCooldown {
                base_duration: 30.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![ActiveEffect::DealDamage(30.0)], // Generic effect
            can_be_hotkeyed: true,
        });

        (passive, active)
    }

    /// Process pending requests through the consensus system
    pub fn process_requests(&mut self) {
        let mut processed_indices = Vec::new();
        let mut new_votes = Vec::new();
        let mut new_developer_reviews = Vec::new();
        let mut skills_to_approve = Vec::new();
        let mut status_updates = Vec::new();

        // First pass: collect information without borrowing mutably
        for (index, request) in self.pending_requests.iter().enumerate() {
            match request.status {
                RequestStatus::Submitted => {
                    new_votes.push(request.clone());
                    status_updates.push((index, RequestStatus::CommunityVoting));
                },
                RequestStatus::CommunityVoting => {
                    // Check if voting period ended
                    if let Some(vote) = self.community_votes.get(&request.requested_skill) {
                        if !vote.is_active && vote.voting_period_end <= Utc::now() {
                            let approval_rate = vote.yes_votes as f64 / vote.total_votes as f64;
                            if approval_rate >= 0.6 { // 60% approval needed
                                new_developer_reviews.push(request.clone());
                                status_updates.push((index, RequestStatus::DeveloperReview));
                            } else {
                                processed_indices.push(index);
                            }
                        }
                    }
                },
                RequestStatus::DeveloperReview => {
                    // Check developer review status
                    if let Some(review) = self.find_developer_review(&request.requested_skill) {
                        match review.review_status {
                            DeveloperReviewStatus::ReadyForImplementation => {
                                skills_to_approve.push(request.clone());
                                status_updates.push((index, RequestStatus::Approved));
                                processed_indices.push(index);
                            },
                            DeveloperReviewStatus::Rejected => {
                                processed_indices.push(index);
                            },
                            _ => {}, // Still in progress
                        }
                    }
                },
                _ => {}, // Other statuses don't need processing
            }
        }

        // Apply all system changes first (no conflicts)
        for request in new_votes {
            self.initiate_community_vote(&request);
        }

        for request in new_developer_reviews {
            self.assign_developer_review(request);
        }

        for request in skills_to_approve {
            self.approve_skill(request);
        }

        // Update status of pending requests
        for (index, new_status) in status_updates {
            if let Some(pending_request) = self.pending_requests.get_mut(index) {
                pending_request.status = new_status;
            }
        }

        // Remove processed requests
        for &index in processed_indices.iter().rev() {
            let request = self.pending_requests.remove(index);
            if let RequestStatus::Rejected(reason) = request.status.clone() {
                self.rejected_requests.push(RejectedRequest {
                    original_request: request,
                    rejection_reason: reason,
                    rejection_date: Utc::now(),
                    rejection_authority: RejectionAuthority::Community, // Could be more specific
                });
            }
        }
    }

    /// Initiate community voting for a skill request
    fn initiate_community_vote(&mut self, request: &EvolutionRequest) {
        let vote = CommunityVote {
            skill_name: request.requested_skill.clone(),
            total_votes: 0,
            yes_votes: 0,
            no_votes: 0,
            abstain_votes: 0,
            voting_period_end: Utc::now() + chrono::Duration::days(7), // 1 week voting period
            voter_ids: Vec::new(),
            is_active: true,
        };

        self.community_votes.insert(request.requested_skill.clone(), vote);
    }

    /// Assign developer review for approved community requests
    fn assign_developer_review(&mut self, request: EvolutionRequest) {
        let review = DeveloperReview {
            request,
            assigned_developer: "Auto-assigned".to_string(), // Would be actual developer
            review_status: DeveloperReviewStatus::Assigned,
            technical_assessment: TechnicalAssessment {
                feasible: true, // Default optimistic
                complexity_score: 5.0,
                required_systems: Vec::new(),
                potential_conflicts: Vec::new(),
                technical_notes: "Pending technical review".to_string(),
            },
            balance_assessment: BalanceAssessment {
                balanced: true,
                power_level: 5.0,
                synergy_concerns: Vec::new(),
                progression_fit: true,
                balance_notes: "Pending balance review".to_string(),
            },
            implementation_estimate: ImplementationEstimate {
                estimated_hours: 8.0, // Default estimate
                required_milestones: Vec::new(),
                testing_requirements: Vec::new(),
                release_target: None,
            },
        };

        self.developer_queue.push(review);
    }

    /// Approve a skill and add it to available skills
    fn approve_skill(&mut self, request: EvolutionRequest) {
        // Create the actual skill from the request
        let skill = Skill {
            name: request.requested_skill.clone(),
            level: 1.0,
            experience: 0.0,
            category: crate::entities::being::SkillCategory::Experimental, // New skills start as experimental
            passive_trait: request.proposed_traits.0,
            active_trait: request.proposed_traits.1,
            discovered_by: vec![request.requesting_being],
            is_innate: false, // Evolved skills are not innate
            unlock_requirements: request.prerequisite_skills.iter()
                .map(|s| crate::entities::being::SkillRequirement::SkillLevel(s.clone(), 10.0))
                .collect(),
            evolution_potential: Vec::new(), // Will be filled as skill is used
            synergy_skills: request.prerequisite_skills.clone(),
            prerequisite_skills: request.prerequisite_skills,
        };

        let approved = ApprovedSkill {
            skill,
            approval_date: Utc::now(),
            requesting_beings: vec![request.requesting_being],
            community_support: self.community_votes.get(&request.requested_skill)
                .map(|v| v.yes_votes as f64 / v.total_votes as f64 * 100.0)
                .unwrap_or(0.0),
            developer_notes: "Approved through consensus system".to_string(),
            implementation_version: "Genesis-1.0".to_string(),
        };

        self.approved_skills.insert(request.requested_skill.clone(), approved);
    }

    /// Community member casts vote on a skill request
    pub fn cast_vote(&mut self, skill_name: &str, voter_id: Uuid, vote: VoteChoice) -> Result<(), String> {
        if let Some(community_vote) = self.community_votes.get_mut(skill_name) {
            if !community_vote.is_active {
                return Err("Voting period has ended".to_string());
            }

            if community_vote.voter_ids.contains(&voter_id) {
                return Err("Already voted".to_string());
            }

            community_vote.voter_ids.push(voter_id);
            community_vote.total_votes += 1;

            match vote {
                VoteChoice::Yes => community_vote.yes_votes += 1,
                VoteChoice::No => community_vote.no_votes += 1,
                VoteChoice::Abstain => community_vote.abstain_votes += 1,
            }

            Ok(())
        } else {
            Err("No active vote for this skill".to_string())
        }
    }

    /// Check if skill already exists in approved skills
    fn skill_exists(&self, skill_name: &str) -> bool {
        self.approved_skills.contains_key(skill_name)
    }

    /// Check if there's already a pending request for this skill
    fn has_pending_request(&self, skill_name: &str) -> bool {
        self.pending_requests.iter().any(|r| r.requested_skill == skill_name)
    }

    /// Find developer review for a skill
    fn find_developer_review(&self, skill_name: &str) -> Option<&DeveloperReview> {
        self.developer_queue.iter().find(|r| r.request.requested_skill == skill_name)
    }

    /// Get all skills available for discovery (approved skills)
    pub fn get_available_skills(&self) -> Vec<&Skill> {
        self.approved_skills.values().map(|a| &a.skill).collect()
    }

    /// Record implementation of an approved skill
    pub fn record_implementation(&mut self, skill_name: &str, version: &str, changelog: Vec<String>) {
        if let Some(approved) = self.approved_skills.get(skill_name) {
            let record = ImplementationRecord {
                skill_name: skill_name.to_string(),
                implemented_date: Utc::now(),
                implementation_version: version.to_string(),
                original_request_id: Uuid::new_v4(), // Would track actual request ID
                final_skill_definition: approved.skill.clone(),
                changelog,
                testing_results: Vec::new(),
            };

            self.implementation_log.push(record);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

/// Example of how NPCs might naturally discover skills leading to requests
pub struct SkillDiscoveryEngine {
    pub observed_combinations: HashMap<Vec<String>, u32>, // Track skill combinations used
    pub environmental_interactions: Vec<EnvironmentalSkillHint>,
    pub combat_adaptations: Vec<CombatSkillNeed>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalSkillHint {
    pub situation: String,
    pub skills_used: Vec<String>,
    pub suggested_skill: String,
    pub frequency: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSkillNeed {
    pub combat_scenario: String,
    pub inadequate_skills: Vec<String>,
    pub needed_capability: String,
    pub urgency: f64,
}

impl SkillDiscoveryEngine {
    pub fn new() -> Self {
        Self {
            observed_combinations: HashMap::new(),
            environmental_interactions: Vec::new(),
            combat_adaptations: Vec::new(),
        }
    }

    /// Record when an NPC uses multiple skills together
    pub fn record_skill_combination(&mut self, skills: Vec<String>) {
        if skills.len() >= 2 {
            let mut sorted_skills = skills;
            sorted_skills.sort();
            *self.observed_combinations.entry(sorted_skills).or_insert(0) += 1;
        }
    }

    /// Analyze combinations to suggest new skills
    pub fn analyze_potential_skills(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        for (combination, frequency) in &self.observed_combinations {
            if *frequency >= 5 { // Used together 5+ times
                match combination.as_slice() {
                    [a, b] if a == "Fire Magic" && b == "Defense" => {
                        suggestions.push("Weapon Enchantment".to_string());
                    },
                    [a, b] if a == "Defense" && b == "Taunt" => {
                        suggestions.push("Wrath".to_string());
                    },
                    [a, b] if a == "Wrath" && b.contains("Combat") => {
                        suggestions.push("Fury".to_string());
                    },
                    _ => {
                        // Generate generic skill name from combination
                        let skill_name = format!("{} Mastery", combination.join(" + "));
                        suggestions.push(skill_name);
                    }
                }
            }
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_evolution_request() {
        let mut system = SkillEvolutionSystem::new();
        
        // Create a mock evolution request
        let request = EvolutionRequest {
            requested_skill: "Test Skill".to_string(),
            requesting_being: Uuid::new_v4(),
            justification: "Test justification".to_string(),
            proposed_traits: system.generate_default_traits("Test Skill"),
            prerequisite_skills: Vec::new(),
            supporting_evidence: Vec::new(),
            timestamp: Utc::now(),
            status: RequestStatus::Submitted,
        };

        assert!(system.submit_request(request).is_ok());
        assert_eq!(system.pending_requests.len(), 1);
    }

    #[test]
    fn test_community_voting() {
        let mut system = SkillEvolutionSystem::new();
        let voter_id = Uuid::new_v4();
        
        // Create a vote
        let vote = CommunityVote {
            skill_name: "Test Skill".to_string(),
            total_votes: 0,
            yes_votes: 0,
            no_votes: 0,
            abstain_votes: 0,
            voting_period_end: Utc::now() + chrono::Duration::days(7),
            voter_ids: Vec::new(),
            is_active: true,
        };

        system.community_votes.insert("Test Skill".to_string(), vote);

        // Cast vote
        assert!(system.cast_vote("Test Skill", voter_id, VoteChoice::Yes).is_ok());
        
        let vote = system.community_votes.get("Test Skill").unwrap();
        assert_eq!(vote.yes_votes, 1);
        assert_eq!(vote.total_votes, 1);
    }

    #[test]
    fn test_skill_discovery_engine() {
        let mut engine = SkillDiscoveryEngine::new();
        
        // Record skill combinations
        engine.record_skill_combination(vec!["Fire Magic".to_string(), "Defense".to_string()]);
        engine.record_skill_combination(vec!["Fire Magic".to_string(), "Defense".to_string()]);
        engine.record_skill_combination(vec!["Fire Magic".to_string(), "Defense".to_string()]);
        engine.record_skill_combination(vec!["Fire Magic".to_string(), "Defense".to_string()]);
        engine.record_skill_combination(vec!["Fire Magic".to_string(), "Defense".to_string()]);

        let suggestions = engine.analyze_potential_skills();
        assert!(suggestions.contains(&"Weapon Enchantment".to_string()));
    }
}
