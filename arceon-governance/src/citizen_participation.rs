use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenParticipationSystem {
    pub participation_registry: ParticipationRegistry,
    pub civic_engagement: CivicEngagement,
    pub public_forums: Vec<PublicForum>,
    pub citizen_assemblies: Vec<CitizenAssembly>,
    pub participatory_budgeting: ParticipatoryBudgeting,
    pub consultation_processes: Vec<ConsultationProcess>,
    pub feedback_mechanisms: Vec<FeedbackMechanism>,
    pub civic_education: CivicEducation,
    pub digital_participation: DigitalParticipation,
    pub community_organizing: CommunityOrganizing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationRegistry {
    pub registered_citizens: HashMap<Uuid, CitizenProfile>,
    pub participation_levels: HashMap<Uuid, ParticipationLevel>,
    pub engagement_history: HashMap<Uuid, Vec<EngagementRecord>>,
    pub civic_contributions: HashMap<Uuid, Vec<CivicContribution>>,
    pub recognition_system: RecognitionSystem,
    pub participation_metrics: ParticipationMetrics,
    pub accessibility_accommodations: AccessibilityAccommodations,
    pub inclusion_initiatives: Vec<InclusionInitiative>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenProfile {
    pub citizen_id: Uuid,
    pub registration_date: DateTime<Utc>,
    pub participation_preferences: ParticipationPreferences,
    pub skill_areas: Vec<SkillArea>,
    pub time_availability: TimeAvailability,
    pub communication_preferences: CommunicationPreferences,
    pub accessibility_needs: Vec<AccessibilityNeed>,
    pub civic_interests: Vec<CivicInterest>,
    pub representation_groups: Vec<RepresentationGroup>,
    pub participation_badges: Vec<ParticipationBadge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicEngagement {
    pub engagement_opportunities: Vec<EngagementOpportunity>,
    pub volunteer_programs: Vec<VolunteerProgram>,
    pub civic_projects: Vec<CivicProject>,
    pub community_initiatives: Vec<CommunityInitiative>,
    pub advocacy_campaigns: Vec<AdvocacyCampaign>,
    pub civic_mentorship: CivicMentorship,
    pub leadership_development: LeadershipDevelopment,
    pub cross_community_collaboration: CrossCommunityCollaboration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicForum {
    pub id: Uuid,
    pub name: String,
    pub purpose: String,
    pub participants: Vec<Uuid>,
    pub moderators: Vec<Uuid>,
    pub topics: Vec<ForumTopic>,
    pub meeting_schedule: MeetingSchedule,
    pub participation_rules: ParticipationRules,
    pub accessibility_features: Vec<AccessibilityFeature>,
    pub documentation_system: DocumentationSystem,
    pub feedback_collection: FeedbackCollection,
    pub outcome_tracking: OutcomeTracking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenAssembly {
    pub id: Uuid,
    pub name: String,
    pub mandate: String,
    pub members: Vec<AssemblyMember>,
    pub selection_process: SelectionProcess,
    pub deliberation_process: DeliberationProcess,
    pub expert_witnesses: Vec<ExpertWitness>,
    pub information_provision: InformationProvision,
    pub facilitation_team: FacilitationTeam,
    pub timeline: AssemblyTimeline,
    pub recommendations: Vec<AssemblyRecommendation>,
    pub implementation_tracking: ImplementationTracking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipatoryBudgeting {
    pub budget_cycles: Vec<BudgetCycle>,
    pub proposal_system: ProposalSystem,
    pub voting_process: VotingProcess,
    pub community_meetings: Vec<CommunityMeeting>,
    pub project_categories: Vec<ProjectCategory>,
    pub evaluation_criteria: EvaluationCriteria,
    pub implementation_monitoring: ImplementationMonitoring,
    pub impact_assessment: ImpactAssessment,
    pub feedback_loops: Vec<FeedbackLoop>,
    pub capacity_building: CapacityBuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationProcess {
    pub id: Uuid,
    pub title: String,
    pub purpose: String,
    pub stakeholders: Vec<Stakeholder>,
    pub consultation_methods: Vec<ConsultationMethod>,
    pub timeline: ConsultationTimeline,
    pub information_materials: Vec<InformationMaterial>,
    pub response_analysis: ResponseAnalysis,
    pub outcome_publication: OutcomePublication,
    pub follow_up_actions: Vec<FollowUpAction>,
    pub quality_assurance: QualityAssurance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackMechanism {
    pub mechanism_type: FeedbackType,
    pub target_audience: Vec<String>,
    pub collection_methods: Vec<CollectionMethod>,
    pub processing_system: ProcessingSystem,
    pub response_protocols: Vec<ResponseProtocol>,
    pub feedback_analysis: FeedbackAnalysis,
    pub improvement_tracking: ImprovementTracking,
    pub transparency_measures: Vec<TransparencyMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicEducation {
    pub educational_programs: Vec<EducationalProgram>,
    pub civic_curriculum: CivicCurriculum,
    pub training_workshops: Vec<TrainingWorkshop>,
    pub resource_library: ResourceLibrary,
    pub mentor_network: MentorNetwork,
    pub skill_development: SkillDevelopment,
    pub certification_programs: Vec<CertificationProgram>,
    pub public_awareness_campaigns: Vec<PublicAwarenessCampaign>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalParticipation {
    pub online_platforms: Vec<OnlinePlatform>,
    pub digital_tools: Vec<DigitalTool>,
    pub virtual_meetings: VirtualMeetingSystem,
    pub e_voting_systems: EVotingSystem,
    pub digital_consultation: DigitalConsultation,
    pub social_media_engagement: SocialMediaEngagement,
    pub mobile_applications: Vec<MobileApplication>,
    pub accessibility_standards: AccessibilityStandards,
    pub digital_divide_initiatives: Vec<DigitalDivideInitiative>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityOrganizing {
    pub grassroots_networks: Vec<GrassrootsNetwork>,
    pub community_leaders: Vec<CommunityLeader>,
    pub organizing_campaigns: Vec<OrganizingCampaign>,
    pub coalition_building: CoalitionBuilding,
    pub resource_mobilization: ResourceMobilization,
    pub advocacy_training: AdvocacyTraining,
    pub power_mapping: PowerMapping,
    pub strategic_planning: StrategicPlanning,
    pub relationship_building: RelationshipBuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipationLevel {
    Observer,
    Participant,
    Contributor,
    Leader,
    Expert,
    Champion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackType {
    PolicyFeedback,
    ServiceFeedback,
    ProcessFeedback,
    IdeaSubmission,
    ComplaintResolution,
    Suggestion,
    Evaluation,
    Survey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementRecord {
    pub activity_id: Uuid,
    pub activity_type: String,
    pub participation_date: DateTime<Utc>,
    pub duration: chrono::Duration,
    pub contribution_type: ContributionType,
    pub impact_level: ImpactLevel,
    pub recognition_received: Vec<String>,
    pub learning_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicContribution {
    pub contribution_id: Uuid,
    pub contribution_type: ContributionType,
    pub description: String,
    pub impact_area: String,
    pub beneficiaries: Vec<String>,
    pub measurable_outcomes: Vec<MeasurableOutcome>,
    pub recognition_level: RecognitionLevel,
    pub collaboration_partners: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    PolicyInput,
    VolunteerWork,
    ExpertAdvice,
    CommunityLeadership,
    ProblemSolving,
    ResourceSharing,
    SkillTraining,
    Advocacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Individual,
    Local,
    Regional,
    System,
    Transformational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecognitionLevel {
    Acknowledgment,
    Appreciation,
    Award,
    Honor,
    Lifetime,
}

impl Default for CitizenParticipationSystem {
    fn default() -> Self {
        Self {
            participation_registry: ParticipationRegistry::default(),
            civic_engagement: CivicEngagement::default(),
            public_forums: Vec::new(),
            citizen_assemblies: Vec::new(),
            participatory_budgeting: ParticipatoryBudgeting::default(),
            consultation_processes: Vec::new(),
            feedback_mechanisms: Vec::new(),
            civic_education: CivicEducation::default(),
            digital_participation: DigitalParticipation::default(),
            community_organizing: CommunityOrganizing::default(),
        }
    }
}

impl Default for ParticipationRegistry {
    fn default() -> Self {
        Self {
            registered_citizens: HashMap::new(),
            participation_levels: HashMap::new(),
            engagement_history: HashMap::new(),
            civic_contributions: HashMap::new(),
            recognition_system: RecognitionSystem::default(),
            participation_metrics: ParticipationMetrics::default(),
            accessibility_accommodations: AccessibilityAccommodations::default(),
            inclusion_initiatives: Vec::new(),
        }
    }
}

impl Default for CivicEngagement {
    fn default() -> Self {
        Self {
            engagement_opportunities: Vec::new(),
            volunteer_programs: Vec::new(),
            civic_projects: Vec::new(),
            community_initiatives: Vec::new(),
            advocacy_campaigns: Vec::new(),
            civic_mentorship: CivicMentorship::default(),
            leadership_development: LeadershipDevelopment::default(),
            cross_community_collaboration: CrossCommunityCollaboration::default(),
        }
    }
}

impl Default for ParticipatoryBudgeting {
    fn default() -> Self {
        Self {
            budget_cycles: Vec::new(),
            proposal_system: ProposalSystem::default(),
            voting_process: VotingProcess::default(),
            community_meetings: Vec::new(),
            project_categories: Vec::new(),
            evaluation_criteria: EvaluationCriteria::default(),
            implementation_monitoring: ImplementationMonitoring::default(),
            impact_assessment: ImpactAssessment::default(),
            feedback_loops: Vec::new(),
            capacity_building: CapacityBuilding::default(),
        }
    }
}

impl Default for CivicEducation {
    fn default() -> Self {
        Self {
            educational_programs: Vec::new(),
            civic_curriculum: CivicCurriculum::default(),
            training_workshops: Vec::new(),
            resource_library: ResourceLibrary::default(),
            mentor_network: MentorNetwork::default(),
            skill_development: SkillDevelopment::default(),
            certification_programs: Vec::new(),
            public_awareness_campaigns: Vec::new(),
        }
    }
}

impl Default for DigitalParticipation {
    fn default() -> Self {
        Self {
            online_platforms: Vec::new(),
            digital_tools: Vec::new(),
            virtual_meetings: VirtualMeetingSystem::default(),
            e_voting_systems: EVotingSystem::default(),
            digital_consultation: DigitalConsultation::default(),
            social_media_engagement: SocialMediaEngagement::default(),
            mobile_applications: Vec::new(),
            accessibility_standards: AccessibilityStandards::default(),
            digital_divide_initiatives: Vec::new(),
        }
    }
}

impl Default for CommunityOrganizing {
    fn default() -> Self {
        Self {
            grassroots_networks: Vec::new(),
            community_leaders: Vec::new(),
            organizing_campaigns: Vec::new(),
            coalition_building: CoalitionBuilding::default(),
            resource_mobilization: ResourceMobilization::default(),
            advocacy_training: AdvocacyTraining::default(),
            power_mapping: PowerMapping::default(),
            strategic_planning: StrategicPlanning::default(),
            relationship_building: RelationshipBuilding::default(),
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
    ParticipationPreferences, SkillArea, TimeAvailability, CommunicationPreferences,
    AccessibilityNeed, CivicInterest, RepresentationGroup, ParticipationBadge,
    RecognitionSystem, ParticipationMetrics, AccessibilityAccommodations,
    InclusionInitiative, EngagementOpportunity, VolunteerProgram, CivicProject,
    CommunityInitiative, AdvocacyCampaign, CivicMentorship, LeadershipDevelopment,
    CrossCommunityCollaboration, ForumTopic, MeetingSchedule, ParticipationRules,
    AccessibilityFeature, DocumentationSystem, FeedbackCollection, OutcomeTracking,
    AssemblyMember, SelectionProcess, DeliberationProcess, ExpertWitness,
    InformationProvision, FacilitationTeam, AssemblyTimeline, AssemblyRecommendation,
    ImplementationTracking, BudgetCycle, ProposalSystem, VotingProcess,
    CommunityMeeting, ProjectCategory, EvaluationCriteria, ImplementationMonitoring,
    ImpactAssessment, FeedbackLoop, CapacityBuilding, Stakeholder, ConsultationMethod,
    ConsultationTimeline, InformationMaterial, ResponseAnalysis, OutcomePublication,
    FollowUpAction, QualityAssurance, CollectionMethod, ProcessingSystem,
    ResponseProtocol, FeedbackAnalysis, ImprovementTracking, TransparencyMeasure,
    EducationalProgram, CivicCurriculum, TrainingWorkshop, ResourceLibrary,
    MentorNetwork, SkillDevelopment, CertificationProgram, PublicAwarenessCampaign,
    OnlinePlatform, DigitalTool, VirtualMeetingSystem, EVotingSystem,
    DigitalConsultation, SocialMediaEngagement, MobileApplication,
    AccessibilityStandards, DigitalDivideInitiative, GrassrootsNetwork,
    CommunityLeader, OrganizingCampaign, CoalitionBuilding, ResourceMobilization,
    AdvocacyTraining, PowerMapping, StrategicPlanning, RelationshipBuilding,
    MeasurableOutcome
);

impl CitizenParticipationSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_citizen(&mut self, citizen_id: Uuid) -> Result<(), String> {
        let profile = CitizenProfile {
            citizen_id,
            registration_date: Utc::now(),
            participation_preferences: ParticipationPreferences::default(),
            skill_areas: Vec::new(),
            time_availability: TimeAvailability::default(),
            communication_preferences: CommunicationPreferences::default(),
            accessibility_needs: Vec::new(),
            civic_interests: Vec::new(),
            representation_groups: Vec::new(),
            participation_badges: Vec::new(),
        };

        self.participation_registry.registered_citizens.insert(citizen_id, profile);
        self.participation_registry.participation_levels.insert(citizen_id, ParticipationLevel::Observer);
        Ok(())
    }

    pub fn create_public_forum(&mut self, name: String, purpose: String) -> Result<Uuid, String> {
        let forum = PublicForum {
            id: Uuid::new_v4(),
            name,
            purpose,
            participants: Vec::new(),
            moderators: Vec::new(),
            topics: Vec::new(),
            meeting_schedule: MeetingSchedule::default(),
            participation_rules: ParticipationRules::default(),
            accessibility_features: Vec::new(),
            documentation_system: DocumentationSystem::default(),
            feedback_collection: FeedbackCollection::default(),
            outcome_tracking: OutcomeTracking::default(),
        };

        let forum_id = forum.id;
        self.public_forums.push(forum);
        Ok(forum_id)
    }

    pub fn create_citizen_assembly(&mut self, name: String, mandate: String) -> Result<Uuid, String> {
        let assembly = CitizenAssembly {
            id: Uuid::new_v4(),
            name,
            mandate,
            members: Vec::new(),
            selection_process: SelectionProcess::default(),
            deliberation_process: DeliberationProcess::default(),
            expert_witnesses: Vec::new(),
            information_provision: InformationProvision::default(),
            facilitation_team: FacilitationTeam::default(),
            timeline: AssemblyTimeline::default(),
            recommendations: Vec::new(),
            implementation_tracking: ImplementationTracking::default(),
        };

        let assembly_id = assembly.id;
        self.citizen_assemblies.push(assembly);
        Ok(assembly_id)
    }

    pub fn start_consultation(&mut self, title: String, purpose: String) -> Result<Uuid, String> {
        let consultation = ConsultationProcess {
            id: Uuid::new_v4(),
            title,
            purpose,
            stakeholders: Vec::new(),
            consultation_methods: Vec::new(),
            timeline: ConsultationTimeline::default(),
            information_materials: Vec::new(),
            response_analysis: ResponseAnalysis::default(),
            outcome_publication: OutcomePublication::default(),
            follow_up_actions: Vec::new(),
            quality_assurance: QualityAssurance::default(),
        };

        let consultation_id = consultation.id;
        self.consultation_processes.push(consultation);
        Ok(consultation_id)
    }

    pub fn submit_feedback(&mut self, citizen_id: Uuid, _feedback_type: FeedbackType, _content: String) -> Result<Uuid, String> {
        let feedback_id = Uuid::new_v4();
        
        let engagement_record = EngagementRecord {
            activity_id: feedback_id,
            activity_type: "Feedback Submission".to_string(),
            participation_date: Utc::now(),
            duration: chrono::Duration::minutes(15),
            contribution_type: ContributionType::PolicyInput,
            impact_level: ImpactLevel::Individual,
            recognition_received: Vec::new(),
            learning_outcomes: Vec::new(),
        };

        self.participation_registry.engagement_history
            .entry(citizen_id)
            .or_insert_with(Vec::new)
            .push(engagement_record);

        Ok(feedback_id)
    }

    pub fn join_forum(&mut self, citizen_id: Uuid, forum_id: Uuid) -> Result<(), String> {
        if let Some(forum) = self.public_forums.iter_mut().find(|f| f.id == forum_id) {
            if !forum.participants.contains(&citizen_id) {
                forum.participants.push(citizen_id);
            }
            Ok(())
        } else {
            Err("Forum not found".to_string())
        }
    }

    pub fn propose_budget_project(&mut self, citizen_id: Uuid, project_description: String, _budget_amount: f64) -> Result<Uuid, String> {
        let project_id = Uuid::new_v4();
        
        let contribution = CivicContribution {
            contribution_id: project_id,
            contribution_type: ContributionType::PolicyInput,
            description: project_description,
            impact_area: "Community Development".to_string(),
            beneficiaries: Vec::new(),
            measurable_outcomes: Vec::new(),
            recognition_level: RecognitionLevel::Acknowledgment,
            collaboration_partners: Vec::new(),
        };

        self.participation_registry.civic_contributions
            .entry(citizen_id)
            .or_insert_with(Vec::new)
            .push(contribution);

        Ok(project_id)
    }

    pub fn get_participation_level(&self, citizen_id: Uuid) -> Option<&ParticipationLevel> {
        self.participation_registry.participation_levels.get(&citizen_id)
    }

    pub fn update_participation_level(&mut self, citizen_id: Uuid, level: ParticipationLevel) -> Result<(), String> {
        self.participation_registry.participation_levels.insert(citizen_id, level);
        Ok(())
    }

    pub fn get_engagement_history(&self, citizen_id: Uuid) -> Option<&Vec<EngagementRecord>> {
        self.participation_registry.engagement_history.get(&citizen_id)
    }

    pub fn get_active_consultations(&self) -> &Vec<ConsultationProcess> {
        &self.consultation_processes
    }

    pub fn get_public_forums(&self) -> &Vec<PublicForum> {
        &self.public_forums
    }

    pub fn get_citizen_assemblies(&self) -> &Vec<CitizenAssembly> {
        &self.citizen_assemblies
    }
}