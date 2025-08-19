use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rand::Rng;
use nalgebra::Vector3;
// use arceon_ai::decision_engine::*; // Module not available

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalSystem {
    pub portal_network: PortalNetwork,
    pub portal_registry: HashMap<Uuid, Portal>,
    pub portal_keys: HashMap<Uuid, PortalKey>,
    pub portal_construction: PortalConstruction,
    pub portal_maintenance: PortalMaintenance,
    pub portal_security: PortalSecurity,
    pub travel_logs: Vec<TravelLog>,
    pub portal_effects: PortalEffects,
    pub dimensional_anchors: HashMap<Uuid, DimensionalAnchor>,
    pub portal_economics: PortalEconomics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalNetwork {
    pub network_topology: NetworkTopology,
    pub hub_portals: Vec<Uuid>,
    pub routing_tables: HashMap<Uuid, RoutingTable>,
    pub network_protocols: NetworkProtocols,
    pub bandwidth_allocation: BandwidthAllocation,
    pub quality_of_service: QualityOfService,
    pub redundancy_paths: HashMap<(Uuid, Uuid), Vec<Vec<Uuid>>>,
    pub network_diagnostics: NetworkDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portal {
    pub id: Uuid,
    pub name: String,
    pub portal_class: PortalClass,
    pub portal_grade: PortalGrade,
    pub activation_state: ActivationState,
    pub origin_anchor: PortalAnchor,
    pub destination_anchor: PortalAnchor,
    pub energy_signature: EnergySignature,
    pub stability_matrix: StabilityMatrix,
    pub dimensional_resonance: f32,
    pub throughput_capacity: ThroughputCapacity,
    pub travel_conditions: TravelConditions,
    pub portal_phenomena: Vec<PortalPhenomenon>,
    pub creation_history: CreationHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalKey {
    pub id: Uuid,
    pub key_type: KeyType,
    pub portal_id: Uuid,
    pub owner_id: Uuid,
    pub access_level: AccessLevel,
    pub usage_count: u32,
    pub usage_limit: Option<u32>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub key_signature: KeySignature,
    pub binding_restrictions: Vec<BindingRestriction>,
    pub transfer_rights: TransferRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalConstruction {
    pub construction_recipes: HashMap<PortalClass, ConstructionRecipe>,
    pub material_requirements: HashMap<PortalClass, MaterialRequirements>,
    pub skill_requirements: HashMap<PortalClass, SkillRequirements>,
    pub construction_time: HashMap<PortalClass, chrono::Duration>,
    pub construction_risks: HashMap<PortalClass, Vec<ConstructionRisk>>,
    pub quality_factors: QualityFactors,
    pub construction_tools: Vec<ConstructionTool>,
    pub safety_protocols: SafetyProtocols,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalMaintenance {
    pub maintenance_schedules: HashMap<Uuid, MaintenanceSchedule>,
    pub repair_procedures: HashMap<String, RepairProcedure>,
    pub degradation_models: DegradationModels,
    pub maintenance_costs: HashMap<PortalClass, MaintenanceCost>,
    pub spare_parts_inventory: SparePartsInventory,
    pub maintenance_crews: Vec<MaintenanceCrew>,
    pub diagnostic_tools: DiagnosticTools,
    pub upgrade_paths: HashMap<PortalClass, Vec<UpgradePath>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalSecurity {
    pub authentication_systems: HashMap<Uuid, AuthenticationSystem>,
    pub access_control_lists: HashMap<Uuid, AccessControlList>,
    pub intrusion_detection: IntrusionDetection,
    pub encryption_protocols: EncryptionProtocols,
    pub security_audits: Vec<SecurityAudit>,
    pub threat_assessment: ThreatAssessment,
    pub emergency_lockdown: EmergencyLockdown,
    pub security_clearances: HashMap<Uuid, SecurityClearance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalEffects {
    pub traversal_effects: HashMap<Uuid, Vec<TraversalEffect>>,
    pub dimensional_sickness: DimensionalSickness,
    pub temporal_displacement: TemporalDisplacement,
    pub reality_distortion: RealityDistortion,
    pub energy_feedback: EnergyFeedback,
    pub psychic_resonance: PsychicResonance,
    pub biological_adaptation: BiologicalAdaptation,
    pub equipment_interference: EquipmentInterference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalAnchor {
    pub id: Uuid,
    pub anchor_type: AnchorType,
    pub dimension_id: Uuid,
    pub coordinates: Vector3<f32>,
    pub stability_field: StabilityField,
    pub power_source: PowerSource,
    pub maintenance_status: MaintenanceStatus,
    pub connected_portals: Vec<Uuid>,
    pub anchor_network: AnchorNetwork,
    pub protective_wards: Vec<ProtectiveWard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalEconomics {
    pub usage_fees: HashMap<PortalClass, UsageFee>,
    pub construction_costs: HashMap<PortalClass, ConstructionCost>,
    pub maintenance_economics: MaintenanceEconomics,
    pub trade_routes: Vec<TradeRoute>,
    pub economic_impact: EconomicImpact,
    pub portal_taxation: PortalTaxation,
    pub commercial_licenses: HashMap<Uuid, CommercialLicense>,
    pub revenue_sharing: RevenueSharingModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelLog {
    pub id: Uuid,
    pub traveler_id: Uuid,
    pub portal_id: Uuid,
    pub origin_dimension: Uuid,
    pub destination_dimension: Uuid,
    pub travel_time: DateTime<Utc>,
    pub travel_duration: chrono::Duration,
    pub energy_consumed: f32,
    pub travel_conditions: String,
    pub incidents: Vec<TravelIncident>,
    pub passenger_manifest: Vec<PassengerRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PortalClass {
    Personal,
    Transport,
    Commercial,
    Military,
    Diplomatic,
    Research,
    Emergency,
    Ceremonial,
    Industrial,
    Experimental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortalGrade {
    Prototype,
    Standard,
    Advanced,
    Master,
    Legendary,
    Artifact,
    Divine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationState {
    Inactive,
    Charging,
    Active,
    Unstable,
    Overloaded,
    Maintenance,
    Emergency,
    Locked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    Personal,
    Master,
    Administrative,
    Emergency,
    Temporary,
    Guild,
    Diplomatic,
    Research,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessLevel {
    None,
    Basic,
    Standard,
    Advanced,
    Master,
    Administrator,
    Unlimited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnchorType {
    Foundation,
    Stabilizing,
    Routing,
    Gateway,
    Emergency,
    Monitoring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalAnchor {
    pub dimension_id: Uuid,
    pub coordinates: Vector3<f32>,
    pub anchor_strength: f32,
    pub dimensional_lock: DimensionalLock,
    pub stability_rating: f32,
    pub anchor_effects: Vec<AnchorEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalLock {
    pub lock_type: LockType,
    pub access_keys: Vec<Uuid>,
    pub lock_strength: f32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LockType {
    Physical,
    Dimensional,
    Temporal,
    Magical,
    Technological,
}

impl Default for DimensionalLock {
    fn default() -> Self {
        Self {
            lock_type: LockType::Dimensional,
            access_keys: Vec::new(),
            lock_strength: 1.0,
            is_active: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergySignature {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub harmonics: Vec<Harmonic>,
    pub resonance_patterns: Vec<ResonancePattern>,
    pub energy_type: EnergyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnergyType {
    Arcane,
    Divine,
    Primal,
    Psionic,
    Elemental,
    Void,
    Temporal,
    Quantum,
}

impl Default for PortalSystem {
    fn default() -> Self {
        Self {
            portal_network: PortalNetwork::default(),
            portal_registry: HashMap::new(),
            portal_keys: HashMap::new(),
            portal_construction: PortalConstruction::default(),
            portal_maintenance: PortalMaintenance::default(),
            portal_security: PortalSecurity::default(),
            travel_logs: Vec::new(),
            portal_effects: PortalEffects::default(),
            dimensional_anchors: HashMap::new(),
            portal_economics: PortalEconomics::default(),
        }
    }
}

impl Default for PortalNetwork {
    fn default() -> Self {
        Self {
            network_topology: NetworkTopology::default(),
            hub_portals: Vec::new(),
            routing_tables: HashMap::new(),
            network_protocols: NetworkProtocols::default(),
            bandwidth_allocation: BandwidthAllocation::default(),
            quality_of_service: QualityOfService::default(),
            redundancy_paths: HashMap::new(),
            network_diagnostics: NetworkDiagnostics::default(),
        }
    }
}

impl Default for PortalConstruction {
    fn default() -> Self {
        Self {
            construction_recipes: HashMap::new(),
            material_requirements: HashMap::new(),
            skill_requirements: HashMap::new(),
            construction_time: HashMap::new(),
            construction_risks: HashMap::new(),
            quality_factors: QualityFactors::default(),
            construction_tools: Vec::new(),
            safety_protocols: SafetyProtocols::default(),
        }
    }
}

impl Default for PortalMaintenance {
    fn default() -> Self {
        Self {
            maintenance_schedules: HashMap::new(),
            repair_procedures: HashMap::new(),
            degradation_models: DegradationModels::default(),
            maintenance_costs: HashMap::new(),
            spare_parts_inventory: SparePartsInventory::default(),
            maintenance_crews: Vec::new(),
            diagnostic_tools: DiagnosticTools::default(),
            upgrade_paths: HashMap::new(),
        }
    }
}

impl Default for PortalSecurity {
    fn default() -> Self {
        Self {
            authentication_systems: HashMap::new(),
            access_control_lists: HashMap::new(),
            intrusion_detection: IntrusionDetection::default(),
            encryption_protocols: EncryptionProtocols::default(),
            security_audits: Vec::new(),
            threat_assessment: ThreatAssessment::default(),
            emergency_lockdown: EmergencyLockdown::default(),
            security_clearances: HashMap::new(),
        }
    }
}

impl Default for PortalEffects {
    fn default() -> Self {
        Self {
            traversal_effects: HashMap::new(),
            dimensional_sickness: DimensionalSickness::default(),
            temporal_displacement: TemporalDisplacement::default(),
            reality_distortion: RealityDistortion::default(),
            energy_feedback: EnergyFeedback::default(),
            psychic_resonance: PsychicResonance::default(),
            biological_adaptation: BiologicalAdaptation::default(),
            equipment_interference: EquipmentInterference::default(),
        }
    }
}

impl Default for PortalEconomics {
    fn default() -> Self {
        Self {
            usage_fees: HashMap::new(),
            construction_costs: HashMap::new(),
            maintenance_economics: MaintenanceEconomics::default(),
            trade_routes: Vec::new(),
            economic_impact: EconomicImpact::default(),
            portal_taxation: PortalTaxation::default(),
            commercial_licenses: HashMap::new(),
            revenue_sharing: RevenueSharingModel::default(),
        }
    }
}

macro_rules! impl_default_for_structs {
    ($($name:ident),*) => {
        $(
            #[derive(Debug, Clone, Serialize, Deserialize, Default)]
            pub struct $name {
                pub placeholder: String,
            }
        )*
    };
}

impl_default_for_structs!(
    NetworkTopology, RoutingTable, NetworkProtocols, BandwidthAllocation,
    QualityOfService, NetworkDiagnostics, StabilityMatrix, ThroughputCapacity,
    TravelConditions, PortalPhenomenon, CreationHistory, KeySignature,
    BindingRestriction, TransferRights, ConstructionRecipe, MaterialRequirements,
    SkillRequirements, ConstructionRisk, QualityFactors, ConstructionTool,
    SafetyProtocols, MaintenanceSchedule, RepairProcedure, DegradationModels,
    MaintenanceCost, SparePartsInventory, MaintenanceCrew, DiagnosticTools,
    UpgradePath, AuthenticationSystem, AccessControlList, IntrusionDetection,
    EncryptionProtocols, SecurityAudit, ThreatAssessment, EmergencyLockdown,
    SecurityClearance, TraversalEffect, DimensionalSickness, TemporalDisplacement,
    RealityDistortion, EnergyFeedback, PsychicResonance, BiologicalAdaptation,
    EquipmentInterference, StabilityField, PowerSource, MaintenanceStatus,
    AnchorNetwork, ProtectiveWard, UsageFee, ConstructionCost, MaintenanceEconomics,
    TradeRoute, EconomicImpact, PortalTaxation, CommercialLicense,
    RevenueSharingModel, TravelIncident, PassengerRecord,
    AnchorEffect, Harmonic, ResonancePattern
);

impl PortalSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize_portal_infrastructure(&mut self) -> Result<(), String> {
        // Initialize construction recipes for different portal classes
        self.setup_construction_recipes()?;
        
        // Initialize maintenance schedules
        self.setup_maintenance_systems()?;
        
        // Initialize security protocols
        self.setup_security_systems()?;
        
        // Initialize economic models
        self.setup_economic_systems()?;
        
        Ok(())
    }

    fn setup_construction_recipes(&mut self) -> Result<(), String> {
        let portal_classes = vec![
            PortalClass::Personal,
            PortalClass::Transport,
            PortalClass::Commercial,
            PortalClass::Military,
            PortalClass::Research,
        ];

        for class in portal_classes {
            let recipe = ConstructionRecipe::default();
            let materials = MaterialRequirements::default();
            let skills = SkillRequirements::default();
            let time = match class {
                PortalClass::Personal => chrono::Duration::hours(24),
                PortalClass::Transport => chrono::Duration::days(7),
                PortalClass::Commercial => chrono::Duration::days(30),
                PortalClass::Military => chrono::Duration::days(60),
                PortalClass::Research => chrono::Duration::days(90),
                _ => chrono::Duration::days(14),
            };

            self.portal_construction.construction_recipes.insert(class.clone(), recipe);
            self.portal_construction.material_requirements.insert(class.clone(), materials);
            self.portal_construction.skill_requirements.insert(class.clone(), skills);
            self.portal_construction.construction_time.insert(class.clone(), time);
        }

        Ok(())
    }

    fn setup_maintenance_systems(&mut self) -> Result<(), String> {
        // Initialize degradation models
        self.portal_maintenance.degradation_models = DegradationModels::default();
        
        // Set up maintenance costs
        for class in [PortalClass::Personal, PortalClass::Transport, PortalClass::Commercial] {
            let cost = MaintenanceCost::default();
            self.portal_maintenance.maintenance_costs.insert(class, cost);
        }

        Ok(())
    }

    fn setup_security_systems(&mut self) -> Result<(), String> {
        // Initialize threat assessment
        self.portal_security.threat_assessment = ThreatAssessment::default();
        
        // Set up encryption protocols
        self.portal_security.encryption_protocols = EncryptionProtocols::default();
        
        Ok(())
    }

    fn setup_economic_systems(&mut self) -> Result<(), String> {
        // Initialize usage fees
        for class in [PortalClass::Personal, PortalClass::Transport, PortalClass::Commercial] {
            let fee = UsageFee::default();
            self.portal_economics.usage_fees.insert(class, fee);
        }

        Ok(())
    }

    pub fn construct_portal(&mut self, 
        constructor_id: Uuid,
        portal_class: PortalClass,
        origin_anchor: PortalAnchor,
        destination_anchor: PortalAnchor,
        name: String
    ) -> Result<Uuid, String> {
        
        // Check if constructor has required skills and materials
        if !self.has_construction_requirements(&constructor_id, &portal_class)? {
            return Err("Insufficient requirements for portal construction".to_string());
        }

        let portal = Portal {
            id: Uuid::new_v4(),
            name,
            portal_class: portal_class.clone(),
            portal_grade: PortalGrade::Standard,
            activation_state: ActivationState::Inactive,
            origin_anchor,
            destination_anchor,
            energy_signature: EnergySignature {
                frequency: rand::thread_rng().gen_range(1.0..100.0),
                amplitude: rand::thread_rng().gen_range(0.5..2.0),
                phase: rand::thread_rng().gen_range(0.0..360.0),
                harmonics: Vec::new(),
                resonance_patterns: Vec::new(),
                energy_type: EnergyType::Arcane,
            },
            stability_matrix: StabilityMatrix::default(),
            dimensional_resonance: 1.0,
            throughput_capacity: ThroughputCapacity::default(),
            travel_conditions: TravelConditions::default(),
            portal_phenomena: Vec::new(),
            creation_history: CreationHistory::default(),
        };

        let portal_id = portal.id;
        self.portal_registry.insert(portal_id, portal);

        // Create a master key for the constructor
        self.create_portal_key(portal_id, constructor_id, KeyType::Master, AccessLevel::Master)?;

        Ok(portal_id)
    }

    pub fn create_portal_key(&mut self, 
        portal_id: Uuid, 
        owner_id: Uuid, 
        key_type: KeyType, 
        access_level: AccessLevel
    ) -> Result<Uuid, String> {
        if !self.portal_registry.contains_key(&portal_id) {
            return Err("Portal does not exist".to_string());
        }

        let key = PortalKey {
            id: Uuid::new_v4(),
            key_type,
            portal_id,
            owner_id,
            access_level,
            usage_count: 0,
            usage_limit: None,
            expiration_date: None,
            key_signature: KeySignature::default(),
            binding_restrictions: Vec::new(),
            transfer_rights: TransferRights::default(),
        };

        let key_id = key.id;
        self.portal_keys.insert(key_id, key);
        Ok(key_id)
    }

    pub fn activate_portal(&mut self, portal_id: Uuid, activator_id: Uuid) -> Result<(), String> {
        // Check if user has activation rights
        if !self.has_portal_access(portal_id, activator_id, AccessLevel::Standard)? {
            return Err("Insufficient access rights to activate portal".to_string());
        }

        if let Some(portal) = self.portal_registry.get_mut(&portal_id) {
            match portal.activation_state {
                ActivationState::Inactive => {
                    portal.activation_state = ActivationState::Charging;
                    // Portal will transition to Active after charging period
                    Ok(())
                }
                ActivationState::Active => Ok(()),
                ActivationState::Maintenance => Err("Portal is under maintenance".to_string()),
                ActivationState::Locked => Err("Portal is locked".to_string()),
                _ => Err("Portal cannot be activated in current state".to_string()),
            }
        } else {
            Err("Portal not found".to_string())
        }
    }

    pub fn travel_through_portal(&mut self, 
        portal_id: Uuid, 
        traveler_id: Uuid, 
        companions: Vec<Uuid>
    ) -> Result<TravelResult, String> {
        
        // Check portal access
        if !self.has_portal_access(portal_id, traveler_id, AccessLevel::Basic)? {
            return Err("Access denied".to_string());
        }

        let portal = self.portal_registry.get(&portal_id)
            .ok_or("Portal not found")?;

        if !matches!(portal.activation_state, ActivationState::Active) {
            return Err("Portal is not active".to_string());
        }

        let travel_time = Utc::now();
        let travel_duration = chrono::Duration::seconds(rand::thread_rng().gen_range(1..10));
        let energy_consumed = 10.0 * (1 + companions.len()) as f32;

        // Create travel log
        let mut passenger_manifest = vec![PassengerRecord { placeholder: format!("Traveler: {}", traveler_id) }];
        for companion in &companions {
            passenger_manifest.push(PassengerRecord { placeholder: format!("Companion: {}", companion) });
        }

        let travel_log = TravelLog {
            id: Uuid::new_v4(),
            traveler_id,
            portal_id,
            origin_dimension: portal.origin_anchor.dimension_id,
            destination_dimension: portal.destination_anchor.dimension_id,
            travel_time,
            travel_duration,
            energy_consumed,
            travel_conditions: "Normal".to_string(),
            incidents: Vec::new(),
            passenger_manifest,
        };

        self.travel_logs.push(travel_log);

        // Calculate travel effects
        let effects = self.calculate_travel_effects(portal_id, traveler_id)?;

        Ok(TravelResult {
            destination_dimension: portal.destination_anchor.dimension_id,
            destination_coordinates: portal.destination_anchor.coordinates,
            travel_effects: effects,
            energy_cost: energy_consumed,
            travel_time: travel_duration,
        })
    }

    pub fn maintain_portal(&mut self, portal_id: Uuid, maintainer_id: Uuid) -> Result<MaintenanceResult, String> {
        // Check access rights first
        if !self.has_portal_access(portal_id, maintainer_id, AccessLevel::Advanced)? {
            return Err("Insufficient access rights for maintenance".to_string());
        }
        
        if let Some(portal) = self.portal_registry.get_mut(&portal_id) {
            portal.activation_state = ActivationState::Maintenance;
            
            // Perform maintenance calculations
            let stability_improvement = 0.1;
            portal.dimensional_resonance = (portal.dimensional_resonance + stability_improvement).min(1.0);

            // Schedule next maintenance
            let schedule = MaintenanceSchedule::default();
            self.portal_maintenance.maintenance_schedules.insert(portal_id, schedule);

            portal.activation_state = ActivationState::Inactive;

            Ok(MaintenanceResult {
                success: true,
                stability_improvement,
                next_maintenance: Utc::now() + chrono::Duration::days(30),
                maintenance_cost: 100.0,
            })
        } else {
            Err("Portal not found".to_string())
        }
    }

    fn has_construction_requirements(&self, _constructor_id: &Uuid, _portal_class: &PortalClass) -> Result<bool, String> {
        // Simplified check - in real implementation would check materials, skills, etc.
        Ok(true)
    }

    fn has_portal_access(&self, portal_id: Uuid, user_id: Uuid, _required_level: AccessLevel) -> Result<bool, String> {
        for key in self.portal_keys.values() {
            if key.portal_id == portal_id && key.owner_id == user_id {
                // Simplified access level comparison
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn calculate_travel_effects(&self, _portal_id: Uuid, _traveler_id: Uuid) -> Result<Vec<String>, String> {
        let mut effects = Vec::new();
        
        // Random travel effects
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.1 {
            effects.push("Mild dimensional disorientation".to_string());
        }
        if rng.gen::<f32>() < 0.05 {
            effects.push("Temporal displacement (minor)".to_string());
        }
        
        Ok(effects)
    }

    pub fn get_portal_network_status(&self) -> NetworkStatus {
        NetworkStatus {
            total_portals: self.portal_registry.len(),
            active_portals: self.portal_registry.values()
                .filter(|p| matches!(p.activation_state, ActivationState::Active))
                .count(),
            network_health: 0.95, // Calculated based on various factors
            average_stability: self.portal_registry.values()
                .map(|p| p.dimensional_resonance)
                .sum::<f32>() / self.portal_registry.len() as f32,
        }
    }

    pub fn get_portal_statistics(&self) -> PortalStatistics {
        PortalStatistics {
            total_travels: self.travel_logs.len(),
            total_energy_consumed: self.travel_logs.iter()
                .map(|log| log.energy_consumed)
                .sum(),
            most_used_portal: self.get_most_used_portal(),
            average_travel_time: self.calculate_average_travel_time(),
        }
    }

    fn get_most_used_portal(&self) -> Option<Uuid> {
        let mut usage_counts = HashMap::new();
        for log in &self.travel_logs {
            *usage_counts.entry(log.portal_id).or_insert(0) += 1;
        }
        usage_counts.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&portal_id, _)| portal_id)
    }

    fn calculate_average_travel_time(&self) -> chrono::Duration {
        if self.travel_logs.is_empty() {
            chrono::Duration::zero()
        } else {
            let total_seconds: i64 = self.travel_logs.iter()
                .map(|log| log.travel_duration.num_seconds())
                .sum();
            chrono::Duration::seconds(total_seconds / self.travel_logs.len() as i64)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelResult {
    pub destination_dimension: Uuid,
    pub destination_coordinates: Vector3<f32>,
    pub travel_effects: Vec<String>,
    pub energy_cost: f32,
    pub travel_time: chrono::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceResult {
    pub success: bool,
    pub stability_improvement: f32,
    pub next_maintenance: DateTime<Utc>,
    pub maintenance_cost: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub total_portals: usize,
    pub active_portals: usize,
    pub network_health: f32,
    pub average_stability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalStatistics {
    pub total_travels: usize,
    pub total_energy_consumed: f32,
    pub most_used_portal: Option<Uuid>,
    pub average_travel_time: chrono::Duration,
}