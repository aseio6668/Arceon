use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::targeting::{TargetingSystem, FoundEntity};
use crate::command_engine::{GameCommand, CommandResult, EntityState};
use crate::trading_system::TradingSystem;
use arceon_core::entities::character_creation::CharacterCreationSystem;

/// Enhanced command processor with targeting and combat support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCommandProcessor {
    pub targeting_system: TargetingSystem,
    pub combat_system: CombatSystem,
    pub chat_system: ChatSystem,
    pub trading_system: TradingSystem,
    pub autoattack_macros: HashMap<Uuid, AutoAttackMacro>,
    pub character_creation: CharacterCreationSystem,
    pub discovered_items: HashMap<Uuid, Vec<String>>, // player_id -> list of discovered item types
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSystem {
    pub combat_states: HashMap<Uuid, CombatState>,
    pub autoattack_enabled: HashMap<Uuid, bool>,
    pub combat_log: Vec<CombatLogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatState {
    pub entity_id: Uuid,
    pub in_combat: bool,
    pub target_id: Option<Uuid>,
    pub health: f64,
    pub max_health: f64,
    pub mana: f64,
    pub max_mana: f64,
    pub energy: f64,
    pub max_energy: f64,
    pub autoattack_enabled: bool,
    pub last_attack_time: i64,
    pub combat_skills: Vec<CombatSkill>,
    pub active_effects: Vec<ActiveEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSkill {
    pub skill_name: String,
    pub skill_level: f64,
    pub cooldown_remaining: f64,
    pub mana_cost: f64,
    pub energy_cost: f64,
    pub damage_base: f64,
    pub can_autocast: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEffect {
    pub effect_name: String,
    pub duration_remaining: f64,
    pub effect_type: EffectType,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    DamageOverTime,
    HealOverTime,
    StatBoost(String),
    StatPenalty(String),
    Stun,
    Slow,
    Haste,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoAttackMacro {
    pub owner_id: Uuid,
    pub skills: Vec<MacroSkill>,
    pub current_index: usize,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroSkill {
    pub skill_name: String,
    pub priority: u32,
    pub conditions: Vec<SkillCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillCondition {
    HealthBelow(f64),
    ManaAbove(f64),
    EnemyCount(u32),
    CooldownReady,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSystem {
    pub chat_history: Vec<ChatMessage>,
    pub private_conversations: HashMap<(Uuid, Uuid), Vec<ChatMessage>>,
    pub npc_speech_patterns: HashMap<Uuid, SpeechPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub recipient_id: Option<Uuid>,
    pub content: String,
    pub chat_type: ChatType,
    pub timestamp: i64,
    pub area_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatType {
    Say,        // Public area chat
    Tell,       // Private message
    Hail,       // Greeting
    Yell,       // Area-wide announcement
    Whisper,    // Very quiet, short range
    Emote,      // Action description
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechPattern {
    pub npc_id: Uuid,
    pub formality_level: f64,
    pub friendliness: f64,
    pub verbosity: f64,
    pub response_patterns: HashMap<String, Vec<String>>,
    pub greeting_phrases: Vec<String>,
    pub farewell_phrases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatLogEntry {
    pub entry_id: Uuid,
    pub timestamp: i64,
    pub attacker_id: Uuid,
    pub defender_id: Uuid,
    pub action: String,
    pub damage: f64,
    pub result: String,
}

impl EnhancedCommandProcessor {
    pub fn new() -> Self {
        Self {
            targeting_system: TargetingSystem::new(),
            combat_system: CombatSystem::new(),
            chat_system: ChatSystem::new(),
            trading_system: TradingSystem::new(),
            autoattack_macros: HashMap::new(),
            character_creation: CharacterCreationSystem::new(),
            discovered_items: HashMap::new(),
        }
    }

    /// Process enhanced commands with targeting support
    pub fn process_command(&mut self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        match command.command_type.as_str() {
            "target" => self.process_target_command(command, entity),
            "hail" => self.process_hail_command(command, entity),
            "say" => self.process_say_command(command, entity),
            "tell" => self.process_tell_command(command, entity),
            "pickup" => self.process_pickup_command(command, entity),
            "trade" => self.process_trade_command(command, entity),
            "look" => self.process_look_command(command, entity),
            "attack" => self.process_attack_command(command, entity),
            "autoattack" => self.process_autoattack_command(command, entity),
            "skill" => self.process_skill_command(command, entity),
            "macro" => self.process_macro_command(command, entity),
            "who" => self.process_who_command(command, entity),
            "clear" => self.process_clear_target_command(command, entity),
            _ => CommandResult::Failure(format!("Unknown enhanced command: {}", command.command_type)),
        }
    }

    /// Process targeting command
    fn process_target_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        if let Some(target_name) = command.parameters.get("name") {
            let area_name = &entity.position.area_name;
            
            if let Some(found_entity) = self.targeting_system.find_entity_by_name(area_name, target_name) {
                match found_entity {
                    FoundEntity::Npc(target_id, _name) | FoundEntity::Player(target_id, _name) => {
                        match self.targeting_system.set_npc_target(entity.id, target_id, area_name) {
                            Ok(message) => CommandResult::Success(message),
                            Err(error) => CommandResult::Failure(error),
                        }
                    },
                    FoundEntity::Object(target_id, _name) => {
                        match self.targeting_system.set_object_target(entity.id, target_id, area_name) {
                            Ok(message) => CommandResult::Success(message),
                            Err(error) => CommandResult::Failure(error),
                        }
                    },
                }
            } else {
                CommandResult::Failure(format!("Could not find '{}' in current area", target_name))
            }
        } else {
            CommandResult::Failure("Usage: target <name>".to_string())
        }
    }

    /// Process hail command (greeting)
    fn process_hail_command(&mut self, _command: &GameCommand, entity: &EntityState) -> CommandResult {
        if let Some(target) = self.targeting_system.get_npc_target(entity.id) {
            if !self.targeting_system.is_within_interaction_range(&entity.position, &target.position) {
                return CommandResult::Failure("Target is too far away to hail".to_string());
            }

            let greeting_message = format!("You hail {}", target.target_name);
            
            // Generate NPC response based on their speech pattern
            let npc_response = self.generate_npc_response(&target.target_id, "hail", entity);
            
            // Record the chat message
            let chat_message = ChatMessage {
                message_id: Uuid::new_v4(),
                sender_id: entity.id,
                sender_name: entity.properties.get("name")
                    .and_then(|p| if let crate::command_engine::PropertyValue::String(s) = p { Some(s.clone()) } else { None })
                    .unwrap_or("Unknown".to_string()),
                recipient_id: Some(target.target_id),
                content: format!("hails {}", target.target_name),
                chat_type: ChatType::Hail,
                timestamp: chrono::Utc::now().timestamp(),
                area_name: entity.position.area_name.clone(),
            };
            
            self.chat_system.chat_history.push(chat_message);
            
            CommandResult::Success(format!("{}\n{}", greeting_message, npc_response))
        } else {
            CommandResult::Failure("No target selected. Use 'target <name>' first".to_string())
        }
    }

    /// Process say command (public chat)
    fn process_say_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        if let Some(message) = command.parameters.get("message") {
            let chat_message = ChatMessage {
                message_id: Uuid::new_v4(),
                sender_id: entity.id,
                sender_name: entity.properties.get("name")
                    .and_then(|p| if let crate::command_engine::PropertyValue::String(s) = p { Some(s.clone()) } else { None })
                    .unwrap_or("Unknown".to_string()),
                recipient_id: None,
                content: message.clone(),
                chat_type: ChatType::Say,
                timestamp: chrono::Utc::now().timestamp(),
                area_name: entity.position.area_name.clone(),
            };
            
            self.chat_system.chat_history.push(chat_message.clone());
            
            // Generate responses from nearby NPCs
            let mut responses = Vec::new();
            let npcs_in_area = self.targeting_system.list_npcs_in_area(&entity.position.area_name);
            
            for npc in npcs_in_area {
                if self.targeting_system.is_within_interaction_range(&entity.position, &npc.position) {
                    let response = self.generate_npc_response(&npc.npc_id, "say", entity);
                    if !response.is_empty() {
                        responses.push(format!("{}: {}", npc.name, response));
                    }
                }
            }
            
            let mut result = format!("You say: {}", message);
            if !responses.is_empty() {
                result.push_str("\n");
                result.push_str(&responses.join("\n"));
            }
            
            CommandResult::Success(result)
        } else {
            CommandResult::Failure("Usage: say <message>".to_string())
        }
    }

    /// Process tell command (private message)
    fn process_tell_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        if let Some(target_name) = command.parameters.get("target") {
            if let Some(message) = command.parameters.get("message") {
                let area_name = &entity.position.area_name;
                
                if let Some(found_entity) = self.targeting_system.find_entity_by_name(area_name, target_name) {
                    let (target_id, target_full_name) = match &found_entity {
                        FoundEntity::Npc(id, name) | FoundEntity::Player(id, name) => (*id, name.clone()),
                        FoundEntity::Object(_, _) => {
                            return CommandResult::Failure("Cannot send private message to an object".to_string());
                        }
                    };
                    
                    let chat_message = ChatMessage {
                        message_id: Uuid::new_v4(),
                        sender_id: entity.id,
                        sender_name: entity.properties.get("name")
                            .and_then(|p| if let crate::command_engine::PropertyValue::String(s) = p { Some(s.clone()) } else { None })
                            .unwrap_or("Unknown".to_string()),
                        recipient_id: Some(target_id),
                        content: message.clone(),
                        chat_type: ChatType::Tell,
                        timestamp: chrono::Utc::now().timestamp(),
                        area_name: entity.position.area_name.clone(),
                    };
                    
                    // Add to private conversation history
                    let conversation_key = (entity.id, target_id);
                    self.chat_system.private_conversations
                        .entry(conversation_key)
                        .or_insert_with(Vec::new)
                        .push(chat_message.clone());
                    
                    // Generate NPC response for private tells
                    let npc_response = if matches!(found_entity, FoundEntity::Npc(_, _)) {
                        self.generate_npc_response(&target_id, "tell", entity)
                    } else {
                        String::new()
                    };
                    
                    let mut result = format!("You tell {}: {}", target_full_name, message);
                    if !npc_response.is_empty() {
                        result.push_str(&format!("\n{} tells you: {}", target_full_name, npc_response));
                    }
                    
                    CommandResult::Success(result)
                } else {
                    CommandResult::Failure(format!("Could not find '{}' to send message to", target_name))
                }
            } else {
                CommandResult::Failure("Usage: tell <target> <message>".to_string())
            }
        } else {
            CommandResult::Failure("Usage: tell <target> <message>".to_string())
        }
    }

    /// Process pickup command
    fn process_pickup_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        if let Some(target) = self.targeting_system.get_object_target(entity.id).cloned() {
            if !self.targeting_system.is_within_interaction_range(&entity.position, &target.position) {
                return CommandResult::Failure("Object is too far away to pick up".to_string());
            }
            
            if !target.can_pickup {
                return CommandResult::Failure(format!("{} cannot be picked up", target.target_name));
            }
            
            // Check if this is a new item type discovery and unlock skills
            let mut messages = vec![format!("You pick up {}", target.target_name)];
            
            // Determine item type from target name and unlock skills
            let item_type = self.determine_item_type(&target.target_name);
            let unlocked_skills = self.check_and_unlock_skills_for_item(entity.id, &item_type);
            
            if !unlocked_skills.is_empty() {
                messages.push(format!("Discovering {} unlocks new skills: {}", 
                    target.target_name, unlocked_skills.join(", ")));
                messages.push("You feel your knowledge expand as you understand new combat techniques!".to_string());
            }
            
            // TODO: Add item to player inventory
            // TODO: Remove item from world
            
            CommandResult::Success(messages.join("\n"))
        } else {
            // Try to find item by name if no target is set
            if let Some(item_name) = command.parameters.get("item") {
                if let Some(found_entity) = self.targeting_system.find_entity_by_name(&entity.position.area_name, item_name) {
                    if let FoundEntity::Object(object_id, _name) = found_entity {
                        match self.targeting_system.set_object_target(entity.id, object_id, &entity.position.area_name) {
                            Ok(_) => self.process_pickup_command(command, entity),
                            Err(error) => CommandResult::Failure(error),
                        }
                    } else {
                        CommandResult::Failure("That is not an item you can pick up".to_string())
                    }
                } else {
                    CommandResult::Failure(format!("Could not find item '{}'", item_name))
                }
            } else {
                CommandResult::Failure("No object targeted. Use 'target <object>' or 'pickup <item_name>'".to_string())
            }
        }
    }

    /// Process trade command (player-to-player trading)
    fn process_trade_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        self.trading_system.process_trade_command(command, entity)
    }

    /// Process look command (examine area or target)
    fn process_look_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        if let Some(target_name) = command.parameters.get("target") {
            // Look at specific target
            if let Some(found_entity) = self.targeting_system.find_entity_by_name(&entity.position.area_name, target_name) {
                match found_entity {
                    FoundEntity::Npc(_, name) => {
                        CommandResult::Success(format!("You examine {}.\nThis is an NPC who appears to be going about their daily activities.", name))
                    },
                    FoundEntity::Player(_, name) => {
                        CommandResult::Success(format!("You examine {}.\nThis is another player in the world.", name))
                    },
                    FoundEntity::Object(_, name) => {
                        CommandResult::Success(format!("You examine {}.\nThis appears to be a useful object.", name))
                    },
                }
            } else {
                CommandResult::Failure(format!("Could not find '{}'", target_name))
            }
        } else {
            // Look at current area
            let area_name = &entity.position.area_name;
            let npcs = self.targeting_system.list_npcs_in_area(area_name);
            let objects = self.targeting_system.list_objects_in_area(area_name);
            let players = self.targeting_system.list_players_in_area(area_name);
            
            let mut description = format!("You are in {}.\n", area_name);
            
            if !npcs.is_empty() {
                description.push_str("NPCs here:\n");
                for npc in npcs {
                    description.push_str(&format!("  {} ({})\n", npc.name, npc.description));
                }
            }
            
            if !players.is_empty() {
                description.push_str("Players here:\n");
                for player in players {
                    if player.player_id != entity.id {
                        description.push_str(&format!("  {}\n", player.name));
                    }
                }
            }
            
            if !objects.is_empty() {
                description.push_str("Objects here:\n");
                for object in objects {
                    description.push_str(&format!("  {} ({})\n", object.name, object.description));
                }
            }
            
            CommandResult::Success(description)
        }
    }

    /// Process attack command
    fn process_attack_command(&mut self, _command: &GameCommand, entity: &EntityState) -> CommandResult {
        if let Some(target) = self.targeting_system.get_npc_target(entity.id) {
            if !target.can_attack {
                return CommandResult::Failure(format!("{} cannot be attacked", target.target_name));
            }
            
            if !self.targeting_system.is_within_attack_range(&entity.position, &target.position) {
                return CommandResult::Failure("Target is too far away to attack".to_string());
            }
            
            // Initialize combat state if not already in combat
            self.combat_system.start_combat(entity.id, target.target_id);
            
            let damage = self.calculate_attack_damage(entity.id);
            
            let log_entry = CombatLogEntry {
                entry_id: Uuid::new_v4(),
                timestamp: chrono::Utc::now().timestamp(),
                attacker_id: entity.id,
                defender_id: target.target_id,
                action: "basic_attack".to_string(),
                damage,
                result: format!("Hit for {} damage", damage),
            };
            
            self.combat_system.combat_log.push(log_entry);
            
            CommandResult::Success(format!("You attack {} for {:.1} damage!", target.target_name, damage))
        } else {
            CommandResult::Failure("No target selected for attack. Use 'target <name>' first".to_string())
        }
    }

    /// Process autoattack toggle command
    fn process_autoattack_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let enable = command.parameters.get("enable")
            .map(|s| s.to_lowercase() == "on" || s.to_lowercase() == "true")
            .unwrap_or_else(|| !self.combat_system.autoattack_enabled.get(&entity.id).unwrap_or(&false));
        
        self.combat_system.autoattack_enabled.insert(entity.id, enable);
        
        if enable {
            CommandResult::Success("Autoattack enabled".to_string())
        } else {
            CommandResult::Success("Autoattack disabled".to_string())
        }
    }

    /// Process skill command
    fn process_skill_command(&mut self, command: &GameCommand, _entity: &EntityState) -> CommandResult {
        if let Some(skill_name) = command.parameters.get("name") {
            // TODO: Integrate with skill system
            CommandResult::Success(format!("You attempt to use skill: {}", skill_name))
        } else {
            CommandResult::Failure("Usage: skill <skill_name>".to_string())
        }
    }

    /// Process macro command
    fn process_macro_command(&mut self, _command: &GameCommand, _entity: &EntityState) -> CommandResult {
        // TODO: Implement skill macro system
        CommandResult::Success("Macro system not yet implemented".to_string())
    }

    /// Process who command (list players and NPCs in area)
    fn process_who_command(&mut self, _command: &GameCommand, entity: &EntityState) -> CommandResult {
        let area_name = &entity.position.area_name;
        let npcs = self.targeting_system.list_npcs_in_area(area_name);
        let players = self.targeting_system.list_players_in_area(area_name);
        
        let mut result = format!("Entities in {}:\n", area_name);
        
        if !players.is_empty() {
            result.push_str("Players:\n");
            for player in players {
                if player.player_id != entity.id {
                    result.push_str(&format!("  {} ({})\n", player.name, format!("{:?}", player.status)));
                }
            }
        }
        
        if !npcs.is_empty() {
            result.push_str("NPCs:\n");
            for npc in npcs {
                result.push_str(&format!("  {} ({:?})\n", npc.name, npc.npc_type));
            }
        }
        
        CommandResult::Success(result)
    }

    /// Process clear target command
    fn process_clear_target_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let target_type = command.parameters.get("type").map(|s| s.as_str()).unwrap_or("npc");
        
        match target_type {
            "npc" | "player" => {
                let message = self.targeting_system.clear_npc_target(entity.id);
                CommandResult::Success(message)
            },
            "object" | "item" => {
                let message = self.targeting_system.clear_object_target(entity.id);
                CommandResult::Success(message)
            },
            "all" => {
                let npc_msg = self.targeting_system.clear_npc_target(entity.id);
                let obj_msg = self.targeting_system.clear_object_target(entity.id);
                CommandResult::Success(format!("{}\n{}", npc_msg, obj_msg))
            },
            _ => CommandResult::Failure("Usage: clear [npc|object|all]".to_string()),
        }
    }

    /// Generate NPC response based on speech patterns and context
    fn generate_npc_response(&self, npc_id: &Uuid, context: &str, entity: &EntityState) -> String {
        if let Some(speech_pattern) = self.chat_system.npc_speech_patterns.get(npc_id) {
            match context {
                "hail" => {
                    if !speech_pattern.greeting_phrases.is_empty() {
                        let greeting = &speech_pattern.greeting_phrases[0];
                        format!("{} greets you: {}", 
                            entity.properties.get("name")
                                .and_then(|p| if let crate::command_engine::PropertyValue::String(s) = p { Some(s) } else { None })
                                .unwrap_or(&"Someone".to_string()), 
                            greeting)
                    } else {
                        "The NPC nods in acknowledgment".to_string()
                    }
                },
                "say" => {
                    if rand::random::<f64>() < speech_pattern.friendliness {
                        "The NPC listens with interest".to_string()
                    } else {
                        String::new()
                    }
                },
                "tell" => {
                    "The NPC considers your words carefully".to_string()
                },
                _ => String::new(),
            }
        } else {
            // Default responses for NPCs without speech patterns
            match context {
                "hail" => "The NPC acknowledges your greeting".to_string(),
                "tell" => "The NPC listens to your private message".to_string(),
                _ => String::new(),
            }
        }
    }

    /// Calculate attack damage based on entity's combat stats
    fn calculate_attack_damage(&self, _entity_id: Uuid) -> f64 {
        // TODO: Integrate with actual skill system
        // For now, return a basic damage value
        10.0 + rand::random::<f64>() * 5.0
    }
    
    /// Determine item type based on item name patterns
    fn determine_item_type(&self, item_name: &str) -> String {
        let item_name_lower = item_name.to_lowercase();
        
        // Map item names to types based on common patterns
        if item_name_lower.contains("sword") || item_name_lower.contains("blade") || item_name_lower.contains("rapier") {
            "sword".to_string()
        } else if item_name_lower.contains("club") || item_name_lower.contains("mace") || item_name_lower.contains("cudgel") {
            "club".to_string()
        } else if item_name_lower.contains("hammer") || item_name_lower.contains("maul") {
            "hammer".to_string()
        } else if item_name_lower.contains("dagger") || item_name_lower.contains("knife") {
            "dagger".to_string()
        } else if item_name_lower.contains("knuckles") || item_name_lower.contains("gauntlet") {
            "knuckles".to_string()
        } else if item_name_lower.contains("polearm") || item_name_lower.contains("spear") || item_name_lower.contains("pike") {
            "polearm".to_string()
        } else if item_name_lower.contains("stave") || item_name_lower.contains("staff") {
            "stave".to_string()
        } else if item_name_lower.contains("bow") && !item_name_lower.contains("crossbow") {
            "bow".to_string()
        } else if item_name_lower.contains("crossbow") {
            "crossbow".to_string()
        } else if item_name_lower.contains("shield") {
            "shield".to_string()
        } else if item_name_lower.contains("pickaxe") || item_name_lower.contains("pick") {
            "pickaxe".to_string()
        } else if item_name_lower.contains("forge") || item_name_lower.contains("smithy") {
            "forge".to_string()
        } else if item_name_lower.contains("anvil") {
            "anvil".to_string()
        } else if item_name_lower.contains("saw") {
            "saw".to_string()
        } else if item_name_lower.contains("chisel") {
            "chisel".to_string()
        } else if item_name_lower.contains("needle") {
            "needle".to_string()
        } else if item_name_lower.contains("loom") {
            "loom".to_string()
        } else if item_name_lower.contains("pottery") && item_name_lower.contains("wheel") {
            "pottery_wheel".to_string()
        } else if item_name_lower.contains("spellbook") || (item_name_lower.contains("book") && item_name_lower.contains("magic")) {
            "spellbook".to_string()
        } else if item_name_lower.contains("scroll") {
            "scroll".to_string()
        } else if item_name_lower.contains("tome") {
            "tome".to_string()
        } else if item_name_lower.contains("lute") {
            "lute".to_string()
        } else if item_name_lower.contains("flute") {
            "flute".to_string()
        } else if item_name_lower.contains("drum") {
            "drum".to_string()
        } else {
            "unknown".to_string()
        }
    }
    
    /// Check if entity has discovered this item type before and unlock skills if new
    fn check_and_unlock_skills_for_item(&mut self, entity_id: Uuid, item_type: &str) -> Vec<String> {
        let mut unlocked_skills = Vec::new();
        
        // Check if player has discovered this item type before
        let player_discovered = self.discovered_items.entry(entity_id).or_insert_with(Vec::new);
        
        if !player_discovered.contains(&item_type.to_string()) && item_type != "unknown" {
            // This is a new item type discovery - unlock related skills
            if let Some(skill_name) = self.character_creation.item_type_skills.get(item_type) {
                unlocked_skills.push(skill_name.clone());
                player_discovered.push(item_type.to_string());
                
                // TODO: Actually unlock the skill in the entity's skill system
                // For now we just track the discovery and report the unlocked skill
            }
        }
        
        unlocked_skills
    }
}

impl CombatSystem {
    pub fn new() -> Self {
        Self {
            combat_states: HashMap::new(),
            autoattack_enabled: HashMap::new(),
            combat_log: Vec::new(),
        }
    }

    pub fn start_combat(&mut self, attacker_id: Uuid, defender_id: Uuid) {
        // Initialize combat state for attacker
        if !self.combat_states.contains_key(&attacker_id) {
            let combat_state = CombatState {
                entity_id: attacker_id,
                in_combat: true,
                target_id: Some(defender_id),
                health: 100.0,
                max_health: 100.0,
                mana: 50.0,
                max_mana: 50.0,
                energy: 100.0,
                max_energy: 100.0,
                autoattack_enabled: false,
                last_attack_time: chrono::Utc::now().timestamp(),
                combat_skills: Vec::new(),
                active_effects: Vec::new(),
            };
            self.combat_states.insert(attacker_id, combat_state);
        }
    }
}

impl ChatSystem {
    pub fn new() -> Self {
        Self {
            chat_history: Vec::new(),
            private_conversations: HashMap::new(),
            npc_speech_patterns: HashMap::new(),
        }
    }
}

