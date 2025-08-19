/*!
# Progression Analytics

Advanced analytics and insights for character progression patterns,
optimization recommendations, and community trends.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

/// Progression analytics system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionAnalytics {
    pub player_analytics: HashMap<Uuid, PlayerAnalytics>,
    pub global_trends: GlobalProgressionTrends,
    pub optimization_engine: OptimizationEngine,
    pub reporting_system: ReportingSystem,
}

/// Analytics for individual players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnalytics {
    pub player_id: Uuid,
    pub progression_velocity: ProgressionVelocity,
    pub skill_preferences: SkillPreferences,
    pub efficiency_metrics: EfficiencyMetrics,
    pub play_patterns: PlayPatterns,
    pub recommendation_history: Vec<Recommendation>,
}

/// Progression speed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionVelocity {
    pub experience_per_hour: f64,
    pub levels_per_session: f64,
    pub skills_unlocked_rate: f64,
    pub achievement_rate: f64,
    pub progression_consistency: f64,
}

/// Player skill preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPreferences {
    pub preferred_categories: Vec<String>,
    pub skill_investment_patterns: HashMap<String, f64>,
    pub build_archetype_affinity: Vec<String>,
    pub experimentation_tendency: f64,
}

/// Player efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyMetrics {
    pub skill_point_efficiency: f64,
    pub experience_efficiency: f64,
    pub resource_utilization: f64,
    pub build_optimization_score: f64,
    pub progression_waste_factor: f64,
}

/// Play pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayPatterns {
    pub session_length_average: f64,
    pub peak_activity_times: Vec<u32>, // Hours of day
    pub activity_consistency: f64,
    pub break_patterns: Vec<f64>,
    pub seasonal_variations: HashMap<String, f64>,
}

/// Recommendations for players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub recommendation_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub recommendation_type: RecommendationType,
    pub content: String,
    pub priority: u32,
    pub followed: bool,
    pub effectiveness_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    SkillAllocation,
    BuildOptimization,
    ProgressionPath,
    PlayStyle,
    TimeManagement,
    ResourceUsage,
}

/// Global progression trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalProgressionTrends {
    pub popular_builds: Vec<BuildPopularity>,
    pub skill_usage_statistics: HashMap<String, SkillUsageStats>,
    pub progression_benchmarks: ProgressionBenchmarks,
    pub community_patterns: CommunityPatterns,
    pub meta_evolution: MetaEvolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPopularity {
    pub build_name: String,
    pub usage_percentage: f64,
    pub success_rate: f64,
    pub average_effectiveness: f64,
    pub trend_direction: TrendDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Rising,
    Stable,
    Declining,
    Emerging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillUsageStats {
    pub skill_name: String,
    pub adoption_rate: f64,
    pub retention_rate: f64,
    pub satisfaction_score: f64,
    pub synergy_frequency: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionBenchmarks {
    pub level_progression_percentiles: HashMap<u32, Vec<f64>>,
    pub skill_unlock_timings: HashMap<String, f64>,
    pub achievement_completion_rates: HashMap<String, f64>,
    pub build_effectiveness_ranges: HashMap<String, (f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPatterns {
    pub cooperation_trends: f64,
    pub competition_intensity: f64,
    pub knowledge_sharing_activity: f64,
    pub innovation_rate: f64,
    pub community_health_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaEvolution {
    pub meta_shifts: Vec<MetaShift>,
    pub emerging_strategies: Vec<String>,
    pub declining_strategies: Vec<String>,
    pub innovation_hotspots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaShift {
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub impact_magnitude: f64,
    pub affected_builds: Vec<String>,
}

/// Optimization engine for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEngine {
    pub optimization_algorithms: Vec<OptimizationAlgorithm>,
    pub recommendation_models: Vec<RecommendationModel>,
    pub learning_systems: Vec<LearningSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAlgorithm {
    pub algorithm_name: String,
    pub algorithm_type: AlgorithmType,
    pub effectiveness_score: f64,
    pub use_cases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlgorithmType {
    GeneticAlgorithm,
    SimulatedAnnealing,
    GradientDescent,
    ParticleSwarm,
    MachineLearning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationModel {
    pub model_name: String,
    pub model_type: String,
    pub accuracy_score: f64,
    pub prediction_confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSystem {
    pub system_name: String,
    pub learning_rate: f64,
    pub adaptation_speed: f64,
    pub memory_retention: f64,
}

/// Reporting system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingSystem {
    pub report_templates: Vec<ReportTemplate>,
    pub scheduled_reports: Vec<ScheduledReport>,
    pub custom_dashboards: HashMap<Uuid, Dashboard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub description: String,
    pub metrics_included: Vec<String>,
    pub visualization_types: Vec<VisualizationType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationType {
    LineChart,
    BarChart,
    PieChart,
    Heatmap,
    ScatterPlot,
    Table,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReport {
    pub report_id: Uuid,
    pub template_id: Uuid,
    pub schedule: ReportSchedule,
    pub recipients: Vec<Uuid>,
    pub last_generated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportSchedule {
    Daily,
    Weekly,
    Monthly,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub dashboard_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub widgets: Vec<DashboardWidget>,
    pub layout: DashboardLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub widget_id: Uuid,
    pub widget_type: WidgetType,
    pub data_source: String,
    pub configuration: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    ProgressChart,
    EfficiencyMeter,
    SkillTree,
    Leaderboard,
    TrendAnalysis,
    Recommendations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub columns: u32,
    pub rows: u32,
    pub widget_positions: HashMap<Uuid, (u32, u32)>,
}

impl ProgressionAnalytics {
    /// Create new analytics system
    pub fn new() -> Self {
        Self {
            player_analytics: HashMap::new(),
            global_trends: GlobalProgressionTrends::default(),
            optimization_engine: OptimizationEngine::default(),
            reporting_system: ReportingSystem::default(),
        }
    }

    /// Record experience gain for analytics
    pub fn record_experience_gain(&mut self, player_id: Uuid, experience: u64, source: &str) -> Result<()> {
        let analytics = self.player_analytics.entry(player_id)
            .or_insert_with(|| PlayerAnalytics::new(player_id));

        // Update progression velocity
        analytics.progression_velocity.experience_per_hour += experience as f64;
        
        // Track source patterns
        let source_category = Self::categorize_experience_source(source);
        *analytics.skill_preferences.skill_investment_patterns
            .entry(source_category)
            .or_insert(0.0) += experience as f64;

        Ok(())
    }

    /// Categorize experience source
    fn categorize_experience_source(source: &str) -> String {
        match source.to_lowercase().as_str() {
            s if s.contains("combat") => "Combat".to_string(),
            s if s.contains("craft") => "Crafting".to_string(),
            s if s.contains("quest") => "Questing".to_string(),
            s if s.contains("social") => "Social".to_string(),
            s if s.contains("explore") => "Exploration".to_string(),
            _ => "General".to_string(),
        }
    }

    /// Generate recommendations for a player
    pub fn generate_recommendations(&mut self, player_id: Uuid) -> Result<Vec<Recommendation>> {
        let analytics = self.player_analytics.get(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player analytics not found"))?;

        let mut recommendations = vec![];

        // Efficiency recommendation
        if analytics.efficiency_metrics.skill_point_efficiency < 0.7 {
            recommendations.push(Recommendation {
                recommendation_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                recommendation_type: RecommendationType::SkillAllocation,
                content: "Consider focusing on fewer skill categories for better efficiency".to_string(),
                priority: 2,
                followed: false,
                effectiveness_score: None,
            });
        }

        // Build optimization recommendation
        if analytics.efficiency_metrics.build_optimization_score < 0.6 {
            recommendations.push(Recommendation {
                recommendation_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                recommendation_type: RecommendationType::BuildOptimization,
                content: "Your current build could be optimized for better performance".to_string(),
                priority: 1,
                followed: false,
                effectiveness_score: None,
            });
        }

        // Store recommendations
        if let Some(player_analytics) = self.player_analytics.get_mut(&player_id) {
            player_analytics.recommendation_history.extend(recommendations.clone());
        }

        Ok(recommendations)
    }

    /// Analyze global trends
    pub fn analyze_global_trends(&mut self) -> Result<()> {
        // Calculate popular builds
        let mut build_usage = HashMap::new();
        for analytics in self.player_analytics.values() {
            for archetype in &analytics.skill_preferences.build_archetype_affinity {
                *build_usage.entry(archetype.clone()).or_insert(0) += 1;
            }
        }

        let total_players = self.player_analytics.len() as f64;
        self.global_trends.popular_builds = build_usage.into_iter()
            .map(|(build, count)| BuildPopularity {
                build_name: build,
                usage_percentage: (count as f64 / total_players) * 100.0,
                success_rate: 0.8, // Placeholder
                average_effectiveness: 0.75, // Placeholder
                trend_direction: TrendDirection::Stable,
            })
            .collect();

        Ok(())
    }

    /// Get player analytics
    pub fn get_player_analytics(&self, player_id: Uuid) -> Option<&PlayerAnalytics> {
        self.player_analytics.get(&player_id)
    }

    /// Get global trends
    pub fn get_global_trends(&self) -> &GlobalProgressionTrends {
        &self.global_trends
    }
}

impl PlayerAnalytics {
    /// Create new player analytics
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            progression_velocity: ProgressionVelocity::default(),
            skill_preferences: SkillPreferences::default(),
            efficiency_metrics: EfficiencyMetrics::default(),
            play_patterns: PlayPatterns::default(),
            recommendation_history: vec![],
        }
    }
}

impl Default for ProgressionVelocity {
    fn default() -> Self {
        Self {
            experience_per_hour: 0.0,
            levels_per_session: 0.0,
            skills_unlocked_rate: 0.0,
            achievement_rate: 0.0,
            progression_consistency: 1.0,
        }
    }
}

impl Default for SkillPreferences {
    fn default() -> Self {
        Self {
            preferred_categories: vec![],
            skill_investment_patterns: HashMap::new(),
            build_archetype_affinity: vec![],
            experimentation_tendency: 0.5,
        }
    }
}

impl Default for EfficiencyMetrics {
    fn default() -> Self {
        Self {
            skill_point_efficiency: 1.0,
            experience_efficiency: 1.0,
            resource_utilization: 1.0,
            build_optimization_score: 0.5,
            progression_waste_factor: 0.0,
        }
    }
}

impl Default for PlayPatterns {
    fn default() -> Self {
        Self {
            session_length_average: 60.0, // 1 hour
            peak_activity_times: vec![19, 20, 21], // 7-9 PM
            activity_consistency: 0.7,
            break_patterns: vec![],
            seasonal_variations: HashMap::new(),
        }
    }
}

impl Default for GlobalProgressionTrends {
    fn default() -> Self {
        Self {
            popular_builds: vec![],
            skill_usage_statistics: HashMap::new(),
            progression_benchmarks: ProgressionBenchmarks::default(),
            community_patterns: CommunityPatterns::default(),
            meta_evolution: MetaEvolution::default(),
        }
    }
}

impl Default for ProgressionBenchmarks {
    fn default() -> Self {
        Self {
            level_progression_percentiles: HashMap::new(),
            skill_unlock_timings: HashMap::new(),
            achievement_completion_rates: HashMap::new(),
            build_effectiveness_ranges: HashMap::new(),
        }
    }
}

impl Default for CommunityPatterns {
    fn default() -> Self {
        Self {
            cooperation_trends: 0.5,
            competition_intensity: 0.5,
            knowledge_sharing_activity: 0.5,
            innovation_rate: 0.5,
            community_health_score: 0.75,
        }
    }
}

impl Default for MetaEvolution {
    fn default() -> Self {
        Self {
            meta_shifts: vec![],
            emerging_strategies: vec![],
            declining_strategies: vec![],
            innovation_hotspots: vec![],
        }
    }
}

impl Default for OptimizationEngine {
    fn default() -> Self {
        Self {
            optimization_algorithms: vec![],
            recommendation_models: vec![],
            learning_systems: vec![],
        }
    }
}

impl Default for ReportingSystem {
    fn default() -> Self {
        Self {
            report_templates: vec![],
            scheduled_reports: vec![],
            custom_dashboards: HashMap::new(),
        }
    }
}

impl Default for ProgressionAnalytics {
    fn default() -> Self {
        Self::new()
    }
}