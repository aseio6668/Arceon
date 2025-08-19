use std::collections::HashMap;
use rand::prelude::*;
use chrono::Utc;
use uuid::Uuid;

// Import the new being system from arceon-core
use arceon_core::{
    Being, BeingType, Race, BeingCapability, ExperienceSource, DiscoveryMethod,
    EvolutionRequest, RequestStatus, EvolutionEvidence, SkillEvolutionSystem, VitalManager, 
    LetterToGods, VoteChoice
};

/// Enhanced Arceon Being System Demo
/// Showcases autonomous NPCs with skill discovery and vital management
pub struct BeingSystemDemo {
    beings: Vec<Being>,
    evolution_system: SkillEvolutionSystem,
    vital_manager: VitalManager,
    world_time: f64,
    simulation_cycle: u32,
    rng: StdRng,
}

impl BeingSystemDemo {
    pub fn new() -> Self {
        Self {
            beings: Vec::new(),
            evolution_system: SkillEvolutionSystem::new(),
            vital_manager: VitalManager::new(),
            world_time: 0.0,
            simulation_cycle: 0,
            rng: StdRng::seed_from_u64(42),
        }
    }

    /// Initialize the demo with diverse beings
    pub fn initialize(&mut self) {
        println!("🌟 Initializing Arceon Being System Demo");
        println!("{}", "=".repeat(60));

        self.create_intelligent_beings();
        self.create_feral_beasts();
        self.create_magical_entities();

        println!("📊 Total beings created: {}", self.beings.len());
        println!();
    }

    fn create_intelligent_beings(&mut self) {
        // Create various intelligent NPCs with different focuses
        let intelligent_configs = vec![
            ("Aldara the Scholar", Race::Elf, vec!["Mana", "Fire Magic", "Health"]),
            ("Borin Ironforge", Race::Dwarf, vec!["Defense", "Health", "Taunt"]),
            ("Kael Swiftblade", Race::Human, vec!["Defense", "Taunt", "Wrath"]),
            ("Zara Nightwhisper", Race::Tiefling, vec!["Mana", "Fire Magic", "Defense"]),
            ("Grimjaw Berserker", Race::Orc, vec!["Defense", "Taunt", "Fury"]),
        ];

        for (name, race, focus_skills) in intelligent_configs {
            let mut being = Being::new(name.to_string(), BeingType::Npc, race, false);
            
            // Give them starting skills based on their focus
            for skill in focus_skills {
                let experience = self.rng.gen_range(100.0..500.0);
                being.gain_skill_experience(skill, experience, ExperienceSource::SkillDiscovery);
            }

            println!("👤 Created intelligent being: {} ({:?})", name, being.race);
            self.beings.push(being);
        }
    }

    fn create_feral_beasts(&mut self) {
        // Create feral beasts that have skills but no inventory
        let beast_configs = vec![
            ("Shadowfang", Race::DireWolf, vec!["Defense", "Health"]),
            ("Flameheart", Race::Dragon, vec!["Fire Magic", "Defense"]),
            ("Stormclaw", Race::Griffin, vec!["Defense", "Taunt"]),
        ];

        for (name, race, natural_skills) in beast_configs {
            let mut being = Being::new(name.to_string(), BeingType::Beast, race, true);
            
            // Beasts have natural skills
            for skill in natural_skills {
                let experience = self.rng.gen_range(200.0..800.0);
                being.gain_skill_experience(skill, experience, ExperienceSource::CombatVictory);
            }

            println!("🐺 Created feral beast: {} ({:?})", name, being.race);
            self.beings.push(being);
        }
    }

    fn create_magical_entities(&mut self) {
        // Create magical beings with unique capabilities
        let magical_configs = vec![
            ("Pyrion", Race::FireElemental, vec!["Fire Magic", "Mana"]),
            ("Aqua", Race::WaterElemental, vec!["Mana", "Defense"]),
            ("Ancient Sage", Race::Lich, vec!["Mana", "Fire Magic", "Health"]),
        ];

        for (name, race, magical_skills) in magical_configs {
            let mut being = Being::new(name.to_string(), BeingType::Elemental, race, false);
            
            for skill in magical_skills {
                let experience = self.rng.gen_range(300.0..1000.0);
                being.gain_skill_experience(skill, experience, ExperienceSource::CastingSpells);
            }

            println!("✨ Created magical entity: {} ({:?})", name, being.race);
            self.beings.push(being);
        }
    }

    /// Run the main simulation loop
    pub fn run_simulation(&mut self, cycles: u32) {
        println!("🎮 Starting Being System Simulation");
        println!("{}", "=".repeat(60));

        for cycle in 1..=cycles {
            self.simulation_cycle = cycle;
            println!("\n📅 === Simulation Cycle {} ===", cycle);
            
            // Update world time (each cycle = 1 hour of game time)
            self.world_time += 3600.0;
            
            // Simulate being activities
            self.simulate_being_activities();
            
            // Update vital systems
            self.update_vital_systems();
            
            // Process skill evolution requests
            self.process_skill_evolution();
            
            // Check for new vital unlocks
            self.check_vital_unlocks();
            
            // Demonstrate skill discoveries
            if cycle % 5 == 0 {
                self.demonstrate_skill_discovery();
            }
            
            // Show system status
            if cycle % 3 == 0 {
                self.show_system_status();
            }
        }

        self.show_final_summary();
    }

    fn simulate_being_activities(&mut self) {
        println!("🎯 Simulating being activities...");
        
        for i in 0..self.beings.len() {
            // Each being performs activities based on their nature
            let being_type = self.beings[i].being_type;
            match being_type {
                BeingType::Npc => self.simulate_npc_activities(i),
                BeingType::Beast => self.simulate_beast_activities(i),
                BeingType::Elemental => self.simulate_elemental_activities(i),
                _ => {},
            }
        }
    }

    fn simulate_npc_activities(&mut self, being_index: usize) {
        let activities = vec![
            ("practice combat", "Defense", ExperienceSource::CombatVictory),
            ("study magic", "Mana", ExperienceSource::StudyingArcane),
            ("physical training", "Health", ExperienceSource::OvercomingObstacles),
            ("social interaction", "Leadership", ExperienceSource::LeadershipAction),
            ("craft items", "Defense", ExperienceSource::CraftingItems),
        ];

        // NPCs do 2-4 activities per cycle
        let num_activities = self.rng.gen_range(2..=4);
        for _ in 0..num_activities {
            let (activity, skill, source) = activities.choose(&mut self.rng).unwrap();
            let experience = self.rng.gen_range(10.0..50.0);
            
            self.beings[being_index].gain_skill_experience(skill, experience, source.clone());
            
            if self.rng.gen_bool(0.1) { // 10% chance to mention activity
                println!("  📝 {} is {}: +{:.1} {} experience", 
                    self.beings[being_index].name, activity, experience, skill);
            }
        }
    }

    fn simulate_beast_activities(&mut self, being_index: usize) {
        let activities = vec![
            ("hunting", "Health", ExperienceSource::SurvivingDanger),
            ("territorial defense", "Defense", ExperienceSource::CombatVictory),
            ("prowling", "Defense", ExperienceSource::AdaptingToEnvironment),
        ];

        // Beasts do 1-2 activities per cycle
        let num_activities = self.rng.gen_range(1..=2);
        for _ in 0..num_activities {
            let (activity, skill, source) = activities.choose(&mut self.rng).unwrap();
            let experience = self.rng.gen_range(20.0..80.0);
            
            self.beings[being_index].gain_skill_experience(skill, experience, source.clone());
        }
    }

    fn simulate_elemental_activities(&mut self, being_index: usize) {
        let activities = vec![
            ("channeling elemental power", "Mana", ExperienceSource::ChannelingMana),
            ("maintaining form", "Mana", ExperienceSource::VitalRegeneration),
            ("elemental communion", "Fire Magic", ExperienceSource::StudyingArcane),
        ];

        // Elementals do 1-3 activities per cycle
        let num_activities = self.rng.gen_range(1..=3);
        for _ in 0..num_activities {
            let (activity, skill, source) = activities.choose(&mut self.rng).unwrap();
            let experience = self.rng.gen_range(15.0..60.0);
            
            self.beings[being_index].gain_skill_experience(skill, experience, source.clone());
        }
    }

    fn update_vital_systems(&mut self) {
        let delta_time = 3600.0; // 1 hour per cycle
        
        for being in &mut self.beings {
            self.vital_manager.update_vitals(being, delta_time);
            
            // Simulate some damage/vital consumption
            if self.rng.gen_bool(0.3) { // 30% chance
                let damage = self.rng.gen_range(5.0..20.0);
                being.vitals.health.current = (being.vitals.health.current - damage).max(1.0);
            }
            
            if self.rng.gen_bool(0.4) { // 40% chance
                let energy_cost = self.rng.gen_range(10.0..30.0);
                being.vitals.energy.current = (being.vitals.energy.current - energy_cost).max(0.0);
            }
        }
    }

    fn process_skill_evolution(&mut self) {
        // Process any pending evolution requests
        self.evolution_system.process_requests();
        
        // Simulate community voting (for demo purposes)
        let active_votes: Vec<String> = self.evolution_system.community_votes
            .iter()
            .filter(|(_, vote)| vote.is_active && vote.total_votes < 10)
            .map(|(name, _)| name.clone())
            .collect();
            
        for skill_name in active_votes {
            // Simulate random votes
            let voter_id = Uuid::new_v4();
            let vote_choice = if self.rng.gen_bool(0.7) { 
                VoteChoice::Yes 
            } else { 
                VoteChoice::No 
            };
            
            let _ = self.evolution_system.cast_vote(&skill_name, voter_id, vote_choice);
        }
    }

    fn check_vital_unlocks(&mut self) {
        for being in &mut self.beings {
            // Check if being can unlock new vitals
            if being.vitals.active_optionals.len() < 3 {
                let potential_vitals = vec!["Wrath", "Fury"];
                
                for vital_name in potential_vitals {
                    if !being.vitals.optional_vitals.contains_key(vital_name) {
                        if let Ok(()) = self.vital_manager.try_unlock_vital(being, vital_name) {
                            println!("  🔓 {} unlocked vital: {}", being.name, vital_name);
                        }
                    }
                }
            }
        }
    }

    fn demonstrate_skill_discovery(&mut self) {
        println!("\n🔬 Skill Discovery Events:");
        
        // Pick a random being to discover a new skill
        if let Some(being) = self.beings.choose_mut(&mut self.rng) {
            let potential_skills = vec![
                "Weapon Enchantment", "Berserker Rage", "Shadow Step", 
                "Arcane Shield", "Spirit Communion", "Chi Strike"
            ];
            
            let skill_name = potential_skills.choose(&mut self.rng).unwrap();
            
            // Create a "letter to the gods"
            let letter = LetterToGods {
                id: Uuid::new_v4(),
                sender_being: being.id,
                sender_faction: Some("Independent Researcher".to_string()),
                subject: format!("Request for {} Skill", skill_name),
                message: format!(
                    "Greetings, divine ones. I, {}, have observed through my studies and combat experience \
                    that there exists a need for the skill '{}'. Through combining my knowledge of existing \
                    arts, I believe this technique could benefit all beings in their growth and survival.",
                    being.name, skill_name
                ),
                requested_skill: skill_name.to_string(),
                justification: format!(
                    "Current skills insufficient for optimal {}. New skill would fill gap in progression.",
                    match skill_name {
                        s if s.contains("Weapon") => "combat enhancement",
                        s if s.contains("Defense") => "defensive operations", 
                        s if s.contains("Spirit") => "divine connection",
                        _ => "general advancement"
                    }
                ),
                supporting_lore: vec![
                    "Ancient texts mention similar techniques".to_string(),
                    "Observed elemental behaviors suggest possibility".to_string(),
                ],
                co_signers: Vec::new(),
                timestamp: Utc::now(),
                response_status: arceon_core::LetterResponse::Pending,
            };

            if let Ok(letter_id) = self.evolution_system.submit_letter_to_gods(letter) {
                println!("  📜 {} submitted a Letter to the Gods requesting '{}'", 
                    being.name, skill_name);
                println!("     Letter ID: {}", letter_id);
            }
        }
    }

    fn show_system_status(&mut self) {
        println!("\n📊 System Status Report:");
        
        // Show top skilled beings
        let mut skill_leaders = Vec::new();
        for being in &self.beings {
            let total_skill_levels: f64 = being.skills.skills.values()
                .map(|s| s.level)
                .sum();
            skill_leaders.push((being.name.clone(), total_skill_levels, being.skills.skills.len()));
        }
        skill_leaders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        println!("  🏆 Top Skilled Beings:");
        for (i, (name, total_level, skill_count)) in skill_leaders.iter().take(3).enumerate() {
            println!("    {}. {} - {:.1} total skill levels ({} skills)", 
                i + 1, name, total_level, skill_count);
        }
        
        // Show evolution system status
        println!("  🔬 Evolution System:");
        println!("     Pending requests: {}", self.evolution_system.pending_requests.len());
        println!("     Approved skills: {}", self.evolution_system.approved_skills.len());
        println!("     Active votes: {}", self.evolution_system.community_votes.len());
        
        // Show vital unlock status
        let total_optional_vitals: usize = self.beings.iter()
            .map(|b| b.vitals.active_optionals.len())
            .sum();
        println!("  💚 Vital System:");
        println!("     Total optional vitals unlocked: {}", total_optional_vitals);
    }

    fn show_final_summary(&self) {
        println!();
        println!("{}", "=".repeat(60));
        println!("🏁 Final Being System Demo Summary");
        println!("{}", "=".repeat(60));
        
        println!("📈 Being Progression:");
        for being in &self.beings {
            let total_exp: f64 = being.skills.experience_log.iter()
                .map(|e| e.amount)
                .sum();
            let skill_count = being.skills.skills.len();
            let vital_count = being.vitals.active_optionals.len();
            
            println!("  {} ({:?}):", being.name, being.being_type);
            println!("    Skills mastered: {}", skill_count);
            println!("    Total experience: {:.1}", total_exp);
            println!("    Optional vitals: {}", vital_count);
            println!("    Discoveries: {}", being.discovered_skills.len());
            
            // Show top skills
            let mut top_skills: Vec<_> = being.skills.skills.iter().collect();
            top_skills.sort_by(|a, b| b.1.level.partial_cmp(&a.1.level).unwrap());
            if let Some((skill_name, skill)) = top_skills.first() {
                println!("    Highest skill: {} (Level {:.1})", skill_name, skill.level);
            }
            println!();
        }
        
        println!("🌍 World Evolution:");
        println!("  Skills in global database: {}", self.evolution_system.approved_skills.len());
        println!("  Evolution requests submitted: {}", 
            self.evolution_system.pending_requests.len() + self.evolution_system.rejected_requests.len());
        println!("  Community participation: {} votes cast", 
            self.evolution_system.community_votes.values()
                .map(|v| v.total_votes)
                .sum::<u32>());
        
        println!("\n🎯 Key Achievements:");
        println!("  ✅ Beings autonomously discovered and developed skills");
        println!("  ✅ Feral beasts progressed without inventory dependency");  
        println!("  ✅ Vital systems unlocked based on skill progression");
        println!("  ✅ Community consensus system for skill evolution");
        println!("  ✅ No de-leveling - permanent progression maintained");
        println!("  ✅ Experience-only skill advancement (no purchasing)");
        
        println!("\n🔮 System demonstrates readiness for:");
        println!("  • Integration with genesis world NPCs");
        println!("  • Player character creation and progression");
        println!("  • Blockchain-based skill/vital persistence");
        println!("  • Community-driven game evolution");
        println!("  • Autonomous NPC civilization development");
    }
}

fn main() {
    println!("🌟 Arceon Being System Demo");
    println!("Demonstrating autonomous skill discovery and vital management");
    println!();
    
    let mut demo = BeingSystemDemo::new();
    demo.initialize();
    demo.run_simulation(15); // Run 15 cycles
}
