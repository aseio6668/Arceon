use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use tokio::sync::RwLock;
use tracing::{info, warn, debug}; // Removed unused error
use uuid::Uuid;
use std::sync::Arc;

use crate::entities::{Being, Area}; // Removed unused Vital
use crate::systems::VitalManager;

/// Death and resurrection system with no-loss death mechanics and multiple resurrection options
pub struct DeathSystem {
    /// Active death records by character ID
    death_records: Arc<RwLock<HashMap<Uuid, DeathRecord>>>,
    /// Bind point registry for all characters
    bind_points: Arc<RwLock<HashMap<Uuid, CharacterBindPoints>>>, // character_id -> bind_points
    /// Area safe spots and resurrection points
    area_safe_spots: Arc<RwLock<HashMap<String, Vec<SafeSpot>>>>, // area_id -> safe_spots
    /// System configuration
    config: DeathSystemConfig,
    /// Death system metrics
    metrics: Arc<RwLock<DeathSystemMetrics>>,
    /// Integration with vital manager
    vital_manager: Option<Arc<RwLock<VitalManager>>>,
}

/// Record of a character's death
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathRecord {
    pub death_id: Uuid,
    pub character_id: Uuid,
    pub death_time: SystemTime,
    pub death_location: (f64, f64),
    pub death_area_id: String,
    pub death_cause: DeathCause,
    pub death_circumstances: DeathCircumstances,
    pub resurrection_options: Vec<ResurrectionOption>,
    pub is_awaiting_resurrection: bool,
    pub death_count_today: u32,
    pub total_death_count: u32,
    pub pre_death_vitals: VitalSnapshot,
    pub items_at_death: Vec<ItemSnapshot>, // No items lost, just for reference
    pub experience_debt: u64, // Optional XP debt system
}

/// Character's bind point configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBindPoints {
    pub character_id: Uuid,
    pub home_bind: Option<BindPoint>,
    pub area_binds: HashMap<String, BindPoint>, // area_id -> bind_point
    pub temporary_binds: Vec<TemporaryBindPoint>,
    pub last_safe_location: Option<SafeLocation>,
    pub bind_history: Vec<BindHistory>,
}

/// A bind point where characters can resurrect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindPoint {
    pub bind_id: Uuid,
    pub bind_type: BindPointType,
    pub location: (f64, f64),
    pub area_id: String,
    pub bind_name: String,
    pub is_active: bool,
    pub created_at: SystemTime,
    pub last_used: Option<SystemTime>,
    pub use_count: u32,
    pub security_level: SecurityLevel,
    pub amenities: Vec<BindPointAmenity>,
    pub owner_id: Option<Uuid>, // For player homes
    pub access_restrictions: AccessRestrictions,
}

/// Types of bind points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindPointType {
    /// Public bind point (default for new characters)
    PublicShrine,
    /// Player's home structure
    PlayerHome,
    /// Guild hall bind point
    GuildHall,
    /// Inn room bind point
    InnRoom,
    /// Special event bind point
    EventLocation,
    /// Dungeon checkpoint
    DungeonCheckpoint,
    /// City center bind point
    CityCenter,
    /// Wilderness camp
    WildernessCamp,
}

/// Security level of bind points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Safe,      // Completely protected from all harm
    Guarded,   // Protected by NPCs, but not absolute
    Neutral,   // No special protection
    Contested, // PvP area, minimal safety
    Dangerous, // Active threats nearby
}

/// Amenities available at bind points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindPointAmenity {
    Healing,           // Instant full heal on resurrection
    RestArea,          // Resting bonus for vitals recovery
    Banking,           // Access to bank storage
    Trading,           // NPC vendors or player trading
    Crafting,          // Crafting stations
    EquipmentRepair,   // Repair damaged equipment
    SupplyVendor,      // Buy consumables and supplies
    TransportHub,      // Fast travel to other locations
    SocialArea,        // Chat and social features
    BuffShrine,        // Temporary beneficial effects
}

/// Access restrictions for bind points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRestrictions {
    pub is_public: bool,
    pub allowed_players: Option<Vec<Uuid>>,
    pub allowed_guilds: Option<Vec<Uuid>>,
    pub level_requirement: Option<u32>,
    pub quest_requirement: Option<String>,
    pub faction_requirement: Option<String>,
    pub cost_to_use: Option<CostRequirement>,
}

/// Cost requirement for using bind point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostRequirement {
    pub currency_type: String, // "gold", "tokens", etc.
    pub amount: u64,
    pub per_use: bool, // If false, one-time cost
}

/// Temporary bind point (limited duration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryBindPoint {
    pub bind_point: BindPoint,
    pub expires_at: SystemTime,
    pub max_uses: Option<u32>,
    pub remaining_uses: u32,
    pub creator_id: Option<Uuid>,
}

/// Safe location automatically tracked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeLocation {
    pub location: (f64, f64),
    pub area_id: String,
    pub recorded_at: SystemTime,
    pub safety_rating: f32, // 0.0 to 1.0
    pub last_verified: SystemTime,
}

/// History of bind point usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindHistory {
    pub used_at: SystemTime,
    pub bind_point_id: Uuid,
    pub reason: String, // "death", "manual_bind", "event", etc.
    pub success: bool,
}

/// Area safe spot for resurrections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeSpot {
    pub spot_id: Uuid,
    pub area_id: String,
    pub location: (f64, f64),
    pub spot_type: SafeSpotType,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub capacity: Option<u32>, // Max concurrent users
    pub current_occupancy: u32,
    pub protection_radius: f64,
    pub amenities: Vec<BindPointAmenity>,
    pub auto_generated: bool, // True if created by AI world building
    pub threat_level: DeathThreatLevel,
    pub last_verified_safe: SystemTime,
}

/// Types of safe spots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafeSpotType {
    ResurrectionGraveyard,  // Primary resurrection point
    TempleShrine,          // Religious/spiritual safe spot
    MagicalSanctuary,      // Magically protected area
    GuardPost,             // Protected by NPCs
    NaturalSanctuary,      // Naturally safe area
    PlayerStructure,       // Safe player-built structure
    DungeonCheckpoint,     // Safe room in dangerous area
    EventSafeZone,         // Temporary event safe area
}

/// Threat level assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeathThreatLevel {
    None,      // No threats whatsoever
    Minimal,   // Trivial threats only
    Low,       // Minor threats, easily handled
    Moderate,  // Some danger, requires caution
    High,      // Significant danger
    Extreme,   // Very dangerous, avoid if possible
}

/// Cause of character death
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeathCause {
    Combat {
        killer_type: KillerType,
        killer_id: Option<Uuid>,
        damage_type: String,
        final_blow_skill: Option<String>,
    },
    Environmental {
        hazard_type: String,
        location: (f64, f64),
    },
    Fall {
        fall_distance: f64,
        fall_damage: u64,
    },
    Drowning {
        underwater_time: Duration,
    },
    Poison {
        poison_type: String,
        duration: Duration,
    },
    Starvation,
    Exhaustion,
    MagicalEffect {
        effect_name: String,
        caster_id: Option<Uuid>,
    },
    Scripted {
        event_name: String,
        is_intentional: bool,
    },
    SystemError {
        error_code: String,
    },
}

/// Type of entity that caused death
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KillerType {
    Player,
    NPC,
    Monster,
    Boss,
    Pet,
    Summon,
    Trap,
    Environmental,
    Unknown,
}

/// Circumstances surrounding the death
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathCircumstances {
    pub was_in_combat: bool,
    pub was_afk: bool,
    pub was_in_pvp_area: bool,
    pub was_in_dungeon: bool,
    pub nearby_players: Vec<Uuid>,
    pub nearby_allies: Vec<Uuid>,
    pub active_effects: Vec<String>,
    pub equipment_durability_lost: HashMap<String, f32>,
    pub warnings_ignored: Vec<String>, // Death warnings that were ignored
}

/// Snapshot of character vitals before death
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSnapshot {
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub other_vitals: HashMap<String, f32>,
}

/// Snapshot of items at time of death (for reference)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSnapshot {
    pub item_id: Uuid,
    pub item_name: String,
    pub quantity: u32,
    pub durability: f32,
    pub location: ItemLocation,
}

/// Location of items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemLocation {
    Inventory,
    Equipment(String), // Equipment slot
    Bank,
    Trade,
    Dropped(f64, f64), // Coordinates where dropped
}

/// Available resurrection options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResurrectionOption {
    pub option_id: Uuid,
    pub option_type: ResurrectionType,
    pub location: (f64, f64),
    pub area_id: String,
    pub location_name: String,
    pub distance_from_death: f64,
    pub resurrection_penalties: ResurrectionPenalties,
    pub cost: Option<ResurrectionCost>,
    pub cooldown: Option<Duration>,
    pub is_available: bool,
    pub availability_reason: Option<String>,
    pub safety_rating: f32, // 0.0 to 1.0
    pub amenities_available: Vec<BindPointAmenity>,
}

/// Types of resurrection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResurrectionType {
    /// Resurrect at the exact spot of death
    DeathLocation,
    /// Resurrect at character's home bind point
    HomeBind,
    /// Resurrect at closest public bind point
    PublicBind,
    /// Resurrect at area safe spot
    AreaSafeSpot,
    /// Resurrect at specific bind point
    SpecificBind { bind_point_id: Uuid },
    /// Emergency resurrection (system failure recovery)
    Emergency,
}

/// Penalties applied on resurrection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResurrectionPenalties {
    /// Vital levels after resurrection (0.0 to 1.0)
    pub health_percentage: f32,
    pub mana_percentage: f32,  
    pub stamina_percentage: f32,
    /// Temporary debuffs applied
    pub temporary_debuffs: Vec<TemporaryDebuff>,
    /// Equipment durability lost
    pub durability_loss: f32, // 0.0 to 1.0
    /// Experience debt (if system enabled)
    pub experience_debt: u64,
    /// Temporary stat reduction
    pub stat_reduction: HashMap<String, f32>,
    /// Duration of resurrection sickness
    pub resurrection_sickness_duration: Option<Duration>,
}

/// Temporary debuff applied after resurrection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryDebuff {
    pub debuff_name: String,
    pub description: String,
    pub duration: Duration,
    pub stat_effects: HashMap<String, f32>, // stat -> modifier
    pub prevents_actions: Vec<String>,
    pub can_be_cleansed: bool,
}

/// Cost for resurrection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResurrectionCost {
    pub currency_type: String,
    pub amount: u64,
    pub item_costs: HashMap<String, u32>, // item_name -> quantity
    pub service_costs: Vec<ServiceCost>,
}

/// Service cost for resurrection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCost {
    pub service_name: String,
    pub provider_id: Option<Uuid>, // NPC or player providing service
    pub cost: u64,
    pub currency: String,
}

/// Configuration for death system
#[derive(Debug, Clone)]
pub struct DeathSystemConfig {
    pub resurrection_sickness_enabled: bool,
    pub resurrection_sickness_duration: Duration,
    pub max_deaths_per_day: Option<u32>,
    pub experience_debt_enabled: bool,
    pub experience_debt_rate: f32, // XP debt per death
    pub equipment_durability_loss: f32, // Percentage lost per death
    pub auto_generate_safe_spots: bool,
    pub safe_spot_generation_radius: f64,
    pub min_safe_spots_per_area: u32,
    pub bind_point_cooldown: Duration,
    pub emergency_resurrection_enabled: bool,
    pub pvp_death_penalties_multiplier: f32,
}

/// Death system performance metrics
#[derive(Debug, Clone, Default)]
pub struct DeathSystemMetrics {
    pub total_deaths: u64,
    pub deaths_by_cause: HashMap<String, u64>,
    pub resurrections_by_type: HashMap<String, u64>,
    pub average_time_to_resurrection: Duration,
    pub most_dangerous_areas: HashMap<String, u64>, // area_id -> death_count
    pub safest_areas: HashMap<String, f32>, // area_id -> safety_rating
    pub bind_point_usage: HashMap<Uuid, u32>, // bind_point_id -> usage_count
    pub safe_spot_effectiveness: HashMap<Uuid, f32>, // spot_id -> effectiveness
    pub resurrection_costs_paid: HashMap<String, u64>, // currency -> total_paid
}

impl DeathSystem {
    /// Create a new death system
    pub async fn new(config: DeathSystemConfig) -> Result<Self> {
        info!("💀 Initializing Death and Resurrection System");

        Ok(Self {
            death_records: Arc::new(RwLock::new(HashMap::new())),
            bind_points: Arc::new(RwLock::new(HashMap::new())),
            area_safe_spots: Arc::new(RwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(RwLock::new(DeathSystemMetrics::default())),
            vital_manager: None,
        })
    }

    /// Process character death
    pub async fn process_death(
        &self,
        character_id: Uuid,
        death_cause: DeathCause,
        death_location: (f64, f64),
        area_id: String,
        being: &Being,
    ) -> Result<DeathRecord> {
        info!("⚰️ Processing death for character: {} in area: {}", character_id, area_id);

        // Create death record
        let death_id = Uuid::new_v4();
        let death_record = DeathRecord {
            death_id,
            character_id,
            death_time: SystemTime::now(),
            death_location,
            death_area_id: area_id.clone(),
            death_cause: death_cause.clone(),
            death_circumstances: self.analyze_death_circumstances(character_id, &area_id).await?,
            resurrection_options: self.generate_resurrection_options(character_id, death_location, &area_id).await?,
            is_awaiting_resurrection: true,
            death_count_today: self.get_daily_death_count(character_id).await + 1,
            total_death_count: self.get_total_death_count(character_id).await + 1,
            pre_death_vitals: self.capture_vital_snapshot(being),
            items_at_death: self.capture_item_snapshot(being),
            experience_debt: self.calculate_experience_debt(character_id, &death_cause).await,
        };

        // Store death record
        let mut death_records = self.death_records.write().await;
        death_records.insert(character_id, death_record.clone());
        drop(death_records);

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.total_deaths += 1;
        let cause_key = format!("{:?}", std::mem::discriminant(&death_cause));
        *metrics.deaths_by_cause.entry(cause_key).or_insert(0) += 1;
        *metrics.most_dangerous_areas.entry(area_id.clone()).or_insert(0) += 1;
        drop(metrics);

        // Log death for analysis
        warn!("💀 Character {} died in {} from {:?}", character_id, area_id, death_cause);

        // Trigger AFK system pause if character was AFK
        if death_record.death_circumstances.was_afk {
            // Would pause AFK session here
            info!("🤖 Pausing AFK session due to death: {}", character_id);
        }

        Ok(death_record)
    }

    /// Execute resurrection for a character
    pub async fn resurrect_character(
        &self,
        character_id: Uuid,
        resurrection_option: ResurrectionOption,
        being: &mut Being,
    ) -> Result<ResurrectionResult> {
        info!("⛑️ Resurrecting character: {} at {:?}", character_id, resurrection_option.option_type);

        // Validate resurrection option
        if !resurrection_option.is_available {
            return Err(anyhow::anyhow!("Resurrection option is not available: {:?}", 
                resurrection_option.availability_reason));
        }

        // Remove death record
        let death_record = {
            let mut death_records = self.death_records.write().await;
            death_records.remove(&character_id)
                .ok_or_else(|| anyhow::anyhow!("No death record found for character: {}", character_id))?
        };

        // Apply resurrection location
        // being.location = resurrection_option.location; // Would update being position

        // Apply resurrection penalties
        self.apply_resurrection_penalties(being, &resurrection_option.resurrection_penalties).await?;

        // Update bind point usage if applicable
        if let ResurrectionType::SpecificBind { bind_point_id } = &resurrection_option.option_type {
            self.update_bind_point_usage(*bind_point_id).await?;
        }

        // Create resurrection result
        let resurrection_result = ResurrectionResult {
            resurrection_id: Uuid::new_v4(),
            character_id,
            resurrection_time: SystemTime::now(),
            resurrection_location: resurrection_option.location,
            resurrection_area_id: resurrection_option.area_id.clone(),
            resurrection_type: resurrection_option.option_type,
            penalties_applied: resurrection_option.resurrection_penalties,
            cost_paid: resurrection_option.cost.clone(),
            time_dead: SystemTime::now()
                .duration_since(death_record.death_time)
                .unwrap_or_default(),
        };

        // Update metrics
        let mut metrics = self.metrics.write().await;
        let res_type_key = format!("{:?}", std::mem::discriminant(&resurrection_result.resurrection_type));
        *metrics.resurrections_by_type.entry(res_type_key).or_insert(0) += 1;
        
        // Update average time to resurrection
        let current_avg = metrics.average_time_to_resurrection.as_secs_f64();
        let new_time = resurrection_result.time_dead.as_secs_f64();
        let total_resurrections: u64 = metrics.resurrections_by_type.values().sum();
        metrics.average_time_to_resurrection = Duration::from_secs_f64(
            (current_avg * (total_resurrections - 1) as f64 + new_time) / total_resurrections as f64
        );
        drop(metrics);

        info!("✅ Character resurrected successfully: {} at area: {}", character_id, resurrection_result.resurrection_area_id);

        Ok(resurrection_result)
    }

    /// Create or update bind point for character
    pub async fn create_bind_point(
        &self,
        character_id: Uuid,
        bind_point_type: BindPointType,
        location: (f64, f64),
        area_id: String,
        bind_name: String,
    ) -> Result<Uuid> {
        info!("🏠 Creating bind point for character: {} at area: {}", character_id, area_id);

        let bind_point = BindPoint {
            bind_id: Uuid::new_v4(),
            bind_type: bind_point_type,
            location,
            area_id,
            bind_name,
            is_active: true,
            created_at: SystemTime::now(),
            last_used: None,
            use_count: 0,
            security_level: SecurityLevel::Safe, // Default to safe
            amenities: vec![BindPointAmenity::Healing], // Default amenity
            owner_id: Some(character_id),
            access_restrictions: AccessRestrictions {
                is_public: false,
                allowed_players: Some(vec![character_id]),
                allowed_guilds: None,
                level_requirement: None,
                quest_requirement: None,
                faction_requirement: None,
                cost_to_use: None,
            },
        };

        let bind_id = bind_point.bind_id;

        // Add to character's bind points
        let mut bind_points = self.bind_points.write().await;
        let character_binds = bind_points.entry(character_id).or_insert_with(|| CharacterBindPoints {
            character_id,
            home_bind: None,
            area_binds: HashMap::new(),
            temporary_binds: Vec::new(),
            last_safe_location: None,
            bind_history: Vec::new(),
        });

        // Set as home bind if it's a player home
        match &bind_point.bind_type {
            BindPointType::PlayerHome => {
                character_binds.home_bind = Some(bind_point.clone());
            }
            _ => {
                character_binds.area_binds.insert(bind_point.area_id.clone(), bind_point);
            }
        }

        // Add to history
        character_binds.bind_history.push(BindHistory {
            used_at: SystemTime::now(),
            bind_point_id: bind_id,
            reason: "manual_bind".to_string(),
            success: true,
        });

        drop(bind_points);

        info!("✅ Bind point created: {} for character: {}", bind_id, character_id);
        Ok(bind_id)
    }

    /// Generate safe spots for an area
    pub async fn generate_area_safe_spots(&self, area_id: String, area: &Area) -> Result<Vec<SafeSpot>> {
        info!("🛡️ Generating safe spots for area: {}", area_id);

        if !self.config.auto_generate_safe_spots {
            return Ok(Vec::new());
        }

        let mut safe_spots = Vec::new();

        // Primary resurrection graveyard (always generated)
        let primary_spot = SafeSpot {
            spot_id: Uuid::new_v4(),
            area_id: area_id.clone(),
            location: (0.0, 0.0), // Would use area center or spawn point
            spot_type: SafeSpotType::ResurrectionGraveyard,
            name: format!("{} Resurrection Graveyard", area.name),
            description: "A peaceful graveyard where the departed can return to life".to_string(),
            is_active: true,
            capacity: Some(50),
            current_occupancy: 0,
            protection_radius: 100.0,
            amenities: vec![
                BindPointAmenity::Healing,
                BindPointAmenity::RestArea,
                BindPointAmenity::BuffShrine,
            ],
            auto_generated: true,
            threat_level: DeathThreatLevel::None,
            last_verified_safe: SystemTime::now(),
        };
        safe_spots.push(primary_spot);

        // Generate additional safe spots based on area characteristics
        for i in 0..self.config.min_safe_spots_per_area {
            let spot = SafeSpot {
                spot_id: Uuid::new_v4(),
                area_id: area_id.clone(),
                location: (
                    (i as f64 * 200.0) % 1000.0,
                    ((i + 1) as f64 * 150.0) % 1000.0,
                ),
                spot_type: SafeSpotType::NaturalSanctuary,
                name: format!("{} Safe Haven #{}", area.name, i + 1),
                description: "A naturally protected area safe from harm".to_string(),
                is_active: true,
                capacity: Some(20),
                current_occupancy: 0,
                protection_radius: 50.0,
                amenities: vec![BindPointAmenity::Healing, BindPointAmenity::RestArea],
                auto_generated: true,
                threat_level: DeathThreatLevel::None,
                last_verified_safe: SystemTime::now(),
            };
            safe_spots.push(spot);
        }

        // Store safe spots
        let mut area_safe_spots = self.area_safe_spots.write().await;
        area_safe_spots.insert(area_id.clone(), safe_spots.clone());
        drop(area_safe_spots);

        info!("✅ Generated {} safe spots for area: {}", safe_spots.len(), area_id);
        Ok(safe_spots)
    }

    /// Get available resurrection options for a character
    async fn generate_resurrection_options(
        &self,
        character_id: Uuid,
        death_location: (f64, f64),
        area_id: &String,
    ) -> Result<Vec<ResurrectionOption>> {
        debug!("Generating resurrection options for character: {}", character_id);

        let mut options = Vec::new();

        // 1. Resurrect at death location (always available)
        let death_location_option = ResurrectionOption {
            option_id: Uuid::new_v4(),
            option_type: ResurrectionType::DeathLocation,
            location: death_location,
            area_id: area_id.clone(),
            location_name: "Death Location".to_string(),
            distance_from_death: 0.0,
            resurrection_penalties: ResurrectionPenalties {
                health_percentage: 0.25, // Low health at death location
                mana_percentage: 0.25,
                stamina_percentage: 0.25,
                ..Default::default()
            },
            cost: None,
            cooldown: None,
            is_available: true,
            availability_reason: None,
            safety_rating: 0.3, // Death location is typically unsafe
            amenities_available: Vec::new(),
        };
        options.push(death_location_option);

        // 2. Home bind point (if available)
        let bind_points = self.bind_points.read().await;
        if let Some(character_binds) = bind_points.get(&character_id) {
            if let Some(home_bind) = &character_binds.home_bind {
                let distance = self.calculate_distance(death_location, home_bind.location);
                let home_option = ResurrectionOption {
                    option_id: Uuid::new_v4(),
                    option_type: ResurrectionType::HomeBind,
                    location: home_bind.location,
                    area_id: home_bind.area_id.clone(),
                    location_name: home_bind.bind_name.clone(),
                    distance_from_death: distance,
                    resurrection_penalties: ResurrectionPenalties {
                        health_percentage: 0.5,
                        mana_percentage: 0.5,
                        stamina_percentage: 0.5,
                        ..Default::default()
                    },
                    cost: None,
                    cooldown: Some(Duration::from_secs(300)), // 5 minute cooldown
                    is_available: true,
                    availability_reason: None,
                    safety_rating: 0.9,
                    amenities_available: home_bind.amenities.clone(),
                };
                options.push(home_option);
            }
        }
        drop(bind_points);

        // 3. Area safe spots
        let area_safe_spots = self.area_safe_spots.read().await;
        if let Some(safe_spots) = area_safe_spots.get(area_id) {
            for safe_spot in safe_spots {
                if safe_spot.is_active && safe_spot.current_occupancy < safe_spot.capacity.unwrap_or(u32::MAX) {
                    let distance = self.calculate_distance(death_location, safe_spot.location);
                    let safe_spot_option = ResurrectionOption {
                        option_id: Uuid::new_v4(),
                        option_type: ResurrectionType::AreaSafeSpot,
                        location: safe_spot.location,
                        area_id: area_id.clone(),
                        location_name: safe_spot.name.clone(),
                        distance_from_death: distance,
                        resurrection_penalties: ResurrectionPenalties {
                            health_percentage: 0.75,
                            mana_percentage: 0.75,
                            stamina_percentage: 0.75,
                            ..Default::default()
                        },
                        cost: None,
                        cooldown: None,
                        is_available: true,
                        availability_reason: None,
                        safety_rating: 1.0,
                        amenities_available: safe_spot.amenities.clone(),
                    };
                    options.push(safe_spot_option);
                }
            }
        }
        drop(area_safe_spots);

        debug!("Generated {} resurrection options for character: {}", options.len(), character_id);
        Ok(options)
    }

    /// Analyze circumstances of death
    async fn analyze_death_circumstances(&self, _character_id: Uuid, area_id: &String) -> Result<DeathCircumstances> {
        // This would analyze the game state at time of death
        // For now, return basic circumstances
        Ok(DeathCircumstances {
            was_in_combat: true, // Would check combat state
            was_afk: false, // Would check AFK status
            was_in_pvp_area: false, // Would check area PvP status
            was_in_dungeon: area_id.contains("dungeon"), // Simple check
            nearby_players: Vec::new(), // Would find nearby players
            nearby_allies: Vec::new(), // Would find nearby allies
            active_effects: Vec::new(), // Would list active buffs/debuffs
            equipment_durability_lost: HashMap::new(), // Would calculate durability loss
            warnings_ignored: Vec::new(), // Would track ignored warnings
        })
    }

    /// Capture snapshot of character's vitals
    fn capture_vital_snapshot(&self, _being: &Being) -> VitalSnapshot {
        VitalSnapshot {
            health: 0.0, // Would get actual vital values
            max_health: 100.0,
            mana: 0.0,
            max_mana: 100.0,
            stamina: 0.0,
            max_stamina: 100.0,
            other_vitals: HashMap::new(),
        }
    }

    /// Capture snapshot of character's items
    fn capture_item_snapshot(&self, _being: &Being) -> Vec<ItemSnapshot> {
        // Would capture actual inventory state
        Vec::new()
    }

    /// Calculate experience debt for death
    async fn calculate_experience_debt(&self, _character_id: Uuid, death_cause: &DeathCause) -> u64 {
        if !self.config.experience_debt_enabled {
            return 0;
        }

        // Base debt calculation
        let base_debt = 1000_u64; // Base XP debt

        // Modify based on death cause
        let cause_multiplier = match death_cause {
            DeathCause::Combat { .. } => 1.0,
            DeathCause::Environmental { .. } => 0.5,
            DeathCause::Fall { .. } => 0.3,
            _ => 0.8,
        };

        (base_debt as f32 * cause_multiplier * self.config.experience_debt_rate) as u64
    }

    /// Apply resurrection penalties to character
    async fn apply_resurrection_penalties(&self, _being: &mut Being, penalties: &ResurrectionPenalties) -> Result<()> {
        debug!("Applying resurrection penalties");

        // Apply vital penalties
        // being.vitals.health = being.vitals.max_health * penalties.health_percentage;
        // being.vitals.mana = being.vitals.max_mana * penalties.mana_percentage;
        // being.vitals.stamina = being.vitals.max_stamina * penalties.stamina_percentage;

        // Apply temporary debuffs
        for debuff in &penalties.temporary_debuffs {
            // being.apply_temporary_effect(debuff);
            debug!("Applied resurrection debuff: {}", debuff.debuff_name);
        }

        // Apply equipment durability loss
        if penalties.durability_loss > 0.0 {
            // being.reduce_equipment_durability(penalties.durability_loss);
            debug!("Applied durability loss: {}", penalties.durability_loss);
        }

        // Apply resurrection sickness if configured
        if self.config.resurrection_sickness_enabled {
            if let Some(duration) = penalties.resurrection_sickness_duration {
                // being.apply_resurrection_sickness(duration);
                debug!("Applied resurrection sickness for: {:?}", duration);
            }
        }

        Ok(())
    }

    /// Update bind point usage statistics
    async fn update_bind_point_usage(&self, bind_point_id: Uuid) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        *metrics.bind_point_usage.entry(bind_point_id).or_insert(0) += 1;
        Ok(())
    }

    /// Calculate distance between two points
    fn calculate_distance(&self, point1: (f64, f64), point2: (f64, f64)) -> f64 {
        ((point1.0 - point2.0).powi(2) + (point1.1 - point2.1).powi(2)).sqrt()
    }

    /// Get daily death count for character
    async fn get_daily_death_count(&self, _character_id: Uuid) -> u32 {
        // Would query death records from today
        0 // Placeholder
    }

    /// Get total death count for character
    async fn get_total_death_count(&self, _character_id: Uuid) -> u32 {
        // Would query total death records
        0 // Placeholder
    }

    /// Get death system statistics
    pub async fn get_system_stats(&self) -> DeathSystemMetrics {
        self.metrics.read().await.clone()
    }

    /// Get character's death record if exists
    pub async fn get_death_record(&self, character_id: Uuid) -> Option<DeathRecord> {
        let death_records = self.death_records.read().await;
        death_records.get(&character_id).cloned()
    }

    /// Get character's bind points
    pub async fn get_character_bind_points(&self, character_id: Uuid) -> Option<CharacterBindPoints> {
        let bind_points = self.bind_points.read().await;
        bind_points.get(&character_id).cloned()
    }

    /// Get safe spots for an area
    pub async fn get_area_safe_spots(&self, area_id: &String) -> Vec<SafeSpot> {
        let area_safe_spots = self.area_safe_spots.read().await;
        area_safe_spots.get(area_id).cloned().unwrap_or_default()
    }
}

/// Result of character resurrection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResurrectionResult {
    pub resurrection_id: Uuid,
    pub character_id: Uuid,
    pub resurrection_time: SystemTime,
    pub resurrection_location: (f64, f64),
    pub resurrection_area_id: String,
    pub resurrection_type: ResurrectionType,
    pub penalties_applied: ResurrectionPenalties,
    pub cost_paid: Option<ResurrectionCost>,
    pub time_dead: Duration,
}

impl Default for DeathSystemConfig {
    fn default() -> Self {
        Self {
            resurrection_sickness_enabled: true,
            resurrection_sickness_duration: Duration::from_minutes(5),
            max_deaths_per_day: None, // No limit
            experience_debt_enabled: false, // No XP loss
            experience_debt_rate: 0.0,
            equipment_durability_loss: 0.05, // 5% durability loss per death
            auto_generate_safe_spots: true,
            safe_spot_generation_radius: 1000.0,
            min_safe_spots_per_area: 3,
            bind_point_cooldown: Duration::from_minutes(5),
            emergency_resurrection_enabled: true,
            pvp_death_penalties_multiplier: 1.0, // Same penalties for PvP
        }
    }
}

trait DurationExt {
    fn from_minutes(minutes: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_minutes(minutes: u64) -> Duration {
        Duration::from_secs(minutes * 60)
    }
}