use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::genesis::{Position, MovementCommand, Direction};

/// Command-based game engine that drives all entity interactions through text commands
/// Graphics are purely visualization - all game logic happens at command level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEngine {
    pub entities: HashMap<Uuid, EntityState>,
    pub command_history: Vec<ExecutedCommand>,
    pub world_state: CommandWorldState,
    pub command_processors: HashMap<String, CommandProcessor>,
}

/// Everything that exists in the world has a command-driven state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub position: Position,
    pub properties: HashMap<String, PropertyValue>,
    pub capabilities: Vec<EntityCapability>,
    pub command_queue: Vec<PendingCommand>,
    pub last_command_result: Option<CommandResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Npc,
    Player,
    Building,
    Item,
    TerrainFeature,
    Runestone,
    TeleportationRing,
    Wildlife,
    Spirit,
    Deity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Vector(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityCapability {
    CanMove,
    CanCraft,
    CanCommunicate,
    CanCast,
    CanTrade,
    CanBuild,
    CanTeach,
    CanLearn,
    CanFight,
    CanHeal,
    CanTeleport,
    CanShapeshift,
}

/// All player and NPC actions are expressed as commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCommand {
    pub command_id: Uuid,
    pub entity_id: Uuid,
    pub command_type: String,
    pub parameters: HashMap<String, String>,
    pub timestamp: i64,
    pub priority: u32,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingCommand {
    pub command: GameCommand,
    pub execution_time: i64,
    pub dependencies: Vec<Uuid>, // Other commands that must complete first
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedCommand {
    pub command: GameCommand,
    pub result: CommandResult,
    pub execution_start: i64,
    pub execution_end: i64,
    pub side_effects: Vec<SideEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandResult {
    Success(String),
    Failure(String),
    Partial(String, f32), // Description and completion percentage
    Queued(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideEffect {
    pub effect_type: String,
    pub affected_entities: Vec<Uuid>,
    pub description: String,
    pub magnitude: f64,
}

/// Movement commands - all movement happens through the command system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementProcessor;

impl MovementProcessor {
    /// Process movement commands like "move north 10 steps" or "directmove forward for 5.24 seconds"
    pub fn process_movement(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        match command.command_type.as_str() {
            "move" => self.process_basic_movement(command, entity),
            "directmove" => self.process_direct_movement(command, entity),
            "teleport" => self.process_teleportation(command, entity),
            "climb" => self.process_climbing(command, entity),
            "swim" => self.process_swimming(command, entity),
            "fly" => self.process_flying(command, entity),
            _ => CommandResult::Failure(format!("Unknown movement command: {}", command.command_type)),
        }
    }

    fn process_basic_movement(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        let direction = command.parameters.get("direction")
            .and_then(|d| self.parse_direction(d));
        let distance = command.parameters.get("distance")
            .and_then(|d| d.parse::<f64>().ok())
            .unwrap_or(1.0);

        if let Some(dir) = direction {
            let (dx, dy) = self.direction_to_delta(&dir);
            let new_x = entity.position.x + dx * distance;
            let new_y = entity.position.y + dy * distance;

            // Validate movement is possible
            if self.validate_position(new_x, new_y, entity.position.z, &entity.position.landmass_id) {
                entity.position.x = new_x;
                entity.position.y = new_y;
                
                let movement_command = MovementCommand {
                    command_type: "move".to_string(),
                    direction: Some(dir),
                    duration: 1.0,
                    distance,
                    from_position: (entity.position.x - dx * distance, entity.position.y - dy * distance, entity.position.z),
                    to_position: (new_x, new_y, entity.position.z),
                    timestamp: chrono::Utc::now().timestamp(),
                    success: true,
                };
                
                entity.position.command_history.push(movement_command);
                CommandResult::Success(format!("Moved {} {} units", direction_to_string(&dir), distance))
            } else {
                CommandResult::Failure("Movement blocked by terrain or obstacles".to_string())
            }
        } else {
            CommandResult::Failure("Invalid direction specified".to_string())
        }
    }

    fn process_direct_movement(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        // Handle "directmove forward for 5.24 seconds" type commands
        let direction = command.parameters.get("direction")
            .and_then(|d| self.parse_direction(d))
            .unwrap_or(Direction::Forward);
        
        let duration = command.parameters.get("duration")
            .and_then(|d| d.parse::<f64>().ok())
            .unwrap_or(1.0);

        // Calculate movement based on entity speed and duration
        let speed = entity.properties.get("movement_speed")
            .and_then(|s| match s {
                PropertyValue::Float(speed) => Some(*speed),
                _ => None,
            })
            .unwrap_or(1.0);

        let distance = speed * duration;
        let (dx, dy) = self.direction_to_delta(&direction);
        
        let new_x = entity.position.x + dx * distance;
        let new_y = entity.position.y + dy * distance;

        if self.validate_position(new_x, new_y, entity.position.z, &entity.position.landmass_id) {
            entity.position.x = new_x;
            entity.position.y = new_y;
            
            let movement_command = MovementCommand {
                command_type: "directmove".to_string(),
                direction: Some(direction),
                duration,
                distance,
                from_position: (entity.position.x - dx * distance, entity.position.y - dy * distance, entity.position.z),
                to_position: (new_x, new_y, entity.position.z),
                timestamp: chrono::Utc::now().timestamp(),
                success: true,
            };
            
            entity.position.command_history.push(movement_command);
            CommandResult::Success(format!("Direct movement {} for {:.2} seconds, distance {:.2}", 
                                         direction_to_string(&direction), duration, distance))
        } else {
            CommandResult::Failure("Direct movement blocked".to_string())
        }
    }

    fn process_teleportation(&self, _command: &GameCommand, _entity: &mut EntityState) -> CommandResult {
        CommandResult::Failure("Teleportation requires magical knowledge".to_string())
    }

    fn process_climbing(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        let height = command.parameters.get("height")
            .and_then(|h| h.parse::<f64>().ok())
            .unwrap_or(1.0);

        // Check if entity has climbing capability
        if entity.capabilities.contains(&EntityCapability::CanMove) {
            entity.position.z += height;
            CommandResult::Success(format!("Climbed {:.2} units up", height))
        } else {
            CommandResult::Failure("Entity cannot climb".to_string())
        }
    }

    fn process_swimming(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        // Check if location has water
        if self.has_water_at_position(&entity.position) {
            let direction = command.parameters.get("direction")
                .and_then(|d| self.parse_direction(d))
                .unwrap_or(Direction::Forward);
            
            let distance = command.parameters.get("distance")
                .and_then(|d| d.parse::<f64>().ok())
                .unwrap_or(1.0);

            let (dx, dy) = self.direction_to_delta(&direction);
            entity.position.x += dx * distance;
            entity.position.y += dy * distance;
            
            CommandResult::Success(format!("Swam {} {:.2} units", direction_to_string(&direction), distance))
        } else {
            CommandResult::Failure("No water present for swimming".to_string())
        }
    }

    fn process_flying(&self, _command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        if entity.capabilities.contains(&EntityCapability::CanCast) {
            CommandResult::Success("Flight magic activated".to_string())
        } else {
            CommandResult::Failure("Entity lacks magical flight capability".to_string())
        }
    }

    fn parse_direction(&self, direction_str: &str) -> Option<Direction> {
        match direction_str.to_lowercase().as_str() {
            "north" | "n" => Some(Direction::North),
            "south" | "s" => Some(Direction::South),
            "east" | "e" => Some(Direction::East),
            "west" | "w" => Some(Direction::West),
            "northeast" | "ne" => Some(Direction::Northeast),
            "northwest" | "nw" => Some(Direction::Northwest),
            "southeast" | "se" => Some(Direction::Southeast),
            "southwest" | "sw" => Some(Direction::Southwest),
            "up" | "u" => Some(Direction::Up),
            "down" | "d" => Some(Direction::Down),
            "forward" | "f" => Some(Direction::Forward),
            "backward" | "back" | "b" => Some(Direction::Backward),
            "left" | "l" => Some(Direction::Left),
            "right" | "r" => Some(Direction::Right),
            _ => None,
        }
    }

    fn direction_to_delta(&self, direction: &Direction) -> (f64, f64) {
        match direction {
            Direction::North => (0.0, 1.0),
            Direction::South => (0.0, -1.0),
            Direction::East => (1.0, 0.0),
            Direction::West => (-1.0, 0.0),
            Direction::Northeast => (0.707, 0.707),
            Direction::Northwest => (-0.707, 0.707),
            Direction::Southeast => (0.707, -0.707),
            Direction::Southwest => (-0.707, -0.707),
            Direction::Forward => (0.0, 1.0), // Assumes facing north
            Direction::Backward => (0.0, -1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
            Direction::Up | Direction::Down => (0.0, 0.0), // Vertical movement
        }
    }

    fn validate_position(&self, _x: f64, _y: f64, _z: f64, _landmass_id: &Uuid) -> bool {
        // TODO: Implement terrain checking, boundary validation, etc.
        true
    }

    fn has_water_at_position(&self, _position: &Position) -> bool {
        // TODO: Check terrain data for water bodies
        true
    }
}

/// Interaction commands - NPCs and players interact through commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionProcessor;

impl InteractionProcessor {
    pub fn process_interaction(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        match command.command_type.as_str() {
            "examine" => self.process_examine(command, entity),
            "talk" => self.process_talk(command, entity),
            "trade" => self.process_trade(command, entity),
            "craft" => self.process_craft(command, entity),
            "build" => self.process_build(command, entity),
            "cast" => self.process_cast_spell(command, entity),
            "teach" => self.process_teach(command, entity),
            "learn" => self.process_learn(command, entity),
            "activate_runestone" => self.process_runestone_activation(command, entity),
            "study_ring" => self.process_ring_study(command, entity),
            _ => CommandResult::Failure(format!("Unknown interaction command: {}", command.command_type)),
        }
    }

    fn process_examine(&self, command: &GameCommand, _entity: &mut EntityState) -> CommandResult {
        let default_target = "nothing".to_string();
        let target = command.parameters.get("target")
            .unwrap_or(&default_target);
        
        CommandResult::Success(format!("You examine the {}. It appears to be ordinary.", target))
    }

    fn process_talk(&self, command: &GameCommand, _entity: &mut EntityState) -> CommandResult {
        let default_target = "air".to_string();
        let default_message = "hello".to_string();
        let target = command.parameters.get("target")
            .unwrap_or(&default_target);
        let message = command.parameters.get("message")
            .unwrap_or(&default_message);
        
        CommandResult::Success(format!("You say '{}' to {}", message, target))
    }

    fn process_trade(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        if entity.capabilities.contains(&EntityCapability::CanTrade) {
            let default_item = "nothing".to_string();
            let item = command.parameters.get("item").unwrap_or(&default_item);
            CommandResult::Success(format!("Attempting to trade {}", item))
        } else {
            CommandResult::Failure("Entity cannot trade".to_string())
        }
    }

    fn process_craft(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        if entity.capabilities.contains(&EntityCapability::CanCraft) {
            let default_item = "basic tool".to_string();
            let item = command.parameters.get("item").unwrap_or(&default_item);
            CommandResult::Success(format!("Crafting {}", item))
        } else {
            CommandResult::Failure("Entity lacks crafting capability".to_string())
        }
    }

    fn process_build(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        if entity.capabilities.contains(&EntityCapability::CanBuild) {
            let default_structure = "basic shelter".to_string();
            let structure = command.parameters.get("structure").unwrap_or(&default_structure);
            CommandResult::Success(format!("Building {}", structure))
        } else {
            CommandResult::Failure("Entity lacks building capability".to_string())
        }
    }

    fn process_cast_spell(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        if entity.capabilities.contains(&EntityCapability::CanCast) {
            let default_spell = "basic cantrip".to_string();
            let spell = command.parameters.get("spell").unwrap_or(&default_spell);
            CommandResult::Success(format!("Casting {}", spell))
        } else {
            CommandResult::Failure("Entity lacks magical capability".to_string())
        }
    }

    fn process_teach(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        if entity.capabilities.contains(&EntityCapability::CanTeach) {
            let default_skill = "basic knowledge".to_string();
            let default_target = "someone".to_string();
            let skill = command.parameters.get("skill").unwrap_or(&default_skill);
            let target = command.parameters.get("target").unwrap_or(&default_target);
            CommandResult::Success(format!("Teaching {} to {}", skill, target))
        } else {
            CommandResult::Failure("Entity lacks teaching capability".to_string())
        }
    }

    fn process_learn(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        if entity.capabilities.contains(&EntityCapability::CanLearn) {
            let default_subject = "general knowledge".to_string();
            let subject = command.parameters.get("subject").unwrap_or(&default_subject);
            CommandResult::Success(format!("Learning about {}", subject))
        } else {
            CommandResult::Failure("Entity lacks learning capability".to_string())
        }
    }

    fn process_runestone_activation(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        let runestone_id = command.parameters.get("runestone_id");
        if let Some(_id) = runestone_id {
            CommandResult::Success("Attempting to activate runestone".to_string())
        } else {
            CommandResult::Failure("No runestone specified".to_string())
        }
    }

    fn process_ring_study(&self, command: &GameCommand, entity: &mut EntityState) -> CommandResult {
        let hours = command.parameters.get("hours")
            .and_then(|h| h.parse::<f64>().ok())
            .unwrap_or(1.0);
        
        CommandResult::Success(format!("Studying teleportation ring for {:.1} hours", hours))
    }
}

/// World state tracked through command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandWorldState {
    pub total_commands_executed: u64,
    pub active_entities: u32,
    pub pending_commands: u32,
    pub world_time: i64,
    pub command_performance: HashMap<String, CommandPerformance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPerformance {
    pub command_type: String,
    pub total_executions: u64,
    pub average_duration_ms: f64,
    pub success_rate: f32,
}

/// Command processor registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandProcessor {
    pub processor_type: String,
    pub supported_commands: Vec<String>,
    pub enabled: bool,
}

impl CommandEngine {
    pub fn new() -> Self {
        let mut processors = HashMap::new();
        
        processors.insert("movement".to_string(), CommandProcessor {
            processor_type: "movement".to_string(),
            supported_commands: vec![
                "move".to_string(),
                "directmove".to_string(),
                "teleport".to_string(),
                "climb".to_string(),
                "swim".to_string(),
                "fly".to_string(),
            ],
            enabled: true,
        });

        processors.insert("interaction".to_string(), CommandProcessor {
            processor_type: "interaction".to_string(),
            supported_commands: vec![
                "examine".to_string(),
                "talk".to_string(),
                "trade".to_string(),
                "craft".to_string(),
                "build".to_string(),
                "cast".to_string(),
                "teach".to_string(),
                "learn".to_string(),
                "activate_runestone".to_string(),
                "study_ring".to_string(),
            ],
            enabled: true,
        });

        Self {
            entities: HashMap::new(),
            command_history: Vec::new(),
            world_state: CommandWorldState {
                total_commands_executed: 0,
                active_entities: 0,
                pending_commands: 0,
                world_time: chrono::Utc::now().timestamp(),
                command_performance: HashMap::new(),
            },
            command_processors: processors,
        }
    }

    /// Execute a command and update world state
    pub fn execute_command(&mut self, command: GameCommand) -> CommandResult {
        let start_time = chrono::Utc::now().timestamp_millis();
        
        let result = if let Some(entity) = self.entities.get_mut(&command.entity_id) {
            match command.command_type.as_str() {
                "move" | "directmove" | "teleport" | "climb" | "swim" | "fly" => {
                    let processor = MovementProcessor;
                    processor.process_movement(&command, entity)
                }
                "examine" | "talk" | "trade" | "craft" | "build" | "cast" | "teach" | "learn" | "activate_runestone" | "study_ring" => {
                    let processor = InteractionProcessor;
                    processor.process_interaction(&command, entity)
                }
                _ => CommandResult::Failure(format!("Unknown command type: {}", command.command_type)),
            }
        } else {
            CommandResult::Failure("Entity not found".to_string())
        };

        let end_time = chrono::Utc::now().timestamp_millis();
        let duration = (end_time - start_time) as f64;

        // Record command execution
        let executed_command = ExecutedCommand {
            command: command.clone(),
            result: result.clone(),
            execution_start: start_time,
            execution_end: end_time,
            side_effects: Vec::new(), // TODO: Calculate side effects
        };

        self.command_history.push(executed_command);
        self.world_state.total_commands_executed += 1;

        // Update performance metrics
        self.update_command_performance(&command.command_type, duration, &result);

        result
    }

    fn update_command_performance(&mut self, command_type: &str, duration: f64, result: &CommandResult) {
        let performance = self.world_state.command_performance
            .entry(command_type.to_string())
            .or_insert(CommandPerformance {
                command_type: command_type.to_string(),
                total_executions: 0,
                average_duration_ms: 0.0,
                success_rate: 0.0,
            });

        performance.total_executions += 1;
        performance.average_duration_ms = (performance.average_duration_ms * (performance.total_executions - 1) as f64 + duration) / performance.total_executions as f64;
        
        let is_success = matches!(result, CommandResult::Success(_));
        performance.success_rate = (performance.success_rate * (performance.total_executions - 1) as f32 + if is_success { 1.0 } else { 0.0 }) / performance.total_executions as f32;
    }

    /// Parse text commands into structured GameCommand objects
    pub fn parse_text_command(&self, entity_id: Uuid, command_text: &str) -> Result<GameCommand, String> {
        let parts: Vec<&str> = command_text.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        let command_type = parts[0].to_lowercase();
        let mut parameters = HashMap::new();

        match command_type.as_str() {
            "tell" => {
                if parts.len() >= 3 {
                    let (target_name, message) = self.parse_tell_command(&parts[1..])?;
                    parameters.insert("target".to_string(), target_name);
                    parameters.insert("message".to_string(), message);
                }
            }
            "say" => {
                if parts.len() >= 2 {
                    parameters.insert("message".to_string(), parts[1..].join(" "));
                }
            }
            "move" => {
                if parts.len() >= 2 {
                    parameters.insert("direction".to_string(), parts[1].to_string());
                    if parts.len() >= 3 {
                        parameters.insert("distance".to_string(), parts[2].to_string());
                    }
                }
            }
            "directmove" => {
                if parts.len() >= 4 && parts[1] == "forward" && parts[2] == "for" {
                    parameters.insert("direction".to_string(), "forward".to_string());
                    parameters.insert("duration".to_string(), parts[3].to_string());
                }
            }
            "examine" | "talk" => {
                if parts.len() >= 2 {
                    parameters.insert("target".to_string(), parts[1..].join(" "));
                }
            }
            "craft" | "build" => {
                if parts.len() >= 2 {
                    parameters.insert("item".to_string(), parts[1..].join(" "));
                }
            }
            _ => {
                // Generic parameter parsing
                for i in (1..parts.len()).step_by(2) {
                    if i + 1 < parts.len() {
                        parameters.insert(parts[i].to_string(), parts[i + 1].to_string());
                    }
                }
            }
        }

        Ok(GameCommand {
            command_id: Uuid::new_v4(),
            entity_id,
            command_type,
            parameters,
            timestamp: chrono::Utc::now().timestamp(),
            priority: 1,
            requires_confirmation: false,
        })
    }

    /// Parse tell command with support for quoted names and messages
    /// Examples: tell 'Eldara the Wise' Hello there
    ///           tell "Bob Smith" "How are you today?"
    ///           tell John Hello
    fn parse_tell_command(&self, parts: &[&str]) -> Result<(String, String), String> {
        if parts.is_empty() {
            return Err("Tell command requires a target and message".to_string());
        }

        let input = parts.join(" ");
        let (target_name, remaining) = self.parse_quoted_or_first_word(&input)?;
        
        if remaining.trim().is_empty() {
            return Err("Tell command requires a message".to_string());
        }

        let message = remaining.trim().to_string();
        Ok((target_name, message))
    }

    /// Parse a quoted string or first word from input
    /// Returns (parsed_string, remaining_input)
    fn parse_quoted_or_first_word(&self, input: &str) -> Result<(String, String), String> {
        let input = input.trim();
        if input.is_empty() {
            return Err("Empty input".to_string());
        }

        // Check for quoted strings (both single and double quotes)
        if input.starts_with('"') {
            if let Some(end_quote) = input[1..].find('"') {
                let quoted_content = input[1..end_quote + 1].to_string();
                let remaining = input[end_quote + 2..].to_string();
                return Ok((quoted_content, remaining));
            } else {
                return Err("Unterminated double quote".to_string());
            }
        } else if input.starts_with('\'') {
            if let Some(end_quote) = input[1..].find('\'') {
                let quoted_content = input[1..end_quote + 1].to_string();
                let remaining = input[end_quote + 2..].to_string();
                return Ok((quoted_content, remaining));
            } else {
                return Err("Unterminated single quote".to_string());
            }
        } else {
            // No quotes, just take the first word
            let words: Vec<&str> = input.split_whitespace().collect();
            if words.is_empty() {
                return Err("No content found".to_string());
            }
            let first_word = words[0].to_string();
            let remaining = words[1..].join(" ");
            Ok((first_word, remaining))
        }
    }
}

fn direction_to_string(direction: &Direction) -> &'static str {
    match direction {
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
        Direction::Northeast => "northeast",
        Direction::Northwest => "northwest",
        Direction::Southeast => "southeast",
        Direction::Southwest => "southwest",
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Forward => "forward",
        Direction::Backward => "backward",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let engine = CommandEngine::new();
        let entity_id = Uuid::new_v4();
        
        let command = engine.parse_text_command(entity_id, "move north 5").unwrap();
        assert_eq!(command.command_type, "move");
        assert_eq!(command.parameters.get("direction"), Some(&"north".to_string()));
        assert_eq!(command.parameters.get("distance"), Some(&"5".to_string()));
    }

    #[test]
    fn test_directmove_parsing() {
        let engine = CommandEngine::new();
        let entity_id = Uuid::new_v4();
        
        let command = engine.parse_text_command(entity_id, "directmove forward for 5.24").unwrap();
        assert_eq!(command.command_type, "directmove");
        assert_eq!(command.parameters.get("direction"), Some(&"forward".to_string()));
        assert_eq!(command.parameters.get("duration"), Some(&"5.24".to_string()));
    }
}
