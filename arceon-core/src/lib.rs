pub mod config;
pub mod entities;
pub mod systems;
pub mod events;
pub mod state;
pub mod error;
pub mod persistence;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use bevy_ecs::prelude::*;
use std::time::Duration;
use rand::seq::SliceRandom;

/// Types of NPCs that can be spawned in areas
#[derive(Debug, Clone)]
pub enum NpcType {
    // Urban NPCs
    Merchant,
    Guard,
    Citizen,
    Scholar,
    Artisan,
    Noble,
    
    // Wilderness NPCs
    Ranger,
    Druid,
    Hermit,
    
    // Specialized NPCs
    Miner,
    MountainGuide,
    Nomad,
    MarshFolk,
    CaveDweller,
    
    // Maritime NPCs
    Sailor,
    Dockworker,
    
    // Magical NPCs
    Mage,
}

pub use config::Config;
pub use error::ArceonError;
pub use entities::*;
pub use systems::*;

use crate::state::GameState;

/// Simple wrapper to make Arc<RwLock<GameState>> a Resource
#[derive(Resource)]
pub struct GameStateResource(pub Arc<RwLock<GameState>>);

/// Network integration trait for external networking systems
pub trait NetworkBridge: Send + Sync {
    fn broadcast_message(&mut self, message: serde_json::Value) -> Result<()>;
    fn get_connected_peers(&self) -> Vec<String>;
}

/// Placeholder for external managers until we can import them properly
#[derive(Resource)]
pub struct NetworkManager;

#[derive(Resource)]
pub struct BlockchainManager;

/// Core game engine that manages all subsystems
pub struct ArceonCore {
    world: World,
    schedule: Schedule,
    _config: Config,
    state: Arc<RwLock<GameState>>,
    network_bridge: Option<Box<dyn NetworkBridge>>,
    is_server_mode: bool,
}

impl ArceonCore {
    /// Create a new Arceon core instance
    pub async fn new(config: Config) -> Result<Self> {
        let world = World::new();
        let schedule = Schedule::default();
        
        // Initialize ECS systems (placeholder for now)
        // schedule.add_systems((
        //     systems::player_movement_system,
        //     systems::npc_ai_system,
        //     systems::world_update_system,
        //     systems::skill_progression_system,
        // ));
        
        let state = Arc::new(RwLock::new(GameState::new()));
        
        Ok(Self {
            world,
            schedule,
            _config: config,
            state,
            network_bridge: None,
            is_server_mode: false,
        })
    }
    
    /// Create a new Arceon core instance in server mode
    pub async fn new_server(config: Config) -> Result<Self> {
        let mut core = Self::new(config).await?;
        core.is_server_mode = true;
        Ok(core)
    }
    
    /// Get a cloned state reference for external access
    pub fn get_state(&self) -> Arc<RwLock<GameState>> {
        self.state.clone()
    }
    
    /// Check if running in server mode
    pub fn is_server(&self) -> bool {
        self.is_server_mode
    }
    
    /// Start the core systems with network and blockchain managers
    pub async fn start(
        &mut self,
        blockchain: BlockchainManager,
        network: NetworkManager,
    ) -> Result<()> {
        // Insert resources into ECS world
        self.world.insert_resource(blockchain);
        self.world.insert_resource(network);
        self.world.insert_resource(GameStateResource(self.state.clone()));
        
        // Initialize world entities
        self.initialize_world().await?;
        
        Ok(())
    }
    
    /// Initialize the game world with default entities
    async fn initialize_world(&mut self) -> Result<()> {
        // World generation will be handled externally to avoid circular dependencies
        // For now, just initialize an empty world state
        println!("🌍 World system initialized (areas will be loaded externally)");
        Ok(())
    }
    
    /// Add areas to the game state (called externally)
    pub async fn add_areas(&mut self, areas: Vec<world::Area>) -> Result<()> {
        let mut state = self.state.write().await;
        for area in areas {
            state.add_area(area);
        }
        println!("🌍 Added {} areas to the world", state.areas.len());
        Ok(())
    }
    
    /// Run one update cycle
    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }
    
    /// Run the main game loop
    pub async fn run_game_loop(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_millis(100)); // 10 TPS
        
        loop {
            interval.tick().await;
            
            // Update world time
            {
                let mut state = self.state.write().await;
                state.world_time += 100; // Increment by 100ms
            }
            
            // Run ECS systems update
            self.update();
            
            // Update world state (placeholder for more complex logic)
            self.update_world_state().await?;
        }
    }
    
    /// Update world state each game tick
    async fn update_world_state(&mut self) -> Result<()> {
        // Update NPC AI behaviors
        self.update_npc_behaviors().await?;
        
        // Spawn NPCs in areas that need them
        self.spawn_missing_npcs().await?;
        
        // Process NPC interactions and actions
        self.process_npc_actions().await?;
        
        // Other world updates
        // - Resource respawning
        // - Weather changes  
        // - Dynamic events
        
        Ok(())
    }
    
    /// Update NPC AI behaviors and decision making
    async fn update_npc_behaviors(&mut self) -> Result<()> {
        let area_data = {
            let state = self.state.read().await;
            state.areas.iter().map(|(id, area)| (id.clone(), area.clone())).collect::<Vec<_>>()
        };
        
        // For each area, update NPCs present
        for (area_id, area) in area_data {
            // Simulate NPC behaviors based on area type and time
            self.simulate_area_npcs(&area_id, &area).await?;
        }
        
        Ok(())
    }
    
    /// Spawn NPCs in areas that need population
    async fn spawn_missing_npcs(&mut self) -> Result<()> {
        let spawn_events = {
            let mut state = self.state.write().await;
            let mut events = Vec::new();
            let area_ids: Vec<String> = state.areas.keys().cloned().collect();
            
            for area_id in area_ids {
                if let Some(area) = state.areas.get(&area_id) {
                    let desired_population = self.calculate_desired_npc_population(area);
                    let current_population = area.locations.values()
                        .map(|loc| loc.npcs_present.len())
                        .sum::<usize>();
                    
                    if current_population < desired_population {
                        let npcs_to_spawn = desired_population - current_population;
                        
                        // Get the area again mutably
                        if let Some(area_mut) = state.areas.get_mut(&area_id) {
                            Self::spawn_npcs_in_area_static(area_mut, npcs_to_spawn)?;
                            events.push((area_id, npcs_to_spawn, desired_population));
                        }
                    }
                }
            }
            events
        };
        
        // Broadcast events after releasing lock
        for (area_id, npcs_spawned, total_population) in spawn_events {
            let spawn_data = serde_json::json!({
                "area_id": area_id,
                "npcs_spawned": npcs_spawned,
                "total_population": total_population
            });
            self.broadcast_network_event("NPCSpawn", spawn_data).await?;
        }
        
        Ok(())
    }
    
    /// Calculate how many NPCs should be in an area
    fn calculate_desired_npc_population(&self, area: &world::Area) -> usize {
        use world::AreaType;
        
        let base_population = match area.area_type {
            AreaType::Capital => 15,      // Major cities have lots of NPCs
            AreaType::City => 8,          // Smaller cities
            AreaType::Village => 5,       // Small settlements
            AreaType::Port => 6,          // Trading ports
            AreaType::Forest => 2,        // Rangers, druids
            AreaType::Mountains => 1,     // Hermits, miners
            AreaType::Desert => 1,        // Nomads, oasis keepers
            AreaType::Swamp => 1,         // Marsh folk
            AreaType::Underground => 3,   // Cave dwellers
            AreaType::Magical => 4,       // Magical beings
            _ => 3,                       // Default
        };
        
        // Adjust based on area size
        let size_multiplier = match area.size {
            world::AreaSize::Tiny => 0.5,
            world::AreaSize::Small => 0.7,
            world::AreaSize::Medium => 1.0,
            world::AreaSize::Large => 1.5,
            world::AreaSize::Massive => 2.0,
        };
        
        (base_population as f32 * size_multiplier).round() as usize
    }
    
    /// Spawn specific NPCs in an area (static version)
    fn spawn_npcs_in_area_static(area: &mut world::Area, count: usize) -> Result<()> {
        use uuid::Uuid;
        
        for _ in 0..count {
            // Choose appropriate NPC type based on area and locations
            let npc_type = Self::choose_npc_type_for_area_static(area);
            let npc_id = Uuid::new_v4();
            
            // Find appropriate location for this NPC
            let location_name = Self::choose_npc_location_static(area, &npc_type);
            
            if let Some(location) = area.locations.get_mut(&location_name) {
                location.npcs_present.push(npc_id);
                
                // TODO: Create actual NPC object in AI system
                println!("📍 Spawned {:?} NPC in {} at {}", npc_type, area.name, location_name);
            }
        }
        
        Ok(())
    }
    
    /// Choose appropriate NPC type for an area (static version)
    fn choose_npc_type_for_area_static(area: &world::Area) -> NpcType {
        use world::AreaType;
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        
        match area.area_type {
            AreaType::Capital | AreaType::City => {
                match rng.gen_range(0..10) {
                    0..=2 => NpcType::Merchant,
                    3..=4 => NpcType::Guard,
                    5..=6 => NpcType::Citizen,
                    7 => NpcType::Scholar,
                    8 => NpcType::Artisan,
                    _ => NpcType::Noble,
                }
            },
            AreaType::Forest => {
                match rng.gen_range(0..4) {
                    0..=1 => NpcType::Ranger,
                    2 => NpcType::Druid,
                    _ => NpcType::Hermit,
                }
            },
            AreaType::Mountains => {
                match rng.gen_range(0..3) {
                    0 => NpcType::Miner,
                    1 => NpcType::Hermit,
                    _ => NpcType::MountainGuide,
                }
            },
            AreaType::Desert => NpcType::Nomad,
            AreaType::Swamp => NpcType::MarshFolk,
            AreaType::Underground => NpcType::CaveDweller,
            AreaType::Magical => NpcType::Mage,
            AreaType::Port => {
                match rng.gen_range(0..4) {
                    0..=1 => NpcType::Sailor,
                    2 => NpcType::Merchant,
                    _ => NpcType::Dockworker,
                }
            },
            _ => NpcType::Citizen,
        }
    }
    
    /// Choose appropriate location for NPC within area (static version)
    fn choose_npc_location_static(area: &world::Area, npc_type: &NpcType) -> String {
        use world::LocationType;
        
        let preferred_locations = match npc_type {
            NpcType::Merchant => vec![LocationType::Market, LocationType::Shop, LocationType::Tavern],
            NpcType::Guard => vec![LocationType::GuardPost, LocationType::Gate, LocationType::Market],
            NpcType::Scholar => vec![LocationType::Library, LocationType::Temple],
            NpcType::Artisan => vec![LocationType::CraftingHall, LocationType::Market],
            NpcType::Ranger => vec![LocationType::Clearing, LocationType::Camp],
            NpcType::Druid => vec![LocationType::Natural, LocationType::Clearing],
            NpcType::Miner => vec![LocationType::Cave, LocationType::Camp],
            NpcType::Sailor => vec![LocationType::Harbor, LocationType::Tavern],
            NpcType::Mage => vec![LocationType::Temple, LocationType::Library],
            _ => vec![LocationType::Residential, LocationType::Market, LocationType::Tavern],
        };
        
        // Find matching locations in the area
        let available_locations: Vec<String> = area.locations.iter()
            .filter(|(_, loc)| preferred_locations.contains(&loc.location_type))
            .map(|(name, _)| name.clone())
            .collect();
        
        if !available_locations.is_empty() {
            available_locations.into_iter().next().unwrap_or_else(|| {
                area.locations.keys().next().unwrap().clone()
            })
        } else {
            // Fallback to first available location
            area.locations.keys().next().unwrap().clone()
        }
    }
    
    /// Simulate NPC behaviors in an area
    async fn simulate_area_npcs(&mut self, area_id: &str, area: &world::Area) -> Result<()> {
        // Simulate NPC activities based on time of day, area type, etc.
        
        // Example behaviors:
        // - Merchants opening shops in the morning
        // - Guards patrolling at regular intervals  
        // - Scholars studying in libraries
        // - Farmers working in fields
        
        // For now, just log some activity
        if !area.locations.is_empty() {
            let total_npcs: usize = area.locations.values()
                .map(|loc| loc.npcs_present.len())
                .sum();
            
            if total_npcs > 0 {
                // Simulate some random NPC actions
                self.generate_npc_actions(area_id, area, total_npcs).await?;
            }
        }
        
        Ok(())
    }
    
    /// Generate random NPC actions for atmosphere
    async fn generate_npc_actions(&mut self, area_id: &str, area: &world::Area, npc_count: usize) -> Result<()> {
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        
        // Occasionally generate NPC actions for atmosphere
        if rng.gen_bool(0.1) { // 10% chance per update
            let actions = vec![
                "is tending to their daily tasks",
                "can be seen going about their business", 
                "is having a conversation with another resident",
                "is working diligently at their trade",
                "pauses to observe the surroundings",
                "is organizing their wares",
                "is practicing their skills",
                "is sharing news with fellow residents",
            ];
            
            let action = actions.choose(&mut rng).unwrap();
            let event_data = serde_json::json!({
                "area_id": area_id,
                "area_name": area.name,
                "action": action,
                "npc_count": npc_count
            });
            
            self.broadcast_network_event("NPCAction", event_data).await?;
        }
        
        Ok(())
    }
    
    /// Process NPC actions and interactions
    async fn process_npc_actions(&mut self) -> Result<()> {
        // This would integrate with the AI system to process:
        // - NPC decision making
        // - Skill development
        // - Social interactions
        // - Economic activities
        // - Knowledge sharing
        
        // For now, this is a placeholder
        Ok(())
    }
    
    /// Handle player interaction with NPCs
    async fn interact_with_npc(&mut self, player_id: &str, npc_name: &str) -> Result<String> {
        let (response, interaction_data) = {
            let state = self.state.read().await;
            
            if let Some(player_data) = state.online_players.get(player_id) {
                if let Some(area) = state.areas.get(&player_data.current_area_id) {
                    // Check if there are NPCs in the current area
                    let total_npcs: usize = area.locations.values()
                        .map(|loc| loc.npcs_present.len())
                        .sum();
                    
                    if total_npcs == 0 {
                        return Ok("There are no NPCs to talk to in this area.".to_string());
                    }
                    
                    // Generate contextual NPC response based on area and NPC type
                    let response = self.generate_npc_interaction(area, npc_name, player_id).await?;
                    
                    let interaction_data = serde_json::json!({
                        "player_id": player_id,
                        "area_id": player_data.current_area_id,
                        "npc_name": npc_name,
                        "interaction_type": "talk"
                    });
                    
                    (response, Some(interaction_data))
                } else {
                    ("You are in an unknown location.".to_string(), None)
                }
            } else {
                ("Player not found.".to_string(), None)
            }
        };
        
        // Broadcast the interaction after releasing state lock
        if let Some(data) = interaction_data {
            self.broadcast_network_event("NPCInteraction", data).await?;
        }
        
        Ok(response)
    }
    
    /// Generate NPC interaction responses
    async fn generate_npc_interaction(&self, area: &world::Area, _npc_name: &str, _player_id: &str) -> Result<String> {
        use rand::seq::SliceRandom;
        use world::AreaType;
        
        // Generate contextual responses based on area type
        let responses = match area.area_type {
            AreaType::Capital | AreaType::City => {
                vec![
                    format!("\"Welcome to {}, traveler! Have you come for trade or seeking knowledge?\"", area.name),
                    format!("\"Ah, a new face in {}! The markets are bustling today, you should take a look.\"", area.name),
                    "\"These are prosperous times, though travelers bring word of strange happenings in distant lands.\"".to_string(),
                    "\"The city guards have been on edge lately. Something about unusual magical disturbances.\"".to_string(),
                    "\"Business has been good, but the roads have become more dangerous. Travel carefully.\"".to_string(),
                ]
            },
            AreaType::Forest => {
                vec![
                    "\"The forest has been restless lately. The old paths whisper of ancient magic stirring.\"".to_string(),
                    "\"Few travelers pass this way. You must have important business to venture so deep into the woods.\"".to_string(),
                    "\"Listen... do you hear that? The trees are trying to tell us something.\"".to_string(),
                    "\"I've been tracking strange creatures through these parts. Stay alert, friend.\"".to_string(),
                    "\"The druids speak of a great awakening. The very earth seems to pulse with life.\"".to_string(),
                ]
            },
            AreaType::Mountains => {
                vec![
                    "\"The mountains have been singing strange songs at night. Even the stone tells tales.\"".to_string(),
                    "\"You've got the look of a climber about you. Mind the loose rocks on the eastern slopes.\"".to_string(),
                    "\"The mines have been yielding unusual crystals lately. Some say they're magical.\"".to_string(),
                    "\"Dragons haven't been seen in these peaks for generations, but lately... well, best be cautious.\"".to_string(),
                ]
            },
            AreaType::Desert => {
                vec![
                    "\"The sands shift more than usual. Ancient ruins appear and disappear like mirages.\"".to_string(),
                    "\"Water is life in the desert, traveler. The oasis ahead is blessed by old magic.\"".to_string(),
                    "\"The crystal formations have been glowing brighter each night. A sign of power awakening.\"".to_string(),
                ]
            },
            AreaType::Magical => {
                vec![
                    "\"The magical energies here are intense today. Can you feel the arcane currents shifting?\"".to_string(),
                    "\"Knowledge seeks those who are ready to receive it. What brings you to this place of learning?\"".to_string(),
                    "\"The ancient texts speak of a convergence. Perhaps you are part of the prophesied events.\"".to_string(),
                    "\"Magic flows differently here than in the mundane world. Be respectful of the forces at work.\"".to_string(),
                ]
            },
            AreaType::Underground => {
                vec![
                    "\"The deep places remember things the surface world has forgotten. Tread carefully.\"".to_string(),
                    "\"These tunnels connect to chambers older than any recorded history.\"".to_string(),
                    "\"The crystals down here pulse with their own inner light. Some say they're alive.\"".to_string(),
                ]
            },
            AreaType::Port => {
                vec![
                    "\"Ships from distant lands bring tales of wonder and danger. The seas are changing, friend.\"".to_string(),
                    "\"The tide has been strange lately. Old salts say it means big changes coming.\"".to_string(),
                    "\"Just back from a voyage to the eastern isles. Saw things out there that shouldn't exist.\"".to_string(),
                ]
            },
            _ => {
                vec![
                    "\"Greetings, traveler. What brings you to our humble settlement?\"".to_string(),
                    "\"Not many visitors come this way. You must have an interesting tale to tell.\"".to_string(),
                    "\"Times are changing, and not all for the better. But we endure, as we always have.\"".to_string(),
                ]
            }
        };
        
        // Add some personality-based variations
        let personality_additions = vec![
            " *nods thoughtfully*",
            " *glances around cautiously*", 
            " *smiles warmly*",
            " *strokes beard contemplatively*",
            " *adjusts their equipment*",
            " *looks you up and down appraisingly*",
        ];
        
        let mut rng = rand::thread_rng();
        let base_response = responses.choose(&mut rng).unwrap();
        let personality = personality_additions.choose(&mut rng).unwrap();
        
        Ok(format!("{}{}", base_response, personality))
    }
    
    /// Create a new player character
    pub async fn create_player(&mut self, player_id: String, player_name: String, race: being::Race) -> Result<String> {
        let mut state = self.state.write().await;
        
        // Find starting area for the race
        let starting_area_id = state.areas
            .values()
            .find(|area| {
                if let Some(affinity) = &area.race_affinity {
                    affinity.primary_race == race
                } else {
                    false
                }
            })
            .map(|area| area.id.to_string())
            .unwrap_or_else(|| {
                // Default to first area if no race-specific area found
                state.areas.keys().next().unwrap().clone()
            });
        
        // Create a new being for the player
        let race_string = format!("{:?}", race);
        let being = being::Being::new_player(player_name.clone(), race);
        let being_id = being.id.to_string();
        let world_time = state.world_time;
        
        // Add being to state
        state.add_being(being);
        
        // Add player data
        state.online_players.insert(player_id.clone(), crate::state::PlayerData {
            being_id: being_id.clone(),
            current_area_id: starting_area_id.clone(),
            last_activity: world_time,
        });
        
        drop(state); // Release the lock before broadcasting
        
        // Broadcast player join event
        let join_data = serde_json::json!({
            "player_id": player_id,
            "player_name": player_name,
            "race": race_string,
            "area_id": starting_area_id
        });
        self.broadcast_network_event("PlayerJoin", join_data).await?;
        
        Ok(being_id)
    }
    
    /// Process a player command
    pub async fn process_command(&mut self, player_id: &str, command: &str) -> Result<String> {
        let mut state = self.state.write().await;
        
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok("Please enter a command.".to_string());
        }
        
        match parts[0].to_lowercase().as_str() {
            "look" | "l" => {
                if let Some(player_data) = state.online_players.get(player_id) {
                    if let Some(area) = state.areas.get(&player_data.current_area_id) {
                        let players_here = state.get_players_in_area(&player_data.current_area_id);
                        let other_players: Vec<&str> = players_here.into_iter()
                            .filter(|&p| p != player_id)
                            .collect();
                        
                        let mut response = format!("You are in {}.\n{}\n", area.name, area.description);
                        
                        if !other_players.is_empty() {
                            response.push_str(&format!("Other players here: {}\n", other_players.join(", ")));
                        }
                        
                        if !area.connected_areas.is_empty() {
                            let exit_names: Vec<String> = area.connected_areas.iter()
                                .enumerate()
                                .map(|(i, conn)| {
                                    if let Some(target_area) = state.areas.values().find(|a| a.id == conn.target_area_id) {
                                        format!("{} - {} ({:?})", i + 1, target_area.name, conn.connection_type)
                                    } else {
                                        format!("{} - Unknown destination", i + 1)
                                    }
                                })
                                .collect();
                            response.push_str(&format!("Exits: {}", exit_names.join(", ")));
                        }
                        
                        Ok(response)
                    } else {
                        Ok("You are in an unknown location.".to_string())
                    }
                } else {
                    Ok("Player not found.".to_string())
                }
            },
            "move" | "go" => {
                if parts.len() < 2 {
                    return Ok("Where do you want to go? Use: move <number>".to_string());
                }
                
                let exit_number = parts[1];
                Self::move_player_static(player_id, exit_number, &mut state).await
            },
            "who" => {
                let player_count = state.online_players.len();
                let players: Vec<String> = state.online_players.keys().cloned().collect();
                Ok(format!("Players online ({}): {}", player_count, players.join(", ")))
            },
            "stats" => {
                if let Some(player_data) = state.online_players.get(player_id) {
                    if let Some(being) = state.beings.get(&player_data.being_id) {
                        let health = &being.vitals.health;
                        let energy = &being.vitals.energy;
                        let mana = &being.vitals.mana;
                        
                        let mut response = format!(
                            "=== {} ===\nRace: {:?}\nHealth: {:.1}/{:.1}\nEnergy: {:.1}/{:.1}\nMana: {:.1}/{:.1}\n\nSkills:\n",
                            being.name, being.race, health.current, health.maximum,
                            energy.current, energy.maximum, mana.current, mana.maximum
                        );
                        
                        // Show skills with 2-decimal precision
                        let formatted_skills = being.get_formatted_skills();
                        for (skill_name, level, experience) in formatted_skills {
                            response.push_str(&format!("  {}: {} (XP: {})\n", skill_name, level, experience));
                        }
                        
                        // Show unlocked archetypes
                        let unlocked_archetypes = being.get_unlocked_archetypes();
                        if !unlocked_archetypes.is_empty() {
                            response.push_str("\nArchetypes:\n");
                            for (archetype_name, level) in unlocked_archetypes {
                                response.push_str(&format!("  {}: {}\n", archetype_name, level));
                            }
                        }
                        
                        Ok(response)
                    } else {
                        Ok("Character not found.".to_string())
                    }
                } else {
                    Ok("Player not found.".to_string())
                }
            },
            "say" => {
                if parts.len() < 2 {
                    return Ok("What do you want to say? Use: say <message>".to_string());
                }
                
                let message = parts[1..].join(" ");
                if let Some(player_data) = state.online_players.get(player_id) {
                    let area_id = player_data.current_area_id.clone();
                    drop(state); // Release lock before broadcasting
                    
                    // Broadcast say message
                    let say_data = serde_json::json!({
                        "player_id": player_id,
                        "area_id": area_id,
                        "message": message
                    });
                    self.broadcast_network_event("Say", say_data).await?;
                    
                    Ok(format!("You say: '{}'", message))
                } else {
                    Ok("Player not found.".to_string())
                }
            },
            "talk" | "speak" => {
                if parts.len() < 2 {
                    return Ok("Who do you want to talk to? Use: talk <npc_name>".to_string());
                }
                
                let npc_name = parts[1..].join(" ");
                drop(state); // Release lock before calling interact_with_npc
                self.interact_with_npc(player_id, &npc_name).await
            },
            "npcs" => {
                if let Some(player_data) = state.online_players.get(player_id) {
                    if let Some(area) = state.areas.get(&player_data.current_area_id) {
                        let mut npc_info = Vec::new();
                        
                        for (location_name, location) in &area.locations {
                            if !location.npcs_present.is_empty() {
                                npc_info.push(format!("  {} - {} NPCs present", location_name, location.npcs_present.len()));
                            }
                        }
                        
                        if npc_info.is_empty() {
                            Ok("No NPCs visible in this area.".to_string())
                        } else {
                            Ok(format!("NPCs in {}:\n{}", area.name, npc_info.join("\n")))
                        }
                    } else {
                        Ok("You are in an unknown location.".to_string())
                    }
                } else {
                    Ok("Player not found.".to_string())
                }
            },
            "quests" => {
                if let Some(player_data) = state.online_players.get(player_id) {
                    if let Some(being) = state.beings.get(&player_data.being_id) {
                        let available_quests = state.quest_system.get_available_quests_for_player(&being.skills.skills);
                        
                        if available_quests.is_empty() {
                            Ok("No quests available at this time.".to_string())
                        } else {
                            let mut response = "Available Quests:\n".to_string();
                            for (i, quest) in available_quests.iter().enumerate() {
                                response.push_str(&format!("{}. {} - {}\n", i + 1, quest.title, quest.description));
                                response.push_str(&format!("   Priority: {:?}\n", quest.priority));
                                if let Some(time_limit) = quest.time_limit {
                                    response.push_str(&format!("   Time Limit: {}\n", time_limit.format("%Y-%m-%d %H:%M")));
                                }
                            }
                            Ok(response)
                        }
                    } else {
                        Ok("Character not found.".to_string())
                    }
                } else {
                    Ok("Player not found.".to_string())
                }
            },
            "reputation" | "rep" => {
                let mut response = "Faction Reputation:\n".to_string();
                for (faction_name, reputation) in &state.quest_system.faction_reputations {
                    response.push_str(&format!("  {}: {} ({:?})\n", 
                        faction_name, 
                        reputation.reputation_value, 
                        reputation.reputation_level
                    ));
                }
                if state.quest_system.faction_reputations.is_empty() {
                    response.push_str("  No faction reputation yet.\n");
                }
                Ok(response)
            },
            "help" => {
                Ok("Available commands:\n  look/l - Look around\n  move/go <number> - Move to exit number\n  say <message> - Say something to nearby players\n  talk <npc> - Talk to an NPC\n  npcs - List NPCs in current area\n  quests - Show available quests\n  reputation/rep - Show faction reputation\n  who - List online players\n  stats - Show your character stats\n  help - Show this help".to_string())
            },
            _ => {
                Ok(format!("Unknown command: {}. Type 'help' for available commands.", parts[0]))
            }
        }
    }
    
    /// Move a player to a connected area
    async fn move_player_static(player_id: &str, exit_number: &str, state: &mut crate::state::GameState) -> Result<String> {
        // Get player data first
        let (current_area_id, player_found) = if let Some(player_data) = state.online_players.get(player_id) {
            (player_data.current_area_id.clone(), true)
        } else {
            return Ok("Player not found.".to_string());
        };

        if !player_found {
            return Ok("Player not found.".to_string());
        }

        // Get current area
        let connection_target = if let Some(current_area) = state.areas.get(&current_area_id) {
            // Parse the exit number
            let exit_index = match exit_number.parse::<usize>() {
                Ok(n) if n > 0 => n - 1, // Convert to 0-based index
                _ => return Ok("Please enter a valid exit number.".to_string()),
            };
            
            // Check if the exit exists
            if let Some(connection) = current_area.connected_areas.get(exit_index) {
                Some(connection.target_area_id)
            } else {
                return Ok(format!("There is no exit {}. Use 'look' to see available exits.", exit_number));
            }
        } else {
            return Ok("You are in an unknown location.".to_string());
        };

        // Get target area info
        let (target_area_id, target_area_name) = if let Some(target_id) = connection_target {
            if let Some(target_area) = state.areas.values().find(|area| area.id == target_id) {
                (target_area.id.to_string(), target_area.name.clone())
            } else {
                return Ok("That path leads to an unknown destination.".to_string());
            }
        } else {
            return Ok("No valid exit found.".to_string());
        };

        // Now move the player
        match state.move_player_to_area(player_id, &target_area_id) {
            Ok(_) => Ok(format!("You travel to {}.", target_area_name)),
            Err(e) => Ok(format!("Failed to move: {}", e)),
        }
    }
    
    /// Set the network bridge for P2P communication
    pub fn set_network_bridge(&mut self, bridge: Box<dyn NetworkBridge>) {
        self.network_bridge = Some(bridge);
    }
    
    /// Broadcast a network event to connected peers
    async fn broadcast_network_event(&mut self, event_type: &str, data: serde_json::Value) -> Result<()> {
        if let Some(bridge) = &mut self.network_bridge {
            let message = serde_json::json!({
                "type": event_type,
                "timestamp": chrono::Utc::now().timestamp(),
                "data": data
            });
            bridge.broadcast_message(message)?;
        }
        Ok(())
    }
    
    /// Get access to the game state
    pub fn state(&self) -> Arc<RwLock<GameState>> {
        self.state.clone()
    }
}
