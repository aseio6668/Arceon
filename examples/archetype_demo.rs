/*!
# Archetype System Demo

Demonstrates the dynamic archetype calculation system that emerges
from player actions and skill development.
*/

use arceon_progression::*;
use std::collections::HashMap;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ARCEON ARCHETYPE SYSTEM DEMO ===\n");

    // Create a new progression system
    let mut progression_system = ProgressionSystem::new();
    
    // Create a test player
    let player_id = Uuid::new_v4();
    let _character_progression = progression_system.initialize_character_progression(player_id)?;

    println!("🧙‍♂️ Player {} started their journey...\n", player_id);

    // Scenario 1: Pure Warrior Path
    println!("📖 SCENARIO 1: The Way of the Warrior");
    println!("Player focuses on combat skills and physical prowess");
    
    let mut warrior_skills = HashMap::new();
    warrior_skills.insert(SkillType::Swordsmanship, SkillLevel { current_level: 25, experience: 1000, max_level: 100 });
    warrior_skills.insert(SkillType::Combat, SkillLevel { current_level: 30, experience: 1200, max_level: 100 });
    warrior_skills.insert(SkillType::Fitness, SkillLevel { current_level: 20, experience: 800, max_level: 100 });
    warrior_skills.insert(SkillType::Tactics, SkillLevel { current_level: 15, experience: 600, max_level: 100 });
    
    let warrior_archetypes = progression_system.update_archetypes(player_id, &warrior_skills)?;
    display_archetype_results("Warrior Path", &warrior_archetypes);

    // Scenario 2: Battle-Mage Path (Hybrid)
    println!("\n📖 SCENARIO 2: The Battle-Mage's Balance");
    println!("Player combines combat prowess with magical study");
    
    let mut battlemage_skills = HashMap::new();
    battlemage_skills.insert(SkillType::Combat, SkillLevel { current_level: 22, experience: 900, max_level: 100 });
    battlemage_skills.insert(SkillType::Magic, SkillLevel { current_level: 25, experience: 1000, max_level: 100 });
    battlemage_skills.insert(SkillType::Swordsmanship, SkillLevel { current_level: 18, experience: 700, max_level: 100 });
    battlemage_skills.insert(SkillType::Enchanting, SkillLevel { current_level: 20, experience: 800, max_level: 100 });
    
    let battlemage_archetypes = progression_system.update_archetypes(player_id, &battlemage_skills)?;
    display_archetype_results("Battle-Mage Path", &battlemage_archetypes);

    // Scenario 3: Geomancer-Miner Path (Magical Gatherer)
    println!("\n📖 SCENARIO 3: The Geomancer-Miner's Communion");
    println!("Player uses earth magic to gather precious materials");
    
    let mut geomancer_skills = HashMap::new();
    geomancer_skills.insert(SkillType::Mining, SkillLevel { current_level: 35, experience: 1400, max_level: 100 });
    geomancer_skills.insert(SkillType::Magic, SkillLevel { current_level: 28, experience: 1100, max_level: 100 });
    geomancer_skills.insert(SkillType::Alchemy, SkillLevel { current_level: 15, experience: 600, max_level: 100 });
    geomancer_skills.insert(SkillType::Geology, SkillLevel { current_level: 25, experience: 1000, max_level: 100 });
    
    let geomancer_archetypes = progression_system.update_archetypes(player_id, &geomancer_skills)?;
    display_archetype_results("Geomancer-Miner Path", &geomancer_archetypes);

    // Scenario 4: Healer Path
    println!("\n📖 SCENARIO 4: The Healer's Compassion");
    println!("Player dedicates themselves to healing and restoration");
    
    let mut healer_skills = HashMap::new();
    healer_skills.insert(SkillType::Magic, SkillLevel { current_level: 30, experience: 1200, max_level: 100 });
    healer_skills.insert(SkillType::Alchemy, SkillLevel { current_level: 25, experience: 1000, max_level: 100 });
    healer_skills.insert(SkillType::Herbalism, SkillLevel { current_level: 20, experience: 800, max_level: 100 });
    healer_skills.insert(SkillType::Cooking, SkillLevel { current_level: 15, experience: 600, max_level: 100 });
    
    let healer_archetypes = progression_system.update_archetypes(player_id, &healer_skills)?;
    display_archetype_results("Healer Path", &healer_archetypes);

    // Scenario 5: Multi-Path Character (Shows emergent complexity)
    println!("\n📖 SCENARIO 5: The Renaissance Adventurer");
    println!("Player has developed skills across multiple domains");
    
    let mut renaissance_skills = HashMap::new();
    renaissance_skills.insert(SkillType::Combat, SkillLevel { current_level: 20, experience: 800, max_level: 100 });
    renaissance_skills.insert(SkillType::Magic, SkillLevel { current_level: 18, experience: 700, max_level: 100 });
    renaissance_skills.insert(SkillType::Mining, SkillLevel { current_level: 25, experience: 1000, max_level: 100 });
    renaissance_skills.insert(SkillType::Alchemy, SkillLevel { current_level: 22, experience: 900, max_level: 100 });
    renaissance_skills.insert(SkillType::Smithing, SkillLevel { current_level: 24, experience: 950, max_level: 100 });
    renaissance_skills.insert(SkillType::Swordsmanship, SkillLevel { current_level: 15, experience: 600, max_level: 100 });
    renaissance_skills.insert(SkillType::Herbalism, SkillLevel { current_level: 12, experience: 500, max_level: 100 });
    
    let renaissance_archetypes = progression_system.update_archetypes(player_id, &renaissance_skills)?;
    display_archetype_results("Renaissance Path", &renaissance_archetypes);

    // Show archetype bonuses
    println!("\n🎁 ARCHETYPE BONUSES & ABILITIES");
    let bonuses = progression_system.get_archetype_bonuses(player_id);
    for bonus in bonuses {
        println!("📜 {} Archetype Bonuses:", bonus.archetype_type.display_name());
        for level_bonus in &bonus.level_bonuses {
            if renaissance_archetypes.archetype_levels.get(&bonus.archetype_type)
                .map_or(false, |level| *level >= level_bonus.required_level) {
                println!("   ✨ Level {}: {}", level_bonus.required_level, level_bonus.description);
            }
        }
        for mastery in &bonus.mastery_abilities {
            if renaissance_archetypes.archetype_levels.get(&bonus.archetype_type)
                .map_or(false, |level| *level >= mastery.required_level) {
                println!("   🎯 {}: {}", mastery.name, mastery.description);
            }
        }
    }

    // Show archetype philosophies
    println!("\n🧭 ARCHETYPE PHILOSOPHIES & APPROACHES");
    for (archetype_type, level) in &renaissance_archetypes.archetype_levels {
        if let Some(definition) = progression_system.get_archetype_info(archetype_type) {
            if *level >= 5.0 { // Only show philosophies for developed archetypes
                println!("🎭 {} (Level {:.2}):", definition.display_name, level);
                println!("   Philosophy: {:?}", definition.philosophy.core_philosophy);
                println!("   Gathering: {:?}", definition.philosophy.gathering_approach);
                println!("   Crafting: {:?}", definition.philosophy.crafting_approach);
                println!("   Magical Alignment: {:?}", definition.philosophy.magical_alignment);
            }
        }
    }

    println!("\n✨ ARCHETYPE SYSTEM FEATURES DEMONSTRATED:");
    println!("• Dynamic calculation from skill combinations");
    println!("• Multiple archetypes can coexist (Renaissance character has 5!)");
    println!("• Synergy bonuses reward skill combinations");
    println!("• Philosophical approaches guide playstyle");
    println!("• Magical vs. Physical gathering/crafting alternatives");
    println!("• Precise decimal tracking (e.g., Blacksmith 24.22, Healer 5.41)");
    println!("• Emergent progression without rigid class restrictions");

    Ok(())
}

fn display_archetype_results(scenario_name: &str, archetypes: &PlayerArchetypes) {
    println!("┌─ {} Results ─┐", scenario_name);
    
    if let Some(dominant) = &archetypes.dominant_archetype {
        let level = archetypes.archetype_levels.get(dominant).unwrap_or(&0.0);
        println!("👑 Dominant: {} ({:.2})", dominant.display_name(), level);
    }
    
    if let Some(secondary) = &archetypes.secondary_archetype {
        let level = archetypes.archetype_levels.get(secondary).unwrap_or(&0.0);
        println!("🥈 Secondary: {} ({:.2})", secondary.display_name(), level);
    }
    
    println!("📊 All Archetype Levels:");
    let mut sorted_archetypes: Vec<(&ArchetypeType, &f64)> = archetypes.archetype_levels.iter().collect();
    sorted_archetypes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    
    for (archetype_type, level) in sorted_archetypes.iter().take(8) { // Show top 8
        println!("   • {} {:.2}", archetype_type.display_name(), level);
    }
    
    println!("📈 Total Archetype Points: {:.2}", archetypes.total_archetype_points);
    println!("└────────────────────────────────┘");
}

// Extension trait for better display names
trait ArchetypeDisplay {
    fn display_name(&self) -> String;
}

impl ArchetypeDisplay for ArchetypeType {
    fn display_name(&self) -> String {
        match self {
            ArchetypeType::Warrior => "Warrior".to_string(),
            ArchetypeType::Guardian => "Guardian".to_string(),
            ArchetypeType::BattleMage => "Battle-Mage".to_string(),
            ArchetypeType::Mage => "Mage".to_string(),
            ArchetypeType::Healer => "Healer".to_string(),
            ArchetypeType::Rogue => "Rogue".to_string(),
            ArchetypeType::Blacksmith => "Blacksmith".to_string(),
            ArchetypeType::GeomancerMiner => "Geomancer-Miner".to_string(),
            ArchetypeType::VerdantMage => "Verdant Mage".to_string(),
            ArchetypeType::Artificer => "Artificer".to_string(),
            ArchetypeType::Alchemist => "Alchemist".to_string(),
            ArchetypeType::Scholar => "Scholar".to_string(),
            ArchetypeType::Explorer => "Explorer".to_string(),
            ArchetypeType::Merchant => "Merchant".to_string(),
            _ => format!("{:?}", self),
        }
    }
}