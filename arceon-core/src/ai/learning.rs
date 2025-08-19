use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rand::Rng;

/// Advanced NPC AI learning system with machine learning patterns
/// NPCs learn from interactions, experiences, and environmental observations
/// Supports pre-genesis training and continuous learning in-world

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcLearningSystem {
    pub npc_id: Uuid,
    pub intelligence_level: f64,        // Base learning capacity
    pub learning_rate: f64,             // How quickly they learn (0.0 to 1.0)
    pub memory_capacity: usize,         // How many experiences they can retain
    pub knowledge_domains: HashMap<String, KnowledgeDomain>,
    pub experiences: Vec<LearningExperience>,
    pub neural_patterns: NeuralPatterns,
    pub personality_traits: PersonalityTraits,
    pub social_connections: HashMap<Uuid, SocialConnection>,
    pub language_model: LanguageModel,
    pub motivations: Vec<Motivation>,
    pub learning_preferences: LearningPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeDomain {
    pub domain_name: String,
    pub proficiency: f64,               // 0.0 to 100.0
    pub sub_topics: HashMap<String, f64>,
    pub learned_facts: Vec<LearnedFact>,
    pub teaching_ability: f64,          // How well they can teach this domain
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedFact {
    pub fact_id: Uuid,
    pub content: String,
    pub source: LearningSource,
    pub confidence: f64,                // How sure they are about this fact
    pub verification_count: u32,        // How many times it's been confirmed
    pub last_verified: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningSource {
    DirectExperience,
    TeachingFromNpc(Uuid),
    TeachingFromPlayer(String),
    ReadFromText(String),               // Book, scroll, etc.
    Observation(String),                // Watching others
    Experimentation,
    Tradition(String),                  // Cultural knowledge
    Intuition,                         // Emergent understanding
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningExperience {
    pub experience_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub experience_type: ExperienceType,
    pub outcome: ExperienceOutcome,
    pub emotional_impact: EmotionalImpact,
    pub knowledge_gained: Vec<String>,
    pub skills_improved: Vec<String>,
    pub participants: Vec<Uuid>,
    pub location: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceType {
    // Social interactions
    Conversation(String),               // Topic discussed
    Teaching(String),                   // What they taught/learned
    Collaboration(String),              // Joint project
    Conflict(String),                   // Disagreement or fight
    Trade(String),                      // Commercial interaction
    
    // Practical experiences
    Crafting(String),                   // What they made
    Construction(String),               // What they built
    Gathering(String),                  // What they collected
    Exploration(String),                // Where they went
    Problem_Solving(String),            // Challenge they faced
    
    // Cultural experiences
    Ritual(String),                     // Ceremony participated in
    Storytelling(String),               // Story heard/told
    Art_Creation(String),               // Creative work
    Music_Performance(String),          // Musical activity
    
    // Learning experiences
    Reading(String),                    // What they read
    Research(String),                   // What they investigated
    Experimentation(String),            // What they tested
    Observation(String),                // What they watched
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceOutcome {
    Success(f64),                       // Success level 0.0 to 1.0
    Failure(String),                    // Reason for failure
    PartialSuccess(f64, String),        // Partial success with notes
    Neutral,                           // No clear outcome
    Discovery(String),                  // Found something new
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalImpact {
    pub satisfaction: f64,              // -1.0 to 1.0
    pub curiosity: f64,                 // How much it sparked interest
    pub confidence: f64,                // Effect on self-confidence
    pub stress: f64,                    // How stressful it was
    pub social_bond_change: f64,        // Effect on relationships
}

/// Neural pattern system for emergent behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralPatterns {
    pub pattern_weights: HashMap<String, f64>,
    pub response_patterns: HashMap<String, Vec<ResponsePattern>>,
    pub decision_tree: DecisionTree,
    pub learned_associations: HashMap<String, Vec<Association>>,
    pub behavioral_tendencies: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePattern {
    pub trigger: String,
    pub response: String,
    pub confidence: f64,
    pub success_rate: f64,
    pub times_used: u32,
    pub context_filters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTree {
    pub root_node: DecisionNode,
    pub learned_branches: HashMap<String, DecisionNode>,
    pub pruned_paths: Vec<String>,      // Paths that proved ineffective
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionNode {
    pub condition: String,
    pub true_branch: Option<Box<DecisionNode>>,
    pub false_branch: Option<Box<DecisionNode>>,
    pub action: Option<String>,
    pub confidence: f64,
    pub success_history: Vec<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    pub concept_a: String,
    pub concept_b: String,
    pub strength: f64,                  // -1.0 to 1.0
    pub formation_count: u32,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub openness: f64,                  // Openness to experience
    pub conscientiousness: f64,         // Organization and persistence
    pub extraversion: f64,              // Social energy
    pub agreeableness: f64,             // Cooperation and trust
    pub neuroticism: f64,               // Emotional stability
    
    // RPG-specific traits
    pub curiosity: f64,                 // Drive to explore and learn
    pub ambition: f64,                  // Desire for achievement
    pub loyalty: f64,                   // Faithfulness to groups/individuals
    pub creativity: f64,                // Innovative thinking
    pub pragmatism: f64,                // Focus on practical solutions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialConnection {
    pub connected_npc: Uuid,
    pub relationship_type: RelationshipType,
    pub trust_level: f64,               // 0.0 to 1.0
    pub shared_experiences: u32,
    pub knowledge_exchange_rate: f64,   // How much they learn from each other
    pub influence: f64,                 // How much this NPC influences decisions
    pub last_interaction: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Friend,
    Mentor,
    Student,
    Colleague,
    Rival,
    Family,
    Romantic,
    Business,
    Neutral,
    Enemy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageModel {
    pub vocabulary: HashMap<String, WordKnowledge>,
    pub grammar_patterns: Vec<GrammarPattern>,
    pub speech_style: SpeechStyle,
    pub dialect: String,
    pub language_proficiency: f64,      // 0.0 to 1.0
    pub archaic_tendency: f64,          // How much they use old-fashioned speech
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordKnowledge {
    pub word: String,
    pub frequency_used: u32,
    pub contexts: Vec<String>,
    pub associated_emotions: HashMap<String, f64>,
    pub formality_level: f64,           // 0.0 casual to 1.0 formal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarPattern {
    pub pattern: String,
    pub usage_frequency: f64,
    pub context: String,
    pub formality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechStyle {
    pub verbosity: f64,                 // How much they talk
    pub formality: f64,                 // How formal their speech is
    pub emotiveness: f64,               // How much emotion they show
    pub directness: f64,                // How straightforward they are
    pub humor_tendency: f64,            // How often they use humor
    pub archaic_words_frequency: f64,   // How often they use old words
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motivation {
    pub motivation_type: MotivationType,
    pub strength: f64,                  // 0.0 to 1.0
    pub priority: u32,                  // Lower numbers = higher priority
    pub satisfaction_level: f64,        // How well it's being fulfilled
    pub related_goals: Vec<String>,
    pub triggers: Vec<String>,          // What activates this motivation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotivationType {
    // Survival motivations
    BasicSurvival,                      // Food, water, shelter, safety
    CollectiveSurvival,                 // Group/community survival
    ResourceSecurity,                   // Ensuring access to materials
    
    // Social motivations
    Belonging,                          // Being part of a community
    Recognition,                        // Being acknowledged/respected
    Teaching,                           // Sharing knowledge
    Learning,                          // Acquiring knowledge
    Leadership,                        // Guiding others
    Cooperation,                       // Working together
    
    // Achievement motivations
    Mastery,                           // Becoming excellent at something
    Creation,                          // Building/making things
    Discovery,                         // Finding new things
    Legacy,                            // Leaving something behind
    
    // Cultural motivations
    Tradition,                         // Preserving cultural practices
    Innovation,                        // Creating new ways
    Justice,                           // Fairness and order
    Beauty,                            // Aesthetic appreciation
    
    // Personal motivations
    Freedom,                           // Independence and choice
    Purpose,                           // Having a meaningful role
    Growth,                            // Personal development
    Adventure,                         // New experiences
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPreferences {
    pub preferred_learning_methods: Vec<LearningMethod>,
    pub teaching_style: TeachingStyle,
    pub focus_areas: Vec<String>,
    pub avoidance_areas: Vec<String>,   // What they don't like to learn about
    pub collaboration_preference: f64,  // How much they like group learning
    pub experimentation_tolerance: f64, // Willingness to try new things
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningMethod {
    DirectInstruction,                  // Being taught explicitly
    Observation,                        // Watching others
    Experimentation,                    // Trial and error
    Reading,                           // Text-based learning
    Conversation,                      // Discussion-based learning
    Practice,                          // Hands-on repetition
    Reflection,                        // Thinking about experiences
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeachingStyle {
    Patient,                           // Slow, thorough explanations
    Energetic,                         // Fast, engaging delivery
    Practical,                         // Focus on hands-on application
    Theoretical,                       // Focus on concepts and principles
    Storytelling,                      // Using narratives to teach
    Questioning,                       // Socratic method
}

impl NpcLearningSystem {
    pub fn new(npc_id: Uuid, intelligence: f64) -> Self {
        Self {
            npc_id,
            intelligence_level: intelligence,
            learning_rate: intelligence * 0.1,
            memory_capacity: (intelligence * 100.0) as usize,
            knowledge_domains: HashMap::new(),
            experiences: Vec::new(),
            neural_patterns: NeuralPatterns::new(),
            personality_traits: PersonalityTraits::random_generate(),
            social_connections: HashMap::new(),
            language_model: LanguageModel::new_archaic_english(),
            motivations: Self::generate_basic_motivations(),
            learning_preferences: LearningPreferences::generate_from_personality(&PersonalityTraits::random_generate()),
        }
    }

    /// Process a new experience and extract learning
    pub fn process_experience(&mut self, experience: LearningExperience) {
        // Add experience to memory
        self.experiences.push(experience.clone());
        
        // Limit memory capacity
        if self.experiences.len() > self.memory_capacity {
            self.experiences.remove(0);
        }

        // Extract knowledge from experience
        self.extract_knowledge_from_experience(&experience);
        
        // Update neural patterns
        self.update_neural_patterns(&experience);
        
        // Adjust motivations based on outcome
        self.adjust_motivations(&experience);
        
        // Update social connections if applicable
        self.update_social_connections(&experience);
    }

    /// Extract knowledge from an experience
    fn extract_knowledge_from_experience(&mut self, experience: &LearningExperience) {
        for knowledge_item in &experience.knowledge_gained {
            let domain = self.determine_knowledge_domain(knowledge_item);
            
            let knowledge_domain = self.knowledge_domains
                .entry(domain)
                .or_insert_with(|| KnowledgeDomain::new(domain.clone()));

            // Create new learned fact
            let fact = LearnedFact {
                fact_id: Uuid::new_v4(),
                content: knowledge_item.clone(),
                source: self.determine_learning_source(&experience.experience_type),
                confidence: self.calculate_initial_confidence(&experience.outcome),
                verification_count: 1,
                last_verified: experience.timestamp,
                tags: self.extract_tags_from_content(knowledge_item),
            };

            knowledge_domain.learned_facts.push(fact);
            knowledge_domain.proficiency += self.learning_rate * 0.1;
            knowledge_domain.last_updated = Utc::now();
        }

        // Update skills based on experience
        for skill in &experience.skills_improved {
            // This would integrate with the skill system
            self.improve_skill_through_experience(skill, &experience.outcome);
        }
    }

    /// Update neural patterns based on experience
    fn update_neural_patterns(&mut self, experience: &LearningExperience) {
        let context = format!("{}_{}", experience.experience_type.to_string(), experience.location);
        
        // Strengthen successful patterns
        if matches!(experience.outcome, ExperienceOutcome::Success(_)) {
            let pattern_key = format!("success_{}", experience.experience_type.to_string());
            let current_weight = self.neural_patterns.pattern_weights
                .get(&pattern_key)
                .unwrap_or(&0.0);
            self.neural_patterns.pattern_weights
                .insert(pattern_key, current_weight + self.learning_rate);
        }

        // Create new associations
        for participant in &experience.participants {
            if *participant != self.npc_id {
                let association = Association {
                    concept_a: experience.experience_type.to_string(),
                    concept_b: format!("participant_{}", participant),
                    strength: experience.emotional_impact.social_bond_change,
                    formation_count: 1,
                    context: context.clone(),
                };

                self.neural_patterns.learned_associations
                    .entry(context.clone())
                    .or_insert_with(Vec::new)
                    .push(association);
            }
        }
    }

    /// Generate speech based on learned language patterns
    pub fn generate_speech(&self, context: &str, topic: &str) -> String {
        let mut speech = String::new();
        
        // Select vocabulary based on context and personality
        let formality_level = self.calculate_formality_level(context);
        let emotion_level = self.personality_traits.extraversion;
        
        // Use archaic tendency to influence word choice
        if self.language_model.archaic_tendency > 0.5 && rand::random::<f64>() < self.language_model.archaic_tendency {
            speech = self.generate_archaic_speech(topic, formality_level);
        } else {
            speech = self.generate_modern_speech(topic, formality_level);
        }

        // Add personality-based speech patterns
        if self.personality_traits.verbosity > 0.7 {
            speech = format!("Verily, {}. Indeed, such matters doth require much consideration.", speech);
        }

        speech
    }

    /// Learn from another NPC through teaching
    pub fn learn_from_npc(&mut self, teacher_id: Uuid, subject: &str, knowledge: &str) {
        let trust_level = self.social_connections
            .get(&teacher_id)
            .map(|conn| conn.trust_level)
            .unwrap_or(0.5);

        let learning_effectiveness = trust_level * self.learning_rate;
        
        let experience = LearningExperience {
            experience_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            experience_type: ExperienceType::Teaching(subject.to_string()),
            outcome: ExperienceOutcome::Success(learning_effectiveness),
            emotional_impact: EmotionalImpact {
                satisfaction: learning_effectiveness * 0.8,
                curiosity: self.personality_traits.curiosity * 0.5,
                confidence: 0.1,
                stress: 0.0,
                social_bond_change: 0.05,
            },
            knowledge_gained: vec![knowledge.to_string()],
            skills_improved: vec![subject.to_string()],
            participants: vec![teacher_id],
            location: "learning_session".to_string(),
            context: format!("Learning {} from teacher", subject),
        };

        self.process_experience(experience);
    }

    /// Teach knowledge to another NPC
    pub fn teach_npc(&mut self, student_id: Uuid, subject: &str) -> Option<String> {
        let domain = self.knowledge_domains.get(subject)?;
        
        if domain.teaching_ability < 0.3 {
            return None; // Not knowledgeable enough to teach
        }

        // Select appropriate knowledge to teach
        let best_fact = domain.learned_facts
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())?;

        // Update teaching experience
        let experience = LearningExperience {
            experience_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            experience_type: ExperienceType::Teaching(subject.to_string()),
            outcome: ExperienceOutcome::Success(domain.teaching_ability),
            emotional_impact: EmotionalImpact {
                satisfaction: 0.6,
                curiosity: 0.0,
                confidence: 0.1,
                stress: 0.1,
                social_bond_change: 0.1,
            },
            knowledge_gained: vec!["teaching_experience".to_string()],
            skills_improved: vec!["Teaching".to_string()],
            participants: vec![student_id],
            location: "teaching_session".to_string(),
            context: format!("Teaching {} to student", subject),
        };

        self.process_experience(experience);

        Some(best_fact.content.clone())
    }

    /// Make a decision based on learned patterns and motivations
    pub fn make_decision(&self, situation: &str, options: &[String]) -> String {
        let mut best_option = options[0].clone();
        let mut best_score = 0.0;

        for option in options {
            let mut score = 0.0;

            // Check neural patterns
            let pattern_key = format!("{}_{}", situation, option);
            if let Some(pattern_weight) = self.neural_patterns.pattern_weights.get(&pattern_key) {
                score += pattern_weight;
            }

            // Check motivations
            for motivation in &self.motivations {
                if motivation.related_goals.contains(option) {
                    score += motivation.strength * motivation.satisfaction_level;
                }
            }

            // Consider personality traits
            score += self.calculate_personality_alignment(option);

            if score > best_score {
                best_score = score;
                best_option = option.clone();
            }
        }

        best_option
    }

    // Helper methods
    fn determine_knowledge_domain(&self, knowledge: &str) -> String {
        // Simple keyword-based domain classification
        if knowledge.contains("craft") || knowledge.contains("build") {
            "Crafting".to_string()
        } else if knowledge.contains("nature") || knowledge.contains("plant") {
            "Nature".to_string()
        } else if knowledge.contains("social") || knowledge.contains("people") {
            "Social".to_string()
        } else if knowledge.contains("combat") || knowledge.contains("fight") {
            "Combat".to_string()
        } else {
            "General".to_string()
        }
    }

    fn determine_learning_source(&self, experience_type: &ExperienceType) -> LearningSource {
        match experience_type {
            ExperienceType::Teaching(_) => LearningSource::TeachingFromNpc(Uuid::new_v4()),
            ExperienceType::Reading(_) => LearningSource::ReadFromText("book".to_string()),
            ExperienceType::Observation(_) => LearningSource::Observation("watching".to_string()),
            ExperienceType::Experimentation(_) => LearningSource::Experimentation,
            _ => LearningSource::DirectExperience,
        }
    }

    fn calculate_initial_confidence(&self, outcome: &ExperienceOutcome) -> f64 {
        match outcome {
            ExperienceOutcome::Success(level) => *level * 0.8,
            ExperienceOutcome::PartialSuccess(level, _) => *level * 0.5,
            ExperienceOutcome::Discovery(_) => 0.9,
            ExperienceOutcome::Failure(_) => 0.2,
            ExperienceOutcome::Neutral => 0.5,
        }
    }

    fn extract_tags_from_content(&self, content: &str) -> Vec<String> {
        // Simple keyword extraction
        let keywords = ["craft", "build", "nature", "social", "combat", "magic", "trade"];
        keywords.iter()
            .filter(|keyword| content.to_lowercase().contains(*keyword))
            .map(|keyword| keyword.to_string())
            .collect()
    }

    fn improve_skill_through_experience(&mut self, skill: &str, outcome: &ExperienceOutcome) {
        // This would integrate with the main skill system
        let improvement = match outcome {
            ExperienceOutcome::Success(level) => *level * self.learning_rate,
            ExperienceOutcome::PartialSuccess(level, _) => *level * self.learning_rate * 0.5,
            _ => 0.0,
        };
        
        // Record skill improvement for integration with main skill system
    }

    fn adjust_motivations(&mut self, experience: &LearningExperience) {
        for motivation in &mut self.motivations {
            if motivation.related_goals.iter().any(|goal| 
                experience.knowledge_gained.contains(goal) || 
                experience.skills_improved.contains(goal)
            ) {
                motivation.satisfaction_level += experience.emotional_impact.satisfaction * 0.1;
                motivation.satisfaction_level = motivation.satisfaction_level.clamp(0.0, 1.0);
            }
        }
    }

    fn update_social_connections(&mut self, experience: &LearningExperience) {
        for participant in &experience.participants {
            if *participant != self.npc_id {
                let connection = self.social_connections
                    .entry(*participant)
                    .or_insert_with(|| SocialConnection::new(*participant));
                
                connection.shared_experiences += 1;
                connection.trust_level += experience.emotional_impact.social_bond_change * 0.1;
                connection.trust_level = connection.trust_level.clamp(0.0, 1.0);
                connection.last_interaction = experience.timestamp;
            }
        }
    }

    fn calculate_formality_level(&self, context: &str) -> f64 {
        let base_formality = self.language_model.speech_style.formality;
        
        // Adjust based on context
        if context.contains("formal") || context.contains("official") {
            (base_formality + 0.3).min(1.0)
        } else if context.contains("casual") || context.contains("friend") {
            (base_formality - 0.3).max(0.0)
        } else {
            base_formality
        }
    }

    fn generate_archaic_speech(&self, topic: &str, formality: f64) -> String {
        let archaic_phrases = [
            "Verily, 'tis true that",
            "Methinks that",
            "Pray tell, doth",
            "In mine opinion",
            "Forsooth, the matter of",
            "Hark! The subject of",
            "By my troth",
            "'Tis well known that",
        ];

        let connectors = [
            "and furthermore",
            "moreover",
            "in addition thereto",
            "withal",
            "notwithstanding",
        ];

        let endings = [
            "if it please thee",
            "I do humbly submit",
            "as Providence doth will",
            "by thy leave",
            "with all due respect",
        ];

        let opener = archaic_phrases[rand::random::<usize>() % archaic_phrases.len()];
        let connector = if rand::random::<f64>() > 0.5 {
            format!(", {}, ", connectors[rand::random::<usize>() % connectors.len()])
        } else {
            " ".to_string()
        };
        let ender = if formality > 0.7 {
            format!(", {}", endings[rand::random::<usize>() % endings.len()])
        } else {
            "".to_string()
        };

        format!("{} {}{}{}.", opener, topic, connector, ender)
    }

    fn generate_modern_speech(&self, topic: &str, formality: f64) -> String {
        if formality > 0.7 {
            format!("I believe that {} is quite important to consider.", topic)
        } else {
            format!("So about {}, I think it's worth looking into.", topic)
        }
    }

    fn calculate_personality_alignment(&self, option: &str) -> f64 {
        let mut score = 0.0;
        
        // Simple personality-option alignment
        if option.contains("explore") || option.contains("discover") {
            score += self.personality_traits.curiosity * self.personality_traits.openness;
        }
        
        if option.contains("build") || option.contains("create") {
            score += self.personality_traits.creativity * self.personality_traits.conscientiousness;
        }
        
        if option.contains("help") || option.contains("cooperate") {
            score += self.personality_traits.agreeableness;
        }
        
        if option.contains("lead") || option.contains("command") {
            score += self.personality_traits.extraversion * self.personality_traits.ambition;
        }

        score
    }

    fn generate_basic_motivations() -> Vec<Motivation> {
        vec![
            Motivation {
                motivation_type: MotivationType::BasicSurvival,
                strength: 0.9,
                priority: 1,
                satisfaction_level: 0.5,
                related_goals: vec!["food".to_string(), "shelter".to_string(), "safety".to_string()],
                triggers: vec!["hunger".to_string(), "cold".to_string(), "danger".to_string()],
            },
            Motivation {
                motivation_type: MotivationType::CollectiveSurvival,
                strength: 0.7,
                priority: 2,
                satisfaction_level: 0.5,
                related_goals: vec!["community".to_string(), "cooperation".to_string()],
                triggers: vec!["group_threat".to_string(), "resource_scarcity".to_string()],
            },
            Motivation {
                motivation_type: MotivationType::Learning,
                strength: 0.6,
                priority: 3,
                satisfaction_level: 0.3,
                related_goals: vec!["knowledge".to_string(), "skills".to_string()],
                triggers: vec!["curiosity".to_string(), "ignorance".to_string()],
            },
        ]
    }
}

// Implementation for other structs...
impl NeuralPatterns {
    pub fn new() -> Self {
        Self {
            pattern_weights: HashMap::new(),
            response_patterns: HashMap::new(),
            decision_tree: DecisionTree::new(),
            learned_associations: HashMap::new(),
            behavioral_tendencies: HashMap::new(),
        }
    }
}

impl DecisionTree {
    pub fn new() -> Self {
        Self {
            root_node: DecisionNode::new("root".to_string()),
            learned_branches: HashMap::new(),
            pruned_paths: Vec::new(),
        }
    }
}

impl DecisionNode {
    pub fn new(condition: String) -> Self {
        Self {
            condition,
            true_branch: None,
            false_branch: None,
            action: None,
            confidence: 0.5,
            success_history: Vec::new(),
        }
    }
}

impl PersonalityTraits {
    pub fn random_generate() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        Self {
            openness: rng.gen_range(0.0..1.0),
            conscientiousness: rng.gen_range(0.0..1.0),
            extraversion: rng.gen_range(0.0..1.0),
            agreeableness: rng.gen_range(0.0..1.0),
            neuroticism: rng.gen_range(0.0..1.0),
            curiosity: rng.gen_range(0.2..0.9), // NPCs should be somewhat curious
            ambition: rng.gen_range(0.0..1.0),
            loyalty: rng.gen_range(0.3..0.8),   // Moderate to high loyalty
            creativity: rng.gen_range(0.0..1.0),
            pragmatism: rng.gen_range(0.2..0.9), // Should be somewhat practical
        }
    }
}

impl SocialConnection {
    pub fn new(connected_npc: Uuid) -> Self {
        Self {
            connected_npc,
            relationship_type: RelationshipType::Neutral,
            trust_level: 0.5,
            shared_experiences: 0,
            knowledge_exchange_rate: 0.1,
            influence: 0.1,
            last_interaction: Utc::now(),
        }
    }
}

impl LanguageModel {
    pub fn new_archaic_english() -> Self {
        Self {
            vocabulary: HashMap::new(),
            grammar_patterns: Vec::new(),
            speech_style: SpeechStyle {
                verbosity: 0.6,
                formality: 0.7,
                emotiveness: 0.5,
                directness: 0.4,
                humor_tendency: 0.2,
                archaic_words_frequency: 0.8,
            },
            dialect: "archaic_english".to_string(),
            language_proficiency: 0.7,
            archaic_tendency: 0.8,
        }
    }
}

impl KnowledgeDomain {
    pub fn new(name: String) -> Self {
        Self {
            domain_name: name,
            proficiency: 0.0,
            sub_topics: HashMap::new(),
            learned_facts: Vec::new(),
            teaching_ability: 0.0,
            last_updated: Utc::now(),
        }
    }
}

impl LearningPreferences {
    pub fn generate_from_personality(traits: &PersonalityTraits) -> Self {
        let mut preferred_methods = Vec::new();
        
        if traits.extraversion > 0.6 {
            preferred_methods.push(LearningMethod::Conversation);
        }
        if traits.conscientiousness > 0.6 {
            preferred_methods.push(LearningMethod::DirectInstruction);
            preferred_methods.push(LearningMethod::Reading);
        }
        if traits.openness > 0.6 {
            preferred_methods.push(LearningMethod::Experimentation);
        }
        if traits.curiosity > 0.7 {
            preferred_methods.push(LearningMethod::Observation);
        }

        Self {
            preferred_learning_methods: preferred_methods,
            teaching_style: if traits.extraversion > 0.6 {
                TeachingStyle::Energetic
            } else if traits.conscientiousness > 0.7 {
                TeachingStyle::Patient
            } else {
                TeachingStyle::Practical
            },
            focus_areas: Vec::new(),
            avoidance_areas: Vec::new(),
            collaboration_preference: traits.agreeableness * traits.extraversion,
            experimentation_tolerance: traits.openness * (1.0 - traits.neuroticism),
        }
    }
}

/// Pre-genesis training system for NPC initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreGenesisTraining {
    pub training_modules: Vec<TrainingModule>,
    pub completion_percentage: f64,
    pub training_start_time: DateTime<Utc>,
    pub estimated_completion_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingModule {
    pub module_name: String,
    pub module_type: TrainingModuleType,
    pub completion_status: TrainingStatus,
    pub knowledge_gained: Vec<String>,
    pub skills_developed: Vec<String>,
    pub neural_patterns_learned: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingModuleType {
    LanguageBasics,          // Basic archaic English speech patterns
    SurvivalInstincts,       // Basic survival motivations
    SocialCooperation,       // Cooperative behavior patterns
    BasicCrafting,           // Simple construction and tool use
    CommunityBuilding,       // Group organization and leadership
    ResourceManagement,      // Gathering and allocation strategies
    CulturalTraditions,      // Basic cultural knowledge and rituals
    ProblemSolving,          // Basic decision-making patterns
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingStatus {
    NotStarted,
    InProgress(f64),    // percentage complete
    Completed,
    Failed(String),     // reason for failure
}

/// Advanced AI learning system manager for the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldLearningSystem {
    pub npc_learning_systems: HashMap<Uuid, NpcLearningSystem>,
    pub global_knowledge_base: GlobalKnowledgeBase,
    pub learning_events: Vec<LearningEvent>,
    pub pre_genesis_training: Option<PreGenesisTraining>,
    pub cultural_evolution: CulturalEvolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalKnowledgeBase {
    pub shared_knowledge: HashMap<String, SharedKnowledge>,
    pub cultural_artifacts: Vec<CulturalArtifact>,
    pub technological_discoveries: Vec<TechnologicalDiscovery>,
    pub linguistic_evolution: LanguageEvolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedKnowledge {
    pub knowledge_id: Uuid,
    pub content: String,
    pub domain: String,
    pub discoverer: Uuid,
    pub verification_count: u32,
    pub spread_rate: f64,        // How quickly it spreads through population
    pub accuracy: f64,           // How accurate the knowledge is
    pub cultural_impact: f64,    // How much it influences society
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalArtifact {
    pub artifact_id: Uuid,
    pub artifact_type: String,
    pub creator: Uuid,
    pub cultural_significance: f64,
    pub preservation_status: f64,
    pub knowledge_contained: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologicalDiscovery {
    pub discovery_id: Uuid,
    pub technology_name: String,
    pub discoverer: Uuid,
    pub prerequisites: Vec<String>,
    pub applications: Vec<String>,
    pub adoption_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageEvolution {
    pub vocabulary_changes: HashMap<String, VocabularyEntry>,
    pub grammar_evolution: Vec<GrammarChange>,
    pub dialect_formation: HashMap<String, Dialect>,
    pub archaic_preservation: f64,   // How well archaic forms are preserved
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyEntry {
    pub word: String,
    pub meaning: String,
    pub usage_frequency: f64,
    pub regional_variations: HashMap<String, String>,
    pub historical_forms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarChange {
    pub change_type: String,
    pub old_form: String,
    pub new_form: String,
    pub adoption_percentage: f64,
    pub first_observed: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialect {
    pub dialect_name: String,
    pub speakers: Vec<Uuid>,
    pub unique_features: Vec<String>,
    pub mutual_intelligibility: HashMap<String, f64>, // With other dialects
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalEvolution {
    pub traditions: HashMap<String, Tradition>,
    pub social_norms: HashMap<String, SocialNorm>,
    pub governance_patterns: Vec<GovernancePattern>,
    pub economic_systems: Vec<EconomicSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tradition {
    pub tradition_name: String,
    pub practitioners: Vec<Uuid>,
    pub cultural_importance: f64,
    pub evolution_rate: f64,
    pub associated_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    pub norm_name: String,
    pub adherence_rate: f64,
    pub enforcement_mechanisms: Vec<String>,
    pub violation_consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernancePattern {
    pub pattern_name: String,
    pub effectiveness: f64,
    pub adoption_areas: Vec<String>,
    pub decision_making_process: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicSystem {
    pub system_name: String,
    pub participants: Vec<Uuid>,
    pub efficiency_rating: f64,
    pub resource_distribution: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub event_id: Uuid,
    pub event_type: LearningEventType,
    pub participants: Vec<Uuid>,
    pub knowledge_transfer: Vec<String>,
    pub cultural_impact: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningEventType {
    KnowledgeDiscovery(String),
    CulturalInnovation(String),
    TechnologicalBreakthrough(String),
    SocialChange(String),
    LanguageEvolution(String),
}

impl WorldLearningSystem {
    pub fn new() -> Self {
        Self {
            npc_learning_systems: HashMap::new(),
            global_knowledge_base: GlobalKnowledgeBase::new(),
            learning_events: Vec::new(),
            pre_genesis_training: None,
            cultural_evolution: CulturalEvolution::new(),
        }
    }

    /// Initialize pre-genesis training for all NPCs
    pub fn start_pre_genesis_training(&mut self, npc_ids: Vec<Uuid>) -> Result<(), String> {
        let training = PreGenesisTraining {
            training_modules: Self::create_basic_training_modules(),
            completion_percentage: 0.0,
            training_start_time: Utc::now(),
            estimated_completion_time: Utc::now() + chrono::Duration::hours(24), // 24 hour training
        };

        self.pre_genesis_training = Some(training);

        // Initialize learning systems for all NPCs
        for npc_id in npc_ids {
            let learning_system = NpcLearningSystem::new(npc_id, 0.5); // Medium intelligence
            self.npc_learning_systems.insert(npc_id, learning_system);
            
            // Start basic training for each NPC
            self.apply_pre_genesis_training_to_npc(npc_id)?;
        }

        println!("🧠 Pre-genesis training started for {} NPCs", self.npc_learning_systems.len());
        Ok(())
    }

    /// Apply pre-genesis training to a specific NPC
    fn apply_pre_genesis_training_to_npc(&mut self, npc_id: Uuid) -> Result<(), String> {
        if let Some(learning_system) = self.npc_learning_systems.get_mut(&npc_id) {
            // Apply basic survival motivations
            learning_system.motivations.push(Motivation {
                motivation_type: MotivationType::BasicSurvival,
                strength: 0.9,
                priority: 1,
                satisfaction_level: 0.3,
                related_goals: vec!["food".to_string(), "shelter".to_string(), "safety".to_string()],
                triggers: vec!["hunger".to_string(), "cold".to_string(), "danger".to_string()],
            });

            learning_system.motivations.push(Motivation {
                motivation_type: MotivationType::CollectiveSurvival,
                strength: 0.8,
                priority: 2,
                satisfaction_level: 0.2,
                related_goals: vec!["community".to_string(), "cooperation".to_string(), "building".to_string()],
                triggers: vec!["isolation".to_string(), "resource_scarcity".to_string()],
            });

            learning_system.motivations.push(Motivation {
                motivation_type: MotivationType::Creation,
                strength: 0.6,
                priority: 3,
                satisfaction_level: 0.1,
                related_goals: vec!["crafting".to_string(), "building".to_string(), "tools".to_string()],
                triggers: vec!["need".to_string(), "opportunity".to_string()],
            });

            // Add basic archaic English vocabulary
            learning_system.language_model.vocabulary.insert("greetings".to_string(), WordKnowledge {
                word: "Hail".to_string(),
                frequency_used: 10,
                contexts: vec!["meeting".to_string(), "formal".to_string()],
                associated_emotions: HashMap::new(),
                formality_level: 0.8,
            });

            learning_system.language_model.vocabulary.insert("agreement".to_string(), WordKnowledge {
                word: "Verily".to_string(),
                frequency_used: 8,
                contexts: vec!["agreement".to_string(), "emphasis".to_string()],
                associated_emotions: HashMap::new(),
                formality_level: 0.7,
            });

            // Pre-train neural patterns for cooperation
            learning_system.neural_patterns.pattern_weights.insert("cooperation_success".to_string(), 0.8);
            learning_system.neural_patterns.pattern_weights.insert("building_success".to_string(), 0.7);
            learning_system.neural_patterns.pattern_weights.insert("sharing_success".to_string(), 0.6);

            // Add basic knowledge domains
            learning_system.knowledge_domains.insert("Construction".to_string(), KnowledgeDomain {
                domain_name: "Construction".to_string(),
                proficiency: 0.3,
                sub_topics: HashMap::new(),
                learned_facts: vec![
                    LearnedFact {
                        fact_id: Uuid::new_v4(),
                        content: "Structures need strong foundations".to_string(),
                        source: LearningSource::Tradition("Ancient Knowledge".to_string()),
                        confidence: 0.8,
                        verification_count: 1,
                        last_verified: Utc::now(),
                        tags: vec!["building".to_string(), "foundation".to_string()],
                    }
                ],
                teaching_ability: 0.2,
                last_updated: Utc::now(),
            });

            learning_system.knowledge_domains.insert("Survival".to_string(), KnowledgeDomain {
                domain_name: "Survival".to_string(),
                proficiency: 0.5,
                sub_topics: HashMap::new(),
                learned_facts: vec![
                    LearnedFact {
                        fact_id: Uuid::new_v4(),
                        content: "Water and food are essential for life".to_string(),
                        source: LearningSource::Intuition,
                        confidence: 0.9,
                        verification_count: 1,
                        last_verified: Utc::now(),
                        tags: vec!["survival".to_string(), "basic needs".to_string()],
                    }
                ],
                teaching_ability: 0.4,
                last_updated: Utc::now(),
            });

            Ok(())
        } else {
            Err(format!("NPC learning system not found for ID: {}", npc_id))
        }
    }

    /// Create basic training modules for pre-genesis training
    fn create_basic_training_modules() -> Vec<TrainingModule> {
        vec![
            TrainingModule {
                module_name: "Basic Language".to_string(),
                module_type: TrainingModuleType::LanguageBasics,
                completion_status: TrainingStatus::NotStarted,
                knowledge_gained: vec![
                    "Basic archaic English vocabulary".to_string(),
                    "Formal speech patterns".to_string(),
                    "Respectful communication".to_string(),
                ],
                skills_developed: vec!["Charisma".to_string()],
                neural_patterns_learned: HashMap::from([
                    ("polite_communication".to_string(), 0.7),
                    ("formal_speech".to_string(), 0.6),
                ]),
            },
            TrainingModule {
                module_name: "Survival Instincts".to_string(),
                module_type: TrainingModuleType::SurvivalInstincts,
                completion_status: TrainingStatus::NotStarted,
                knowledge_gained: vec![
                    "Basic needs identification".to_string(),
                    "Resource conservation".to_string(),
                    "Threat assessment".to_string(),
                ],
                skills_developed: vec!["Wisdom".to_string(), "Vitality".to_string()],
                neural_patterns_learned: HashMap::from([
                    ("survival_priority".to_string(), 0.9),
                    ("resource_awareness".to_string(), 0.8),
                ]),
            },
            TrainingModule {
                module_name: "Social Cooperation".to_string(),
                module_type: TrainingModuleType::SocialCooperation,
                completion_status: TrainingStatus::NotStarted,
                knowledge_gained: vec![
                    "Group dynamics".to_string(),
                    "Collaborative work patterns".to_string(),
                    "Conflict resolution basics".to_string(),
                ],
                skills_developed: vec!["Charisma".to_string(), "Wisdom".to_string()],
                neural_patterns_learned: HashMap::from([
                    ("cooperation_success".to_string(), 0.8),
                    ("group_harmony".to_string(), 0.7),
                ]),
            },
            TrainingModule {
                module_name: "Basic Construction".to_string(),
                module_type: TrainingModuleType::BasicCrafting,
                completion_status: TrainingStatus::NotStarted,
                knowledge_gained: vec![
                    "Tool usage".to_string(),
                    "Material properties".to_string(),
                    "Basic building techniques".to_string(),
                ],
                skills_developed: vec!["Carpentry".to_string(), "Tool Use".to_string()],
                neural_patterns_learned: HashMap::from([
                    ("building_success".to_string(), 0.7),
                    ("tool_efficiency".to_string(), 0.6),
                ]),
            },
        ]
    }

    /// Process learning interactions between NPCs
    pub fn process_npc_interaction(&mut self, npc1_id: Uuid, npc2_id: Uuid, interaction_type: ExperienceType, context: &str) {
        // Create shared learning experience
        let experience = LearningExperience {
            experience_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            experience_type: interaction_type.clone(),
            outcome: ExperienceOutcome::Success(0.7), // Moderate success by default
            emotional_impact: EmotionalImpact {
                satisfaction: 0.5,
                curiosity: 0.3,
                confidence: 0.1,
                stress: 0.0,
                social_bond_change: 0.1,
            },
            knowledge_gained: self.determine_knowledge_from_interaction(&interaction_type, context),
            skills_improved: vec!["Charisma".to_string()],
            participants: vec![npc2_id],
            location: context.to_string(),
            context: format!("Interaction with other NPC: {}", context),
        };

        // Apply experience to both NPCs
        if let Some(npc1_learning) = self.npc_learning_systems.get_mut(&npc1_id) {
            npc1_learning.process_experience(experience.clone());
        }

        if let Some(npc2_learning) = self.npc_learning_systems.get_mut(&npc2_id) {
            let mut npc2_experience = experience.clone();
            npc2_experience.participants = vec![npc1_id];
            npc2_learning.process_experience(npc2_experience);
        }

        // Record learning event
        self.learning_events.push(LearningEvent {
            event_id: Uuid::new_v4(),
            event_type: LearningEventType::SocialChange("NPC Interaction".to_string()),
            participants: vec![npc1_id, npc2_id],
            knowledge_transfer: experience.knowledge_gained,
            cultural_impact: 0.1,
            timestamp: Utc::now(),
        });
    }

    /// Determine knowledge gained from interaction type
    fn determine_knowledge_from_interaction(&self, interaction_type: &ExperienceType, context: &str) -> Vec<String> {
        match interaction_type {
            ExperienceType::Conversation(_) => vec![
                "Social interaction patterns".to_string(),
                "Communication effectiveness".to_string(),
            ],
            ExperienceType::Collaboration(_) => vec![
                "Teamwork benefits".to_string(),
                "Shared labor efficiency".to_string(),
            ],
            ExperienceType::Teaching(_) => vec![
                "Knowledge transfer methods".to_string(),
                "Learning from others".to_string(),
            ],
            ExperienceType::Construction(_) => vec![
                "Building techniques".to_string(),
                "Material usage".to_string(),
                "Structural integrity".to_string(),
            ],
            ExperienceType::Crafting(_) => vec![
                "Tool creation".to_string(),
                "Resource transformation".to_string(),
            ],
            _ => vec![format!("General knowledge from {}", context)],
        }
    }

    /// Update global knowledge base when NPCs make discoveries
    pub fn record_discovery(&mut self, discoverer_id: Uuid, knowledge_content: &str, domain: &str) {
        let shared_knowledge = SharedKnowledge {
            knowledge_id: Uuid::new_v4(),
            content: knowledge_content.to_string(),
            domain: domain.to_string(),
            discoverer: discoverer_id,
            verification_count: 1,
            spread_rate: 0.1,
            accuracy: 0.8,
            cultural_impact: 0.2,
        };

        self.global_knowledge_base.shared_knowledge.insert(
            knowledge_content.to_string(),
            shared_knowledge
        );

        // Record as learning event
        self.learning_events.push(LearningEvent {
            event_id: Uuid::new_v4(),
            event_type: LearningEventType::KnowledgeDiscovery(domain.to_string()),
            participants: vec![discoverer_id],
            knowledge_transfer: vec![knowledge_content.to_string()],
            cultural_impact: 0.2,
            timestamp: Utc::now(),
        });
    }

    /// Get NPC learning system for decision making
    pub fn get_npc_decision(&self, npc_id: Uuid, situation: &str, options: &[String]) -> Option<String> {
        if let Some(learning_system) = self.npc_learning_systems.get(&npc_id) {
            Some(learning_system.make_decision(situation, options))
        } else {
            None
        }
    }

    /// Generate NPC speech using learned language patterns
    pub fn generate_npc_speech(&self, npc_id: Uuid, context: &str, topic: &str) -> Option<String> {
        if let Some(learning_system) = self.npc_learning_systems.get(&npc_id) {
            Some(learning_system.generate_speech(context, topic))
        } else {
            None
        }
    }
}

impl GlobalKnowledgeBase {
    pub fn new() -> Self {
        Self {
            shared_knowledge: HashMap::new(),
            cultural_artifacts: Vec::new(),
            technological_discoveries: Vec::new(),
            linguistic_evolution: LanguageEvolution::new(),
        }
    }
}

impl LanguageEvolution {
    pub fn new() -> Self {
        Self {
            vocabulary_changes: HashMap::new(),
            grammar_evolution: Vec::new(),
            dialect_formation: HashMap::new(),
            archaic_preservation: 0.8, // High preservation of archaic forms
        }
    }
}

impl CulturalEvolution {
    pub fn new() -> Self {
        Self {
            traditions: HashMap::new(),
            social_norms: HashMap::new(),
            governance_patterns: Vec::new(),
            economic_systems: Vec::new(),
        }
    }
}

impl ExperienceType {
    pub fn to_string(&self) -> String {
        match self {
            ExperienceType::Conversation(topic) => format!("conversation_{}", topic),
            ExperienceType::Teaching(subject) => format!("teaching_{}", subject),
            ExperienceType::Crafting(item) => format!("crafting_{}", item),
            ExperienceType::Construction(structure) => format!("construction_{}", structure),
            ExperienceType::Gathering(resource) => format!("gathering_{}", resource),
            ExperienceType::Exploration(area) => format!("exploration_{}", area),
            ExperienceType::Collaboration(project) => format!("collaboration_{}", project),
            ExperienceType::Conflict(reason) => format!("conflict_{}", reason),
            ExperienceType::Trade(goods) => format!("trade_{}", goods),
            ExperienceType::Problem_Solving(challenge) => format!("problem_solving_{}", challenge),
            ExperienceType::Ritual(ceremony) => format!("ritual_{}", ceremony),
            ExperienceType::Storytelling(story) => format!("storytelling_{}", story),
            ExperienceType::Art_Creation(artwork) => format!("art_creation_{}", artwork),
            ExperienceType::Music_Performance(performance) => format!("music_performance_{}", performance),
            ExperienceType::Reading(content) => format!("reading_{}", content),
            ExperienceType::Research(topic) => format!("research_{}", topic),
            ExperienceType::Experimentation(experiment) => format!("experimentation_{}", experiment),
            ExperienceType::Observation(observation) => format!("observation_{}", observation),
        }
    }
}