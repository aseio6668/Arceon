pub mod weather_system;
pub mod seasonal_system;
pub mod natural_events;

pub use weather_system::{WeatherSystem, WeatherState, PrecipitationType, WindPattern, PatternType};
pub use seasonal_system::{SeasonalSystem, SeasonState, Season, VegetationState};
pub use natural_events::{NaturalEventsSystem, NaturalEvent, NaturalEventType, VolcanoType};
pub use natural_events::EventSeverity as NaturalEventSeverity;
pub use weather_system::EventSeverity as WeatherEventSeverity;

use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct WorldSystemsManager {
    pub weather: WeatherSystem,
    pub seasons: SeasonalSystem,
    pub natural_events: NaturalEventsSystem,
    pub time_scale: f64,
    pub last_update: DateTime<Utc>,
}

impl Default for WorldSystemsManager {
    fn default() -> Self {
        Self {
            weather: WeatherSystem::new(),
            seasons: SeasonalSystem::new(),
            natural_events: NaturalEventsSystem::new(),
            time_scale: 1.0,
            last_update: Utc::now(),
        }
    }
}

impl WorldSystemsManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.initialize().unwrap_or_else(|e| {
            eprintln!("Failed to initialize world systems: {}", e);
        });
        manager
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize climate zones
        self.weather.initialize_climate_zones()?;
        
        // Initialize seasonal cycles
        self.seasons.initialize_seasonal_cycles()?;
        
        // Create some initial volcanic systems
        self.natural_events.create_volcanic_system(
            "Mount Ignis".to_string(),
            Vector3::new(500.0, 2000.0, 500.0),
            VolcanoType::Stratovolcano
        )?;
        
        self.natural_events.create_volcanic_system(
            "Ember Peak".to_string(),
            Vector3::new(-300.0, 1500.0, 800.0),
            VolcanoType::CinderCone
        )?;

        Ok(())
    }

    pub fn update(&mut self, delta_time: f64) -> Result<(), String> {
        let now = Utc::now();
        let actual_delta = now.signed_duration_since(self.last_update).num_seconds() as f64;
        let scaled_delta = actual_delta * self.time_scale;

        // Update all weather zones
        let zone_ids: Vec<Uuid> = self.weather.climate_zones.keys().cloned().collect();
        for zone_id in zone_ids {
            self.weather.update_weather(zone_id, scaled_delta)?;
            self.weather.apply_weather_effects(zone_id)?;
            self.seasons.update_season(zone_id, now)?;
            
            // Update vegetation based on season
            let current_season = self.seasons.get_season(zone_id).map(|state| state.current_season.clone());
            if let Some(season) = current_season {
                self.seasons.update_vegetation(zone_id, &season)?;
            }
        }

        // Simulate atmospheric dynamics
        self.weather.simulate_atmospheric_dynamics(scaled_delta)?;

        // Update natural events
        self.natural_events.update_events(scaled_delta)?;

        self.last_update = now;
        Ok(())
    }

    pub fn get_environment_at(&self, zone_id: Uuid) -> EnvironmentState {
        EnvironmentState {
            weather: self.weather.get_weather(zone_id).cloned(),
            season: self.seasons.get_season(zone_id).cloned(),
            active_events: self.natural_events.active_events
                .iter()
                .filter(|e| {
                    // Check if event affects this zone (simplified)
                    true
                })
                .cloned()
                .collect(),
        }
    }

    pub fn trigger_weather_pattern(&mut self, pattern_type: PatternType, origin: Vector3<f32>) -> Result<Uuid, String> {
        self.weather.generate_weather_pattern(pattern_type, origin)
    }

    pub fn trigger_natural_disaster(&mut self, event_type: NaturalEventType, location: Vector3<f32>, severity: NaturalEventSeverity) -> Result<Uuid, String> {
        let event_id = self.natural_events.trigger_event(event_type, location, severity.clone())?;
        
        // Issue warning if severe enough
        if matches!(severity, NaturalEventSeverity::Major | NaturalEventSeverity::Severe | NaturalEventSeverity::Extreme | NaturalEventSeverity::Catastrophic) {
            self.natural_events.issue_warning(event_id)?;
        }
        
        Ok(event_id)
    }

    pub fn simulate_earthquake(&mut self, epicenter: Vector3<f32>, magnitude: f32) -> Result<Uuid, String> {
        self.natural_events.simulate_earthquake(epicenter, magnitude)
    }

    pub fn simulate_flood(&mut self, affected_zones: Vec<Uuid>, water_level: f32) -> Result<Uuid, String> {
        self.natural_events.simulate_flood(affected_zones, water_level)
    }

    pub fn get_weather_forecast(&mut self, zone_id: Uuid, hours_ahead: i64) -> Result<WeatherState, String> {
        self.weather.forecast_weather(zone_id, hours_ahead)
    }

    pub fn get_seasonal_resources(&self, zone_id: Uuid) -> Vec<String> {
        if let Some(season_state) = self.seasons.get_season(zone_id) {
            self.seasons.get_seasonal_resources(&season_state.current_season)
        } else {
            Vec::new()
        }
    }

    pub fn calculate_crop_yield(&self, zone_id: Uuid, crop: &str) -> f32 {
        if let Some(season_state) = self.seasons.get_season(zone_id) {
            let seasonal_yield = self.seasons.calculate_crop_yield(crop, &season_state.current_season);
            
            // Apply weather modifiers
            if let Some(weather) = self.weather.get_weather(zone_id) {
                let weather_modifier = match weather.precipitation.precipitation_type {
                    PrecipitationType::Rain if weather.precipitation.intensity < 0.5 => 1.1,
                    PrecipitationType::Rain if weather.precipitation.intensity > 0.8 => 0.8,
                    PrecipitationType::None => 0.5, // Represents drought-like conditions
                    _ => 1.0,
                };
                seasonal_yield * weather_modifier
            } else {
                seasonal_yield
            }
        } else {
            0.0
        }
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale.max(0.0f64).min(100.0f64);
    }

    pub fn get_disaster_risk(&mut self, zone_id: Uuid) -> HashMap<NaturalEventType, f32> {
        let mut risks = HashMap::new();
        
        // Calculate risk for various disaster types
        let location = Vector3::new(0.0, 0.0, 0.0); // Simplified location
        
        for event_type in vec![
            NaturalEventType::Earthquake,
            NaturalEventType::Flood,
            NaturalEventType::Wildfire,
            NaturalEventType::Hurricane,
            NaturalEventType::Drought,
        ] {
            if let Ok(risk) = self.natural_events.predict_event(event_type.clone(), location) {
                risks.insert(event_type, risk);
            }
        }
        
        risks
    }
}

#[derive(Debug, Clone)]
pub struct EnvironmentState {
    pub weather: Option<WeatherState>,
    pub season: Option<SeasonState>,
    pub active_events: Vec<NaturalEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_systems_initialization() {
        let mut manager = WorldSystemsManager::new();
        assert!(!manager.weather.climate_zones.is_empty());
        assert!(!manager.seasons.seasonal_cycles.is_empty());
    }

    #[test]
    fn test_weather_update() {
        let mut manager = WorldSystemsManager::new();
        let zone_id = *manager.weather.climate_zones.keys().next().unwrap();
        
        manager.weather.update_weather(zone_id, 1.0).unwrap();
        assert!(manager.weather.get_weather(zone_id).is_some());
    }

    #[test]
    fn test_seasonal_update() {
        let mut manager = WorldSystemsManager::new();
        let zone_id = Uuid::new_v4();
        
        manager.seasons.update_season(zone_id, Utc::now()).unwrap();
        assert!(manager.seasons.get_season(zone_id).is_some());
    }

    #[test]
    fn test_natural_event_trigger() {
        let mut manager = WorldSystemsManager::new();
        
        let event_id = manager.trigger_natural_disaster(
            NaturalEventType::Earthquake,
            Vector3::new(0.0, 0.0, 0.0),
            EventSeverity::Moderate
        ).unwrap();
        
        assert!(!manager.natural_events.active_events.is_empty());
        assert!(manager.natural_events.active_events.iter().any(|e| e.id == event_id));
    }
}
