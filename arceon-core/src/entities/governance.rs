use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// NPC Governance and Autonomous Town/City Building System
/// This system allows NPCs to form governments, establish settlements, and build civilizations

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceSystem {
    pub settlements: HashMap<Uuid, Settlement>,
    pub governments: HashMap<Uuid, Government>,
    pub leaders: HashMap<Uuid, Leader>,
    pub policies: HashMap<Uuid, Policy>,
    pub construction_projects: HashMap<Uuid, ConstructionProject>,
    pub governance_events: Vec<GovernanceEvent>,
    pub political_relationships: Vec<PoliticalRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub settlement_id: Uuid,
    pub name: String,
    pub settlement_type: SettlementType,
    pub location: String,
    pub founder_id: Uuid,
    pub population: Vec<Uuid>,              // NPC IDs
    pub government_id: Option<Uuid>,
    pub infrastructure: Infrastructure,
    pub resources: HashMap<String, f64>,
    pub economic_status: EconomicStatus,
    pub cultural_identity: CulturalIdentity,
    pub establishment_date: DateTime<Utc>,
    pub growth_stage: GrowthStage,
    pub specializations: Vec<String>,
    pub trade_routes: Vec<TradeRoute>,
    pub defensive_capabilities: DefensiveCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementType {
    Camp,           // Temporary settlement
    Village,        // Small permanent settlement
    Town,           // Medium settlement with services
    City,           // Large settlement with complex systems
    Metropolis,     // Major urban center
    Fortress,       // Defensive settlement
    TradingPost,    // Commercial hub
    MonasticCommunity, // Religious community
    ScholarlyEnclave,  // Academic settlement
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infrastructure {
    pub housing_units: u32,
    pub workshop_facilities: u32,
    pub storage_capacity: f64,
    pub roads_built: u32,
    pub water_systems: WaterSystem,
    pub waste_management: WasteManagement,
    pub defensive_walls: bool,
    pub marketplace: bool,
    pub library: Option<Uuid>,
    pub school: Option<Uuid>,
    pub religious_buildings: Vec<Uuid>,
    pub administrative_buildings: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterSystem {
    pub wells: u32,
    pub aqueducts: u32,
    pub cisterns: u32,
    pub quality: f64,           // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteManagement {
    pub sanitation_level: f64,  // 0.0 to 1.0
    pub waste_disposal_systems: u32,
    pub composting_facilities: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicStatus {
    pub prosperity_level: f64,      // 0.0 to 1.0
    pub trade_volume: f64,
    pub employment_rate: f64,
    pub wealth_distribution: WealthDistribution,
    pub primary_industries: Vec<String>,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WealthDistribution {
    Equal,
    Moderate,
    Concentrated,
    Extreme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalIdentity {
    pub traditions: Vec<String>,
    pub values: Vec<String>,
    pub language: String,
    pub artistic_styles: Vec<String>,
    pub religious_beliefs: Vec<String>,
    pub social_norms: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GrowthStage {
    Founding,
    Establishment,
    Growth,
    Maturity,
    Expansion,
    Decline,
    Renewal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    pub route_id: Uuid,
    pub destination_settlement: Uuid,
    pub goods_traded: Vec<String>,
    pub frequency: f64,         // Trades per time period
    pub profitability: f64,
    pub safety_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefensiveCapabilities {
    pub walls: bool,
    pub guards: u32,
    pub watchtowers: u32,
    pub militia_size: u32,
    pub defensive_rating: f64,
    pub siege_supplies: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Government {
    pub government_id: Uuid,
    pub settlement_id: Uuid,
    pub government_type: GovernmentType,
    pub leader_id: Uuid,
    pub council_members: Vec<Uuid>,
    pub policies: Vec<Uuid>,
    pub laws: Vec<Law>,
    pub tax_system: TaxSystem,
    pub justice_system: JusticeSystem,
    pub legitimacy: f64,        // 0.0 to 1.0
    pub effectiveness: f64,     // 0.0 to 1.0
    pub establishment_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentType {
    Autocracy,      // Single ruler
    Oligarchy,      // Rule by few
    Democracy,      // Rule by majority
    Council,        // Council of elders/experts
    Theocracy,      // Religious rule
    Meritocracy,    // Rule by merit/expertise
    Confederation,  // Loose alliance
    Anarchy,        // No formal government
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leader {
    pub leader_id: Uuid,
    pub npc_id: Uuid,
    pub leadership_style: LeadershipStyle,
    pub competencies: Vec<LeadershipCompetency>,
    pub approval_rating: f64,
    pub experience: f64,
    pub vision: Vec<String>,
    pub term_start: DateTime<Utc>,
    pub term_length: Option<u32>,   // None for indefinite terms
    pub achievements: Vec<String>,
    pub challenges_faced: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeadershipStyle {
    Authoritarian,
    Democratic,
    Collaborative,
    Visionary,
    Pragmatic,
    Charismatic,
    Technocratic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeadershipCompetency {
    Economic,
    Military,
    Diplomatic,
    Administrative,
    Cultural,
    Technological,
    Social,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Law {
    pub law_id: Uuid,
    pub name: String,
    pub description: String,
    pub enforcement_level: f64,
    pub penalties: Vec<String>,
    pub public_support: f64,
    pub enactment_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSystem {
    pub tax_rate: f64,
    pub tax_types: Vec<TaxType>,
    pub collection_efficiency: f64,
    pub revenue: f64,
    pub expenditures: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaxType {
    Income,
    Property,
    Trade,
    Luxury,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JusticeSystem {
    pub court_system: bool,
    pub judges: Vec<Uuid>,
    pub law_enforcement: Vec<Uuid>,
    pub prison_capacity: u32,
    pub justice_rating: f64,    // Fairness and effectiveness
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub policy_id: Uuid,
    pub name: String,
    pub description: String,
    pub policy_type: PolicyType,
    pub effectiveness: f64,
    pub public_support: f64,
    pub implementation_date: DateTime<Utc>,
    pub review_date: Option<DateTime<Utc>>,
    pub outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    Economic,
    Social,
    Infrastructure,
    Defense,
    Education,
    Healthcare,
    Environment,
    Cultural,
    Trade,
    Immigration,
    Administrative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionProject {
    pub project_id: Uuid,
    pub name: String,
    pub project_type: ProjectType,
    pub settlement_id: Uuid,
    pub initiator_id: Uuid,
    pub architects: Vec<Uuid>,
    pub workers: Vec<Uuid>,
    pub required_resources: HashMap<String, f64>,
    pub allocated_resources: HashMap<String, f64>,
    pub progress: f64,          // 0.0 to 1.0
    pub estimated_completion: DateTime<Utc>,
    pub priority: Priority,
    pub public_support: f64,
    pub funding_source: FundingSource,
    pub challenges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    Housing,
    Infrastructure,
    Commercial,
    Religious,
    Educational,
    Administrative,
    Defensive,
    Cultural,
    Industrial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FundingSource {
    Government,
    Private,
    Community,
    Trade,
    Tribute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceEvent {
    pub event_id: Uuid,
    pub event_type: GovernanceEventType,
    pub participants: Vec<Uuid>,
    pub settlement_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub significance: f64,
    pub outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceEventType {
    SettlementFounding(Uuid),
    GovernmentEstablishment(Uuid),
    LeadershipChange(Uuid, Uuid), // old leader, new leader
    PolicyEnactment(Uuid),
    ConstructionCompletion(Uuid),
    TradeAgreement(Uuid, Uuid),   // settlement 1, settlement 2
    War(Uuid, Uuid),
    Peace(Uuid, Uuid),
    Revolution,
    Reform,
    Crisis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalRelationship {
    pub relationship_id: Uuid,
    pub entity1: PoliticalEntity,
    pub entity2: PoliticalEntity,
    pub relationship_type: RelationshipType,
    pub strength: f64,          // -1.0 to 1.0
    pub history: Vec<RelationshipEvent>,
    pub trade_agreements: Vec<Uuid>,
    pub military_agreements: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoliticalEntity {
    Settlement(Uuid),
    Government(Uuid),
    Leader(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Allied,
    Friendly,
    Neutral,
    Tense,
    Hostile,
    Vassal,
    Overlord,
    Trading,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEvent {
    pub event_description: String,
    pub impact: f64,
    pub timestamp: DateTime<Utc>,
}

impl GovernanceSystem {
    pub fn new() -> Self {
        Self {
            settlements: HashMap::new(),
            governments: HashMap::new(),
            leaders: HashMap::new(),
            policies: HashMap::new(),
            construction_projects: HashMap::new(),
            governance_events: Vec::new(),
            political_relationships: Vec::new(),
        }
    }

    /// NPCs autonomously found a new settlement
    pub fn found_settlement(&mut self, founder_id: Uuid, location: String, settlement_type: SettlementType) -> Uuid {
        let settlement_id = Uuid::new_v4();
        let name = self.generate_settlement_name(&settlement_type);

        let settlement = Settlement {
            settlement_id,
            name: name.clone(),
            settlement_type: settlement_type.clone(),
            location: location.clone(),
            founder_id,
            population: vec![founder_id],
            government_id: None,
            infrastructure: Infrastructure {
                housing_units: 1,
                workshop_facilities: 0,
                storage_capacity: 10.0,
                roads_built: 0,
                water_systems: WaterSystem {
                    wells: 1,
                    aqueducts: 0,
                    cisterns: 1,
                    quality: 0.7,
                },
                waste_management: WasteManagement {
                    sanitation_level: 0.5,
                    waste_disposal_systems: 0,
                    composting_facilities: 1,
                },
                defensive_walls: false,
                marketplace: false,
                library: None,
                school: None,
                religious_buildings: Vec::new(),
                administrative_buildings: Vec::new(),
            },
            resources: HashMap::from([
                ("Food".to_string(), 20.0),
                ("Wood".to_string(), 15.0),
                ("Stone".to_string(), 10.0),
            ]),
            economic_status: EconomicStatus {
                prosperity_level: 0.3,
                trade_volume: 0.0,
                employment_rate: 1.0,
                wealth_distribution: WealthDistribution::Equal,
                primary_industries: vec!["Gathering".to_string()],
                exports: Vec::new(),
                imports: Vec::new(),
            },
            cultural_identity: CulturalIdentity {
                traditions: vec!["Foundation Day".to_string()],
                values: vec!["Self-sufficiency".to_string(), "Cooperation".to_string()],
                language: "Common Tongue".to_string(),
                artistic_styles: Vec::new(),
                religious_beliefs: Vec::new(),
                social_norms: vec!["Mutual aid".to_string()],
            },
            establishment_date: Utc::now(),
            growth_stage: GrowthStage::Founding,
            specializations: Vec::new(),
            trade_routes: Vec::new(),
            defensive_capabilities: DefensiveCapabilities {
                walls: false,
                guards: 0,
                watchtowers: 0,
                militia_size: 1,
                defensive_rating: 0.2,
                siege_supplies: 5.0,
            },
        };

        self.settlements.insert(settlement_id, settlement);

        // Record the founding event
        self.governance_events.push(GovernanceEvent {
            event_id: Uuid::new_v4(),
            event_type: GovernanceEventType::SettlementFounding(settlement_id),
            participants: vec![founder_id],
            settlement_id: Some(settlement_id),
            timestamp: Utc::now(),
            significance: 0.8,
            outcomes: vec![format!("Settlement {} founded", name)],
        });

        settlement_id
    }

    /// Establish government for a settlement
    pub fn establish_government(&mut self, settlement_id: Uuid, leader_id: Uuid, government_type: GovernmentType) -> Result<Uuid, String> {
        let settlement = self.settlements.get_mut(&settlement_id)
            .ok_or("Settlement not found")?;

        if settlement.government_id.is_some() {
            return Err("Settlement already has a government".to_string());
        }

        let government_id = Uuid::new_v4();

        let government = Government {
            government_id,
            settlement_id,
            government_type: government_type.clone(),
            leader_id,
            council_members: Vec::new(),
            policies: Vec::new(),
            laws: vec![
                Law {
                    law_id: Uuid::new_v4(),
                    name: "Basic Order".to_string(),
                    description: "Maintain peace and order within the settlement".to_string(),
                    enforcement_level: 0.8,
                    penalties: vec!["Warning".to_string(), "Expulsion".to_string()],
                    public_support: 0.9,
                    enactment_date: Utc::now(),
                }
            ],
            tax_system: TaxSystem {
                tax_rate: 0.1,
                tax_types: vec![TaxType::Trade],
                collection_efficiency: 0.7,
                revenue: 0.0,
                expenditures: HashMap::new(),
            },
            justice_system: JusticeSystem {
                court_system: false,
                judges: vec![leader_id],
                law_enforcement: vec![leader_id],
                prison_capacity: 0,
                justice_rating: 0.6,
            },
            legitimacy: 0.8,
            effectiveness: 0.6,
            establishment_date: Utc::now(),
        };

        let leader = Leader {
            leader_id: Uuid::new_v4(),
            npc_id: leader_id,
            leadership_style: LeadershipStyle::Pragmatic,
            competencies: vec![LeadershipCompetency::Administrative],
            approval_rating: 0.8,
            experience: 0.0,
            vision: vec!["Growth and prosperity".to_string()],
            term_start: Utc::now(),
            term_length: None,
            achievements: Vec::new(),
            challenges_faced: Vec::new(),
        };

        settlement.government_id = Some(government_id);
        self.governments.insert(government_id, government);
        self.leaders.insert(leader.leader_id, leader);

        // Record the event
        self.governance_events.push(GovernanceEvent {
            event_id: Uuid::new_v4(),
            event_type: GovernanceEventType::GovernmentEstablishment(government_id),
            participants: vec![leader_id],
            settlement_id: Some(settlement_id),
            timestamp: Utc::now(),
            significance: 0.9,
            outcomes: vec![format!("Government established with type: {:?}", government_type)],
        });

        Ok(government_id)
    }

    /// Initiate autonomous construction project
    pub fn initiate_construction_project(&mut self, settlement_id: Uuid, initiator_id: Uuid, project_type: ProjectType, name: String) -> Result<Uuid, String> {
        let _settlement = self.settlements.get(&settlement_id)
            .ok_or("Settlement not found")?;

        let project_id = Uuid::new_v4();
        let required_resources = self.calculate_required_resources(&project_type);

        let project = ConstructionProject {
            project_id,
            name: name.clone(),
            project_type: project_type.clone(),
            settlement_id,
            initiator_id,
            architects: Vec::new(),
            workers: Vec::new(),
            required_resources: required_resources.clone(),
            allocated_resources: HashMap::new(),
            progress: 0.0,
            estimated_completion: Utc::now() + chrono::Duration::weeks(4),
            priority: Priority::Medium,
            public_support: 0.6,
            funding_source: FundingSource::Community,
            challenges: Vec::new(),
        };

        self.construction_projects.insert(project_id, project);

        Ok(project_id)
    }

    /// Autonomous settlement growth and development
    pub fn process_settlement_growth(&mut self, settlement_id: Uuid) -> Result<Vec<String>, String> {
        let mut developments = Vec::new();
        
        // Get settlement data without mutable borrow
        let (founder_id, population_size, housing_units, has_marketplace, current_stage) = {
            let settlement = self.settlements.get(&settlement_id)
                .ok_or("Settlement not found")?;
            (
                settlement.founder_id,
                settlement.population.len(),
                settlement.infrastructure.housing_units,
                settlement.infrastructure.marketplace,
                settlement.growth_stage
            )
        };

        // Check if settlement should grow
        if population_size > 10 && housing_units < population_size as u32 {
            // Need more housing
            let _project_id = self.initiate_construction_project(
                settlement_id, 
                founder_id,
                ProjectType::Housing,
                "Expansion Housing".to_string()
            )?;
            developments.push("Housing expansion project initiated".to_string());
        }

        // Check for infrastructure needs
        if population_size > 20 && !has_marketplace {
            let _project_id = self.initiate_construction_project(
                settlement_id,
                founder_id,
                ProjectType::Commercial,
                "Central Marketplace".to_string()
            )?;
            developments.push("Marketplace construction initiated".to_string());
        }

        // Update growth stage
        let new_stage = match population_size {
            0..=5 => GrowthStage::Founding,
            6..=15 => GrowthStage::Establishment,
            16..=50 => GrowthStage::Growth,
            51..=150 => GrowthStage::Maturity,
            _ => GrowthStage::Expansion,
        };

        if current_stage != new_stage {
            if let Some(settlement) = self.settlements.get_mut(&settlement_id) {
                settlement.growth_stage = new_stage;
                developments.push(format!("Settlement entered new growth stage: {:?}", new_stage));
            }
        }

        Ok(developments)
    }

    /// NPCs make autonomous governance decisions
    pub fn process_governance_decisions(&mut self, government_id: Uuid) -> Result<Vec<String>, String> {
        let mut decisions = Vec::new();
        let mut new_policies = Vec::new();
        
        // Get government data without holding mutable borrow
        let (settlement_id, effectiveness) = {
            let government = self.governments.get(&government_id)
                .ok_or("Government not found")?;
            (government.settlement_id, government.effectiveness)
        };

        // Check if new policies are needed
        if effectiveness < 0.5 {
            let policy_id = self.create_policy(
                "Efficiency Improvement".to_string(),
                "Measures to improve governmental effectiveness".to_string(),
                PolicyType::Administrative
            );
            new_policies.push(policy_id);
            decisions.push("New efficiency policy enacted".to_string());
        }

        // Check for settlement needs and respond
        if let Some(settlement) = self.settlements.get(&settlement_id) {
            if settlement.economic_status.prosperity_level < 0.4 {
                let policy_id = self.create_policy(
                    "Economic Stimulus".to_string(),
                    "Measures to boost economic activity".to_string(),
                    PolicyType::Economic
                );
                new_policies.push(policy_id);
                decisions.push("Economic stimulus policy enacted".to_string());
            }
        }

        // Add policies to government
        if let Some(government) = self.governments.get_mut(&government_id) {
            government.policies.extend(new_policies);
        }

        Ok(decisions)
    }

    /// Generate settlement name based on type and founder
    fn generate_settlement_name(&self, settlement_type: &SettlementType) -> String {
        let prefixes = match settlement_type {
            SettlementType::Village => vec!["Green", "Stone", "River", "Hill", "Wood"],
            SettlementType::Town => vec!["New", "Great", "North", "South", "West"],
            SettlementType::City => vec!["Grand", "Royal", "Imperial", "Noble"],
            SettlementType::Fortress => vec!["Iron", "Strong", "High", "Guard"],
            _ => vec!["New", "First", "Haven"],
        };

        let suffixes = match settlement_type {
            SettlementType::Village => vec!["vale", "brook", "field", "grove"],
            SettlementType::Town => vec!["town", "burg", "haven", "bridge"],
            SettlementType::City => vec!["city", "metropolis", "capital"],
            SettlementType::Fortress => vec!["hold", "keep", "guard", "watch"],
            _ => vec!["settlement", "place", "home"],
        };

        let prefix = prefixes[rand::random::<usize>() % prefixes.len()];
        let suffix = suffixes[rand::random::<usize>() % suffixes.len()];

        format!("{}{}", prefix, suffix)
    }

    /// Calculate required resources for construction projects
    fn calculate_required_resources(&self, project_type: &ProjectType) -> HashMap<String, f64> {
        match project_type {
            ProjectType::Housing => HashMap::from([
                ("Wood".to_string(), 20.0),
                ("Stone".to_string(), 10.0),
                ("Labor".to_string(), 30.0),
            ]),
            ProjectType::Infrastructure => HashMap::from([
                ("Stone".to_string(), 50.0),
                ("Metal".to_string(), 15.0),
                ("Labor".to_string(), 60.0),
            ]),
            ProjectType::Commercial => HashMap::from([
                ("Wood".to_string(), 30.0),
                ("Stone".to_string(), 20.0),
                ("Labor".to_string(), 40.0),
            ]),
            _ => HashMap::from([
                ("Wood".to_string(), 15.0),
                ("Stone".to_string(), 15.0),
                ("Labor".to_string(), 25.0),
            ]),
        }
    }

    /// Create a new policy
    fn create_policy(&mut self, name: String, description: String, policy_type: PolicyType) -> Uuid {
        let policy_id = Uuid::new_v4();
        
        let policy = Policy {
            policy_id,
            name,
            description,
            policy_type,
            effectiveness: 0.6,
            public_support: 0.7,
            implementation_date: Utc::now(),
            review_date: Some(Utc::now() + chrono::Duration::weeks(52)),
            outcomes: Vec::new(),
        };

        self.policies.insert(policy_id, policy);
        policy_id
    }

    /// Get settlement information
    pub fn get_settlement(&self, settlement_id: Uuid) -> Option<&Settlement> {
        self.settlements.get(&settlement_id)
    }

    /// Get government information
    pub fn get_government(&self, government_id: Uuid) -> Option<&Government> {
        self.governments.get(&government_id)
    }

    /// Get all settlements
    pub fn get_all_settlements(&self) -> Vec<&Settlement> {
        self.settlements.values().collect()
    }

    /// Get construction projects for a settlement
    pub fn get_settlement_projects(&self, settlement_id: Uuid) -> Vec<&ConstructionProject> {
        self.construction_projects.values()
            .filter(|project| project.settlement_id == settlement_id)
            .collect()
    }
}

// use rand::Rng; // Unused import