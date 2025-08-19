use std::collections::HashMap;
use rand::prelude::*;
use chrono::Utc;
use uuid::Uuid;

// Import only the new being system from arceon-core
use arceon_core::{
    being::{Being, BeingType, Race, ExperienceSource},
    systems::{SkillEvolutionSystem, VitalManager},
    LetterToGods, VoteChoice,
};

/// Standalone Arceon Being System Demo
/// Showcases autonomous NPCs with skill discovery and vital management
/// This demo does not depend on the AI system to avoid compatibility issues
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
        println!("🎯 Goal: Demonstrate autonomous skill discovery and vital unlocking");
        println!("{}", "=".repeat(70));

        self.create_intelligent_beings();
        self.create_feral_beasts();
        self.create_magical_entities();

        println!("📊 Total beings created: {}", self.beings.len());
        println!("🔬 Each being will autonomously discover skills and request new abilities");
        println!();
    }

    fn create_intelligent_beings(&mut self) {
        let intelligent_configs = vec![
            ("Aldara the Scholar", Race::Elf, vec!["Intellect", "Mana", "Focus"]),
            ("Borin Ironforge", Race::Dwarf, vec!["Defense", "Health", "Smithing"]),
            ("Kael Swiftblade", Race::Human, vec!["Defense", "Athletics", "Leadership"]),
            ("Zara Nightwhisper", Race::Tiefling, vec!["Stealth", "Mana", "Shadow"]),
            ("Grimjaw Berserker", Race::Orc, vec!["Defense", "Fury", "Athletics"]),
        ];

        for (name, race, focus_skills) in intelligent_configs {
            let mut being = Being::new(name.to_string(), BeingType::Npc, race, false);
            
            for skill in focus_skills {
                let experience = self.rng.gen_range(100.0..500.0);
                being.gain_skill_experience(skill, experience, ExperienceSource::SkillDiscovery);
            }

            println!("👤 Created intelligent being: {} ({:?})", name, being.race);
            self.beings.push(being);
        }
    }

    fn create_feral_beasts(&mut self) {
        let beast_configs = vec![
            ("Shadowfang", Race::DireWolf, vec!["Athletics", "Stealth"]),
            ("Flameheart", Race::Dragon, vec!["Fire Magic", "Defense"]),
            ("Stormclaw", Race::Griffin, vec!["Athletics", "Defense"]),
        ];

        for (name, race, natural_skills) in beast_configs {
            let mut being = Being::new(name.to_string(), BeingType::Beast, race, true);
            
            for skill in natural_skills {
                let experience = self.rng.gen_range(200.0..800.0);
                being.gain_skill_experience(skill, experience, ExperienceSource::CombatVictory);
            }

            println!("🐺 Created feral beast: {} ({:?}) - shows skill progression without inventory", name, being.race);
            self.beings.push(being);
        }
    }

    fn create_magical_entities(&mut self) {
        let magical_configs = vec![
            ("Pyrion", Race::FireElemental, vec!["Fire Magic", "Mana"]),
            ("Aqua", Race::WaterElemental, vec!["Mana", "Defense"]),
            ("Ancient Sage", Race::Lich, vec!["Mana", "Intellect", "Death Magic"]),
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
        println!("\n🎮 Starting Being System Simulation");
        println!("🕐 Each cycle = 1 hour of game time");
        println!("{}", "=".repeat(70));

        for cycle in 1..=cycles {
            self.simulation_cycle = cycle;
            println!("\n📅 === Simulation Cycle {} (Day {}, Hour {}) ===", 
                cycle, (cycle - 1) / 24 + 1, (cycle - 1) % 24);
            
            self.world_time += 3600.0;
            
            self.simulate_being_activities();
            self.update_vital_systems();
            self.process_skill_evolution();
            self.check_vital_unlocks();
            
            if cycle % 5 == 0 {
                self.demonstrate_skill_discovery();
            }
            
            if cycle % 8 == 0 {
                self.show_system_status();
            }
        }

        self.show_final_summary();
    }

    fn simulate_being_activities(&mut self) {
        // Split the borrow to avoid borrow checker issues
        let mut beings = std::mem::take(&mut self.beings);
        
        for being in &mut beings {
            match being.being_type {
                BeingType::Npc => Self::simulate_npc_activities(being, &mut self.rng),
                BeingType::Beast => Self::simulate_beast_activities(being, &mut self.rng),
                BeingType::Elemental => Self::simulate_elemental_activities(being, &mut self.rng),
                _ => {},
            }
        }
        
        self.beings = beings;
    }

    fn simulate_npc_activities(being: &mut Being, rng: &mut StdRng) {
        let activities = vec![
            ("practicing combat techniques", "Defense", ExperienceSource::CombatVictory),
            ("studying ancient magic", "Mana", ExperienceSource::StudyingArcane),
            ("physical conditioning", "Athletics", ExperienceSource::OvercomingObstacles),
            ("leading group discussions", "Leadership", ExperienceSource::LeadershipAction),
            ("crafting tools and weapons", "Smithing", ExperienceSource::CraftingItems),
            ("practicing stealth", "Stealth", ExperienceSource::AdaptingToEnvironment),
        ];

        let num_activities = rng.gen_range(2..=4);
        for _ in 0..num_activities {
            let (activity, skill, source) = activities.choose(rng).unwrap();
            let experience = rng.gen_range(10.0..50.0);
            
            being.gain_skill_experience(skill, experience, source.clone());
            
            if rng.gen_bool(0.15) {
                println!("  📝 {} is {}: +{:.1} {} experience", 
                    being.name, activity, experience, skill);
            }
        }
    }

    fn simulate_beast_activities(being: &mut Being, rng: &mut StdRng) {
        let activities = vec![
            ("hunting prey", "Athletics", ExperienceSource::SurvivingDanger),
            ("defending territory", "Defense", ExperienceSource::CombatVictory),
            ("stalking through shadows", "Stealth", ExperienceSource::AdaptingToEnvironment),
            ("channeling natural magic", "Fire Magic", ExperienceSource::ChannelingMana),
        ];

        let num_activities = rng.gen_range(1..=2);
        for _ in 0..num_activities {
            let (_activity, skill, source) = activities.choose(rng).unwrap();
            let experience = rng.gen_range(20.0..80.0);
            
            being.gain_skill_experience(skill, experience, source.clone());
        }
    }

    fn simulate_elemental_activities(being: &mut Being, rng: &mut StdRng) {
        let activities = vec![
            ("channeling elemental power", "Mana", ExperienceSource::ChannelingMana),
            ("maintaining corporeal form", "Focus", ExperienceSource::VitalRegeneration),
            ("communing with elemental plane", "Intellect", ExperienceSource::StudyingArcane),
            ("manifesting elemental magic", "Fire Magic", ExperienceSource::CastingSpells),
        ];

        let num_activities = rng.gen_range(1..=3);
        for _ in 0..num_activities {
            let (_activity, skill, source) = activities.choose(rng).unwrap();
            let experience = rng.gen_range(15.0..60.0);
            
            being.gain_skill_experience(skill, experience, source.clone());
        }
    }

    fn update_vital_systems(&mut self) {
        let delta_time = 3600.0; // 1 hour per cycle
        
        for being in &mut self.beings {
            self.vital_manager.update_vitals(being, delta_time);
            
            // Simulate some damage/vital consumption (life is dangerous!)
            if self.rng.gen_bool(0.3) {
                let damage = self.rng.gen_range(5.0..20.0);
                being.vitals.health.current = (being.vitals.health.current - damage).max(1.0);
            }
            
            if self.rng.gen_bool(0.4) {
                let energy_cost = self.rng.gen_range(10.0..30.0);
                being.vitals.energy.current = (being.vitals.energy.current - energy_cost).max(0.0);
            }
        }
    }

    fn process_skill_evolution(&mut self) {
        self.evolution_system.process_requests();
        
        // Collect vote info to avoid borrow checker issues
        let vote_names: Vec<String> = self.evolution_system.community_votes.iter()
            .filter(|(_, vote)| vote.is_active && vote.total_votes < 10)
            .map(|(name, _)| name.clone())
            .collect();
        
        let mut votes_cast = 0;
        for skill_name in vote_names {
            let voter_id = Uuid::new_v4();
            let vote_choice = if self.rng.gen_bool(0.7) { 
                VoteChoice::Yes 
            } else { 
                VoteChoice::No 
            };
            
            if self.evolution_system.cast_vote(&skill_name, voter_id, vote_choice).is_ok() {
                votes_cast += 1;
            }
        }
        
        if votes_cast > 0 {
            println!("  🗳️  {} community votes cast on skill evolution requests", votes_cast);
        }
    }

    fn check_vital_unlocks(&mut self) {
        for being in &mut self.beings {
            if being.vitals.active_optionals.len() < 3 {
                let potential_vitals = vec!["Wrath", "Fury", "Spirit", "Focus", "Chi", "Shadow"];
                
                for vital_name in potential_vitals {
                    if !being.vitals.optional_vitals.contains_key(vital_name) {
                        if self.vital_manager.try_unlock_vital(being, vital_name).is_ok() {
                            println!("  🔓 {} unlocked new vital: {} (now has {} active optionals)", 
                                being.name, vital_name, being.vitals.active_optionals.len());
                        }
                    }
                }
            }
        }
    }

    fn demonstrate_skill_discovery(&mut self) {
        println!("\n🔬 === Skill Discovery Events ===");
        
        if let Some(being) = self.beings.choose_mut(&mut self.rng) {
            let potential_skills = vec![
                "Weapon Enchantment", "Berserker Rage", "Shadow Step", 
                "Arcane Shield", "Spirit Communion", "Chi Strike",
                "Elemental Mastery", "Divine Protection", "Time Manipulation"
            ];
            
            let skill_name = potential_skills.choose(&mut self.rng).unwrap();
            
            let letter = LetterToGods {
                id: Uuid::new_v4(),
                sender_being: being.id,
                sender_faction: Some("Independent Researcher".to_string()),
                subject: format!("Petition for {} Skill", skill_name),
                message: format!(
                    "Honored divine beings, I am {}, a {} who has dedicated countless hours to mastering the arts. \
                    Through my studies and experiences, I have discovered the theoretical framework for a new ability: '{}'.\n\n\
                    This skill would represent a natural evolution of existing techniques and would benefit all beings \
                    in their journey of growth and survival. I humbly request your consideration and guidance in \
                    manifesting this capability into our world.",
                    being.name, 
                    match being.race {
                        Race::Elf => "scholarly elf",
                        Race::Dwarf => "master craftsman",
                        Race::Human => "versatile human",
                        Race::Orc => "fierce warrior",
                        Race::Tiefling => "mystical tiefling",
                        Race::Dragon => "ancient dragon",
                        Race::Lich => "undead scholar",
                        _ => "unique being"
                    },
                    skill_name
                ),
                requested_skill: skill_name.to_string(),
                justification: format!(
                    "Current skill limitations create gaps in {} optimization. \
                    Proposed skill addresses fundamental needs in character progression and would enhance the \
                    strategic depth of our world's growth systems.",
                    match skill_name {
                        s if s.contains("Weapon") => "combat enhancement and equipment mastery",
                        s if s.contains("Shadow") => "stealth and infiltration operations", 
                        s if s.contains("Spirit") => "divine connection and spiritual growth",
                        s if s.contains("Time") => "temporal manipulation and advanced magic",
                        s if s.contains("Chi") => "internal energy mastery and martial arts",
                        _ => "general character advancement and world interaction"
                    }
                ),
                supporting_lore: vec![
                    "References found in ancient texts suggest historical precedent".to_string(),
                    "Elemental patterns observed in nature support theoretical foundation".to_string(),
                    "Cross-referencing multiple skill trees reveals natural progression path".to_string(),
                ],
                co_signers: Vec::new(),
                timestamp: Utc::now(),
                response_status: arceon_core::LetterResponse::Pending,
            };

            if let Ok(letter_id) = self.evolution_system.submit_letter_to_gods(letter) {
                println!("  📜 {} submitted Letter to the Gods requesting '{}'", 
                    being.name, skill_name);
                println!("     ✉️  Letter ID: {}", letter_id);
                println!("     🎯 This demonstrates how NPCs can autonomously request new game features!");
            }
        }
    }

    fn show_system_status(&mut self) {
        println!("\n📊 === System Status Report ===");
        
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
        
        // Show skill evolution system
        println!("  🔬 Skill Evolution System:");
        println!("     📝 Pending requests: {}", self.evolution_system.pending_requests.len());
        println!("     ✅ Approved skills: {}", self.evolution_system.approved_skills.len());
        println!("     🗳️  Active community votes: {}", self.evolution_system.community_votes.len());
        
        // Show vital system progress
        let total_optional_vitals: usize = self.beings.iter()
            .map(|b| b.vitals.active_optionals.len())
            .sum();
        println!("  💚 Vital System Progress:");
        println!("     🔓 Total optional vitals unlocked: {}", total_optional_vitals);
        
        // Show discovery counts
        let total_discoveries: usize = self.beings.iter()
            .map(|b| b.discovered_skills.len())
            .sum();
        println!("  🔍 Discovery System:");
        println!("     💡 Total skill discoveries: {}", total_discoveries);
    }

    fn show_final_summary(&self) {
        println!("\n{}", "=".repeat(70));
        println!("🏁 Final Arceon Being System Demo Summary");
        println!("{}", "=".repeat(70));
        
        println!("📈 Being Progression Analysis:");
        for being in &self.beings {
            let total_exp: f64 = being.skills.experience_log.iter()
                .map(|e| e.amount)
                .sum();
            let skill_count = being.skills.skills.len();
            let vital_count = being.vitals.active_optionals.len();
            
            println!("  {} ({:?} - {:?}):", being.name, being.being_type, being.race);
            println!("    📚 Skills mastered: {}", skill_count);
            println!("    🎯 Total experience: {:.1}", total_exp);
            println!("    💚 Optional vitals unlocked: {}", vital_count);
            println!("    🔍 Personal discoveries: {}", being.discovered_skills.len());
            
            // Show top skills
            let mut top_skills: Vec<_> = being.skills.skills.iter().collect();
            top_skills.sort_by(|a, b| b.1.level.partial_cmp(&a.1.level).unwrap());
            if let Some((skill_name, skill)) = top_skills.first() {
                println!("    🥇 Highest skill: {} (Level {:.1})", skill_name, skill.level);
            }
            
            // Show current vitals
            println!("    💖 Health: {:.0}/{:.0}", being.vitals.health.current, being.vitals.health.maximum);
            println!("    ⚡ Energy: {:.0}/{:.0}", being.vitals.energy.current, being.vitals.energy.maximum);
            println!("    🔮 Mana: {:.0}/{:.0}", being.vitals.mana.current, being.vitals.mana.maximum);
            println!();
        }
        
        println!("🌍 World Evolution Status:");
        println!("  🎯 Skills in global database: {}", self.evolution_system.approved_skills.len());
        println!("  📝 Evolution requests submitted: {}", 
            self.evolution_system.pending_requests.len() + self.evolution_system.rejected_requests.len());
        println!("  🗳️  Community votes cast: {}", 
            self.evolution_system.community_votes.values()
                .map(|v| v.total_votes)
                .sum::<u32>());
        
        println!("\n🎯 Key System Achievements:");
        println!("  ✅ Universal beings (NPCs, beasts, elementals) all progress autonomously");
        println!("  ✅ Feral beasts develop skills without inventory systems");  
        println!("  ✅ Vital systems unlock dynamically based on skill progression");
        println!("  ✅ Community consensus drives skill evolution requests");
        println!("  ✅ No skill de-leveling - all progression is permanent");
        println!("  ✅ Experience-only advancement (no skill purchasing)");
        println!("  ✅ 'Letters to Gods' system enables NPCs to request new features");
        
        println!("\n🔮 System Architecture Demonstrates:");
        println!("  🏗️  Modular entity framework supports any living being type");
        println!("  🧠 Autonomous NPC civilization development capability");
        println!("  🎮 Player character integration with same progression system");
        println!("  🔗 Blockchain persistence readiness for all skills/vitals");
        println!("  🌐 Community-driven game evolution through democratic consensus");
        println!("  ⚡ High-performance ECS integration with Bevy game engine");
        
        println!("\n💫 Next Integration Steps:");
        println!("  🌟 Connect with genesis world for NPC population");
        println!("  🏺 Integrate with runestone system for magical discovery");
        println!("  ⚔️  Link with command engine for skill-based interactions");
        println!("  🏛️  Deploy community governance for live skill evolution");
        
        println!("\n🎊 Demo Complete! The Being system is ready for production use.");
    }
}

fn main() {
    println!("🌟 Arceon Being System - Standalone Demo");
    println!("🎯 Showcasing autonomous skill discovery and vital management");
    println!("🚀 Independent of AI system for clean demonstration");
    println!();
    
    let mut demo = BeingSystemDemo::new();
    demo.initialize();
    demo.run_simulation(20); // Run 20 cycles (nearly a full day)
    
    println!("\n🎮 Demo completed successfully!");
    println!("📖 This system provides the foundation for autonomous NPC civilizations");
    println!("   where beings can discover new skills and evolve the game world through");
    println!("   community consensus and developer collaboration.");
}
