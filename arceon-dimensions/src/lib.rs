pub mod dimensional_system;
pub mod portal_system;
pub mod realm_exploration;

pub use dimensional_system::{DimensionalSystem, Dimension, DimensionalConnection, DimensionalPhysics, RealityAnchor, DimensionalStorm, NexusPoint, PortalType};
pub use portal_system::{PortalSystem, Portal, PortalKey, PortalConstruction, PortalMaintenance, PortalSecurity, TravelLog, PortalEffects, DimensionalAnchor, PortalClass, AccessLevel, PortalAnchor, DimensionalLock};
pub use realm_exploration::{RealmExplorationSystem, DiscoveredRealm, ExplorationExpedition, RealmSurvey, DimensionalCartography, ExplorationGuild, ExpeditionStatus};

use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct MultiDimensionalManager {
    pub dimensional_system: DimensionalSystem,
    pub portal_system: PortalSystem,
    pub realm_exploration: RealmExplorationSystem,
    pub active_travelers: HashMap<Uuid, DimensionalTraveler>,
    pub dimensional_events: Vec<DimensionalEvent>,
    pub cross_dimensional_effects: CrossDimensionalEffects,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DimensionalTraveler {
    pub entity_id: Uuid,
    pub current_dimension: Uuid,
    pub travel_history: Vec<DimensionalJump>,
    pub dimensional_adaptation: f32,
    pub portal_access_level: AccessLevel,
    pub exploration_achievements: Vec<String>,
    pub dimensional_knowledge: DimensionalKnowledge,
}

#[derive(Debug, Clone)]
pub struct DimensionalJump {
    pub from_dimension: Uuid,
    pub to_dimension: Uuid,
    pub portal_used: Option<Uuid>,
    pub jump_time: DateTime<Utc>,
    pub energy_cost: f32,
    pub side_effects: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DimensionalEvent {
    pub id: Uuid,
    pub event_type: DimensionalEventType,
    pub affected_dimensions: Vec<Uuid>,
    pub start_time: DateTime<Utc>,
    pub duration: chrono::Duration,
    pub severity: EventSeverity,
    pub consequences: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DimensionalEventType {
    PortalStorm,
    RealityConvergence,
    DimensionalRift,
    PlanarAlignment,
    VoidIncursion,
    TemporalFlux,
    MagicalSurge,
    GravitationalAnomaly,
}

#[derive(Debug, Clone)]
pub enum EventSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
    Catastrophic,
}

#[derive(Debug, Clone)]
pub struct CrossDimensionalEffects {
    pub reality_bleed: HashMap<Uuid, f32>,
    pub dimensional_interference: HashMap<(Uuid, Uuid), f32>,
    pub planar_resonance: HashMap<Uuid, ResonanceLevel>,
    pub void_corruption: HashMap<Uuid, f32>,
    pub temporal_distortion: HashMap<Uuid, f32>,
}

#[derive(Debug, Clone)]
pub enum ResonanceLevel {
    Harmonious,
    Neutral,
    Dissonant,
    Chaotic,
}

#[derive(Debug, Clone)]
pub struct DimensionalKnowledge {
    pub known_dimensions: HashSet<Uuid>,
    pub portal_locations: HashMap<Uuid, Vec<Vector3<f32>>>,
    pub dimensional_maps: HashMap<Uuid, f32>, // Completeness percentage
    pub hazard_awareness: HashMap<Uuid, Vec<String>>,
    pub cultural_knowledge: HashMap<Uuid, f32>,
}

impl Default for MultiDimensionalManager {
    fn default() -> Self {
        Self {
            dimensional_system: DimensionalSystem::new(),
            portal_system: PortalSystem::new(),
            realm_exploration: RealmExplorationSystem::new(),
            active_travelers: HashMap::new(),
            dimensional_events: Vec::new(),
            cross_dimensional_effects: CrossDimensionalEffects::default(),
            last_update: Utc::now(),
        }
    }
}

impl Default for CrossDimensionalEffects {
    fn default() -> Self {
        Self {
            reality_bleed: HashMap::new(),
            dimensional_interference: HashMap::new(),
            planar_resonance: HashMap::new(),
            void_corruption: HashMap::new(),
            temporal_distortion: HashMap::new(),
        }
    }
}

impl Default for DimensionalKnowledge {
    fn default() -> Self {
        Self {
            known_dimensions: HashSet::new(),
            portal_locations: HashMap::new(),
            dimensional_maps: HashMap::new(),
            hazard_awareness: HashMap::new(),
            cultural_knowledge: HashMap::new(),
        }
    }
}

impl MultiDimensionalManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.initialize().unwrap_or_else(|e| {
            eprintln!("Failed to initialize multi-dimensional systems: {}", e);
        });
        manager
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize core dimensions
        self.dimensional_system.initialize_core_dimensions()?;
        
        // Initialize portal infrastructure
        self.portal_system.initialize_portal_infrastructure()?;
        
        // Initialize exploration systems
        self.realm_exploration.initialize_exploration_systems()?;
        
        Ok(())
    }

    pub fn update(&mut self, delta_time: f64) -> Result<(), String> {
        let now = Utc::now();
        
        // Update dimensional system
        self.dimensional_system.update_dimensional_system(delta_time)?;
        
        // Update cross-dimensional effects
        self.update_cross_dimensional_effects(delta_time)?;
        
        // Process dimensional events
        self.process_dimensional_events(delta_time)?;
        
        // Update traveler adaptations
        self.update_traveler_adaptations(delta_time)?;
        
        self.last_update = now;
        Ok(())
    }

    pub fn create_portal_between_dimensions(&mut self, 
        origin_dim: Uuid, 
        dest_dim: Uuid, 
        creator_id: Uuid,
        portal_type: PortalType
    ) -> Result<Uuid, String> {
        
        // Create portal in dimensional system
        let origin_pos = Vector3::new(0.0, 0.0, 0.0);
        let dest_pos = Vector3::new(0.0, 0.0, 0.0);
        let portal_id = self.dimensional_system.create_portal(
            portal_type.clone(),
            origin_dim,
            dest_dim,
            origin_pos,
            dest_pos
        )?;
        
        // Register portal in portal system
        let origin_anchor = PortalAnchor {
            dimension_id: origin_dim,
            coordinates: origin_pos,
            anchor_strength: 1.0,
            dimensional_lock: DimensionalLock::default(),
            stability_rating: 1.0,
            anchor_effects: Vec::new(),
        };
        
        let dest_anchor = PortalAnchor {
            dimension_id: dest_dim,
            coordinates: dest_pos,
            anchor_strength: 1.0,
            dimensional_lock: DimensionalLock::default(),
            stability_rating: 1.0,
            anchor_effects: Vec::new(),
        };
        
        self.portal_system.construct_portal(
            creator_id,
            PortalClass::Transport,
            origin_anchor,
            dest_anchor,
            format!("Portal {}", portal_id)
        )?;
        
        Ok(portal_id)
    }

    pub fn travel_between_dimensions(&mut self, 
        traveler_id: Uuid, 
        portal_id: Uuid
    ) -> Result<DimensionalTravelResult, String> {
        
        // Process travel through dimensional system
        let (dest_dimension, dest_coordinates) = self.dimensional_system.travel_through_portal(traveler_id, portal_id)?;
        
        // Process travel through portal system
        let portal_result = self.portal_system.travel_through_portal(portal_id, traveler_id, Vec::new())?;
        
        // Update traveler information
        let traveler = self.active_travelers.entry(traveler_id).or_insert_with(|| {
            DimensionalTraveler {
                entity_id: traveler_id,
                current_dimension: dest_dimension,
                travel_history: Vec::new(),
                dimensional_adaptation: 1.0,
                portal_access_level: AccessLevel::Basic,
                exploration_achievements: Vec::new(),
                dimensional_knowledge: DimensionalKnowledge::default(),
            }
        });
        
        // Record the jump
        let jump = DimensionalJump {
            from_dimension: traveler.current_dimension,
            to_dimension: dest_dimension,
            portal_used: Some(portal_id),
            jump_time: Utc::now(),
            energy_cost: portal_result.energy_cost,
            side_effects: portal_result.travel_effects.clone(),
        };
        
        traveler.travel_history.push(jump);
        traveler.current_dimension = dest_dimension;
        traveler.dimensional_knowledge.known_dimensions.insert(dest_dimension);
        
        Ok(DimensionalTravelResult {
            success: true,
            destination_dimension: dest_dimension,
            destination_coordinates: dest_coordinates,
            travel_effects: portal_result.travel_effects,
            energy_cost: portal_result.energy_cost,
        })
    }

    pub fn discover_new_realm(&mut self, 
        explorer_id: Uuid, 
        dimension_id: Uuid,
        realm_name: String
    ) -> Result<Uuid, String> {
        
        let realm_id = self.realm_exploration.discover_realm(explorer_id, dimension_id, realm_name)?;
        
        // Update explorer's knowledge
        if let Some(traveler) = self.active_travelers.get_mut(&explorer_id) {
            traveler.exploration_achievements.push(format!("Discovered realm {}", realm_id));
            traveler.dimensional_knowledge.known_dimensions.insert(dimension_id);
        }
        
        Ok(realm_id)
    }

    pub fn organize_exploration_expedition(&mut self, 
        leader_id: Uuid, 
        target_realm: Uuid, 
        team_members: Vec<Uuid>,
        objectives: Vec<String>
    ) -> Result<Uuid, String> {
        
        self.realm_exploration.organize_expedition(leader_id, target_realm, team_members, objectives)
    }

    pub fn trigger_dimensional_event(&mut self, 
        event_type: DimensionalEventType, 
        affected_dimensions: Vec<Uuid>
    ) -> Result<Uuid, String> {
        
        let event = DimensionalEvent {
            id: Uuid::new_v4(),
            event_type,
            affected_dimensions: affected_dimensions.clone(),
            start_time: Utc::now(),
            duration: chrono::Duration::hours(6),
            severity: EventSeverity::Moderate,
            consequences: Vec::new(),
        };
        
        let event_id = event.id;
        
        // Apply immediate effects based on event type
        for dim_id in &affected_dimensions {
            match &event.event_type {
                DimensionalEventType::PortalStorm => {
                    self.cross_dimensional_effects.dimensional_interference
                        .insert((*dim_id, *dim_id), 0.8);
                }
                DimensionalEventType::VoidIncursion => {
                    self.cross_dimensional_effects.void_corruption
                        .insert(*dim_id, 0.3);
                }
                DimensionalEventType::TemporalFlux => {
                    self.cross_dimensional_effects.temporal_distortion
                        .insert(*dim_id, 0.5);
                }
                _ => {}
            }
        }
        
        self.dimensional_events.push(event);
        Ok(event_id)
    }

    fn update_cross_dimensional_effects(&mut self, delta_time: f64) -> Result<(), String> {
        // Gradually decay dimensional interference
        for (_, interference) in self.cross_dimensional_effects.dimensional_interference.iter_mut() {
            *interference *= 0.99_f32.powf(delta_time as f32);
        }
        
        // Reduce void corruption over time (slowly)
        for (_, corruption) in self.cross_dimensional_effects.void_corruption.iter_mut() {
            *corruption *= 0.995_f32.powf(delta_time as f32);
        }
        
        // Stabilize temporal distortions
        for (_, distortion) in self.cross_dimensional_effects.temporal_distortion.iter_mut() {
            *distortion *= 0.98_f32.powf(delta_time as f32);
        }
        
        Ok(())
    }

    fn process_dimensional_events(&mut self, delta_time: f64) -> Result<(), String> {
        let now = Utc::now();
        
        // Remove expired events
        self.dimensional_events.retain(|event| {
            let elapsed = now.signed_duration_since(event.start_time);
            elapsed < event.duration
        });
        
        Ok(())
    }

    fn update_traveler_adaptations(&mut self, delta_time: f64) -> Result<(), String> {
        for traveler in self.active_travelers.values_mut() {
            // Improve adaptation over time
            traveler.dimensional_adaptation = (traveler.dimensional_adaptation + 0.001 * delta_time as f32).min(2.0);
        }
        Ok(())
    }

    // Getters for external access
    pub fn get_dimensional_traveler(&self, traveler_id: Uuid) -> Option<&DimensionalTraveler> {
        self.active_travelers.get(&traveler_id)
    }

    pub fn get_available_dimensions(&self, from_dimension: Uuid) -> Vec<Uuid> {
        self.dimensional_system.get_accessible_dimensions(from_dimension)
    }

    pub fn get_active_portals_in_dimension(&self, dimension_id: Uuid) -> Vec<&Portal> {
        self.portal_system.portal_registry.values()
            .filter(|portal| portal.origin_anchor.dimension_id == dimension_id || 
                           portal.destination_anchor.dimension_id == dimension_id)
            .collect()
    }

    pub fn get_dimensional_statistics(&self) -> DimensionalStatistics {
        DimensionalStatistics {
            total_dimensions: self.dimensional_system.dimensions.len(),
            active_portals: self.dimensional_system.active_portals.len(),
            active_travelers: self.active_travelers.len(),
            discovered_realms: self.realm_exploration.discovered_realms.len(),
            active_expeditions: self.realm_exploration.exploration_expeditions.iter()
                .filter(|e| matches!(e.expedition_status, ExpeditionStatus::InProgress))
                .count(),
            dimensional_events: self.dimensional_events.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DimensionalTravelResult {
    pub success: bool,
    pub destination_dimension: Uuid,
    pub destination_coordinates: Vector3<f32>,
    pub travel_effects: Vec<String>,
    pub energy_cost: f32,
}

#[derive(Debug, Clone)]
pub struct DimensionalStatistics {
    pub total_dimensions: usize,
    pub active_portals: usize,
    pub active_travelers: usize,
    pub discovered_realms: usize,
    pub active_expeditions: usize,
    pub dimensional_events: usize,
}

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_dimensional_initialization() {
        let manager = MultiDimensionalManager::new();
        assert!(!manager.dimensional_system.dimensions.is_empty());
        assert!(manager.portal_system.portal_registry.is_empty());
        assert!(manager.realm_exploration.discovered_realms.is_empty());
    }

    #[test]
    fn test_portal_creation() {
        let mut manager = MultiDimensionalManager::new();
        
        let dimensions: Vec<Uuid> = manager.dimensional_system.dimensions.keys().cloned().collect();
        if dimensions.len() >= 2 {
            let creator_id = Uuid::new_v4();
            let portal_id = manager.create_portal_between_dimensions(
                dimensions[0],
                dimensions[1],
                creator_id,
                PortalType::TwoWay
            ).unwrap();
            
            assert!(manager.dimensional_system.active_portals.contains_key(&portal_id));
        }
    }

    #[test]
    fn test_realm_discovery() {
        let mut manager = MultiDimensionalManager::new();
        
        let dimensions: Vec<Uuid> = manager.dimensional_system.dimensions.keys().cloned().collect();
        if !dimensions.is_empty() {
            let explorer_id = Uuid::new_v4();
            let realm_id = manager.discover_new_realm(
                explorer_id,
                dimensions[0],
                "Test Realm".to_string()
            ).unwrap();
            
            assert!(manager.realm_exploration.discovered_realms.contains_key(&realm_id));
            assert!(manager.active_travelers.contains_key(&explorer_id));
        }
    }

    #[test]
    fn test_dimensional_events() {
        let mut manager = MultiDimensionalManager::new();
        
        let dimensions: Vec<Uuid> = manager.dimensional_system.dimensions.keys().cloned().take(1).collect();
        let event_id = manager.trigger_dimensional_event(
            DimensionalEventType::PortalStorm,
            dimensions
        ).unwrap();
        
        assert!(!manager.dimensional_events.is_empty());
        assert!(manager.dimensional_events.iter().any(|e| e.id == event_id));
    }
}
