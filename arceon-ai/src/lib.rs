pub mod npc;
pub mod ml_text;
pub mod ai_manager;
pub mod neural_network;
pub mod adaptive_behavior;
pub mod emergent_storytelling;

// Re-export main types with module prefixes to avoid conflicts
pub use npc::*;

// ML Text types with prefix
pub use ml_text::{
    MlTextProcessor, Vocabulary, TextModel, ModelType,
    TrainingStatus as MlTrainingStatus,
    PerformanceMetrics as MlPerformanceMetrics,
    GameBook, BookGenre,
};

// AI Manager types
pub use ai_manager::{
    AiManager, WorldState, AreaState, ResourceNode, Structure,
    CollaborativeProject, ProjectType, FactionSystem, Faction, FactionType,
    RelationshipStrength as AiRelationshipStrength,
};

// Neural Network types with prefix
pub use neural_network::{
    NeuralNetworkManager, NeuralNetwork, NetworkType, Layer, LayerType, ActivationFunction,
    TrainingStatus as NetworkTrainingStatus,
    PerformanceMetrics as NetworkPerformanceMetrics,
    BehaviorChange as NetworkBehaviorChange,
    ChangeType as NetworkChangeType,
    NetworkUpdate as NeuralNetworkUpdate,
    TriggerCondition as NetworkTriggerCondition,
    BehaviorTemplate as NetworkBehaviorTemplate,
    ImpactAssessment as NetworkImpactAssessment,
};

// Adaptive Behavior types with prefix
pub use adaptive_behavior::{
    AdaptiveBehaviorSystem, BehaviorManager, AdaptiveBehaviorProfile,
    PersonalityVector, BehavioralAdaptation, AdaptationType,
    RelationshipStrength as AdaptiveRelationshipStrength,
    BehaviorChange as AdaptiveBehaviorChange,
    ChangeType as AdaptiveChangeType,
    NetworkUpdate as AdaptiveNetworkUpdate,
    BehaviorTemplate as AdaptiveBehaviorTemplate,
    AdaptationTrigger,
    CulturalInfluence as AdaptiveCulturalInfluence,
    InteractionAnalyzer, PersonalityEngine, SocialDynamicsTracker,
};

// Emergent Storytelling types with prefix  
pub use emergent_storytelling::{
    EmergentStorytellingSystem, NarrativeEngine, ActiveNarrative, StoryBeat,
    EmotionalArc, StoryGenerator, DialogueEngine, PlotCoordinator,
    TriggerCondition as StoryTriggerCondition,
    ImpactAssessment as StoryImpactAssessment,
    AdaptationTrigger as StoryAdaptationTrigger,
    CulturalInfluence as StoryCulturalInfluence,
    NarrativeTemplate, EventGenerator, TimelineManager,
};
