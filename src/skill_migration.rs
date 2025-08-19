use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;
use std::sync::Arc;

use crate::authentication::{SharedAuthManager, UserAccount, CharacterSlot};
use crate::database::{SharedDatabaseManager, PlayerSkills, SkillLevel, SkillBonus, PlayerData, CharacterData, WorldPosition, CharacterAppearance, GameProgress, PlayerInventory, SocialData, PlayerSettings, PlayerStatistics, NotificationSettings, PrivacySettings, ProfileVisibility};

/// Skill migration and update system for existing players
#[derive(Debug, Clone)]
pub struct SkillMigrationManager {
    pub auth_manager: SharedAuthManager,
    pub database_manager: SharedDatabaseManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMigrationReport {
    pub success: bool,
    pub total_players_processed: u32,
    pub players_updated: u32,
    pub new_skills_added: Vec<String>,
    pub errors: Vec<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSkillUpdate {
    pub player_id: Uuid,
    pub character_name: String,
    pub skills_before: u32,
    pub skills_after: u32,
    pub new_skills_added: Vec<String>,
    pub updated: bool,
}

impl SkillMigrationManager {
    pub fn new(auth_manager: SharedAuthManager, database_manager: SharedDatabaseManager) -> Self {
        Self {
            auth_manager,
            database_manager,
        }
    }

    /// Migrate all existing players to updated skill system
    pub async fn migrate_all_players(&self) -> Result<SkillMigrationReport> {
        let start_time = SystemTime::now();
        info!("🔄 Starting skill migration for all existing players...");

        let mut report = SkillMigrationReport {
            success: true,
            total_players_processed: 0,
            players_updated: 0,
            new_skills_added: self.get_new_skills_list(),
            errors: Vec::new(),
            processing_time_ms: 0,
        };

        // Get all users with characters
        let auth = self.auth_manager.read().await;
        let mut players_to_update = Vec::new();

        for user in auth.users.values() {
            for character_slot in &user.character_slots {
                if let Some(character_id) = character_slot.character_id {
                    players_to_update.push((user.clone(), character_slot.clone(), character_id));
                }
            }
        }

        report.total_players_processed = players_to_update.len() as u32;
        drop(auth); // Release the read lock

        info!("📊 Found {} characters to process for skill updates", report.total_players_processed);

        // Process each player
        for (user, character_slot, character_id) in players_to_update {
            match self.update_player_skills(&user, &character_slot, character_id).await {
                Ok(update_result) => {
                    if update_result.updated {
                        report.players_updated += 1;
                        info!("✅ Updated skills for character: {} ({})", 
                            update_result.character_name, update_result.player_id);
                    }
                },
                Err(e) => {
                    let error_msg = format!("Failed to update character {}: {}", 
                        character_slot.character_name.as_deref().unwrap_or("Unknown"), e);
                    error!("{}", error_msg);
                    report.errors.push(error_msg);
                    report.success = false;
                }
            }
        }

        let duration = start_time.elapsed().unwrap_or_default();
        report.processing_time_ms = duration.as_millis() as u64;

        if report.success {
            info!("✅ Skill migration completed successfully!");
            info!("📈 Updated {} out of {} characters in {}ms", 
                report.players_updated, report.total_players_processed, report.processing_time_ms);
        } else {
            warn!("⚠️ Skill migration completed with {} errors", report.errors.len());
        }

        Ok(report)
    }

    /// Update skills for a specific player
    async fn update_player_skills(&self, user: &UserAccount, character_slot: &CharacterSlot, character_id: Uuid) -> Result<PlayerSkillUpdate> {
        let character_name = character_slot.character_name.clone().unwrap_or_else(|| "Unknown".to_string());
        
        info!("🔧 Processing character: {} ({})", character_name, character_id);

        // Try to load existing player data
        let db = self.database_manager.read().await;
        let existing_player_data = db.load_player_data(character_id).await?;
        drop(db);

        let mut update_result = PlayerSkillUpdate {
            player_id: character_id,
            character_name: character_name.clone(),
            skills_before: 0,
            skills_after: 0,
            new_skills_added: Vec::new(),
            updated: false,
        };

        let player_data = match existing_player_data {
            Some(mut data) => {
                // Player exists - update their skills
                update_result.skills_before = self.count_total_skills(&data.skills);
                self.update_existing_player_skills(&mut data.skills, &mut update_result)?;
                data
            },
            None => {
                // Player doesn't exist in database - create new player data
                info!("📝 Creating new player data for character: {}", character_name);
                self.create_new_player_data(user, character_slot, character_id)?
            }
        };

        update_result.skills_after = self.count_total_skills(&player_data.skills);
        update_result.updated = update_result.skills_after > update_result.skills_before;

        // Save updated player data to database
        let mut db = self.database_manager.write().await;
        db.save_player_data(&player_data).await?;
        drop(db);

        Ok(update_result)
    }

    /// Update skills for an existing player
    fn update_existing_player_skills(&self, skills: &mut PlayerSkills, update_result: &mut PlayerSkillUpdate) -> Result<()> {
        let new_skills = self.get_skill_definitions();

        // Add any missing combat skills
        for skill_name in &new_skills.combat_skills {
            if !skills.combat_skills.contains_key(skill_name) {
                skills.combat_skills.insert(skill_name.clone(), self.create_new_skill_level(skill_name));
                update_result.new_skills_added.push(format!("Combat: {}", skill_name));
            }
        }

        // Add any missing crafting skills
        for skill_name in &new_skills.crafting_skills {
            if !skills.crafting_skills.contains_key(skill_name) {
                skills.crafting_skills.insert(skill_name.clone(), self.create_new_skill_level(skill_name));
                update_result.new_skills_added.push(format!("Crafting: {}", skill_name));
            }
        }

        // Add any missing gathering skills
        for skill_name in &new_skills.gathering_skills {
            if !skills.gathering_skills.contains_key(skill_name) {
                skills.gathering_skills.insert(skill_name.clone(), self.create_new_skill_level(skill_name));
                update_result.new_skills_added.push(format!("Gathering: {}", skill_name));
            }
        }

        // Add any missing social skills
        for skill_name in &new_skills.social_skills {
            if !skills.social_skills.contains_key(skill_name) {
                skills.social_skills.insert(skill_name.clone(), self.create_new_skill_level(skill_name));
                update_result.new_skills_added.push(format!("Social: {}", skill_name));
            }
        }

        // Add any missing magic skills
        for skill_name in &new_skills.magic_skills {
            if !skills.magic_skills.contains_key(skill_name) {
                skills.magic_skills.insert(skill_name.clone(), self.create_new_skill_level(skill_name));
                update_result.new_skills_added.push(format!("Magic: {}", skill_name));
            }
        }

        // Add any missing survival skills
        for skill_name in &new_skills.survival_skills {
            if !skills.survival_skills.contains_key(skill_name) {
                skills.survival_skills.insert(skill_name.clone(), self.create_new_skill_level(skill_name));
                update_result.new_skills_added.push(format!("Survival: {}", skill_name));
            }
        }

        Ok(())
    }

    /// Create new player data for a character that doesn't exist in database
    fn create_new_player_data(&self, user: &UserAccount, character_slot: &CharacterSlot, character_id: Uuid) -> Result<PlayerData> {
        let character_name = character_slot.character_name.clone().unwrap_or_else(|| "Unknown".to_string());
        let character_race = character_slot.character_race.clone().unwrap_or_else(|| "Human".to_string());
        let character_level = character_slot.character_level.unwrap_or(1);

        Ok(PlayerData {
            player_id: character_id,
            user_id: user.user_id,
            character_data: CharacterData {
                character_id,
                name: character_name,
                race: character_race,
                class: None,
                level: character_level,
                experience: 0,
                health: 100.0,
                max_health: 100.0,
                mana: 50.0,
                max_mana: 50.0,
                energy: 100.0,
                max_energy: 100.0,
                position: WorldPosition {
                    landmass_id: Uuid::new_v4(),
                    area_name: "Starting Area".to_string(),
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    rotation: 0.0,
                },
                appearance: CharacterAppearance {
                    hair_color: "Brown".to_string(),
                    eye_color: "Brown".to_string(),
                    skin_tone: "Medium".to_string(),
                    height: 1.75,
                    build: "Average".to_string(),
                    clothing: HashMap::new(),
                    accessories: Vec::new(),
                },
                attributes: HashMap::new(),
            },
            game_progress: GameProgress {
                current_quest: None,
                completed_quests: Vec::new(),
                available_quests: Vec::new(),
                reputation: HashMap::new(),
                unlocked_areas: vec!["Starting Area".to_string()],
                discovered_locations: Vec::new(),
                storyline_progress: HashMap::new(),
            },
            inventory: PlayerInventory {
                items: HashMap::new(),
                equipped_items: HashMap::new(),
                backpack_size: 20,
                currency: HashMap::new(),
                bank_storage: HashMap::new(),
            },
            skills: self.create_initial_skills(),
            achievements: Vec::new(),
            social_data: SocialData {
                friends: Vec::new(),
                blocked_players: Vec::new(),
                guild_id: None,
                group_id: None,
                chat_channels: Vec::new(),
                private_messages: Vec::new(),
                social_settings: crate::database::SocialSettings {
                    accept_friend_requests: true,
                    accept_guild_invites: true,
                    accept_group_invites: true,
                    show_online_status: true,
                    allow_private_messages: true,
                },
            },
            settings: PlayerSettings {
                graphics_quality: "Medium".to_string(),
                audio_volume: 0.8,
                music_volume: 0.6,
                ui_scale: 1.0,
                keybindings: HashMap::new(),
                chat_filters: Vec::new(),
                notification_settings: NotificationSettings {
                    in_game_notifications: true,
                    email_notifications: false,
                    push_notifications: false,
                    sound_notifications: true,
                },
                privacy_settings: PrivacySettings {
                    profile_visibility: ProfileVisibility::Public,
                    location_sharing: false,
                    activity_sharing: true,
                    statistics_sharing: true,
                },
            },
            statistics: PlayerStatistics {
                total_playtime: 0,
                login_count: 0,
                commands_executed: 0,
                distance_traveled: 0.0,
                enemies_defeated: 0,
                items_crafted: 0,
                quests_completed: 0,
                currency_earned: HashMap::new(),
                deaths: 0,
                resurrections: 0,
                first_login: SystemTime::now(),
                last_logout: SystemTime::now(),
            },
            created_at: SystemTime::now(),
            last_played: character_slot.last_played.unwrap_or_else(SystemTime::now),
        })
    }

    /// Create initial skills for a new player
    fn create_initial_skills(&self) -> PlayerSkills {
        let skill_definitions = self.get_skill_definitions();
        
        let mut skills = PlayerSkills {
            combat_skills: HashMap::new(),
            crafting_skills: HashMap::new(),
            gathering_skills: HashMap::new(),
            social_skills: HashMap::new(),
            magic_skills: HashMap::new(),
            survival_skills: HashMap::new(),
            available_skill_points: 5, // Start with some skill points
            total_experience: 0,
        };

        // Initialize all skill categories
        for skill_name in skill_definitions.combat_skills {
            skills.combat_skills.insert(skill_name.clone(), self.create_new_skill_level(&skill_name));
        }
        
        for skill_name in skill_definitions.crafting_skills {
            skills.crafting_skills.insert(skill_name.clone(), self.create_new_skill_level(&skill_name));
        }
        
        for skill_name in skill_definitions.gathering_skills {
            skills.gathering_skills.insert(skill_name.clone(), self.create_new_skill_level(&skill_name));
        }
        
        for skill_name in skill_definitions.social_skills {
            skills.social_skills.insert(skill_name.clone(), self.create_new_skill_level(&skill_name));
        }
        
        for skill_name in skill_definitions.magic_skills {
            skills.magic_skills.insert(skill_name.clone(), self.create_new_skill_level(&skill_name));
        }
        
        for skill_name in skill_definitions.survival_skills {
            skills.survival_skills.insert(skill_name.clone(), self.create_new_skill_level(&skill_name));
        }

        skills
    }

    /// Create a new skill level with starting values
    fn create_new_skill_level(&self, skill_name: &str) -> SkillLevel {
        SkillLevel {
            skill_name: skill_name.to_string(),
            current_level: 1,
            current_experience: 0,
            experience_to_next: 100,
            max_level: 100,
            bonuses: Vec::new(),
            unlock_requirements: Vec::new(),
        }
    }

    /// Count total skills across all categories
    fn count_total_skills(&self, skills: &PlayerSkills) -> u32 {
        (skills.combat_skills.len() + 
         skills.crafting_skills.len() + 
         skills.gathering_skills.len() + 
         skills.social_skills.len() + 
         skills.magic_skills.len() + 
         skills.survival_skills.len()) as u32
    }

    /// Get comprehensive skill definitions
    fn get_skill_definitions(&self) -> SkillDefinitions {
        SkillDefinitions {
            combat_skills: vec![
                "One-Handed Sword".to_string(),
                "Two-Handed Sword".to_string(),
                "Axe".to_string(),
                "Mace".to_string(),
                "Dagger".to_string(),
                "Bow".to_string(),
                "Crossbow".to_string(),
                "Shield".to_string(),
                "Dodge".to_string(),
                "Parry".to_string(),
                "Block".to_string(),
                "Unarmed Combat".to_string(),
                "Tactical Combat".to_string(),
                "Berserker Rage".to_string(),
                "Precise Strike".to_string(),
            ],
            crafting_skills: vec![
                "Smithing".to_string(),
                "Tailoring".to_string(),
                "Alchemy".to_string(),
                "Cooking".to_string(),
                "Enchanting".to_string(),
                "Carpentry".to_string(),
                "Gemcutting".to_string(),
                "Inscription".to_string(),
                "Artificing".to_string(),
                "Mechanical Engineering".to_string(),
                "Brewing".to_string(),
                "Poison Making".to_string(),
            ],
            gathering_skills: vec![
                "Mining".to_string(),
                "Herbalism".to_string(),
                "Logging".to_string(),
                "Fishing".to_string(),
                "Hunting".to_string(),
                "Archaeology".to_string(),
                "Scavenging".to_string(),
                "Treasure Hunting".to_string(),
                "Foraging".to_string(),
                "Crystal Harvesting".to_string(),
            ],
            social_skills: vec![
                "Persuasion".to_string(),
                "Intimidation".to_string(),
                "Deception".to_string(),
                "Insight".to_string(),
                "Leadership".to_string(),
                "Trading".to_string(),
                "Performance".to_string(),
                "Teaching".to_string(),
                "Diplomacy".to_string(),
                "Bartering".to_string(),
                "Storytelling".to_string(),
                "Networking".to_string(),
            ],
            magic_skills: vec![
                "Fire Magic".to_string(),
                "Water Magic".to_string(),
                "Earth Magic".to_string(),
                "Air Magic".to_string(),
                "Light Magic".to_string(),
                "Dark Magic".to_string(),
                "Healing Magic".to_string(),
                "Illusion Magic".to_string(),
                "Necromancy".to_string(),
                "Divination".to_string(),
                "Transmutation".to_string(),
                "Summoning".to_string(),
                "Temporal Magic".to_string(),
                "Spatial Magic".to_string(),
                "Spirit Magic".to_string(),
            ],
            survival_skills: vec![
                "Tracking".to_string(),
                "Stealth".to_string(),
                "Lockpicking".to_string(),
                "Trap Detection".to_string(),
                "Animal Handling".to_string(),
                "Navigation".to_string(),
                "Climbing".to_string(),
                "Swimming".to_string(),
                "Survival Instinct".to_string(),
                "Weather Prediction".to_string(),
                "Camouflage".to_string(),
                "Escape Artist".to_string(),
            ],
        }
    }

    /// Get list of newly added skills for reporting
    fn get_new_skills_list(&self) -> Vec<String> {
        vec![
            "Tactical Combat".to_string(),
            "Berserker Rage".to_string(),
            "Precise Strike".to_string(),
            "Artificing".to_string(),
            "Mechanical Engineering".to_string(),
            "Brewing".to_string(),
            "Poison Making".to_string(),
            "Archaeology".to_string(),
            "Scavenging".to_string(),
            "Treasure Hunting".to_string(),
            "Crystal Harvesting".to_string(),
            "Diplomacy".to_string(),
            "Bartering".to_string(),
            "Storytelling".to_string(),
            "Networking".to_string(),
            "Transmutation".to_string(),
            "Summoning".to_string(),
            "Temporal Magic".to_string(),
            "Spatial Magic".to_string(),
            "Spirit Magic".to_string(),
            "Survival Instinct".to_string(),
            "Weather Prediction".to_string(),
            "Camouflage".to_string(),
            "Escape Artist".to_string(),
        ]
    }
}

#[derive(Debug, Clone)]
struct SkillDefinitions {
    pub combat_skills: Vec<String>,
    pub crafting_skills: Vec<String>,
    pub gathering_skills: Vec<String>,
    pub social_skills: Vec<String>,
    pub magic_skills: Vec<String>,
    pub survival_skills: Vec<String>,
}

/// Thread-safe skill migration manager
pub type SharedSkillMigrationManager = Arc<RwLock<SkillMigrationManager>>;

pub fn create_shared_skill_migration_manager(auth_manager: SharedAuthManager, database_manager: SharedDatabaseManager) -> SharedSkillMigrationManager {
    Arc::new(RwLock::new(SkillMigrationManager::new(auth_manager, database_manager)))
}