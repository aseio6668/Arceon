use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::archetypes::ArchetypeSystem;

/// Universal Being system - represents any living entity (NPC, Player, Beast, etc.)
/// All beings can have skills, vitals, and capabilities regardless of type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Being {
    pub id: Uuid,
    pub name: String,
    pub being_type: BeingType,
    pub race: Race,
    pub is_feral: bool, // Feral beings don't use inventory/equipment but have skills
    
    // Core systems
    pub vitals: VitalSystem,
    pub skills: SkillSystem,
    pub archetypes: ArchetypeSystem,
    pub capabilities: Vec<BeingCapability>,
    
    // Life cycle tracking
    pub incarnation_history: Vec<Incarnation>,
    pub current_incarnation: Incarnation,
    
    // Discovery and evolution tracking
    pub discovered_skills: Vec<String>,
    pub evolution_contributions: Vec<EvolutionContribution>,
    
    // Optional inventory system (None for feral beings)
    pub inventory: Option<InventorySystem>,
    pub equipment: Option<EquipmentSystem>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BeingType {
    Player,
    Npc,
    Beast,        // Dire wolves, dragons, etc.
    Elemental,    // Fire elementals, earth sprites, etc.
    Undead,       // Skeletons, zombies, etc.
    Construct,    // Golems, automatons, etc.
    Spirit,       // Ghosts, phantoms, etc.
    Deity,        // Gods, demigods, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Race {
    // Standard races
    Human, Elf, Dwarf, Halfling, Orc, Gnome, Dragonborn, Tiefling,
    
    // Beast races
    DireWolf, Dragon, Griffin, Phoenix, Basilisk,
    
    // Elemental races  
    FireElemental, WaterElemental, EarthElemental, AirElemental,
    
    // Other races
    Lich, Vampire, Golem, Treant, Fey,
    
    // Allow for future expansion
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BeingCapability {
    // Universal capabilities
    CanMove, CanCommunicate, CanLearn, CanTeach,
    
    // Intelligence-based capabilities
    CanReason, CanPlan, CanRemember, CanDream,
    
    // Physical capabilities
    CanFly, CanSwim, CanBurrow, CanClimb,
    
    // Social capabilities
    CanTrade, CanLead, CanFollow, CanNegotiate,
    
    // Magical capabilities
    CanCastSpells, CanChannelMana, CanSenseAuras,
    
    // Craft capabilities
    CanCraft, CanBuild, CanEnchant, CanAlchemy,
    
    // Combat capabilities
    CanFight, CanParry, CanDodge, CanBerserk,
}

/// Primitive Vitals System - Core life forces of all beings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSystem {
    // Essential vitals (always present)
    pub health: Vital,      // Hitpoints - 0 = death
    pub energy: Vital,      // For physical actions, sprinting, climbing
    pub mana: Vital,        // For magical spells and abilities
    
    // Optional vitals (being can choose 2-3 additional)
    pub optional_vitals: HashMap<String, Vital>,
    
    // Vital preferences for this being
    pub active_optionals: Vec<String>, // Max 2-3 optional vitals active
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vital {
    pub name: String,
    pub current: f64,
    pub maximum: f64,
    pub base_maximum: f64,     // Base max before skill modifications
    pub regeneration_rate: f64, // Per second
    pub skill_influence: String, // Related skill that affects this vital
    pub is_essential: bool,    // True for health/energy/mana
    pub unlock_requirements: Vec<SkillRequirement>,
}

/// Comprehensive Skill System with discovery and evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSystem {
    pub skills: HashMap<String, Skill>,
    pub skill_discovery_log: Vec<SkillDiscovery>,
    pub evolution_requests: Vec<EvolutionRequest>,
    pub experience_log: Vec<ExperienceGain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: f64,           // Continuous progression (no caps)
    pub experience: f64,      // Current experience in this skill
    pub category: SkillCategory,
    
    // Dual trait system - every skill has both
    pub passive_trait: PassiveTrait,
    pub active_trait: Option<ActiveTrait>, // If None, only passive exists
    
    // Discovery and evolution
    pub discovered_by: Vec<Uuid>,  // Which beings discovered this skill
    pub is_innate: bool,          // True for basic skills like Defense, Taunt
    pub unlock_requirements: Vec<SkillRequirement>,
    pub evolution_potential: Vec<String>, // Skills this might lead to
    
    // Skill relationships
    pub synergy_skills: Vec<String>,      // Skills that enhance this one
    pub prerequisite_skills: Vec<String>, // Skills needed to unlock this
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum SkillType {
    // Core skills that exist in the system
    Defense, Health, Mana, Taunt, Wrath, Fury, FireMagic, WeaponEnchantment,
    
    // AI-expected skills for compatibility
    Mining, Smithing, Carpentry, OneHandedSword, Leadership, Trading,
    
    // Crafting skills
    Metalworking, Herbalism, Woodworking, Leatherworking,
    
    // Additional skills for expansion
    Athletics, Stealth, Intellect, Focus, Shadow, DeathMagic, Enchanting, Alchemy, Geology,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SkillCategory {
    // Combat skills
    Combat, Defense, Weapons, Archery, Unarmed, Armor,
    
    // Magic skills
    Arcane, Divine, Elemental, Enchantment, Alchemy,
    
    // Physical skills
    Athletics, Stealth, Survival, Acrobatics,
    
    // Mental skills
    Intellect, Wisdom, Focus, Memory,
    
    // Social skills
    Leadership, Persuasion, Intimidation, Deception,
    
    // Craft skills
    Smithing, Tailoring, Cooking, Engineering,
    
    // Vital skills (influence vitals)
    Health, Energy, Mana, Wrath, Fury, Spirit,
    
    // Discovery category for new skills
    Experimental,
}

/// Passive traits - always active, influence being's capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveTrait {
    pub name: String,
    pub description: String,
    pub effects: Vec<PassiveEffect>,
    pub formula: SkillFormula, // How skill level affects the passive
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassiveEffect {
    // Vital modifications
    IncreaseMaxHealth(f64),
    IncreaseMaxMana(f64),
    IncreaseMaxEnergy(f64),
    IncreaseRegeneration(String, f64), // vital_name, multiplier
    
    // Combat modifications  
    IncreaseDamage(f64),
    IncreaseDefense(f64),
    IncreaseThreat(f64),        // For taunt skill passive
    IncreaseParryChance(f64),
    
    // Skill synergies
    BoostSkillExperience(String, f64), // skill_name, multiplier
    ReduceSkillCooldowns(String, f64), // skill_category, reduction
    
    // Special abilities
    EnableCapability(BeingCapability),
    ModifyMovementSpeed(f64),
    GrantResistance(String, f64), // damage_type, reduction
}

/// Active traits - can be triggered with cooldowns/costs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTrait {
    pub name: String,
    pub description: String,
    pub activation_cost: ActivationCost,
    pub cooldown: SkillCooldown,
    pub effects: Vec<ActiveEffect>,
    pub can_be_hotkeyed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationCost {
    Energy(f64),
    Mana(f64),
    Health(f64),
    OptionalVital(String, f64), // vital_name, amount
    Combination(Vec<ActivationCost>),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCooldown {
    pub base_duration: f64,    // Base cooldown in seconds
    pub current_remaining: f64, // Current cooldown remaining
    pub reduction_formula: SkillFormula, // How skill level reduces cooldown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActiveEffect {
    // Direct damage/healing
    DealDamage(f64),
    HealSelf(f64),
    RestoreVital(String, f64),
    
    // Combat effects
    Taunt(f64),              // Force target to attack you
    Stun(f64),               // Disable target for duration
    Knockback(f64),          // Push target away
    
    // Buff/debuff effects
    TemporaryBuff(String, f64, f64), // stat, amount, duration
    AreaOfEffect(f64, Vec<ActiveEffect>), // radius, effects to apply
    
    // Movement effects  
    Teleport(f64),           // Max distance
    Sprint(f64, f64),        // Speed multiplier, duration
    
    // Magical effects
    CastSpell(String),       // Spell name to cast
    EnchantWeapon(String, f64), // Element, duration
    
    // Crafting and progression effects
    CraftItem(String, u32),  // Item name, quantity
    GrantExperience(String, f64), // Skill name, experience amount
}

/// Formula system for skill level calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillFormula {
    Linear(f64),              // level * multiplier
    Logarithmic(f64, f64),    // base * log(level + offset)
    Exponential(f64, f64),    // base * (level ^ exponent)
    Threshold(Vec<(f64, f64)>), // [(threshold, value), ...] for step functions
    Custom(String),           // Custom formula as string
}

/// Skill requirements for unlocking skills or vitals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillRequirement {
    SkillLevel(String, f64),  // skill_name, min_level
    CombinedSkills(Vec<(String, f64)>), // Multiple skills needed
    VitalThreshold(String, f64), // vital_name, min_value
    BeingType(BeingType),
    Race(Race),
    Capability(BeingCapability),
    DiscoveredSkill(String),  // Must have discovered specific skill
    EvolutionContribution(String), // Must have contributed to skill evolution
}

/// Experience gain tracking - only way to improve skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceGain {
    pub skill_name: String,
    pub amount: f64,
    pub source: ExperienceSource,
    pub timestamp: DateTime<Utc>,
    pub being_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceSource {
    // Combat sources
    CombatVictory, CombatDefeat, TakingDamage, DealingDamage,
    BlockingAttack, ParryingAttack, DodgingAttack,
    
    // Skill usage sources
    ActiveSkillUse(String),   // Used active trait
    PassiveSkillTrigger(String), // Passive trait activated
    SkillSynergy(String, String), // skill1 boosted skill2
    
    // Discovery sources
    SkillDiscovery, SkillEvolution, TeachingSkill,
    LearningFromOthers, ExperimentingWithSkills,
    
    // Survival sources
    SurvivingDanger, OvercomingObstacles, AdaptingToEnvironment,
    
    // Social sources
    LeadershipAction, FollowingOrders, Negotiation,
    
    // Magical sources
    CastingSpells, ChannelingMana, StudyingArcane,
    EnchantingItems, AlchemyExperiment,
    
    // Craft sources
    CraftingItems, BuildingStructures, RepairingEquipment,
    
    // Vital sources (for health/mana/energy skills)
    VitalRegeneration, VitalDepletion, VitalMaxIncrease,
}

/// Skill discovery system - NPCs can find new skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDiscovery {
    pub skill_name: String,
    pub discovered_by: Uuid,
    pub discovery_method: DiscoveryMethod,
    pub timestamp: DateTime<Utc>,
    pub prerequisites_met: Vec<SkillRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Experimentation,          // Trying different skill combinations
    ObservingOthers,         // Watching other beings use skills
    EnvironmentalInteraction, // Learning from world interactions
    CombatAdaptation,        // Developing skills during combat
    ArcaneStudy,             // Studying magical phenomena
    RunestoneRevelation,     // Learning from ancient knowledge
    EmotionalBreakthrough,   // Unlocking through intense emotion
    PhysicalChallenge,       // Overcoming physical limitations
}

/// Evolution system - NPCs can request new skills from consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRequest {
    pub requested_skill: String,
    pub requesting_being: Uuid,
    pub justification: String,   // Why this skill should exist
    pub proposed_traits: (PassiveTrait, Option<ActiveTrait>),
    pub prerequisite_skills: Vec<String>,
    pub supporting_evidence: Vec<EvolutionEvidence>,
    pub timestamp: DateTime<Utc>,
    pub status: RequestStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionEvidence {
    SkillCombinationUsed(Vec<String>), // Used these skills together
    ObservedPhenomena(String),         // Saw this happen in world
    RunestoneKnowledge(String),        // Ancient texts mention this
    CombatNecessity(String),           // Needed this in battle
    CraftingInnovation(String),        // Discovered through crafting
    MagicalDiscovery(String),          // Found through magic study
    SocialNeed(String),                // Community needs this skill
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStatus {
    Submitted,
    UnderReview,
    CommunityVoting,
    DeveloperReview,
    Approved,
    Rejected(String), // Reason for rejection
    Implemented,
}

/// Evolution contribution tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionContribution {
    pub skill_name: String,
    pub contribution_type: ContributionType,
    pub timestamp: DateTime<Utc>,
    pub impact_level: f64, // How much this contributed to skill creation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    FirstDiscovery,           // First to discover prerequisite
    ConceptDevelopment,       // Helped develop the skill concept
    TestingAndRefinement,     // Tested skill mechanics
    CommunityAdvocacy,        // Advocated for skill in community
    ImplementationFeedback,   // Provided feedback during implementation
}

/// Incarnation system - tracks being's life cycles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incarnation {
    pub id: Uuid,
    pub being_id: Uuid,
    pub incarnation_number: u32,
    pub race: Race,
    pub being_type: BeingType,
    pub birth_timestamp: DateTime<Utc>,
    pub death_timestamp: Option<DateTime<Utc>>,
    pub cause_of_death: Option<String>,
    pub chose_reincarnation: bool,
    pub afterlife_plane: Option<String>,
    pub skill_achievements: Vec<String>, // Major skills mastered in this life
    pub evolution_contributions: Vec<String>, // Skills they helped create
    pub is_final_death: bool,         // True if being chose permanent death
}

/// Optional inventory system (None for feral beings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySystem {
    pub items: HashMap<Uuid, InventoryItem>,
    pub max_weight: Option<f64>, // None = unlimited (Arceon style)
    pub max_slots: Option<u32>,  // None = unlimited
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: Uuid,
    pub stack_size: u32,
    pub max_stack: u32, // Up to 999+ as mentioned
}

/// Optional equipment system (None for feral beings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentSystem {
    pub head: Option<Uuid>,
    pub shoulders: Option<Uuid>,
    pub arms: Option<Uuid>,
    pub chest: Option<Uuid>,
    pub hands: Option<Uuid>,
    pub waist: Option<Uuid>,
    pub legs: Option<Uuid>,
    pub feet: Option<Uuid>,
    pub back: Option<Uuid>,
    pub rings: Vec<Uuid>,        // Multiple rings allowed
    pub neck: Option<Uuid>,
    pub main_hand: Option<Uuid>,
    pub off_hand: Option<Uuid>,
}

impl Being {
    /// Create a new being with essential vitals and basic capabilities
    pub fn new(name: String, being_type: BeingType, race: Race, is_feral: bool) -> Self {
        let id = Uuid::new_v4();
        let incarnation = Incarnation {
            id: Uuid::new_v4(),
            being_id: id,
            incarnation_number: 1,
            race: race.clone(),
            being_type: being_type.clone(),
            birth_timestamp: Utc::now(),
            death_timestamp: None,
            cause_of_death: None,
            chose_reincarnation: false,
            afterlife_plane: None,
            skill_achievements: Vec::new(),
            evolution_contributions: Vec::new(),
            is_final_death: false,
        };

        let capabilities = Self::default_capabilities(&being_type, &race, is_feral);

        Self {
            id,
            name,
            being_type,
            race,
            is_feral,
            vitals: VitalSystem::new(),
            skills: SkillSystem::new(),
            archetypes: ArchetypeSystem::new(),
            capabilities,
            incarnation_history: Vec::new(),
            current_incarnation: incarnation,
            discovered_skills: Vec::new(),
            evolution_contributions: Vec::new(),
            inventory: if is_feral { None } else { Some(InventorySystem::new()) },
            equipment: if is_feral { None } else { Some(EquipmentSystem::new()) },
        }
    }

    /// Get default capabilities based on being type and race
    fn default_capabilities(being_type: &BeingType, race: &Race, is_feral: bool) -> Vec<BeingCapability> {
        let mut capabilities = vec![
            BeingCapability::CanMove,
            BeingCapability::CanLearn,
        ];

        // Add capabilities based on being type
        match being_type {
            BeingType::Player | BeingType::Npc => {
                capabilities.extend(vec![
                    BeingCapability::CanCommunicate,
                    BeingCapability::CanTeach,
                    BeingCapability::CanReason,
                    BeingCapability::CanPlan,
                    BeingCapability::CanRemember,
                ]);
            },
            BeingType::Beast => {
                if !is_feral {
                    capabilities.push(BeingCapability::CanCommunicate);
                }
            },
            BeingType::Deity => {
                capabilities.extend(vec![
                    BeingCapability::CanCommunicate,
                    BeingCapability::CanTeach,
                    BeingCapability::CanCastSpells,
                    BeingCapability::CanChannelMana,
                ]);
            },
            _ => {}, // Other types get basic capabilities only
        }

        // Add race-specific capabilities
        match race {
            Race::Dragon | Race::Phoenix => {
                capabilities.push(BeingCapability::CanFly);
            },
            Race::DireWolf => {
                // Feral wolves are naturally aggressive
            },
            Race::Elf => {
                capabilities.push(BeingCapability::CanSenseAuras);
            },
            Race::Dwarf => {
                capabilities.push(BeingCapability::CanCraft);
            },
            _ => {},
        }

        // Non-feral beings can trade and use equipment
        if !is_feral && matches!(being_type, BeingType::Player | BeingType::Npc) {
            capabilities.extend(vec![
                BeingCapability::CanTrade,
                BeingCapability::CanCraft,
                BeingCapability::CanBuild,
            ]);
        }

        capabilities
    }

    /// Create a new player being with default stats
    pub fn new_player(name: String, race: Race) -> Self {
        let mut being = Self::new(name, BeingType::Player, race, false);
        
        // Give player characters the core attribute skills
        let skill_registry = super::skills::SkillRegistry::new();
        
        // Add core attributes with starting levels
        let core_skills = vec![
            ("Dexterity", 5.0), ("Courage", 5.0), ("Wisdom", 5.0),
            ("Strength", 5.0), ("Vitality", 5.0), ("Charisma", 5.0), ("Intelligence", 5.0)
        ];
        
        for (skill_name, starting_level) in core_skills {
            if let Some(mut skill) = skill_registry.create_skill(skill_name) {
                skill.level = starting_level;
                skill.experience = (starting_level * starting_level * 10.0).max(25.0); // Scaled experience
                skill.discovered_by.push(being.id);
                being.skills.skills.insert(skill_name.to_string(), skill);
            }
        }
        
        // Give traditional starting skills
        being.skills.skills.insert("Health".to_string(), Skill {
            name: "Health".to_string(),
            level: 10.0,
            experience: 100.0,
            category: SkillCategory::Health,
            passive_trait: PassiveTrait {
                name: "Health Boost".to_string(),
                description: "Improves maximum health".to_string(),
                effects: vec![PassiveEffect::IncreaseMaxHealth(10.0)],
                formula: SkillFormula::Linear(1.0),
            },
            active_trait: None,
            discovered_by: vec![being.id],
            is_innate: true,
            unlock_requirements: Vec::new(),
            evolution_potential: Vec::new(),
            synergy_skills: Vec::new(),
            prerequisite_skills: Vec::new(),
        });
        
        being.skills.skills.insert("Defense".to_string(), Skill {
            name: "Defense".to_string(),
            level: 5.0,
            experience: 50.0,
            category: SkillCategory::Defense,
            passive_trait: PassiveTrait {
                name: "Basic Defense".to_string(),
                description: "Improves damage resistance".to_string(),
                effects: vec![PassiveEffect::IncreaseDefense(0.05)],
                formula: SkillFormula::Linear(1.0),
            },
            active_trait: None,
            discovered_by: vec![being.id],
            is_innate: true,
            unlock_requirements: Vec::new(),
            evolution_potential: Vec::new(),
            synergy_skills: Vec::new(),
            prerequisite_skills: Vec::new(),
        });
        
        // Initialize archetypes based on starting skills
        being.update_archetypes();
        
        being
    }

    /// Gain experience in a skill from an action
    pub fn gain_skill_experience(&mut self, skill_name: &str, amount: f64, source: ExperienceSource) {
        // Create skill if it doesn't exist (discovery)
        if !self.skills.skills.contains_key(skill_name) {
            if let Some(skill) = self.discover_skill(skill_name, DiscoveryMethod::Experimentation) {
                self.skills.skills.insert(skill_name.to_string(), skill);
            } else {
                return; // Skill couldn't be discovered
            }
        }

        // Add experience to skill
        if let Some(skill) = self.skills.skills.get_mut(skill_name) {
            skill.experience += amount;
            
            // Calculate new level (continuous progression)
            let new_level = Self::calculate_skill_level(skill.experience);
            let old_level = skill.level;
            skill.level = new_level;

            // Log experience gain
            self.skills.experience_log.push(ExperienceGain {
                skill_name: skill_name.to_string(),
                amount,
                source,
                timestamp: Utc::now(),
                being_id: self.id,
            });

            // Check for skill evolution potential
            if new_level > old_level {
                self.check_skill_evolution(skill_name);
                // Update archetypes when skills change
                self.update_archetypes();
            }
        }
    }

    /// Calculate skill level from experience (no level caps)
    fn calculate_skill_level(experience: f64) -> f64 {
        // Logarithmic progression: level = log2(exp/100 + 1) * 10
        // This gives smooth, continuous progression without caps
        ((experience / 100.0 + 1.0).log2() * 10.0).max(1.0)
    }

    /// Attempt to discover a new skill
    fn discover_skill(&mut self, skill_name: &str, _method: DiscoveryMethod) -> Option<Skill> {
        // Check if skill exists in global skill database (would be loaded from config)
        // For now, create basic skills that NPCs might discover
        match skill_name {
            "Defense" => Some(Self::create_defense_skill()),
            "Taunt" => Some(Self::create_taunt_skill()),
            "Fire Magic" => Some(Self::create_fire_magic_skill()),
            "Weapon Enchantment" => Some(Self::create_weapon_enchantment_skill()),
            "Health" => Some(Self::create_health_skill()),
            "Wrath" => Some(Self::create_wrath_skill()),
            "Fury" => Some(Self::create_fury_skill()),
            _ => {
                // Unknown skill - could request from evolution system
                self.request_skill_evolution(skill_name.to_string());
                None
            }
        }
    }

    /// Create the Defense skill as an example
    fn create_defense_skill() -> Skill {
        Skill {
            name: "Defense".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Defense,
            passive_trait: PassiveTrait {
                name: "Natural Defense".to_string(),
                description: "Passively reduces incoming damage".to_string(),
                effects: vec![
                    PassiveEffect::IncreaseDefense(0.5), // 0.5% per level
                ],
                formula: SkillFormula::Linear(0.005), // 0.5% per level
            },
            active_trait: Some(ActiveTrait {
                name: "Defensive Stance".to_string(),
                description: "Greatly increase defense for a short time".to_string(),
                activation_cost: ActivationCost::Energy(20.0),
                cooldown: SkillCooldown {
                    base_duration: 30.0,
                    current_remaining: 0.0,
                    reduction_formula: SkillFormula::Linear(0.1), // -0.1s per level
                },
                effects: vec![
                    ActiveEffect::TemporaryBuff("defense".to_string(), 50.0, 10.0),
                ],
                can_be_hotkeyed: true,
            }),
            discovered_by: Vec::new(),
            is_innate: true,
            unlock_requirements: Vec::new(),
            evolution_potential: vec!["Shield Mastery".to_string(), "Armor Expertise".to_string()],
            synergy_skills: vec!["Health".to_string()],
            prerequisite_skills: Vec::new(),
        }
    }

    /// Create the Taunt skill as an example
    fn create_taunt_skill() -> Skill {
        Skill {
            name: "Taunt".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Combat,
            passive_trait: PassiveTrait {
                name: "Threatening Presence".to_string(),
                description: "Passively increases threat generation in combat".to_string(),
                effects: vec![
                    PassiveEffect::IncreaseThreat(1.0), // 1% per level
                ],
                formula: SkillFormula::Linear(0.01),
            },
            active_trait: Some(ActiveTrait {
                name: "Taunt".to_string(),
                description: "Force target to attack you".to_string(),
                activation_cost: ActivationCost::Energy(15.0),
                cooldown: SkillCooldown {
                    base_duration: 15.0,
                    current_remaining: 0.0,
                    reduction_formula: SkillFormula::Linear(0.05),
                },
                effects: vec![
                    ActiveEffect::Taunt(5.0), // 5 second taunt
                ],
                can_be_hotkeyed: true,
            }),
            discovered_by: Vec::new(),
            is_innate: true,
            unlock_requirements: Vec::new(),
            evolution_potential: vec!["Intimidation".to_string(), "Battle Cry".to_string()],
            synergy_skills: vec!["Defense".to_string()],
            prerequisite_skills: Vec::new(),
        }
    }

    /// Create Fire Magic skill
    fn create_fire_magic_skill() -> Skill {
        Skill {
            name: "Fire Magic".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Elemental,
            passive_trait: PassiveTrait {
                name: "Fire Affinity".to_string(),
                description: "Increases fire damage and resistance to fire".to_string(),
                effects: vec![
                    PassiveEffect::GrantResistance("fire".to_string(), 2.0), // 2% per level
                    PassiveEffect::IncreaseDamage(1.0), // When using fire spells
                ],
                formula: SkillFormula::Linear(0.02),
            },
            active_trait: Some(ActiveTrait {
                name: "Fireball".to_string(),
                description: "Launch a ball of fire at target".to_string(),
                activation_cost: ActivationCost::Mana(25.0),
                cooldown: SkillCooldown {
                    base_duration: 3.0,
                    current_remaining: 0.0,
                    reduction_formula: SkillFormula::Linear(0.01),
                },
                effects: vec![
                    ActiveEffect::DealDamage(50.0), // Base damage, scales with level
                ],
                can_be_hotkeyed: true,
            }),
            discovered_by: Vec::new(),
            is_innate: false,
            unlock_requirements: vec![
                SkillRequirement::VitalThreshold("mana".to_string(), 50.0),
            ],
            evolution_potential: vec!["Weapon Enchantment".to_string(), "Flame Shield".to_string()],
            synergy_skills: vec!["Mana".to_string()],
            prerequisite_skills: Vec::new(),
        }
    }

    /// Create Weapon Enchantment skill (example of NPC-requested skill)
    fn create_weapon_enchantment_skill() -> Skill {
        Skill {
            name: "Weapon Enchantment".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Enchantment,
            passive_trait: PassiveTrait {
                name: "Enchanting Knowledge".to_string(),
                description: "Understand weapon properties and magical enhancement".to_string(),
                effects: vec![
                    PassiveEffect::BoostSkillExperience("Fire Magic".to_string(), 0.1),
                ],
                formula: SkillFormula::Linear(0.01),
            },
            active_trait: Some(ActiveTrait {
                name: "Enchant Weapon".to_string(),
                description: "Temporarily enchant weapon with elemental damage".to_string(),
                activation_cost: ActivationCost::Combination(vec![
                    ActivationCost::Mana(40.0),
                    ActivationCost::Energy(20.0),
                ]),
                cooldown: SkillCooldown {
                    base_duration: 60.0,
                    current_remaining: 0.0,
                    reduction_formula: SkillFormula::Linear(0.2),
                },
                effects: vec![
                    ActiveEffect::EnchantWeapon("fire".to_string(), 300.0), // 5 minutes
                ],
                can_be_hotkeyed: true,
            }),
            discovered_by: Vec::new(),
            is_innate: false,
            unlock_requirements: vec![
                SkillRequirement::SkillLevel("Fire Magic".to_string(), 25.0),
                SkillRequirement::CombinedSkills(vec![
                    ("Fire Magic".to_string(), 20.0),
                    ("Defense".to_string(), 15.0), // Understanding of weapon properties
                ]),
            ],
            evolution_potential: vec!["Permanent Enchantment".to_string(), "Multi-Element Enchanting".to_string()],
            synergy_skills: vec!["Fire Magic".to_string(), "Mana".to_string()],
            prerequisite_skills: vec!["Fire Magic".to_string()],
        }
    }

    /// Create Health vital skill
    fn create_health_skill() -> Skill {
        Skill {
            name: "Health".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Health,
            passive_trait: PassiveTrait {
                name: "Vital Constitution".to_string(),
                description: "Increases maximum health and regeneration".to_string(),
                effects: vec![
                    PassiveEffect::IncreaseMaxHealth(5.0), // 5 HP per level
                    PassiveEffect::IncreaseRegeneration("health".to_string(), 0.01), // 1% faster per level
                ],
                formula: SkillFormula::Linear(1.0),
            },
            active_trait: None, // Health is passive only
            discovered_by: Vec::new(),
            is_innate: true,
            unlock_requirements: Vec::new(),
            evolution_potential: vec!["Regeneration".to_string(), "Disease Resistance".to_string()],
            synergy_skills: vec!["Defense".to_string()],
            prerequisite_skills: Vec::new(),
        }
    }

    /// Create Wrath vital/skill (warrior-type)
    fn create_wrath_skill() -> Skill {
        Skill {
            name: "Wrath".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Wrath,
            passive_trait: PassiveTrait {
                name: "Righteous Anger".to_string(),
                description: "Build wrath in combat, increases damage when wounded".to_string(),
                effects: vec![
                    PassiveEffect::IncreaseDamage(2.0), // When health is low
                ],
                formula: SkillFormula::Linear(0.02),
            },
            active_trait: Some(ActiveTrait {
                name: "Wrathful Strike".to_string(),
                description: "Consume wrath for a devastating attack".to_string(),
                activation_cost: ActivationCost::OptionalVital("wrath".to_string(), 30.0),
                cooldown: SkillCooldown {
                    base_duration: 20.0,
                    current_remaining: 0.0,
                    reduction_formula: SkillFormula::Linear(0.1),
                },
                effects: vec![
                    ActiveEffect::DealDamage(100.0), // High damage attack
                ],
                can_be_hotkeyed: true,
            }),
            discovered_by: Vec::new(),
            is_innate: false,
            unlock_requirements: vec![
                SkillRequirement::CombinedSkills(vec![
                    ("Defense".to_string(), 20.0),
                    ("Taunt".to_string(), 15.0),
                ]),
                SkillRequirement::BeingType(BeingType::Npc), // Example: warriors unlock this
            ],
            evolution_potential: vec!["Berserker Rage".to_string()],
            synergy_skills: vec!["Defense".to_string(), "Taunt".to_string()],
            prerequisite_skills: vec!["Defense".to_string()],
        }
    }

    /// Create Fury vital/skill (berserker-type)
    fn create_fury_skill() -> Skill {
        Skill {
            name: "Fury".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Fury,
            passive_trait: PassiveTrait {
                name: "Battle Fury".to_string(),
                description: "Attack speed increases as fury builds".to_string(),
                effects: vec![
                    PassiveEffect::ModifyMovementSpeed(1.0), // Faster attacks/movement
                ],
                formula: SkillFormula::Linear(0.01),
            },
            active_trait: Some(ActiveTrait {
                name: "Berserker Fury".to_string(),
                description: "Enter a berserker state, trading defense for offense".to_string(),
                activation_cost: ActivationCost::OptionalVital("fury".to_string(), 50.0),
                cooldown: SkillCooldown {
                    base_duration: 45.0,
                    current_remaining: 0.0,
                    reduction_formula: SkillFormula::Linear(0.2),
                },
                effects: vec![
                    ActiveEffect::TemporaryBuff("damage".to_string(), 100.0, 15.0),
                    ActiveEffect::TemporaryBuff("defense".to_string(), -50.0, 15.0), // Reduced defense
                ],
                can_be_hotkeyed: true,
            }),
            discovered_by: Vec::new(),
            is_innate: false,
            unlock_requirements: vec![
                SkillRequirement::SkillLevel("Wrath".to_string(), 30.0),
            ],
            evolution_potential: vec!["Unstoppable Force".to_string()],
            synergy_skills: vec!["Wrath".to_string()],
            prerequisite_skills: vec!["Wrath".to_string()],
        }
    }

    /// Check if skill evolution opportunities exist
    fn check_skill_evolution(&mut self, skill_name: &str) {
        // Get evolution potential before borrowing mutably
        let evolution_potential = if let Some(skill) = self.skills.skills.get(skill_name) {
            skill.evolution_potential.clone()
        } else {
            return;
        };

        // Check if skill can evolve into new skills
        for potential_skill in evolution_potential {
            if !self.skills.skills.contains_key(&potential_skill) {
                // Check if requirements are met for evolution
                if self.can_unlock_skill(&potential_skill) {
                    // Automatically discover evolved skill
                    if let Some(new_skill) = self.discover_skill(&potential_skill, DiscoveryMethod::EnvironmentalInteraction) {
                        self.skills.skills.insert(potential_skill, new_skill);
                    }
                }
            }
        }
    }

    /// Check if being can unlock a specific skill
    fn can_unlock_skill(&self, skill_name: &str) -> bool {
        // This would check skill requirements (simplified for example)
        match skill_name {
            "Weapon Enchantment" => {
                self.skills.skills.get("Fire Magic").map_or(false, |s| s.level >= 25.0)
            },
            "Fury" => {
                self.skills.skills.get("Wrath").map_or(false, |s| s.level >= 30.0)
            },
            _ => true, // Default allow for basic skills
        }
    }

    /// Request evolution of a new skill through consensus system
    fn request_skill_evolution(&mut self, skill_name: String) {
        let request = EvolutionRequest {
            requested_skill: skill_name.clone(),
            requesting_being: self.id,
            justification: format!("Being {} discovered need for skill '{}' through gameplay", self.name, skill_name),
            proposed_traits: (
                PassiveTrait {
                    name: format!("{} Mastery", skill_name),
                    description: "Passive understanding of this ability".to_string(),
                    effects: vec![PassiveEffect::IncreaseDamage(1.0)], // Placeholder
                    formula: SkillFormula::Linear(0.01),
                },
                Some(ActiveTrait {
                    name: skill_name.clone(),
                    description: "Active use of this ability".to_string(),
                    activation_cost: ActivationCost::Energy(20.0),
                    cooldown: SkillCooldown {
                        base_duration: 30.0,
                        current_remaining: 0.0,
                        reduction_formula: SkillFormula::Linear(0.1),
                    },
                    effects: vec![ActiveEffect::DealDamage(25.0)],
                    can_be_hotkeyed: true,
                })
            ),
            prerequisite_skills: Vec::new(),
            supporting_evidence: vec![
                EvolutionEvidence::ObservedPhenomena(
                    format!("Being {} has observed the need for this skill in combat/exploration", self.name)
                )
            ],
            timestamp: Utc::now(),
            status: RequestStatus::Submitted,
        };

        self.skills.evolution_requests.push(request);
    }

    /// Update archetype levels based on current skills
    pub fn update_archetypes(&mut self) {
        self.archetypes.update_archetypes(&self.skills.skills);
    }

    /// Format skill level for display (2 decimal precision)
    pub fn format_skill_level(level: f64) -> String {
        format!("{:.2}", level)
    }

    /// Get formatted skill list for display
    pub fn get_formatted_skills(&self) -> Vec<(String, String, String)> {
        self.skills.skills
            .iter()
            .map(|(name, skill)| {
                (
                    name.clone(),
                    Self::format_skill_level(skill.level),
                    format!("{:.1}", skill.experience),
                )
            })
            .collect()
    }

    /// Get unlocked archetypes for display
    pub fn get_unlocked_archetypes(&self) -> Vec<(String, String)> {
        self.archetypes
            .get_unlocked_archetypes()
            .into_iter()
            .map(|archetype| {
                (
                    archetype.name.clone(),
                    super::archetypes::ArchetypeSystem::format_archetype_level(archetype.level),
                )
            })
            .collect()
    }

    /// Die and potentially reincarnate
    pub fn die(&mut self, cause: String, choose_reincarnation: bool) {
        // Mark current incarnation as ended
        self.current_incarnation.death_timestamp = Some(Utc::now());
        self.current_incarnation.cause_of_death = Some(cause);
        self.current_incarnation.chose_reincarnation = choose_reincarnation;
        
        // Record skill achievements
        self.current_incarnation.skill_achievements = self.skills.skills
            .iter()
            .filter(|(_, skill)| skill.level >= 50.0) // Consider skills above level 50 as "mastered"
            .map(|(name, _)| name.clone())
            .collect();

        // Add to incarnation history
        self.incarnation_history.push(self.current_incarnation.clone());

        if choose_reincarnation {
            // Create new incarnation
            let new_incarnation = Incarnation {
                id: Uuid::new_v4(),
                being_id: self.id,
                incarnation_number: self.current_incarnation.incarnation_number + 1,
                race: self.race.clone(), // Could change race in reincarnation
                being_type: self.being_type.clone(),
                birth_timestamp: Utc::now(),
                death_timestamp: None,
                cause_of_death: None,
                chose_reincarnation: false,
                afterlife_plane: None,
                skill_achievements: Vec::new(),
                evolution_contributions: Vec::new(),
                is_final_death: false,
            };

            self.current_incarnation = new_incarnation;
            
            // Reset vitals but KEEP ALL SKILLS (as specified - no de-leveling)
            self.vitals = VitalSystem::new();
        } else {
            // Final death - mark as such
            self.current_incarnation.is_final_death = true;
        }
    }
}

impl VitalSystem {
    pub fn new() -> Self {
        Self {
            health: Vital {
                name: "Health".to_string(),
                current: 100.0,
                maximum: 100.0,
                base_maximum: 100.0,
                regeneration_rate: 1.0, // 1 HP per second
                skill_influence: "Health".to_string(),
                is_essential: true,
                unlock_requirements: Vec::new(),
            },
            energy: Vital {
                name: "Energy".to_string(),
                current: 100.0,
                maximum: 100.0,
                base_maximum: 100.0,
                regeneration_rate: 2.0, // 2 energy per second
                skill_influence: "Energy".to_string(),
                is_essential: true,
                unlock_requirements: Vec::new(),
            },
            mana: Vital {
                name: "Mana".to_string(),
                current: 50.0,
                maximum: 50.0,
                base_maximum: 50.0,
                regeneration_rate: 1.5, // 1.5 mana per second
                skill_influence: "Mana".to_string(),
                is_essential: true,
                unlock_requirements: Vec::new(),
            },
            optional_vitals: HashMap::new(),
            active_optionals: Vec::new(),
        }
    }

    /// Add an optional vital (like Wrath, Fury, etc.)
    pub fn add_optional_vital(&mut self, vital: Vital) -> Result<(), String> {
        if self.active_optionals.len() >= 3 {
            return Err("Maximum of 3 optional vitals allowed".to_string());
        }

        self.optional_vitals.insert(vital.name.clone(), vital.clone());
        self.active_optionals.push(vital.name);
        Ok(())
    }
}

impl SkillSystem {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            skill_discovery_log: Vec::new(),
            evolution_requests: Vec::new(),
            experience_log: Vec::new(),
        }
    }
}

impl InventorySystem {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            max_weight: None, // Unlimited weight as per Arceon design
            max_slots: None,  // Unlimited slots
        }
    }
}

impl EquipmentSystem {
    pub fn new() -> Self {
        Self {
            head: None,
            shoulders: None,
            arms: None,
            chest: None,
            hands: None,
            waist: None,
            legs: None,
            feet: None,
            back: None,
            rings: Vec::new(),
            neck: None,
            main_hand: None,
            off_hand: None,
        }
    }
}
