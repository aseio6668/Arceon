use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use arceon_core::entities::{Race, SkillType};

/// Represents an AI NPC with machine learning capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiNpc {
    pub id: Uuid,
    pub name: String,
    pub race: Race,
    pub archetype: NpcArchetype,
    pub skills: HashMap<SkillType, u32>,
    pub personality: PersonalityProfile,
    pub knowledge_base: KnowledgeBase,
    pub ml_capabilities: MachineLearningCapabilities,
    pub current_goals: Vec<Goal>,
    pub location: Option<Uuid>, // Area ID
    pub inventory: Vec<Uuid>, // Item IDs
    pub social_bonds: HashMap<Uuid, RelationshipType>, // NPC ID -> Relationship
}

/// Different types of NPCs with specialized behaviors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcArchetype {
    Farmer,
    Scholar,
    Trader,
    Artisan,
    Warrior,
    Geomancer,
    Druid,
    Teacher,
    Leader,
    Crafter(CraftingSpecialty),
    MythicalBeing(MythicalType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingSpecialty {
    Blacksmith,
    Carpenter,
    Tailor,
    Alchemist,
    Enchanter,
    Cook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MythicalType {
    Treant,
    Dragon,
    Phoenix,
    ElementalSpirit,
    AncientWisdom,
}

/// Personality traits that influence NPC behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    pub curiosity: f32,      // 0.0 - 1.0, affects learning and exploration
    pub sociability: f32,    // 0.0 - 1.0, affects interaction with others
    pub creativity: f32,     // 0.0 - 1.0, affects art and innovation
    pub leadership: f32,     // 0.0 - 1.0, affects taking charge
    pub patience: f32,       // 0.0 - 1.0, affects long-term projects
    pub aggression: f32,     // 0.0 - 1.0, affects combat and conflict
    pub altruism: f32,       // 0.0 - 1.0, affects helping others
}

/// Machine learning capabilities for text processing and generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineLearningCapabilities {
    pub has_text_generation: bool,
    pub has_text_comprehension: bool,
    pub has_art_generation: bool,
    pub has_terrain_modification: bool,
    pub learning_rate: f32,
    pub training_iterations: u32,
    pub model_path: Option<PathBuf>,
}

/// Knowledge that an NPC has learned from books, conversations, and experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub books_read: Vec<BookKnowledge>,
    pub learned_skills: HashMap<SkillType, f32>, // Skill -> proficiency level
    pub cultural_knowledge: HashMap<Race, f32>, // Understanding of other races
    pub terrain_knowledge: Vec<TerrainType>,
    pub recipes_known: Vec<Recipe>,
    pub social_knowledge: HashMap<Uuid, NpcReputation>, // What they know about other NPCs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookKnowledge {
    pub book_id: Uuid,
    pub title: String,
    pub content_summary: String,
    pub learned_concepts: Vec<String>,
    pub date_read: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: String,
    pub skill_required: SkillType,
    pub difficulty: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcReputation {
    pub trust_level: f32,
    pub expertise_areas: Vec<SkillType>,
    pub personality_assessment: PersonalityProfile,
}

/// Goals that drive NPC behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub goal_type: GoalType,
    pub priority: f32,
    pub progress: f32,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub collaborators: Vec<Uuid>, // Other NPCs involved
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalType {
    LearnSkill(SkillType, u32), // Skill, target level
    CraftItem(String),      // Item name
    BuildStructure(StructureType),
    TeachOthers(SkillType),
    WriteBook(String),      // Topic
    ExploreArea(Uuid),      // Area ID
    FormAlliance(Uuid),     // NPC ID
    DefendTerritory(Uuid),  // Area ID
    GatherResources(String, u32), // Resource type, quantity
    CreateArt(ArtType),
    ModifyTerrain(TerrainModification),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureType {
    House,
    Workshop,
    Mine,
    Farm,
    Library,
    Fortress,
    Market,
    Temple,
    MagicalStructure(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtType {
    Painting,
    Sculpture,
    Music,
    Literature,
    GameTexture(String), // Texture type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainModification {
    CreateMountain,
    DigCanyon,
    PlantForest,
    CreateLake,
    BuildRoad,
    CustomLandscape(String), // Description
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Mountain,
    Forest,
    Plains,
    Desert,
    Swamp,
    Ocean,
    Underground,
    Magical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Friend,
    Ally,
    Neutral,
    Rival,
    Enemy,
    Teacher,
    Student,
    Collaborator,
    Leader,
    Follower,
}

impl AiNpc {
    /// Create a new AI NPC with specified archetype
    pub fn new(name: String, race: Race, archetype: NpcArchetype) -> Self {
        let personality = PersonalityProfile::generate_for_archetype(&archetype);
        let ml_capabilities = MachineLearningCapabilities::default_for_archetype(&archetype);
        
        Self {
            id: Uuid::new_v4(),
            name,
            race,
            archetype,
            skills: HashMap::new(),
            personality,
            knowledge_base: KnowledgeBase::new(),
            ml_capabilities,
            current_goals: Vec::new(),
            location: None,
            inventory: Vec::new(),
            social_bonds: HashMap::new(),
        }
    }

    /// Determine what actions this NPC should take based on their goals and context
    pub fn decide_actions(&self) -> Vec<NpcAction> {
        let mut actions = Vec::new();
        
        // Process current goals by priority
        let mut sorted_goals = self.current_goals.clone();
        sorted_goals.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        
        for goal in sorted_goals.iter().take(3) { // Focus on top 3 goals
            if let Some(action) = self.plan_action_for_goal(goal) {
                actions.push(action);
            }
        }
        
        // Add archetype-specific routine actions
        actions.extend(self.get_routine_actions());
        
        actions
    }

    fn plan_action_for_goal(&self, goal: &Goal) -> Option<NpcAction> {
        match &goal.goal_type {
            GoalType::LearnSkill(skill, target_level) => {
                Some(NpcAction::PracticeSkill(*skill))
            }
            GoalType::CraftItem(item_name) => {
                Some(NpcAction::CraftItem(item_name.clone()))
            }
            GoalType::BuildStructure(structure_type) => {
                Some(NpcAction::BuildStructure(structure_type.clone()))
            }
            GoalType::TeachOthers(skill) => {
                Some(NpcAction::TeachSkill(*skill))
            }
            GoalType::WriteBook(topic) => {
                Some(NpcAction::WriteBook(topic.clone()))
            }
            GoalType::GatherResources(resource, quantity) => {
                Some(NpcAction::GatherResource(resource.clone(), *quantity))
            }
            GoalType::CreateArt(art_type) => {
                Some(NpcAction::CreateArt(art_type.clone()))
            }
            GoalType::ModifyTerrain(modification) => {
                Some(NpcAction::ModifyTerrain(modification.clone()))
            }
            _ => None,
        }
    }

    fn get_routine_actions(&self) -> Vec<NpcAction> {
        match &self.archetype {
            NpcArchetype::Farmer => vec![
                NpcAction::TendCrops,
                NpcAction::FeedAnimals,
                NpcAction::CheckWeather,
            ],
            NpcArchetype::Scholar => vec![
                NpcAction::ReadBook,
                NpcAction::ResearchTopic("ancient history".to_string()),
                NpcAction::OrganizeLibrary,
            ],
            NpcArchetype::Trader => vec![
                NpcAction::CheckMarketPrices,
                NpcAction::NegotiateTrade,
                NpcAction::PlanTradeRoute,
            ],
            NpcArchetype::Geomancer => vec![
                NpcAction::SenseTerrainHealth,
                NpcAction::CommuneWithEarth,
                NpcAction::PlanLandscapeChange,
            ],
            _ => vec![NpcAction::Socialize], // Default social interaction
        }
    }
}

/// Actions that NPCs can perform in the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcAction {
    // Basic actions
    Move(Uuid),                    // Move to area
    Socialize,                     // Talk with nearby NPCs
    Rest,                          // Recover energy
    Eat,                          // Consume food
    
    // Skill-based actions
    PracticeSkill(SkillType),         // Improve a skill
    TeachSkill(SkillType),            // Teach skill to others
    
    // Crafting and building
    CraftItem(String),            // Create an item
    BuildStructure(StructureType), // Construct building
    GatherResource(String, u32),   // Collect materials
    
    // Learning and knowledge
    ReadBook,                     // Read available books
    WriteBook(String),            // Author new book
    ResearchTopic(String),        // Study specific subject
    ShareKnowledge(Uuid),         // Teach another NPC
    
    // Art and creativity
    CreateArt(ArtType),           // Generate artistic works
    GenerateTexture(String),      // Create game textures
    ComposeMusic,                 // Write songs
    
    // Environmental
    ModifyTerrain(TerrainModification), // Change landscape
    PlantCrops,                   // Agricultural work
    TendCrops,                    // Maintain farms
    FeedAnimals,                  // Care for livestock
    
    // Social and economic
    Trade(Uuid, String),          // Exchange goods with NPC
    NegotiateTrade,              // Establish trade agreements
    FormAlliance(Uuid),          // Create partnerships
    LeadGroup(Vec<Uuid>),        // Direct other NPCs
    
    // Exploration and discovery
    ExploreArea(Uuid),           // Investigate new locations
    SearchForResources,          // Find materials
    MapTerritory,                // Chart unknown areas
    
    // Specialized actions
    CommuneWithEarth,            // Geomancer earth magic
    SenseTerrainHealth,          // Check land condition
    ConsultAncientWisdom,        // Mythical being abilities
    PlanLandscapeChange,         // Design terrain modifications
    CheckMarketPrices,           // Economic analysis
    CheckWeather,                // Environmental awareness
    OrganizeLibrary,             // Maintain knowledge systems
    PlanTradeRoute,              // Logistics planning
}

impl PersonalityProfile {
    fn generate_for_archetype(archetype: &NpcArchetype) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        match archetype {
            NpcArchetype::Scholar => Self {
                curiosity: rng.gen_range(0.7..1.0),
                sociability: rng.gen_range(0.3..0.7),
                creativity: rng.gen_range(0.5..0.8),
                leadership: rng.gen_range(0.4..0.7),
                patience: rng.gen_range(0.7..1.0),
                aggression: rng.gen_range(0.1..0.3),
                altruism: rng.gen_range(0.6..0.9),
            },
            NpcArchetype::Warrior => Self {
                curiosity: rng.gen_range(0.3..0.6),
                sociability: rng.gen_range(0.4..0.7),
                creativity: rng.gen_range(0.2..0.5),
                leadership: rng.gen_range(0.6..0.9),
                patience: rng.gen_range(0.3..0.6),
                aggression: rng.gen_range(0.6..0.9),
                altruism: rng.gen_range(0.5..0.8),
            },
            NpcArchetype::Geomancer => Self {
                curiosity: rng.gen_range(0.6..0.9),
                sociability: rng.gen_range(0.2..0.6),
                creativity: rng.gen_range(0.8..1.0),
                leadership: rng.gen_range(0.3..0.6),
                patience: rng.gen_range(0.8..1.0),
                aggression: rng.gen_range(0.1..0.4),
                altruism: rng.gen_range(0.4..0.7),
            },
            _ => Self {
                curiosity: rng.gen_range(0.3..0.8),
                sociability: rng.gen_range(0.4..0.8),
                creativity: rng.gen_range(0.3..0.7),
                leadership: rng.gen_range(0.3..0.7),
                patience: rng.gen_range(0.4..0.8),
                aggression: rng.gen_range(0.2..0.6),
                altruism: rng.gen_range(0.4..0.8),
            }
        }
    }
}

impl MachineLearningCapabilities {
    fn default_for_archetype(archetype: &NpcArchetype) -> Self {
        match archetype {
            NpcArchetype::Scholar | NpcArchetype::Teacher => Self {
                has_text_generation: true,
                has_text_comprehension: true,
                has_art_generation: false,
                has_terrain_modification: false,
                learning_rate: 0.01,
                training_iterations: 1000,
                model_path: None,
            },
            NpcArchetype::Geomancer => Self {
                has_text_generation: false,
                has_text_comprehension: true,
                has_art_generation: true,
                has_terrain_modification: true,
                learning_rate: 0.005,
                training_iterations: 2000,
                model_path: None,
            },
            NpcArchetype::Artisan => Self {
                has_text_generation: false,
                has_text_comprehension: true,
                has_art_generation: true,
                has_terrain_modification: false,
                learning_rate: 0.008,
                training_iterations: 1500,
                model_path: None,
            },
            _ => Self {
                has_text_generation: false,
                has_text_comprehension: true,
                has_art_generation: false,
                has_terrain_modification: false,
                learning_rate: 0.01,
                training_iterations: 500,
                model_path: None,
            }
        }
    }
}

impl KnowledgeBase {
    fn new() -> Self {
        Self {
            books_read: Vec::new(),
            learned_skills: HashMap::new(),
            cultural_knowledge: HashMap::new(),
            terrain_knowledge: Vec::new(),
            recipes_known: Vec::new(),
            social_knowledge: HashMap::new(),
        }
    }
}
