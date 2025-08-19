use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Individual skill types for easy reference
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Skill {
    // Combat Skills
    OneHandedSword,
    TwoHandedSword,
    Axe,
    Mace,
    Dagger,
    Bow,
    Crossbow,
    Shield,
    Dodge,
    Parry,
    Block,
    Unarmed,
    
    // Crafting Skills
    Smithing,
    Tailoring,
    Alchemy,
    Cooking,
    Enchanting,
    Carpentry,
    Mining,
    Herbalism,
    Gemcutting,
    Inscription,
    
    // Magic Skills
    FireMagic,
    WaterMagic,
    EarthMagic,
    AirMagic,
    LightMagic,
    DarkMagic,
    HealingMagic,
    IllusionMagic,
    Necromancy,
    Divination,
    
    // Survival Skills
    Tracking,
    Stealth,
    Lockpicking,
    TrapDetection,
    Foraging,
    AnimalHandling,
    Navigation,
    Climbing,
    Swimming,
    
    // Social Skills
    Persuasion,
    Intimidation,
    Deception,
    Insight,
    Leadership,
    Trading,
    Performance,
    Teaching,
}

/// Skill progression system
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub combat: CombatSkills,
    pub crafting: CraftingSkills,
    pub magic: MagicSkills,
    pub survival: SurvivalSkills,
    pub social: SocialSkills,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSkills {
    pub one_handed_sword: SkillLevel,
    pub two_handed_sword: SkillLevel,
    pub axe: SkillLevel,
    pub mace: SkillLevel,
    pub dagger: SkillLevel,
    pub bow: SkillLevel,
    pub crossbow: SkillLevel,
    pub shield: SkillLevel,
    pub dodge: SkillLevel,
    pub parry: SkillLevel,
    pub block: SkillLevel,
    pub unarmed: SkillLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingSkills {
    pub smithing: SkillLevel,
    pub tailoring: SkillLevel,
    pub alchemy: SkillLevel,
    pub cooking: SkillLevel,
    pub enchanting: SkillLevel,
    pub carpentry: SkillLevel,
    pub mining: SkillLevel,
    pub herbalism: SkillLevel,
    pub gemcutting: SkillLevel,
    pub inscription: SkillLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicSkills {
    pub fire_magic: SkillLevel,
    pub water_magic: SkillLevel,
    pub earth_magic: SkillLevel,
    pub air_magic: SkillLevel,
    pub light_magic: SkillLevel,
    pub dark_magic: SkillLevel,
    pub healing_magic: SkillLevel,
    pub illusion_magic: SkillLevel,
    pub necromancy: SkillLevel,
    pub divination: SkillLevel,
    pub individual_spells: HashMap<String, SkillLevel>, // Individual spell progression
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalSkills {
    pub tracking: SkillLevel,
    pub stealth: SkillLevel,
    pub lockpicking: SkillLevel,
    pub trap_detection: SkillLevel,
    pub foraging: SkillLevel,
    pub animal_handling: SkillLevel,
    pub navigation: SkillLevel,
    pub climbing: SkillLevel,
    pub swimming: SkillLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSkills {
    pub persuasion: SkillLevel,
    pub intimidation: SkillLevel,
    pub deception: SkillLevel,
    pub insight: SkillLevel,
    pub leadership: SkillLevel,
    pub trading: SkillLevel,
    pub performance: SkillLevel,
    pub teaching: SkillLevel,
}

/// Individual skill level and progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLevel {
    pub level: u32,
    pub experience: u64,
    pub experience_to_next: u64,
    pub practice_count: u64,
    pub last_used: i64,
    pub decay_rate: f32,        // How fast skill degrades without use
}

impl SkillLevel {
    pub fn new() -> Self {
        Self {
            level: 1,
            experience: 0,
            experience_to_next: 100,
            practice_count: 0,
            last_used: 0,
            decay_rate: 0.01,
        }
    }
    
    /// Add experience to this skill
    pub fn add_experience(&mut self, amount: u64) {
        self.experience += amount;
        self.practice_count += 1;
        
        // Check for level up
        while self.experience >= self.experience_to_next {
            self.level_up();
        }
    }
    
    /// Level up the skill
    fn level_up(&mut self) {
        self.experience -= self.experience_to_next;
        self.level += 1;
        
        // Calculate next level requirement (exponential growth)
        self.experience_to_next = ((self.level as f64).powf(2.2) * 100.0) as u64;
    }
    
    /// Get effective skill level (accounting for decay)
    pub fn effective_level(&self, current_time: i64) -> f32 {
        let time_since_use = (current_time - self.last_used) as f32;
        let decay = (time_since_use * self.decay_rate).min(self.level as f32 * 0.5);
        (self.level as f32 - decay).max(1.0)
    }
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            combat: CombatSkills {
                one_handed_sword: SkillLevel::new(),
                two_handed_sword: SkillLevel::new(),
                axe: SkillLevel::new(),
                mace: SkillLevel::new(),
                dagger: SkillLevel::new(),
                bow: SkillLevel::new(),
                crossbow: SkillLevel::new(),
                shield: SkillLevel::new(),
                dodge: SkillLevel::new(),
                parry: SkillLevel::new(),
                block: SkillLevel::new(),
                unarmed: SkillLevel::new(),
            },
            crafting: CraftingSkills {
                smithing: SkillLevel::new(),
                tailoring: SkillLevel::new(),
                alchemy: SkillLevel::new(),
                cooking: SkillLevel::new(),
                enchanting: SkillLevel::new(),
                carpentry: SkillLevel::new(),
                mining: SkillLevel::new(),
                herbalism: SkillLevel::new(),
                gemcutting: SkillLevel::new(),
                inscription: SkillLevel::new(),
            },
            magic: MagicSkills {
                fire_magic: SkillLevel::new(),
                water_magic: SkillLevel::new(),
                earth_magic: SkillLevel::new(),
                air_magic: SkillLevel::new(),
                light_magic: SkillLevel::new(),
                dark_magic: SkillLevel::new(),
                healing_magic: SkillLevel::new(),
                illusion_magic: SkillLevel::new(),
                necromancy: SkillLevel::new(),
                divination: SkillLevel::new(),
                individual_spells: HashMap::new(),
            },
            survival: SurvivalSkills {
                tracking: SkillLevel::new(),
                stealth: SkillLevel::new(),
                lockpicking: SkillLevel::new(),
                trap_detection: SkillLevel::new(),
                foraging: SkillLevel::new(),
                animal_handling: SkillLevel::new(),
                navigation: SkillLevel::new(),
                climbing: SkillLevel::new(),
                swimming: SkillLevel::new(),
            },
            social: SocialSkills {
                persuasion: SkillLevel::new(),
                intimidation: SkillLevel::new(),
                deception: SkillLevel::new(),
                insight: SkillLevel::new(),
                leadership: SkillLevel::new(),
                trading: SkillLevel::new(),
                performance: SkillLevel::new(),
                teaching: SkillLevel::new(),
            },
        }
    }
}
