use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

#[cfg(feature = "advanced-algorithms")]
use kdtree::{KdTree, distance::squared_euclidean};

/// Comprehensive building and construction system
pub struct BuildingSystem {
    pub construction_projects: HashMap<Uuid, ConstructionProject>,
    pub building_templates: HashMap<Uuid, BuildingTemplate>,
    #[cfg(feature = "advanced-algorithms")]
    pub spatial_index: KdTree<f64, BuildingInstance, [f64; 3]>,
    pub zoning_manager: ZoningManager,
    pub infrastructure_network: InfrastructureNetwork,
    pub building_permissions: BuildingPermissions,
}

/// Represents an active construction project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionProject {
    pub project_id: Uuid,
    pub builder_id: Uuid,
    pub template_id: Uuid,
    pub location: WorldPosition,
    pub construction_state: ConstructionState,
    pub material_requirements: Vec<MaterialRequirement>,
    pub labor_requirements: LaborRequirements,
    pub construction_phases: Vec<ConstructionPhase>,
    pub current_phase: usize,
    pub quality_targets: QualityTargets,
    pub environmental_considerations: EnvironmentalFactors,
    pub permits_and_approvals: Vec<BuildingPermit>,
    pub timeline: ConstructionTimeline,
    pub budget_tracking: BudgetTracking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstructionState {
    Planning,
    PermitsPending,
    MaterialGathering,
    FoundationWork,
    StructuralWork,
    SystemsInstallation,
    Finishing,
    QualityInspection,
    Completed,
    Suspended { reason: String },
    Abandoned { reason: String },
}

/// Template defining how buildings can be constructed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingTemplate {
    pub template_id: Uuid,
    pub building_name: String,
    pub building_type: BuildingType,
    pub size_category: SizeCategory,
    pub architectural_style: ArchitecturalStyle,
    pub base_materials: Vec<MaterialSpec>,
    pub structural_requirements: StructuralRequirements,
    pub utility_requirements: UtilityRequirements,
    pub functional_spaces: Vec<FunctionalSpace>,
    pub customization_options: Vec<CustomizationOption>,
    pub prerequisite_skills: HashMap<String, f64>,
    pub cultural_requirements: Option<String>,
    pub environmental_constraints: Vec<EnvironmentalConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BuildingType {
    Residential { max_occupants: u32 },
    Workshop { crafting_bonus: f64 },
    Storage { capacity: u64 },
    Social { gathering_capacity: u32 },
    Defense { defensive_rating: f64 },
    Infrastructure { service_radius: f64 },
    Religious { cultural_influence: f64 },
    Commercial { trade_efficiency: f64 },
    Agricultural { production_bonus: f64 },
    Research { knowledge_generation: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SizeCategory {
    Tiny,      // Single room structures
    Small,     // Small houses, workshops
    Medium,    // Large houses, small community buildings
    Large,     // Manor houses, large workshops
    Massive,   // Castles, major community buildings
    Gigantic,  // City-scale infrastructure
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalStyle {
    pub style_name: String,
    pub cultural_origin: String,
    pub aesthetic_modifiers: HashMap<String, f64>,
    pub material_preferences: Vec<String>,
    pub structural_characteristics: Vec<String>,
    pub climate_adaptations: Vec<String>,
}

/// Specific material requirements for construction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialRequirement {
    pub material_id: Uuid,
    pub quantity_needed: u32,
    pub quality_threshold: f64,
    pub substitution_allowed: bool,
    pub acceptable_substitutes: Vec<MaterialSubstitute>,
    pub procurement_priority: u32,
    pub delivery_phase: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialSubstitute {
    pub substitute_id: Uuid,
    pub conversion_ratio: f64,
    pub quality_modifier: f64,
    pub cost_modifier: f64,
    pub availability_modifier: f64,
}

/// Labor and skill requirements for construction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborRequirements {
    pub required_skills: HashMap<String, SkillRequirement>,
    pub minimum_workers: u32,
    pub optimal_workers: u32,
    pub maximum_workers: u32,
    pub specialized_roles: Vec<SpecializedRole>,
    pub supervision_requirements: SupervisionRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRequirement {
    pub skill_name: String,
    pub minimum_level: f64,
    pub preferred_level: f64,
    pub worker_count: u32,
    pub critical_phases: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedRole {
    pub role_name: String,
    pub required_skills: HashMap<String, f64>,
    pub certification_required: bool,
    pub exclusive_tasks: Vec<String>,
    pub leadership_role: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRequirements {
    pub master_craftsman_required: bool,
    pub architect_oversight: bool,
    pub safety_inspector: bool,
    pub quality_assurance: bool,
    pub supervision_ratios: HashMap<String, f64>,
}

/// Individual phases of construction with specific requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionPhase {
    pub phase_name: String,
    pub phase_order: usize,
    pub estimated_duration: u64, // in game time units
    pub prerequisite_phases: Vec<usize>,
    pub required_materials: Vec<Uuid>,
    pub required_skills: Vec<String>,
    pub weather_dependencies: Vec<WeatherRequirement>,
    pub quality_checkpoints: Vec<QualityCheckpoint>,
    pub safety_considerations: Vec<SafetyRequirement>,
    pub environmental_impact: EnvironmentalImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherRequirement {
    pub weather_type: String,
    pub acceptable_conditions: WeatherConditions,
    pub work_efficiency_modifier: f64,
    pub can_proceed_in_poor_conditions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherConditions {
    pub temperature_range: (f64, f64),
    pub humidity_range: (f64, f64),
    pub wind_speed_max: f64,
    pub precipitation_tolerance: f64,
}

/// Instance of a constructed building in the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingInstance {
    pub instance_id: Uuid,
    pub template_id: Uuid,
    pub owner_id: Uuid,
    pub location: WorldPosition,
    pub orientation: f64,
    pub construction_quality: f64,
    pub current_condition: BuildingCondition,
    pub installed_systems: Vec<BuildingSubsystem>,
    pub occupancy: BuildingOccupancy,
    pub maintenance_history: Vec<MaintenanceRecord>,
    pub modifications: Vec<BuildingModification>,
    pub utilities_connected: HashMap<String, UtilityConnection>,
    pub structural_integrity: StructuralIntegrity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub world_id: Uuid,
    pub region_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingCondition {
    pub overall_condition: f64, // 0.0 to 1.0
    pub structural_condition: f64,
    pub aesthetic_condition: f64,
    pub system_functionality: f64,
    pub wear_patterns: Vec<WearPattern>,
    pub damage_reports: Vec<DamageReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WearPattern {
    pub location: String,
    pub wear_type: WearType,
    pub severity: f64,
    pub progression_rate: f64,
    pub last_maintenance: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WearType {
    Weather,
    Usage,
    Age,
    Negligence,
    Environmental,
    Structural,
}

/// Zoning and land use management
#[derive(Debug, Default)]
pub struct ZoningManager {
    pub zones: HashMap<Uuid, Zone>,
    pub zoning_restrictions: HashMap<Uuid, Vec<ZoningRestriction>>,
    pub land_use_patterns: HashMap<Uuid, LandUsePattern>,
    pub development_regulations: Vec<DevelopmentRegulation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub zone_id: Uuid,
    pub zone_name: String,
    pub zone_type: ZoneType,
    pub boundaries: Vec<WorldPosition>,
    pub allowed_building_types: Vec<BuildingType>,
    pub density_restrictions: DensityRestrictions,
    pub height_restrictions: HeightRestrictions,
    pub environmental_protections: Vec<EnvironmentalProtection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneType {
    Residential,
    Commercial,
    Industrial,
    Agricultural,
    Wilderness,
    Sacred,
    Military,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DensityRestrictions {
    pub max_buildings_per_area: f64,
    pub min_spacing_between_buildings: f64,
    pub max_occupancy_density: f64,
    pub green_space_requirements: f64,
}

/// Infrastructure systems connecting buildings
#[derive(Debug, Default)]
pub struct InfrastructureNetwork {
    pub utility_networks: HashMap<String, UtilityNetwork>,
    pub transportation_networks: HashMap<String, TransportationNetwork>,
    pub communication_networks: HashMap<String, CommunicationNetwork>,
    pub service_coverage: HashMap<Uuid, ServiceCoverage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityNetwork {
    pub network_id: Uuid,
    pub utility_type: UtilityType,
    pub network_nodes: Vec<UtilityNode>,
    pub network_connections: Vec<UtilityConnection>,
    pub capacity_management: CapacityManagement,
    pub maintenance_schedule: MaintenanceSchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UtilityType {
    Water,
    Sewage,
    Power,
    Gas,
    Communications,
    MagicalEnergy,
}

impl BuildingSystem {
    /// Create a new building system
    pub fn new() -> Self {
        Self {
            construction_projects: HashMap::new(),
            building_templates: HashMap::new(),
            #[cfg(feature = "advanced-algorithms")]
            spatial_index: KdTree::new(3),
            zoning_manager: ZoningManager::default(),
            infrastructure_network: InfrastructureNetwork::default(),
            building_permissions: BuildingPermissions::default(),
        }
    }

    /// Start a new construction project
    pub fn start_construction_project(
        &mut self,
        builder_id: Uuid,
        template_id: Uuid,
        location: WorldPosition,
    ) -> Result<Uuid> {
        // Validate zoning and permissions
        self.validate_construction_site(&location, &template_id)?;

        // Create construction project
        let project_id = Uuid::new_v4();
        let template = self.building_templates.get(&template_id)
            .ok_or_else(|| anyhow::anyhow!("Building template not found"))?;

        let project = ConstructionProject {
            project_id,
            builder_id,
            template_id,
            location: location.clone(),
            construction_state: ConstructionState::Planning,
            material_requirements: self.calculate_material_requirements(template)?,
            labor_requirements: self.calculate_labor_requirements(template)?,
            construction_phases: self.generate_construction_phases(template)?,
            current_phase: 0,
            quality_targets: self.establish_quality_targets(template),
            environmental_considerations: self.assess_environmental_factors(&location),
            permits_and_approvals: Vec::new(),
            timeline: self.create_construction_timeline(template),
            budget_tracking: BudgetTracking::new(),
        };

        self.construction_projects.insert(project_id, project);
        Ok(project_id)
    }

    /// Validate that construction can proceed at the specified location
    fn validate_construction_site(&self, location: &WorldPosition, template_id: &Uuid) -> Result<()> {
        // Check zoning restrictions
        if let Some(zone) = self.find_zone_for_location(location) {
            let template = self.building_templates.get(template_id)
                .ok_or_else(|| anyhow::anyhow!("Building template not found"))?;
            
            if !zone.allowed_building_types.contains(&template.building_type) {
                return Err(anyhow::anyhow!("Building type not allowed in this zone"));
            }

            // Check density restrictions
            let nearby_buildings = self.find_nearby_buildings(location, 100.0);
            if nearby_buildings.len() as f64 > zone.density_restrictions.max_buildings_per_area {
                return Err(anyhow::anyhow!("Density restrictions violated"));
            }
        }

        // Check for conflicts with existing buildings
        let nearby_buildings = self.find_nearby_buildings(location, 10.0);
        if !nearby_buildings.is_empty() {
            return Err(anyhow::anyhow!("Too close to existing buildings"));
        }

        Ok(())
    }

    /// Find the zone that contains the given location
    fn find_zone_for_location(&self, location: &WorldPosition) -> Option<&Zone> {
        for zone in self.zoning_manager.zones.values() {
            if self.point_in_zone(location, zone) {
                return Some(zone);
            }
        }
        None
    }

    /// Check if a point is within a zone's boundaries
    fn point_in_zone(&self, point: &WorldPosition, zone: &Zone) -> bool {
        // Simplified point-in-polygon check
        // In practice, this would use a proper geometric algorithm
        if zone.boundaries.len() < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = zone.boundaries.len() - 1;

        for i in 0..zone.boundaries.len() {
            let xi = zone.boundaries[i].x;
            let yi = zone.boundaries[i].y;
            let xj = zone.boundaries[j].x;
            let yj = zone.boundaries[j].y;

            if ((yi > point.y) != (yj > point.y)) && 
               (point.x < (xj - xi) * (point.y - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
            j = i;
        }

        inside
    }

    /// Find buildings near a location using spatial indexing
    #[cfg(feature = "advanced-algorithms")]
    fn find_nearby_buildings(&self, location: &WorldPosition, radius: f64) -> Vec<&BuildingInstance> {
        let search_point = [location.x, location.y, location.z];
        self.spatial_index
            .within(&search_point, radius * radius, &squared_euclidean)
            .unwrap_or_default()
            .into_iter()
            .map(|(_, building)| building)
            .collect()
    }

    /// Find buildings near a location (fallback implementation)
    #[cfg(not(feature = "advanced-algorithms"))]
    fn find_nearby_buildings(&self, _location: &WorldPosition, _radius: f64) -> Vec<&BuildingInstance> {
        // Simple fallback - return empty vec
        Vec::new()
    }

    /// Calculate material requirements for a building template
    fn calculate_material_requirements(&self, template: &BuildingTemplate) -> Result<Vec<MaterialRequirement>> {
        let mut requirements = Vec::new();

        for material_spec in &template.base_materials {
            let requirement = MaterialRequirement {
                material_id: material_spec.material_id,
                quantity_needed: material_spec.base_quantity,
                quality_threshold: material_spec.minimum_quality,
                substitution_allowed: material_spec.substitution_allowed,
                acceptable_substitutes: material_spec.substitutes.clone(),
                procurement_priority: material_spec.priority,
                delivery_phase: material_spec.required_phase,
            };
            requirements.push(requirement);
        }

        Ok(requirements)
    }

    /// Calculate labor requirements for construction
    fn calculate_labor_requirements(&self, template: &BuildingTemplate) -> Result<LaborRequirements> {
        let mut required_skills = HashMap::new();
        
        // Base construction skills
        required_skills.insert("Construction".to_string(), SkillRequirement {
            skill_name: "Construction".to_string(),
            minimum_level: 20.0,
            preferred_level: 40.0,
            worker_count: 2,
            critical_phases: vec![1, 2, 3],
        });

        // Add specialized skills based on building type
        match &template.building_type {
            BuildingType::Workshop { .. } => {
                required_skills.insert("Workshop Design".to_string(), SkillRequirement {
                    skill_name: "Workshop Design".to_string(),
                    minimum_level: 30.0,
                    preferred_level: 50.0,
                    worker_count: 1,
                    critical_phases: vec![2, 3],
                });
            }
            BuildingType::Defense { .. } => {
                required_skills.insert("Fortification".to_string(), SkillRequirement {
                    skill_name: "Fortification".to_string(),
                    minimum_level: 40.0,
                    preferred_level: 60.0,
                    worker_count: 1,
                    critical_phases: vec![1, 2],
                });
            }
            _ => {}
        }

        Ok(LaborRequirements {
            required_skills,
            minimum_workers: 2,
            optimal_workers: 4,
            maximum_workers: 8,
            specialized_roles: vec![],
            supervision_requirements: SupervisionRequirements {
                master_craftsman_required: true,
                architect_oversight: matches!(template.size_category, SizeCategory::Large | SizeCategory::Massive | SizeCategory::Gigantic),
                safety_inspector: true,
                quality_assurance: true,
                supervision_ratios: HashMap::new(),
            },
        })
    }

    /// Generate construction phases for the building
    fn generate_construction_phases(&self, template: &BuildingTemplate) -> Result<Vec<ConstructionPhase>> {
        let mut phases = Vec::new();

        // Foundation phase
        phases.push(ConstructionPhase {
            phase_name: "Foundation".to_string(),
            phase_order: 0,
            estimated_duration: self.estimate_phase_duration(template, "Foundation"),
            prerequisite_phases: vec![],
            required_materials: self.get_foundation_materials(template),
            required_skills: vec!["Construction".to_string(), "Excavation".to_string()],
            weather_dependencies: vec![
                WeatherRequirement {
                    weather_type: "Precipitation".to_string(),
                    acceptable_conditions: WeatherConditions {
                        temperature_range: (0.0, 40.0),
                        humidity_range: (0.0, 80.0),
                        wind_speed_max: 30.0,
                        precipitation_tolerance: 0.1,
                    },
                    work_efficiency_modifier: 0.7,
                    can_proceed_in_poor_conditions: false,
                }
            ],
            quality_checkpoints: vec![
                QualityCheckpoint {
                    checkpoint_name: "Foundation Integrity".to_string(),
                    required_quality: 0.8,
                    inspection_criteria: vec!["Level".to_string(), "Stability".to_string()],
                    failure_consequences: "Rebuild foundation".to_string(),
                }
            ],
            safety_considerations: vec![
                SafetyRequirement {
                    requirement_type: "Excavation Safety".to_string(),
                    mandatory: true,
                    safety_equipment: vec!["Hard Hats".to_string(), "Safety Harnesses".to_string()],
                    training_required: vec!["Excavation Safety".to_string()],
                }
            ],
            environmental_impact: EnvironmentalImpact {
                soil_disturbance: 0.8,
                noise_level: 0.6,
                dust_generation: 0.7,
                wildlife_disruption: 0.4,
                mitigation_measures: vec!["Dust Control".to_string(), "Noise Barriers".to_string()],
            },
        });

        // Structural phase
        phases.push(ConstructionPhase {
            phase_name: "Structural".to_string(),
            phase_order: 1,
            estimated_duration: self.estimate_phase_duration(template, "Structural"),
            prerequisite_phases: vec![0],
            required_materials: self.get_structural_materials(template),
            required_skills: vec!["Construction".to_string(), "Carpentry".to_string(), "Masonry".to_string()],
            weather_dependencies: vec![],
            quality_checkpoints: vec![],
            safety_considerations: vec![],
            environmental_impact: EnvironmentalImpact {
                soil_disturbance: 0.3,
                noise_level: 0.8,
                dust_generation: 0.5,
                wildlife_disruption: 0.3,
                mitigation_measures: vec!["Noise Control".to_string()],
            },
        });

        // Finishing phase
        phases.push(ConstructionPhase {
            phase_name: "Finishing".to_string(),
            phase_order: 2,
            estimated_duration: self.estimate_phase_duration(template, "Finishing"),
            prerequisite_phases: vec![1],
            required_materials: self.get_finishing_materials(template),
            required_skills: vec!["Finishing".to_string(), "Decoration".to_string()],
            weather_dependencies: vec![],
            quality_checkpoints: vec![],
            safety_considerations: vec![],
            environmental_impact: EnvironmentalImpact {
                soil_disturbance: 0.1,
                noise_level: 0.3,
                dust_generation: 0.2,
                wildlife_disruption: 0.1,
                mitigation_measures: vec![],
            },
        });

        Ok(phases)
    }

    /// Advance construction project to next phase
    pub fn advance_construction_phase(&mut self, project_id: Uuid) -> Result<()> {
        // Validate current phase completion first
        {
            let project = self.construction_projects.get(&project_id)
                .ok_or_else(|| anyhow::anyhow!("Construction project not found"))?;
            self.validate_phase_completion(project)?;
        }
        
        let project = self.construction_projects.get_mut(&project_id)
            .ok_or_else(|| anyhow::anyhow!("Construction project not found"))?;

        // Advance to next phase
        if project.current_phase + 1 < project.construction_phases.len() {
            project.current_phase += 1;
            project.construction_state = match project.current_phase {
                0 => ConstructionState::FoundationWork,
                1 => ConstructionState::StructuralWork,
                2 => ConstructionState::Finishing,
                _ => ConstructionState::QualityInspection,
            };
        } else {
            // Construction complete
            project.construction_state = ConstructionState::Completed;
            self.finalize_building_construction(project_id)?;
        }

        Ok(())
    }

    /// Complete construction and create building instance
    fn finalize_building_construction(&mut self, project_id: Uuid) -> Result<()> {
        let project = self.construction_projects.get(&project_id)
            .ok_or_else(|| anyhow::anyhow!("Construction project not found"))?;

        let _building_instance = BuildingInstance {
            instance_id: Uuid::new_v4(),
            template_id: project.template_id,
            owner_id: project.builder_id,
            location: project.location.clone(),
            orientation: 0.0,
            construction_quality: self.calculate_final_quality(project),
            current_condition: BuildingCondition {
                overall_condition: 1.0,
                structural_condition: 1.0,
                aesthetic_condition: 1.0,
                system_functionality: 1.0,
                wear_patterns: vec![],
                damage_reports: vec![],
            },
            installed_systems: vec![],
            occupancy: BuildingOccupancy {
                current_occupants: vec![],
                max_occupancy: 10,
                occupancy_restrictions: vec![],
            },
            maintenance_history: vec![],
            modifications: vec![],
            utilities_connected: HashMap::new(),
            structural_integrity: StructuralIntegrity {
                load_bearing_capacity: 1.0,
                seismic_resistance: 0.7,
                weather_resistance: 0.8,
                fire_resistance: 0.6,
                age_degradation_rate: 0.001,
            },
        };

        // Add to spatial index
        #[cfg(feature = "advanced-algorithms")]
        {
            let position = [
                building_instance.location.x,
                building_instance.location.y,
                building_instance.location.z,
            ];
            self.spatial_index.add(position, building_instance).map_err(|e| anyhow::anyhow!("Failed to add building to spatial index: {:?}", e))?;
        }

        Ok(())
    }

    // Helper methods for material calculation
    fn estimate_phase_duration(&self, _template: &BuildingTemplate, _phase: &str) -> u64 {
        7 // 7 game time units
    }

    fn get_foundation_materials(&self, _template: &BuildingTemplate) -> Vec<Uuid> {
        vec![] // Would return appropriate material IDs
    }

    fn get_structural_materials(&self, _template: &BuildingTemplate) -> Vec<Uuid> {
        vec![] // Would return appropriate material IDs
    }

    fn get_finishing_materials(&self, _template: &BuildingTemplate) -> Vec<Uuid> {
        vec![] // Would return appropriate material IDs
    }

    fn establish_quality_targets(&self, _template: &BuildingTemplate) -> QualityTargets {
        QualityTargets {
            structural_quality: 0.8,
            aesthetic_quality: 0.7,
            functional_quality: 0.9,
            durability_target: 0.8,
        }
    }

    fn assess_environmental_factors(&self, _location: &WorldPosition) -> EnvironmentalFactors {
        EnvironmentalFactors {
            soil_stability: 0.8,
            drainage_quality: 0.7,
            weather_exposure: 0.6,
            natural_hazard_risk: 0.3,
            ecological_sensitivity: 0.4,
        }
    }

    fn create_construction_timeline(&self, _template: &BuildingTemplate) -> ConstructionTimeline {
        ConstructionTimeline {
            estimated_start: SystemTime::now(),
            estimated_completion: SystemTime::now(),
            critical_milestones: vec![],
            buffer_time: 0.2,
        }
    }

    fn validate_phase_completion(&self, _project: &ConstructionProject) -> Result<()> {
        // Validate that current phase is properly completed
        Ok(())
    }

    fn calculate_final_quality(&self, _project: &ConstructionProject) -> f64 {
        0.85 // Simplified quality calculation
    }
}

// Supporting structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialSpec {
    pub material_id: Uuid,
    pub base_quantity: u32,
    pub minimum_quality: f64,
    pub substitution_allowed: bool,
    pub substitutes: Vec<MaterialSubstitute>,
    pub priority: u32,
    pub required_phase: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralRequirements {
    pub load_bearing_capacity: f64,
    pub foundation_depth: f64,
    pub seismic_resistance: f64,
    pub wind_resistance: f64,
    pub fire_resistance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityRequirements {
    pub power_consumption: f64,
    pub water_consumption: f64,
    pub sewage_generation: f64,
    pub heating_requirements: f64,
    pub ventilation_requirements: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalSpace {
    pub space_name: String,
    pub space_type: SpaceType,
    pub minimum_area: f64,
    pub special_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpaceType {
    Living,
    Working,
    Storage,
    Circulation,
    Utility,
    Ceremonial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationOption {
    pub option_name: String,
    pub option_type: CustomizationType,
    pub cost_modifier: f64,
    pub material_changes: Vec<MaterialChange>,
    pub skill_requirements: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomizationType {
    MaterialUpgrade,
    SizeIncrease,
    FeatureAddition,
    StyleModification,
    SystemUpgrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialChange {
    pub original_material: Uuid,
    pub new_material: Uuid,
    pub quantity_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalConstraint {
    pub constraint_type: String,
    pub severity: f64,
    pub mitigation_options: Vec<String>,
    pub compliance_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheckpoint {
    pub checkpoint_name: String,
    pub required_quality: f64,
    pub inspection_criteria: Vec<String>,
    pub failure_consequences: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRequirement {
    pub requirement_type: String,
    pub mandatory: bool,
    pub safety_equipment: Vec<String>,
    pub training_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalImpact {
    pub soil_disturbance: f64,
    pub noise_level: f64,
    pub dust_generation: f64,
    pub wildlife_disruption: f64,
    pub mitigation_measures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTargets {
    pub structural_quality: f64,
    pub aesthetic_quality: f64,
    pub functional_quality: f64,
    pub durability_target: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalFactors {
    pub soil_stability: f64,
    pub drainage_quality: f64,
    pub weather_exposure: f64,
    pub natural_hazard_risk: f64,
    pub ecological_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionTimeline {
    pub estimated_start: SystemTime,
    pub estimated_completion: SystemTime,
    pub critical_milestones: Vec<String>,
    pub buffer_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetTracking {
    pub estimated_cost: f64,
    pub actual_cost: f64,
    pub cost_overruns: Vec<CostOverrun>,
}

impl BudgetTracking {
    pub fn new() -> Self {
        Self {
            estimated_cost: 0.0,
            actual_cost: 0.0,
            cost_overruns: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostOverrun {
    pub reason: String,
    pub amount: f64,
    pub phase: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingOccupancy {
    pub current_occupants: Vec<Uuid>,
    pub max_occupancy: u32,
    pub occupancy_restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceRecord {
    pub maintenance_id: Uuid,
    pub maintenance_type: String,
    pub performed_date: SystemTime,
    pub performed_by: Uuid,
    pub cost: f64,
    pub quality_improvement: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingModification {
    pub modification_id: Uuid,
    pub modification_type: String,
    pub description: String,
    pub cost: f64,
    pub impact_on_functionality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityConnection {
    pub utility_type: UtilityType,
    pub connection_quality: f64,
    pub monthly_cost: f64,
    pub connection_date: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralIntegrity {
    pub load_bearing_capacity: f64,
    pub seismic_resistance: f64,
    pub weather_resistance: f64,
    pub fire_resistance: f64,
    pub age_degradation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoningRestriction {
    pub restriction_type: String,
    pub severity: f64,
    pub compliance_deadline: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandUsePattern {
    pub pattern_id: Uuid,
    pub dominant_use: String,
    pub secondary_uses: Vec<String>,
    pub development_pressure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentRegulation {
    pub regulation_name: String,
    pub applicable_zones: Vec<Uuid>,
    pub requirements: Vec<String>,
    pub penalties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeightRestrictions {
    pub max_height: f64,
    pub max_stories: u32,
    pub setback_requirements: Vec<SetbackRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetbackRequirement {
    pub boundary_type: String,
    pub minimum_distance: f64,
    pub height_scaling: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalProtection {
    pub protection_type: String,
    pub protected_features: Vec<String>,
    pub buffer_zones: Vec<BufferZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferZone {
    pub feature_type: String,
    pub buffer_distance: f64,
    pub restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityNode {
    pub node_id: Uuid,
    pub node_type: NodeType,
    pub location: WorldPosition,
    pub capacity: f64,
    pub current_load: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Source,
    Junction,
    Terminus,
    Booster,
    Control,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityManagement {
    pub total_capacity: f64,
    pub current_utilization: f64,
    pub peak_demand_patterns: Vec<DemandPattern>,
    pub expansion_plans: Vec<ExpansionPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPattern {
    pub time_pattern: String,
    pub demand_multiplier: f64,
    pub seasonal_variations: Vec<SeasonalVariation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalVariation {
    pub season: String,
    pub demand_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionPlan {
    pub plan_id: Uuid,
    pub target_capacity: f64,
    pub estimated_cost: f64,
    pub timeline: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub routine_intervals: HashMap<String, u64>,
    pub preventive_actions: Vec<PreventiveAction>,
    pub emergency_procedures: Vec<EmergencyProcedure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreventiveAction {
    pub action_name: String,
    pub frequency: u64,
    pub target_components: Vec<String>,
    pub cost_estimate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyProcedure {
    pub emergency_type: String,
    pub response_time: u64,
    pub required_resources: Vec<String>,
    pub escalation_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportationNetwork {
    pub network_id: Uuid,
    pub transport_type: TransportType,
    pub routes: Vec<TransportRoute>,
    pub traffic_management: TrafficManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportType {
    Road,
    Rail,
    Waterway,
    Aerial,
    Magical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportRoute {
    pub route_id: Uuid,
    pub waypoints: Vec<WorldPosition>,
    pub capacity: f64,
    pub surface_quality: f64,
    pub maintenance_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficManagement {
    pub flow_patterns: Vec<FlowPattern>,
    pub congestion_points: Vec<CongestionPoint>,
    pub optimization_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPattern {
    pub time_period: String,
    pub traffic_density: f64,
    pub primary_directions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongestionPoint {
    pub location: WorldPosition,
    pub severity: f64,
    pub typical_causes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationNetwork {
    pub network_id: Uuid,
    pub communication_type: CommunicationType,
    pub coverage_areas: Vec<CoverageArea>,
    pub signal_quality: SignalQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationType {
    Messenger,
    Magical,
    Mechanical,
    Optical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageArea {
    pub area_id: Uuid,
    pub boundaries: Vec<WorldPosition>,
    pub signal_strength: f64,
    pub reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalQuality {
    pub clarity: f64,
    pub reliability: f64,
    pub range: f64,
    pub interference_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCoverage {
    pub location: WorldPosition,
    pub available_services: HashMap<String, ServiceLevel>,
    pub service_quality_ratings: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLevel {
    pub service_type: String,
    pub availability: f64,
    pub response_time: u64,
    pub cost: f64,
}

#[derive(Debug, Default)]
pub struct BuildingPermissions {
    pub permits: HashMap<Uuid, BuildingPermit>,
    pub approval_processes: Vec<ApprovalProcess>,
    pub regulatory_compliance: RegulatoryCompliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingPermit {
    pub permit_id: Uuid,
    pub permit_type: PermitType,
    pub issued_date: SystemTime,
    pub expiration_date: SystemTime,
    pub conditions: Vec<String>,
    pub inspection_requirements: Vec<InspectionRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermitType {
    Construction,
    Occupancy,
    Modification,
    Demolition,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionRequirement {
    pub inspection_type: String,
    pub required_at_phase: usize,
    pub inspector_qualifications: Vec<String>,
    pub pass_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalProcess {
    pub process_name: String,
    pub required_for: Vec<BuildingType>,
    pub approval_stages: Vec<ApprovalStage>,
    pub typical_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalStage {
    pub stage_name: String,
    pub required_documents: Vec<String>,
    pub review_authority: String,
    pub stage_duration: u64,
}

#[derive(Debug, Default)]
pub struct RegulatoryCompliance {
    pub building_codes: Vec<BuildingCode>,
    pub safety_standards: Vec<SafetyStandard>,
    pub environmental_regulations: Vec<EnvironmentalRegulation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingCode {
    pub code_name: String,
    pub applicable_building_types: Vec<BuildingType>,
    pub requirements: Vec<CodeRequirement>,
    pub enforcement_level: EnforcementLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRequirement {
    pub requirement_description: String,
    pub compliance_criteria: Vec<String>,
    pub testing_methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Advisory,
    Recommended,
    Mandatory,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyStandard {
    pub standard_name: String,
    pub safety_category: String,
    pub compliance_requirements: Vec<String>,
    pub inspection_frequency: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalRegulation {
    pub regulation_name: String,
    pub environmental_impact_categories: Vec<String>,
    pub mitigation_requirements: Vec<String>,
    pub monitoring_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageReport {
    pub report_id: Uuid,
    pub damage_type: DamageType,
    pub severity: f64,
    pub location: String,
    pub cause: String,
    pub repair_estimate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageType {
    Structural,
    Cosmetic,
    System,
    Environmental,
    Vandalism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingSubsystem {
    pub system_id: Uuid,
    pub system_type: SystemType,
    pub installation_quality: f64,
    pub operational_status: f64,
    pub maintenance_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    Heating,
    Ventilation,
    Plumbing,
    Electrical,
    Security,
    Magical,
}