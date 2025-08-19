use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::npc::{AiNpc, NpcAction, Goal, NpcArchetype, StructureType};
use crate::ml_text::{MlTextProcessor, GameBook, BookGenre};
use arceon_core::entities::{SkillType, Race};

/// Central AI system that manages all NPCs and their interactions
pub struct AiManager {
    pub npcs: Arc<RwLock<HashMap<Uuid, AiNpc>>>,
    pub ml_processor: Arc<RwLock<MlTextProcessor>>,
    pub books: Arc<RwLock<HashMap<Uuid, GameBook>>>,
    pub world_state: Arc<RwLock<WorldState>>,
    pub faction_system: Arc<RwLock<FactionSystem>>,
}

/// Current state of the world that NPCs can observe and modify
#[derive(Debug, Default)]
pub struct WorldState {
    pub areas: HashMap<Uuid, AreaState>,
    pub resource_nodes: HashMap<Uuid, ResourceNode>,
    pub structures: HashMap<Uuid, Structure>,
    pub active_projects: HashMap<Uuid, CollaborativeProject>,
}

#[derive(Debug, Clone)]
pub struct AreaState {
    pub id: Uuid,
    pub name: String,
    pub terrain_type: String,
    pub resource_availability: HashMap<String, u32>,
    pub population: Vec<Uuid>, // NPC IDs in this area
    pub structures: Vec<Uuid>, // Structure IDs in this area
    pub safety_level: f32, // 0.0 - 1.0
    pub development_level: f32, // 0.0 - 1.0
}

#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub id: Uuid,
    pub resource_type: String,
    pub quantity: u32,
    pub regeneration_rate: u32,
    pub location: Uuid, // Area ID
    pub quality: f32, // 0.0 - 1.0
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub id: Uuid,
    pub structure_type: StructureType,
    pub location: Uuid, // Area ID
    pub builders: Vec<Uuid>, // NPC IDs who built it
    pub completion_progress: f32, // 0.0 - 1.0
    pub functionality: HashMap<String, f32>, // Different uses and their effectiveness
}

#[derive(Debug, Clone)]
pub struct CollaborativeProject {
    pub id: Uuid,
    pub project_type: ProjectType,
    pub leader: Option<Uuid>,
    pub participants: Vec<Uuid>,
    pub required_skills: HashMap<SkillType, u32>,
    pub progress: f32,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub resources_needed: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub enum ProjectType {
    BuildTown(String), // Town name
    EstablishTradeRoute(Uuid, Uuid), // From area, to area
    ResearchTopic(String),
    CreateArtwork(String),
    TerrainModification(Uuid, String), // Area ID, modification description
    DefensePreparation(Uuid), // Area to defend
    CulturalExchange(Race, Race), // Between races
}

/// Faction system for NPC alliances and conflicts
#[derive(Debug, Default)]
pub struct FactionSystem {
    pub factions: HashMap<Uuid, Faction>,
    pub relationships: HashMap<(Uuid, Uuid), RelationshipStrength>,
}

#[derive(Debug, Clone)]
pub struct Faction {
    pub id: Uuid,
    pub name: String,
    pub leader: Option<Uuid>,
    pub members: Vec<Uuid>,
    pub faction_type: FactionType,
    pub goals: Vec<Goal>,
    pub territory: Vec<Uuid>, // Area IDs controlled
    pub resources: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub enum FactionType {
    Racial(Race),
    Professional(SkillType),
    Regional(Uuid), // Area-based
    Ideological(String), // Philosophy-based
    Economic(String), // Trade-based
}

#[derive(Debug, Clone)]
pub enum RelationshipStrength {
    Allied(f32),    // 0.0 - 1.0 strength
    Neutral,
    Rival(f32),     // 0.0 - 1.0 intensity
    Hostile(f32),   // 0.0 - 1.0 aggression
}

impl AiManager {
    /// Create a new AI manager
    pub fn new(training_data_path: String) -> Self {
        Self {
            npcs: Arc::new(RwLock::new(HashMap::new())),
            ml_processor: Arc::new(RwLock::new(MlTextProcessor::new(training_data_path))),
            books: Arc::new(RwLock::new(HashMap::new())),
            world_state: Arc::new(RwLock::new(WorldState::default())),
            faction_system: Arc::new(RwLock::new(FactionSystem::default())),
        }
    }

    /// Initialize the AI system with training data and basic NPCs
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Load ML training data
        {
            let mut processor = self.ml_processor.write().await;
            processor.load_training_data().await?;
        }

        // Create initial population of NPCs
        self.create_initial_population().await;

        // Train ML models for capable NPCs
        self.train_initial_models().await?;

        // Create initial books and knowledge
        self.create_initial_books().await;

        tracing::info!("AI system initialized successfully");
        Ok(())
    }

    /// Create the initial population of NPCs
    async fn create_initial_population(&self) {
        let mut npcs = self.npcs.write().await;
        
        // Create diverse NPCs for each race and archetype
        let races = [Race::Human, Race::Elf, Race::Dwarf, Race::Orc];
        let archetypes = [
            NpcArchetype::Farmer,
            NpcArchetype::Scholar,
            NpcArchetype::Trader,
            NpcArchetype::Artisan,
            NpcArchetype::Warrior,
            NpcArchetype::Geomancer,
            NpcArchetype::Teacher,
        ];

        for race in &races {
            for archetype in &archetypes {
                let name = format!("{:?} {:?}", race, archetype);
                let npc = AiNpc::new(name, race.clone(), archetype.clone());
                npcs.insert(npc.id, npc);
            }
        }

        // Create some mythical beings
        let treant = AiNpc::new(
            "Ancient Oakenheart".to_string(),
            Race::Elf, // Base race for mythical beings
            NpcArchetype::MythicalBeing(crate::npc::MythicalType::Treant)
        );
        npcs.insert(treant.id, treant);

        tracing::info!("Created {} initial NPCs", npcs.len());
    }

    /// Train ML models for NPCs with learning capabilities
    async fn train_initial_models(&self) -> Result<(), Box<dyn std::error::Error>> {
        let npcs = self.npcs.read().await;
        let mut processor = self.ml_processor.write().await;

        for npc in npcs.values() {
            if npc.ml_capabilities.has_text_generation || npc.ml_capabilities.has_text_comprehension {
                processor.train_model_for_npc(npc).await?;
                tracing::info!("Trained ML model for NPC: {}", npc.name);
            }
        }

        Ok(())
    }

    /// Create initial books with core knowledge
    async fn create_initial_books(&self) {
        let mut books = self.books.write().await;

        // Create fundamental knowledge books
        let basic_skills_book = GameBook::create_by_npc(
            Uuid::nil(), // System-generated
            "Fundamental Skills of Arceon".to_string(),
            "This tome covers the basic skills needed to thrive in Arceon: mining for resources, crafting useful items, growing food, and working together with others. Every citizen should master these foundations.".to_string(),
            BookGenre::Technical
        );

        let cooperation_book = GameBook::create_by_npc(
            Uuid::nil(),
            "The Art of Cooperation".to_string(),
            "When beings work together, they achieve far more than any individual could alone. This book explores how to form teams, delegate tasks, and build lasting alliances that benefit everyone involved.".to_string(),
            BookGenre::Philosophical
        );

        let common_tongue_book = GameBook::create_by_npc(
            Uuid::nil(),
            "Mastering the Common Tongue".to_string(),
            "The Common Tongue is the foundation of all communication in Arceon. This comprehensive guide teaches proper grammar, vocabulary, and the cultural nuances that make conversation effective and respectful.".to_string(),
            BookGenre::Cultural
        );

        books.insert(basic_skills_book.id, basic_skills_book);
        books.insert(cooperation_book.id, cooperation_book);
        books.insert(common_tongue_book.id, common_tongue_book);

        tracing::info!("Created {} initial books", books.len());
    }

    /// Main AI update loop - processes all NPC actions and decisions
    pub async fn update_ai_systems(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Process each NPC's decisions and actions
        let npc_actions = self.gather_npc_actions().await;
        
        // Execute actions and update world state
        for (npc_id, actions) in npc_actions {
            for action in actions {
                self.execute_npc_action(npc_id, action).await?;
            }
        }

        // Update collaborative projects
        self.update_collaborative_projects().await?;

        // Process faction relationships
        self.update_faction_relationships().await?;

        // Generate new content (books, art, etc.)
        self.generate_new_content().await?;

        Ok(())
    }

    /// Gather actions that each NPC wants to take
    async fn gather_npc_actions(&self) -> HashMap<Uuid, Vec<NpcAction>> {
        let npcs = self.npcs.read().await;
        let mut npc_actions = HashMap::new();

        for (npc_id, npc) in npcs.iter() {
            let actions = npc.decide_actions();
            npc_actions.insert(*npc_id, actions);
        }

        npc_actions
    }

    /// Execute a specific action for an NPC
    async fn execute_npc_action(&self, npc_id: Uuid, action: NpcAction) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            NpcAction::WriteBook(topic) => {
                self.npc_write_book(npc_id, topic).await?;
            }
            NpcAction::ReadBook => {
                self.npc_read_available_book(npc_id).await?;
            }
            NpcAction::BuildStructure(structure_type) => {
                self.npc_build_structure(npc_id, structure_type).await?;
            }
            NpcAction::TeachSkill(skill) => {
                self.npc_teach_skill(npc_id, skill).await?;
            }
            NpcAction::GatherResource(resource, quantity) => {
                self.npc_gather_resource(npc_id, resource, quantity).await?;
            }
            NpcAction::ModifyTerrain(modification) => {
                self.npc_modify_terrain(npc_id, modification).await?;
            }
            NpcAction::FormAlliance(target_npc) => {
                self.npc_form_alliance(npc_id, target_npc).await?;
            }
            _ => {
                // Handle other actions...
                tracing::debug!("Executed action {:?} for NPC {}", action, npc_id);
            }
        }

        Ok(())
    }

    /// Have an NPC write a book using their ML capabilities
    async fn npc_write_book(&self, npc_id: Uuid, topic: String) -> Result<(), Box<dyn std::error::Error>> {
        let npcs = self.npcs.read().await;
        let processor = self.ml_processor.read().await;
        
        if let Some(npc) = npcs.get(&npc_id) {
            if npc.ml_capabilities.has_text_generation {
                let prompt = format!("Write a comprehensive guide about {}", topic);
                let content = processor.generate_text(npc_id, &prompt, 2000)?;
                
                let title = format!("A Guide to {}", topic);
                let mut book = GameBook::create_by_npc(npc_id, title, content, BookGenre::Technical);
                book.analyze_content_for_skills();
                
                let mut books = self.books.write().await;
                books.insert(book.id, book);
                
                tracing::info!("NPC {} wrote a book about {}", npc.name, topic);
            }
        }

        Ok(())
    }

    /// Have an NPC read and learn from an available book
    async fn npc_read_available_book(&self, npc_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let mut books = self.books.write().await;
        let mut npcs = self.npcs.write().await;
        
        if let Some(npc) = npcs.get_mut(&npc_id) {
            // Find a book the NPC hasn't read yet
            for book in books.values_mut() {
                if !book.read_by.contains(&npc_id) {
                    book.mark_read_by(npc_id);
                    
                    // NPC learns from the book
                    let _concepts: Vec<String> = vec![]; // Would extract from book content
                    for skill in &book.skills_taught {
                        let current_level = npc.skills.get(skill).unwrap_or(&0);
                        npc.skills.insert(*skill, current_level + 1);
                    }
                    
                    tracing::info!("NPC {} read book: {}", npc.name, book.title);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Have an NPC build a structure
    async fn npc_build_structure(&self, npc_id: Uuid, structure_type: StructureType) -> Result<(), Box<dyn std::error::Error>> {
        let npcs = self.npcs.read().await;
        let mut world_state = self.world_state.write().await;
        
        if let Some(npc) = npcs.get(&npc_id) {
            if let Some(location) = npc.location {
                let structure = Structure {
                    id: Uuid::new_v4(),
                    structure_type: structure_type.clone(),
                    location,
                    builders: vec![npc_id],
                    completion_progress: 0.1, // Started construction
                    functionality: HashMap::new(),
                };
                
                world_state.structures.insert(structure.id, structure);
                tracing::info!("NPC {} started building {:?} at location {}", npc.name, structure_type, location);
            }
        }

        Ok(())
    }

    /// Have an NPC teach a skill to others
    async fn npc_teach_skill(&self, teacher_id: Uuid, skill: SkillType) -> Result<(), Box<dyn std::error::Error>> {
        let mut npcs = self.npcs.write().await;
        
        if let Some(teacher) = npcs.get(&teacher_id).cloned() {
            let teacher_skill_level = teacher.skills.get(&skill).unwrap_or(&0);
            
            // Find NPCs in the same location who could learn
            let students: Vec<Uuid> = npcs.iter()
                .filter(|(id, npc)| {
                    **id != teacher_id && 
                    npc.location == teacher.location &&
                    npc.skills.get(&skill).unwrap_or(&0) < teacher_skill_level
                })
                .map(|(id, _)| *id)
                .collect();

            // Teach up to 3 students
            for student_id in students.iter().take(3) {
                if let Some(student) = npcs.get_mut(student_id) {
                    let current_level = student.skills.get(&skill).unwrap_or(&0);
                    student.skills.insert(skill, current_level + 1);
                    tracing::info!("NPC {} taught {:?} to {}", teacher.name, skill, student.name);
                }
            }
        }

        Ok(())
    }

    /// Have an NPC gather resources
    async fn npc_gather_resource(&self, npc_id: Uuid, resource: String, quantity: u32) -> Result<(), Box<dyn std::error::Error>> {
        let npcs = self.npcs.read().await;
        let mut world_state = self.world_state.write().await;
        
        if let Some(npc) = npcs.get(&npc_id) {
            if let Some(location) = npc.location {
                // Find resource nodes in the area
                for node in world_state.resource_nodes.values_mut() {
                    if node.location == location && node.resource_type == resource {
                        let gathered = std::cmp::min(quantity, node.quantity);
                        node.quantity -= gathered;
                        
                        tracing::info!("NPC {} gathered {} {} from resource node", npc.name, gathered, resource);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Have a Geomancer NPC modify terrain
    async fn npc_modify_terrain(&self, npc_id: Uuid, modification: crate::npc::TerrainModification) -> Result<(), Box<dyn std::error::Error>> {
        let npcs = self.npcs.read().await;
        let mut world_state = self.world_state.write().await;
        
        if let Some(npc) = npcs.get(&npc_id) {
            if matches!(npc.archetype, NpcArchetype::Geomancer) {
                if let Some(location) = npc.location {
                    if let Some(area) = world_state.areas.get_mut(&location) {
                        // Apply terrain modification
                        match modification {
                            crate::npc::TerrainModification::PlantForest => {
                                area.terrain_type = "Forest".to_string();
                                area.resource_availability.insert("Wood".to_string(), 100);
                            }
                            crate::npc::TerrainModification::CreateLake => {
                                area.terrain_type = "Lake".to_string();
                                area.resource_availability.insert("Fish".to_string(), 50);
                            }
                            crate::npc::TerrainModification::CreateMountain => {
                                area.terrain_type = "Mountain".to_string();
                                area.resource_availability.insert("Stone".to_string(), 200);
                                area.resource_availability.insert("Ore".to_string(), 75);
                            }
                            _ => {}
                        }
                        
                        tracing::info!("Geomancer {} modified terrain at {} with {:?}", npc.name, location, modification);
                    }
                }
            }
        }

        Ok(())
    }

    /// Have an NPC form an alliance with another NPC
    async fn npc_form_alliance(&self, npc1_id: Uuid, npc2_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let mut npcs = self.npcs.write().await;
        
        // Check if both NPCs exist first
        if npcs.contains_key(&npc1_id) && npcs.contains_key(&npc2_id) {
            // Get names for logging
            let npc1_name = npcs[&npc1_id].name.clone();
            let npc2_name = npcs[&npc2_id].name.clone();
            
            // Update first NPC
            if let Some(npc1) = npcs.get_mut(&npc1_id) {
                npc1.social_bonds.insert(npc2_id, crate::npc::RelationshipType::Ally);
            }
            
            // Update second NPC
            if let Some(npc2) = npcs.get_mut(&npc2_id) {
                npc2.social_bonds.insert(npc1_id, crate::npc::RelationshipType::Ally);
            }
            
            tracing::info!("Alliance formed between {} and {}", npc1_name, npc2_name);
        }

        Ok(())
    }

    /// Update progress on collaborative projects
    async fn update_collaborative_projects(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut world_state = self.world_state.write().await;
        
        for project in world_state.active_projects.values_mut() {
            // Simulate progress based on participant skills
            project.progress += 0.01; // Small incremental progress
            
            if project.progress >= 1.0 {
                tracing::info!("Collaborative project {:?} completed!", project.project_type);
            }
        }

        Ok(())
    }

    /// Update faction relationships based on NPC interactions
    async fn update_faction_relationships(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder for faction relationship updates
        Ok(())
    }

    /// Generate new content like books and art
    async fn generate_new_content(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Occasionally have creative NPCs generate new content
        Ok(())
    }

    /// Add a new NPC to the world
    pub async fn add_npc(&self, npc: AiNpc) {
        let mut npcs = self.npcs.write().await;
        npcs.insert(npc.id, npc);
    }

    /// Get information about all NPCs
    pub async fn get_npc_summary(&self) -> HashMap<Uuid, String> {
        let npcs = self.npcs.read().await;
        npcs.iter().map(|(id, npc)| {
            (*id, format!("{} ({:?} {:?})", npc.name, npc.race, npc.archetype))
        }).collect()
    }
}
