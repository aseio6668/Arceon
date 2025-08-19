pub mod infrastructure;
pub mod monitoring;
pub mod deployment;
pub mod backup;
pub mod security;
pub mod scaling;
pub mod health;
pub mod configuration;

pub use infrastructure::*;
pub use monitoring::*;
pub use deployment::*;
pub use backup::*;
pub use security::*;
pub use scaling::*;
pub use health::*;
pub use configuration::*;

use anyhow::Result;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use std::time::Duration;

/// Production deployment system manager
pub struct ProductionDeploymentSystem {
    pub infrastructure: Arc<InfrastructureManager>,
    pub monitoring: Arc<MonitoringSystem>,
    pub deployment: Arc<DeploymentManager>,
    pub backup: Arc<BackupManager>,
    pub security: Arc<SecurityManager>,
    pub scaling: Arc<AutoScalingManager>,
    pub health: Arc<HealthManager>,
    pub config: DeploymentConfig,
}

/// Production deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub environment: Environment,
    pub infrastructure_config: InfrastructureConfig,
    pub monitoring_config: MonitoringConfig,
    pub deployment_strategy: DeploymentStrategy,
    pub backup_config: BackupConfig,
    pub security_config: SecurityConfig,
    pub scaling_config: ScalingConfig,
    pub health_config: HealthConfig,
}

/// Deployment environment types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Environment {
    Development,
    Staging,
    Production,
    DisasterRecovery,
}

/// Infrastructure configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureConfig {
    pub cloud_provider: CloudProvider,
    pub regions: Vec<String>,
    pub availability_zones: Vec<String>,
    pub compute_resources: ComputeResources,
    pub network_config: NetworkConfig,
    pub storage_config: StorageConfig,
}

/// Cloud provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS { 
        access_key: String, 
        secret_key: String, 
        region: String 
    },
    GCP { 
        project_id: String, 
        credentials_path: String 
    },
    Azure { 
        subscription_id: String, 
        tenant_id: String, 
        client_id: String, 
        client_secret: String 
    },
    OnPremise { 
        datacenter: String, 
        management_endpoint: String 
    },
}

/// Compute resource specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResources {
    pub game_servers: ServerSpecs,
    pub database_servers: ServerSpecs,
    pub cache_servers: ServerSpecs,
    pub load_balancers: ServerSpecs,
    pub monitoring_servers: ServerSpecs,
}

/// Server specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSpecs {
    pub instance_type: String,
    pub cpu_cores: u16,
    pub memory_gb: u16,
    pub storage_gb: u32,
    pub min_instances: u16,
    pub max_instances: u16,
    pub desired_instances: u16,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub vpc_cidr: String,
    pub subnet_cidrs: Vec<String>,
    pub security_groups: Vec<SecurityGroup>,
    pub load_balancer_config: LoadBalancerConfig,
    pub cdn_config: CDNConfig,
}

/// Security group configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGroup {
    pub name: String,
    pub description: String,
    pub ingress_rules: Vec<SecurityRule>,
    pub egress_rules: Vec<SecurityRule>,
}

/// Security rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub protocol: String,
    pub port_range: String,
    pub source_cidr: String,
    pub description: String,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    pub lb_type: LoadBalancerType,
    pub health_check_interval: Duration,
    pub timeout: Duration,
    pub ssl_certificates: Vec<String>,
}

/// Load balancer types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancerType {
    ApplicationLoadBalancer,
    NetworkLoadBalancer,
    ClassicLoadBalancer,
}

/// CDN configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDNConfig {
    pub provider: String,
    pub cache_behaviors: Vec<CacheBehavior>,
    pub origins: Vec<Origin>,
}

/// Cache behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheBehavior {
    pub path_pattern: String,
    pub ttl: Duration,
    pub compress: bool,
}

/// CDN origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
    pub domain: String,
    pub path: String,
    pub protocol_policy: String,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database_storage: DatabaseStorage,
    pub file_storage: FileStorage,
    pub backup_storage: BackupStorage,
}

/// Database storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStorage {
    pub engine: String,
    pub version: String,
    pub instance_class: String,
    pub allocated_storage: u32,
    pub max_allocated_storage: u32,
    pub backup_retention_days: u8,
    pub multi_az: bool,
    pub encryption_at_rest: bool,
}

/// File storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStorage {
    pub storage_type: String,
    pub capacity: u64,
    pub performance_mode: String,
    pub throughput_mode: String,
}

/// Backup storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStorage {
    pub storage_class: String,
    pub retention_policy: RetentionPolicy,
    pub encryption: bool,
    pub cross_region_replication: bool,
}

/// Backup retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub daily_backups: u8,
    pub weekly_backups: u8,
    pub monthly_backups: u8,
    pub yearly_backups: u8,
}

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStrategy {
    pub strategy_type: DeploymentStrategyType,
    pub rollback_enabled: bool,
    pub health_check_grace_period: Duration,
    pub deployment_timeout: Duration,
    pub canary_config: Option<CanaryConfig>,
}

/// Deployment strategy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStrategyType {
    BlueGreen,
    RollingUpdate,
    Canary,
    Recreate,
}

/// Canary deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryConfig {
    pub canary_percentage: u8,
    pub canary_duration: Duration,
    pub success_criteria: Vec<SuccessCriterion>,
}

/// Success criterion for canary deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    pub metric: String,
    pub threshold: f64,
    pub comparison: ComparisonOperator,
}

/// Comparison operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equals,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl ProductionDeploymentSystem {
    /// Initialize the production deployment system
    pub async fn new(config: DeploymentConfig) -> Result<Self> {
        info!("🚀 Initializing Production Deployment System");

        let infrastructure = Arc::new(InfrastructureManager::new(&config.infrastructure_config).await?);
        let monitoring = Arc::new(MonitoringSystem::new(&config.monitoring_config).await?);
        let deployment = Arc::new(DeploymentManager::new(&config.deployment_strategy).await?);
        let backup = Arc::new(BackupManager::new(&config.backup_config).await?);
        let security = Arc::new(SecurityManager::new(&config.security_config).await?);
        let scaling = Arc::new(AutoScalingManager::new(&config.scaling_config).await?);
        let health = Arc::new(HealthManager::new(&config.health_config).await?);

        info!("✅ Production deployment system initialized");
        info!("   Environment: {:?}", config.environment);
        info!("   Cloud provider: {:?}", config.infrastructure_config.cloud_provider);
        info!("   Regions: {:?}", config.infrastructure_config.regions);

        Ok(Self {
            infrastructure,
            monitoring,
            deployment,
            backup,
            security,
            scaling,
            health,
            config,
        })
    }

    /// Deploy the entire Arceon MMORPG infrastructure
    pub async fn deploy_infrastructure(&self) -> Result<DeploymentResult> {
        info!("🏗️ Starting infrastructure deployment");

        // Phase 1: Provision base infrastructure
        let infra_result = self.infrastructure.provision_infrastructure().await?;
        info!("✅ Base infrastructure provisioned: {:?}", infra_result);

        // Phase 2: Setup security and networking
        self.security.configure_security().await?;
        info!("✅ Security configured");

        // Phase 3: Deploy databases and storage
        self.infrastructure.deploy_databases().await?;
        info!("✅ Databases deployed");

        // Phase 4: Deploy application servers
        let app_result = self.deployment.deploy_application().await?;
        info!("✅ Application servers deployed");

        // Phase 5: Configure monitoring and health checks
        self.monitoring.setup_monitoring().await?;
        self.health.configure_health_checks().await?;
        info!("✅ Monitoring and health checks configured");

        // Phase 6: Setup auto-scaling
        self.scaling.configure_auto_scaling().await?;
        info!("✅ Auto-scaling configured");

        // Phase 7: Setup backup systems
        self.backup.initialize_backup_systems().await?;
        info!("✅ Backup systems initialized");

        info!("🎉 Infrastructure deployment completed successfully");

        Ok(DeploymentResult {
            deployment_id: Uuid::new_v4(),
            status: DeploymentStatus::Success,
            infrastructure_endpoints: infra_result.endpoints,
            application_endpoints: app_result.application_endpoints,
            deployment_time: chrono::Utc::now(),
            rollback_available: true,
        })
    }

    /// Perform a rolling update deployment
    pub async fn rolling_update(&self, version: String) -> Result<DeploymentResult> {
        info!("🔄 Starting rolling update to version {}", version);

        // Pre-deployment health check
        let health_status = self.health.check_system_health().await?;
        if !health_status.is_healthy() {
            return Err(anyhow::anyhow!("System health check failed, aborting deployment"));
        }

        // Create backup before deployment
        let backup_id = self.backup.create_pre_deployment_backup().await?;
        info!("💾 Pre-deployment backup created: {}", backup_id);

        // Execute rolling update
        let deployment_result = self.deployment.execute_rolling_update(version).await?;

        // Post-deployment health check
        tokio::time::sleep(self.config.deployment_strategy.health_check_grace_period).await;
        let post_health = self.health.check_system_health().await?;

        if !post_health.is_healthy() {
            warn!("⚠️ Post-deployment health check failed, initiating rollback");
            self.rollback_deployment(deployment_result.deployment_id).await?;
            return Err(anyhow::anyhow!("Deployment failed health check, rolled back"));
        }

        info!("✅ Rolling update completed successfully");
        Ok(deployment_result)
    }

    /// Rollback a deployment
    pub async fn rollback_deployment(&self, deployment_id: Uuid) -> Result<()> {
        info!("🔙 Rolling back deployment: {}", deployment_id);

        // Stop new traffic
        self.infrastructure.drain_traffic().await?;

        // Restore from backup
        self.backup.restore_from_backup(deployment_id).await?;

        // Restart services with previous version
        self.deployment.rollback_to_previous_version().await?;

        // Resume traffic
        self.infrastructure.resume_traffic().await?;

        info!("✅ Deployment rollback completed");
        Ok(())
    }

    /// Scale the system based on demand
    pub async fn scale_system(&self, scaling_request: ScalingRequest) -> Result<ScalingResult> {
        info!("⚡ Scaling system: {:?}", scaling_request);

        let result = self.scaling.execute_scaling(scaling_request).await?;

        // Update monitoring for new instances
        if result.new_instances > 0 {
            self.monitoring.configure_monitoring_for_new_instances(&result.instance_ids).await?;
        }

        info!("✅ System scaling completed: {:?}", result);
        Ok(result)
    }

    /// Perform disaster recovery
    pub async fn disaster_recovery(&self, recovery_point: chrono::DateTime<chrono::Utc>) -> Result<()> {
        error!("🚨 Initiating disaster recovery to point: {}", recovery_point);

        // Stop all services
        self.deployment.emergency_stop_all_services().await?;

        // Restore from disaster recovery backup
        self.backup.disaster_recovery_restore(recovery_point).await?;

        // Rebuild infrastructure if needed
        let health = self.health.assess_infrastructure_health().await?;
        if !health.infrastructure_healthy {
            self.infrastructure.rebuild_failed_components().await?;
        }

        // Restart all services
        self.deployment.restart_all_services().await?;

        // Verify system health
        let final_health = self.health.check_system_health().await?;
        if !final_health.is_healthy() {
            return Err(anyhow::anyhow!("Disaster recovery failed - system not healthy"));
        }

        error!("✅ Disaster recovery completed successfully");
        Ok(())
    }

    /// Get system status and metrics
    pub async fn get_system_status(&self) -> Result<SystemStatus> {
        let infrastructure_status = self.infrastructure.get_status().await?;
        let deployment_status = self.deployment.get_status().await?;
        let monitoring_metrics = self.monitoring.get_current_metrics().await?;
        let health_status = self.health.get_health_summary().await?;
        let backup_status = self.backup.get_backup_status().await?;
        let scaling_status = self.scaling.get_scaling_status().await?;

        Ok(SystemStatus {
            environment: self.config.environment.clone(),
            overall_health: health_status.is_healthy(),
            infrastructure_status,
            deployment_status,
            monitoring_metrics,
            health_status,
            backup_status,
            scaling_status,
            last_updated: chrono::Utc::now(),
        })
    }
}

/// Deployment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResult {
    pub deployment_id: Uuid,
    pub status: DeploymentStatus,
    pub infrastructure_endpoints: Vec<String>,
    pub application_endpoints: Vec<String>,
    pub deployment_time: chrono::DateTime<chrono::Utc>,
    pub rollback_available: bool,
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentStatus {
    Success,
    Failed,
    InProgress,
    RolledBack,
}

/// Scaling request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingRequest {
    pub component: String,
    pub target_instances: u16,
    pub scaling_reason: ScalingReason,
    pub priority: ScalingPriority,
}

/// Scaling reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingReason {
    HighCPUUtilization,
    HighMemoryUtilization,
    HighPlayerCount,
    LowPlayerCount,
    ScheduledMaintenance,
    Manual,
}

/// Scaling priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Scaling result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingResult {
    pub component: String,
    pub previous_instances: u16,
    pub new_instances: u16,
    pub instance_ids: Vec<String>,
    pub scaling_time: Duration,
    pub success: bool,
}

/// System status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub environment: Environment,
    pub overall_health: bool,
    pub infrastructure_status: InfrastructureStatus,
    pub deployment_status: CurrentDeploymentStatus,
    pub monitoring_metrics: SystemMetrics,
    pub health_status: HealthSummary,
    pub backup_status: BackupStatus,
    pub scaling_status: ScalingStatus,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Development,
            infrastructure_config: InfrastructureConfig {
                cloud_provider: CloudProvider::OnPremise {
                    datacenter: "local".to_string(),
                    management_endpoint: "http://localhost:8080".to_string(),
                },
                regions: vec!["us-east-1".to_string()],
                availability_zones: vec!["us-east-1a".to_string(), "us-east-1b".to_string()],
                compute_resources: ComputeResources {
                    game_servers: ServerSpecs {
                        instance_type: "t3.large".to_string(),
                        cpu_cores: 2,
                        memory_gb: 8,
                        storage_gb: 100,
                        min_instances: 2,
                        max_instances: 20,
                        desired_instances: 4,
                    },
                    database_servers: ServerSpecs {
                        instance_type: "db.r5.xlarge".to_string(),
                        cpu_cores: 4,
                        memory_gb: 32,
                        storage_gb: 500,
                        min_instances: 1,
                        max_instances: 3,
                        desired_instances: 2,
                    },
                    cache_servers: ServerSpecs {
                        instance_type: "cache.r6g.large".to_string(),
                        cpu_cores: 2,
                        memory_gb: 16,
                        storage_gb: 0,
                        min_instances: 1,
                        max_instances: 5,
                        desired_instances: 2,
                    },
                    load_balancers: ServerSpecs {
                        instance_type: "application".to_string(),
                        cpu_cores: 0,
                        memory_gb: 0,
                        storage_gb: 0,
                        min_instances: 1,
                        max_instances: 3,
                        desired_instances: 2,
                    },
                    monitoring_servers: ServerSpecs {
                        instance_type: "t3.medium".to_string(),
                        cpu_cores: 2,
                        memory_gb: 4,
                        storage_gb: 50,
                        min_instances: 1,
                        max_instances: 2,
                        desired_instances: 1,
                    },
                },
                network_config: NetworkConfig {
                    vpc_cidr: "10.0.0.0/16".to_string(),
                    subnet_cidrs: vec!["10.0.1.0/24".to_string(), "10.0.2.0/24".to_string()],
                    security_groups: Vec::new(),
                    load_balancer_config: LoadBalancerConfig {
                        lb_type: LoadBalancerType::ApplicationLoadBalancer,
                        health_check_interval: Duration::from_secs(30),
                        timeout: Duration::from_secs(5),
                        ssl_certificates: Vec::new(),
                    },
                    cdn_config: CDNConfig {
                        provider: "cloudflare".to_string(),
                        cache_behaviors: Vec::new(),
                        origins: Vec::new(),
                    },
                },
                storage_config: StorageConfig {
                    database_storage: DatabaseStorage {
                        engine: "postgresql".to_string(),
                        version: "15.0".to_string(),
                        instance_class: "db.r5.large".to_string(),
                        allocated_storage: 100,
                        max_allocated_storage: 1000,
                        backup_retention_days: 7,
                        multi_az: true,
                        encryption_at_rest: true,
                    },
                    file_storage: FileStorage {
                        storage_type: "gp3".to_string(),
                        capacity: 1000,
                        performance_mode: "generalPurpose".to_string(),
                        throughput_mode: "provisioned".to_string(),
                    },
                    backup_storage: BackupStorage {
                        storage_class: "STANDARD_IA".to_string(),
                        retention_policy: RetentionPolicy {
                            daily_backups: 7,
                            weekly_backups: 4,
                            monthly_backups: 12,
                            yearly_backups: 5,
                        },
                        encryption: true,
                        cross_region_replication: true,
                    },
                },
            },
            monitoring_config: MonitoringConfig::default(),
            deployment_strategy: DeploymentStrategy {
                strategy_type: DeploymentStrategyType::RollingUpdate,
                rollback_enabled: true,
                health_check_grace_period: Duration::from_secs(300),
                deployment_timeout: Duration::from_secs(1800),
                canary_config: None,
            },
            backup_config: BackupConfig::default(),
            security_config: SecurityConfig::default(),
            scaling_config: ScalingConfig::default(),
            health_config: HealthConfig::default(),
        }
    }
}