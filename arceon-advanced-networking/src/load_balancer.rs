use anyhow::Result;
use crate::{LoadBalancerConfig, LoadBalancingAlgorithm, ServerEndpoint, ServiceDiscovery, GeoLocation};
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use tracing::{info, warn};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Advanced load balancer for global server infrastructure
pub struct LoadBalancer {
    pub config: LoadBalancerConfig,
    pub service_discovery: Arc<ServiceDiscovery>,
    pub server_pool: Arc<RwLock<ServerPool>>,
    pub health_checker: Arc<RwLock<HealthChecker>>,
    pub circuit_breaker: Arc<RwLock<CircuitBreaker>>,
    pub metrics: Arc<RwLock<LoadBalancerMetrics>>,
    pub session_affinity: Arc<RwLock<SessionAffinityManager>>,
}

/// Pool of available servers
#[derive(Debug)]
pub struct ServerPool {
    pub servers: HashMap<String, LoadBalancedServer>,
    pub region_servers: HashMap<String, Vec<String>>, // region -> server IDs
    pub healthy_servers: Vec<String>,
    pub unhealthy_servers: Vec<String>,
    pub last_update: Instant,
}

/// Server with load balancing metadata
#[derive(Debug, Clone)]
pub struct LoadBalancedServer {
    pub endpoint: ServerEndpoint,
    pub health_status: HealthStatus,
    pub current_connections: u32,
    pub weight: u32,
    pub response_times: Vec<Duration>,
    pub last_health_check: Instant,
    pub circuit_breaker_state: CircuitBreakerState,
    pub sticky_sessions: HashMap<String, Uuid>, // session_id -> player_id
}

/// Server health status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Health checking system
#[derive(Debug)]
pub struct HealthChecker {
    pub checks: HashMap<String, HealthCheck>,
    pub check_interval: Duration,
    pub timeout: Duration,
    pub failure_threshold: u32,
    pub success_threshold: u32,
}

/// Individual health check
#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub server_id: String,
    pub check_type: HealthCheckType,
    pub last_check: Instant,
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
    pub response_time: Option<Duration>,
}

/// Types of health checks
#[derive(Debug, Clone)]
pub enum HealthCheckType {
    HTTP { endpoint: String, expected_status: u16 },
    TCP { port: u16 },
    UDP { port: u16 },
    Ping,
    Custom { command: String },
}

/// Circuit breaker for fault tolerance
#[derive(Debug)]
pub struct CircuitBreaker {
    pub breakers: HashMap<String, ServerCircuitBreaker>,
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
}

/// Circuit breaker for individual server
#[derive(Debug, Clone)]
pub struct ServerCircuitBreaker {
    pub server_id: String,
    pub state: CircuitBreakerState,
    pub failure_count: u32,
    pub last_failure: Option<Instant>,
    pub next_attempt: Option<Instant>,
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitBreakerState {
    Closed,    // Normal operation
    Open,      // Failing fast, not sending requests
    HalfOpen,  // Testing if service has recovered
}

/// Session affinity management for sticky sessions
#[derive(Debug)]
pub struct SessionAffinityManager {
    pub player_sessions: HashMap<Uuid, String>, // player_id -> server_id
    pub session_timeouts: HashMap<Uuid, Instant>,
    pub session_timeout: Duration,
}

/// Load balancer performance metrics
#[derive(Debug, Clone)]
pub struct LoadBalancerMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: Duration,
    pub requests_per_second: f32,
    pub server_utilization: HashMap<String, f32>,
    pub average_load: f32,
    pub circuit_breaker_trips: u32,
    pub load_balancing_decisions: HashMap<String, u64>, // algorithm -> count
}

/// Load balancing decision context
#[derive(Debug, Clone)]
pub struct LoadBalancingContext {
    pub player_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub player_location: Option<GeoLocation>,
    pub required_capacity: u32,
    pub preferred_region: Option<String>,
    pub service_requirements: Vec<String>,
}

impl LoadBalancer {
    /// Create new load balancer
    pub async fn new(config: &LoadBalancerConfig, service_discovery: Arc<ServiceDiscovery>) -> Result<Self> {
        info!("⚖️ Initializing Load Balancer");

        let server_pool = ServerPool {
            servers: HashMap::new(),
            region_servers: HashMap::new(),
            healthy_servers: Vec::new(),
            unhealthy_servers: Vec::new(),
            last_update: Instant::now(),
        };

        let health_checker = HealthChecker {
            checks: HashMap::new(),
            check_interval: config.health_check_interval,
            timeout: Duration::from_secs(5),
            failure_threshold: config.circuit_breaker_threshold,
            success_threshold: 2,
        };

        let circuit_breaker = CircuitBreaker {
            breakers: HashMap::new(),
            failure_threshold: config.circuit_breaker_threshold,
            recovery_timeout: Duration::from_secs(30),
        };

        let session_affinity = SessionAffinityManager {
            player_sessions: HashMap::new(),
            session_timeouts: HashMap::new(),
            session_timeout: Duration::from_hours(4), // 4 hour session timeout
        };

        let metrics = LoadBalancerMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: Duration::from_millis(0),
            requests_per_second: 0.0,
            server_utilization: HashMap::new(),
            average_load: 0.0,
            circuit_breaker_trips: 0,
            load_balancing_decisions: HashMap::new(),
        };

        info!("✅ Load balancer initialized");
        info!("   Algorithm: {:?}", config.algorithm);
        info!("   Health checks: {}s interval", config.health_check_interval.as_secs());
        info!("   Circuit breaker threshold: {}", config.circuit_breaker_threshold);

        Ok(Self {
            config: config.clone(),
            service_discovery,
            server_pool: Arc::new(RwLock::new(server_pool)),
            health_checker: Arc::new(RwLock::new(health_checker)),
            circuit_breaker: Arc::new(RwLock::new(circuit_breaker)),
            metrics: Arc::new(RwLock::new(metrics)),
            session_affinity: Arc::new(RwLock::new(session_affinity)),
        })
    }

    /// Start load balancer services
    pub async fn start(&self) -> Result<()> {
        info!("🚀 Starting load balancer services");

        // Start health checking
        self.start_health_checking().await?;
        
        // Start server discovery
        self.start_server_discovery().await?;
        
        // Start metrics collection
        self.start_metrics_collection().await?;
        
        // Start session cleanup
        self.start_session_cleanup().await?;

        info!("✅ Load balancer services started");
        Ok(())
    }

    /// Stop load balancer services
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping load balancer services");
        // Implementation would stop background tasks
        Ok(())
    }

    /// Select optimal server for a player
    pub async fn select_server(&self, player_location: GeoLocation, player_id: Uuid) -> Result<ServerEndpoint> {
        let context = LoadBalancingContext {
            player_id: Some(player_id),
            session_id: None,
            player_location: Some(player_location),
            required_capacity: 1,
            preferred_region: None,
            service_requirements: vec!["game_server".to_string()],
        };

        self.select_server_with_context(context).await
    }

    /// Select server with full context
    pub async fn select_server_with_context(&self, context: LoadBalancingContext) -> Result<ServerEndpoint> {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;

        // Check for existing session affinity first
        if let Some(player_id) = context.player_id {
            if self.config.sticky_sessions {
                if let Some(server_id) = self.get_affinity_server(player_id).await? {
                    if let Some(server) = self.get_server_if_healthy(&server_id).await? {
                        metrics.successful_requests += 1;
                        return Ok(server.endpoint);
                    }
                }
            }
        }

        // Select server using configured algorithm
        let selected_server = match &self.config.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                self.round_robin_selection().await?
            }
            LoadBalancingAlgorithm::LeastConnections => {
                self.least_connections_selection().await?
            }
            LoadBalancingAlgorithm::WeightedRoundRobin { weights } => {
                self.weighted_round_robin_selection(weights.clone()).await?
            }
            LoadBalancingAlgorithm::GeographicProximity => {
                self.geographic_proximity_selection(&context).await?
            }
            LoadBalancingAlgorithm::ResourceBased => {
                self.resource_based_selection().await?
            }
            LoadBalancingAlgorithm::PlayerAffinity => {
                self.player_affinity_selection(&context).await?
            }
        };

        // Update session affinity if enabled
        if let Some(player_id) = context.player_id {
            if self.config.sticky_sessions {
                self.set_session_affinity(player_id, &selected_server.endpoint.id).await?;
            }
        }

        // Update metrics
        metrics.successful_requests += 1;
        let algorithm_name = format!("{:?}", self.config.algorithm);
        *metrics.load_balancing_decisions.entry(algorithm_name).or_insert(0) += 1;

        Ok(selected_server.endpoint)
    }

    /// Mark server as unhealthy
    pub async fn mark_server_unhealthy(&self, server_id: &str) -> Result<()> {
        let mut server_pool = self.server_pool.write().await;
        
        if let Some(server) = server_pool.servers.get_mut(server_id) {
            server.health_status = HealthStatus::Unhealthy;
            server.last_health_check = Instant::now();
        }

        // Remove from healthy servers list
        server_pool.healthy_servers.retain(|id| id != server_id);
        
        // Add to unhealthy servers list if not already there
        if !server_pool.unhealthy_servers.contains(&server_id.to_string()) {
            server_pool.unhealthy_servers.push(server_id.to_string());
        }

        warn!("❌ Marked server {} as unhealthy", server_id);
        Ok(())
    }

    /// Mark server as healthy
    pub async fn mark_server_healthy(&self, server_id: &str) -> Result<()> {
        let mut server_pool = self.server_pool.write().await;
        
        if let Some(server) = server_pool.servers.get_mut(server_id) {
            server.health_status = HealthStatus::Healthy;
            server.last_health_check = Instant::now();
        }

        // Add to healthy servers list if not already there
        if !server_pool.healthy_servers.contains(&server_id.to_string()) {
            server_pool.healthy_servers.push(server_id.to_string());
        }
        
        // Remove from unhealthy servers list
        server_pool.unhealthy_servers.retain(|id| id != server_id);

        info!("✅ Marked server {} as healthy", server_id);
        Ok(())
    }

    /// Get load balancer metrics
    pub async fn get_metrics(&self) -> Result<LoadBalancerMetrics> {
        Ok(self.metrics.read().await.clone())
    }

    /// Update server metrics
    pub async fn update_server_metrics(&self, server_id: &str, connections: u32, response_time: Duration) -> Result<()> {
        let mut server_pool = self.server_pool.write().await;
        
        if let Some(server) = server_pool.servers.get_mut(server_id) {
            server.current_connections = connections;
            server.response_times.push(response_time);
            
            // Keep only last 100 response times for rolling average
            if server.response_times.len() > 100 {
                server.response_times.remove(0);
            }
        }

        Ok(())
    }

    // Private helper methods
    async fn start_health_checking(&self) -> Result<()> {
        let health_checker = self.health_checker.clone();
        let server_pool = self.server_pool.clone();
        let check_interval = self.config.health_check_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            
            loop {
                interval.tick().await;
                
                let servers: Vec<String> = {
                    let pool = server_pool.read().await;
                    pool.servers.keys().cloned().collect()
                };

                for server_id in servers {
                    if let Err(e) = Self::perform_health_check(&health_checker, &server_pool, &server_id).await {
                        warn!("Health check failed for server {}: {}", server_id, e);
                    }
                }
            }
        });

        Ok(())
    }

    async fn perform_health_check(
        health_checker: &Arc<RwLock<HealthChecker>>,
        server_pool: &Arc<RwLock<ServerPool>>,
        server_id: &str,
    ) -> Result<()> {
        let health_check = {
            let checker = health_checker.read().await;
            checker.checks.get(server_id).cloned()
        };

        if let Some(mut check) = health_check {
            let start_time = Instant::now();
            let health_result = Self::execute_health_check(&check).await;
            let response_time = start_time.elapsed();

            check.response_time = Some(response_time);
            check.last_check = Instant::now();

            match health_result {
                Ok(()) => {
                    check.consecutive_successes += 1;
                    check.consecutive_failures = 0;
                    
                    // Mark server as healthy if it meets success threshold
                    let success_threshold = health_checker.read().await.success_threshold;
                    if check.consecutive_successes >= success_threshold {
                        let mut pool = server_pool.write().await;
                        if let Some(server) = pool.servers.get_mut(server_id) {
                            server.health_status = HealthStatus::Healthy;
                        }
                    }
                }
                Err(_) => {
                    check.consecutive_failures += 1;
                    check.consecutive_successes = 0;
                    
                    // Mark server as unhealthy if it meets failure threshold
                    let failure_threshold = health_checker.read().await.failure_threshold;
                    if check.consecutive_failures >= failure_threshold {
                        let mut pool = server_pool.write().await;
                        if let Some(server) = pool.servers.get_mut(server_id) {
                            server.health_status = HealthStatus::Unhealthy;
                        }
                    }
                }
            }

            // Update health check
            let mut checker = health_checker.write().await;
            checker.checks.insert(server_id.to_string(), check);
        }

        Ok(())
    }

    async fn execute_health_check(check: &HealthCheck) -> Result<()> {
        match &check.check_type {
            HealthCheckType::HTTP { endpoint, expected_status } => {
                let response = reqwest::get(endpoint).await?;
                if response.status().as_u16() == *expected_status {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Unexpected status code: {}", response.status()))
                }
            }
            HealthCheckType::TCP { port: _ } => {
                // Implementation would attempt TCP connection
                Ok(())
            }
            HealthCheckType::UDP { port: _ } => {
                // Implementation would send UDP packet
                Ok(())
            }
            HealthCheckType::Ping => {
                // Implementation would send ICMP ping
                Ok(())
            }
            HealthCheckType::Custom { command: _ } => {
                // Implementation would execute custom command
                Ok(())
            }
        }
    }

    async fn start_server_discovery(&self) -> Result<()> {
        let service_discovery = self.service_discovery.clone();
        let server_pool = self.server_pool.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Discover new servers
                if let Ok(discovered_servers) = service_discovery.discover_services("game_server").await {
                    let mut pool = server_pool.write().await;
                    
                    for server_endpoint in discovered_servers {
                        if !pool.servers.contains_key(&server_endpoint.id) {
                            let lb_server = LoadBalancedServer {
                                endpoint: server_endpoint,
                                health_status: HealthStatus::Unknown,
                                current_connections: 0,
                                weight: 1,
                                response_times: Vec::new(),
                                last_health_check: Instant::now(),
                                circuit_breaker_state: CircuitBreakerState::Closed,
                                sticky_sessions: HashMap::new(),
                            };
                            
                            pool.servers.insert(lb_server.endpoint.id.clone(), lb_server);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn start_metrics_collection(&self) -> Result<()> {
        let metrics = self.metrics.clone();
        let server_pool = self.server_pool.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                // Collect and update metrics
                let mut metrics_guard = metrics.write().await;
                let pool = server_pool.read().await;
                
                // Calculate average server utilization
                let mut total_utilization = 0.0;
                let mut server_count = 0;
                
                for server in pool.servers.values() {
                    if server.health_status == HealthStatus::Healthy {
                        let utilization = (server.current_connections as f32 / server.endpoint.capacity as f32) * 100.0;
                        metrics_guard.server_utilization.insert(server.endpoint.id.clone(), utilization);
                        total_utilization += utilization;
                        server_count += 1;
                    }
                }
                
                if server_count > 0 {
                    metrics_guard.average_load = total_utilization / server_count as f32;
                }
            }
        });

        Ok(())
    }

    async fn start_session_cleanup(&self) -> Result<()> {
        let session_affinity = self.session_affinity.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                let mut affinity = session_affinity.write().await;
                let now = Instant::now();
                let session_timeout = affinity.session_timeout;
                
                // Remove expired sessions
                let expired_players: Vec<Uuid> = affinity.session_timeouts
                    .iter()
                    .filter(|(_, &timeout)| now.duration_since(timeout) > session_timeout)
                    .map(|(&player_id, _)| player_id)
                    .collect();
                
                for player_id in expired_players {
                    affinity.player_sessions.remove(&player_id);
                    affinity.session_timeouts.remove(&player_id);
                }
            }
        });

        Ok(())
    }

    async fn get_affinity_server(&self, player_id: Uuid) -> Result<Option<String>> {
        let affinity = self.session_affinity.read().await;
        Ok(affinity.player_sessions.get(&player_id).cloned())
    }

    async fn set_session_affinity(&self, player_id: Uuid, server_id: &str) -> Result<()> {
        let mut affinity = self.session_affinity.write().await;
        affinity.player_sessions.insert(player_id, server_id.to_string());
        affinity.session_timeouts.insert(player_id, Instant::now());
        Ok(())
    }

    async fn get_server_if_healthy(&self, server_id: &str) -> Result<Option<LoadBalancedServer>> {
        let pool = self.server_pool.read().await;
        
        if let Some(server) = pool.servers.get(server_id) {
            if server.health_status == HealthStatus::Healthy {
                return Ok(Some(server.clone()));
            }
        }
        
        Ok(None)
    }

    // Load balancing algorithm implementations
    async fn round_robin_selection(&self) -> Result<LoadBalancedServer> {
        let pool = self.server_pool.read().await;
        let healthy_servers = &pool.healthy_servers;
        
        if healthy_servers.is_empty() {
            return Err(anyhow::anyhow!("No healthy servers available"));
        }

        // Simple round-robin (would need proper state tracking in production)
        let server_id = &healthy_servers[0];
        pool.servers.get(server_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Server not found"))
    }

    async fn least_connections_selection(&self) -> Result<LoadBalancedServer> {
        let pool = self.server_pool.read().await;
        
        let mut best_server: Option<LoadBalancedServer> = None;
        let mut min_connections = u32::MAX;

        for server_id in &pool.healthy_servers {
            if let Some(server) = pool.servers.get(server_id) {
                if server.current_connections < min_connections {
                    min_connections = server.current_connections;
                    best_server = Some(server.clone());
                }
            }
        }

        best_server.ok_or_else(|| anyhow::anyhow!("No healthy servers available"))
    }

    async fn weighted_round_robin_selection(&self, weights: HashMap<String, u32>) -> Result<LoadBalancedServer> {
        let pool = self.server_pool.read().await;
        
        // Implementation would use weighted selection algorithm
        // For now, fall back to regular round robin
        self.round_robin_selection().await
    }

    async fn geographic_proximity_selection(&self, context: &LoadBalancingContext) -> Result<LoadBalancedServer> {
        if let Some(player_location) = &context.player_location {
            let pool = self.server_pool.read().await;
            
            let mut best_server: Option<LoadBalancedServer> = None;
            let mut best_score = f32::MIN;

            for server_id in &pool.healthy_servers {
                if let Some(server) = pool.servers.get(server_id) {
                    let score = self.calculate_geographic_score(server, player_location).await;
                    if score > best_score {
                        best_score = score;
                        best_server = Some(server.clone());
                    }
                }
            }

            best_server.ok_or_else(|| anyhow::anyhow!("No suitable server found"))
        } else {
            self.least_connections_selection().await
        }
    }

    async fn resource_based_selection(&self) -> Result<LoadBalancedServer> {
        let pool = self.server_pool.read().await;
        
        let mut best_server: Option<LoadBalancedServer> = None;
        let mut best_score = f32::MIN;

        for server_id in &pool.healthy_servers {
            if let Some(server) = pool.servers.get(server_id) {
                let score = self.calculate_resource_score(server).await;
                if score > best_score {
                    best_score = score;
                    best_server = Some(server.clone());
                }
            }
        }

        best_server.ok_or_else(|| anyhow::anyhow!("No healthy servers available"))
    }

    async fn player_affinity_selection(&self, context: &LoadBalancingContext) -> Result<LoadBalancedServer> {
        // Implementation would consider player's social connections, guilds, etc.
        // For now, fall back to geographic proximity
        self.geographic_proximity_selection(context).await
    }

    async fn calculate_geographic_score(&self, server: &LoadBalancedServer, player_location: &GeoLocation) -> f32 {
        // Simple scoring based on latency and load
        let load_factor = 1.0 - (server.current_connections as f32 / server.endpoint.capacity as f32);
        let latency_factor = 1.0 - (server.endpoint.latency_score / 1000.0); // Normalize latency
        
        (load_factor * 0.6) + (latency_factor * 0.4)
    }

    async fn calculate_resource_score(&self, server: &LoadBalancedServer) -> f32 {
        let load_factor = 1.0 - (server.current_connections as f32 / server.endpoint.capacity as f32);
        let response_factor = if let Some(latest_response) = server.response_times.last() {
            1.0 - (latest_response.as_millis() as f32 / 1000.0).min(1.0)
        } else {
            0.5
        };
        
        (load_factor * 0.7) + (response_factor * 0.3)
    }
}

// Helper trait for duration conversion
trait DurationExt {
    fn from_hours(hours: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 3600)
    }
}