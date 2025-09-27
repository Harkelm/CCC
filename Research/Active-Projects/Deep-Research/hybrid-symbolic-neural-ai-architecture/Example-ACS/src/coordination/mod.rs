//! Agent Coordination System
//!
//! This module demonstrates the core coordination patterns identified in the research,
//! showing how agents with algorithmic behaviors coordinate through REDB state management
//! and semantic understanding via Qdrant integration.

use crate::behavioral::{AgentBehavior, AgentContext, AgentAction, AgentError, Intent};
use crate::storage::{HybridStorage, HybridStorageCoordinator, AgentEntity, KnowledgeEntity, CoordinationEntity};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod task_delegation;
pub mod consensus;
pub mod event_sourcing;

/// Multi-agent coordination hub implementing research patterns
pub struct AgentCoordinationHub {
    // Hybrid storage for state and semantic operations
    storage: Arc<HybridStorageCoordinator>,

    // Registered agents with their behavioral implementations
    agents: Arc<RwLock<HashMap<Uuid, Arc<dyn AgentBehavior>>>>,

    // Coordination state and session management
    coordination_state: Arc<RwLock<CoordinationState>>,

    // Configuration
    config: CoordinationConfig,
}

/// Coordination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    pub max_concurrent_agents: usize,
    pub coordination_timeout_ms: u64,
    pub consensus_threshold: f64,
    pub enable_semantic_routing: bool,
    pub enable_cloud_delegation: bool,
}

/// Coordination state tracking active sessions and operations
#[derive(Debug, Default)]
struct CoordinationState {
    active_sessions: HashMap<Uuid, CoordinationSession>,
    agent_status: HashMap<Uuid, AgentStatus>,
    performance_metrics: CoordinationMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoordinationSession {
    session_id: Uuid,
    agents: Vec<Uuid>,
    current_task: Option<TaskExecution>,
    status: SessionStatus,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskExecution {
    task_id: Uuid,
    task_type: String,
    assigned_agent: Uuid,
    status: TaskStatus,
    results: Option<serde_json::Value>,
    started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SessionStatus {
    Active,
    Completed,
    Failed,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Delegated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AgentStatus {
    Available,
    Busy,
    Offline,
    Error,
}

#[derive(Debug, Default, Clone)]
struct CoordinationMetrics {
    total_sessions: u64,
    successful_completions: u64,
    failed_operations: u64,
    average_completion_time_ms: f64,
    agent_utilization: HashMap<Uuid, f64>,
}

/// Coordination traits for multi-agent operations
#[async_trait]
pub trait AgentCoordination: Send + Sync {
    /// Register an agent with the coordination hub
    async fn register_agent(&self, agent: Arc<dyn AgentBehavior>) -> Result<Uuid, CoordinationError>;

    /// Execute coordinated multi-agent task
    async fn execute_coordinated_task(&self, task: &CoordinationTask) -> Result<CoordinationResult, CoordinationError>;

    /// Route task to appropriate agent based on semantic understanding
    async fn route_task_semantically(&self, intent: &Intent) -> Result<Uuid, CoordinationError>;

    /// Delegate complex tasks to cloud services if configured
    async fn delegate_to_cloud(&self, task: &CloudDelegationTask) -> Result<serde_json::Value, CoordinationError>;

    /// Get coordination metrics and status
    async fn get_coordination_status(&self) -> CoordinationStatus;
}

/// Task coordination structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationTask {
    pub task_id: Uuid,
    pub intent: Intent,
    pub required_capabilities: Vec<String>,
    pub priority: TaskPriority,
    pub timeout_ms: Option<u64>,
    pub context: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationResult {
    pub task_id: Uuid,
    pub agent_id: Uuid,
    pub status: TaskStatus,
    pub results: serde_json::Value,
    pub execution_time_ms: u64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDelegationTask {
    pub task_type: String,
    pub prompt: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub model_preference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationStatus {
    pub active_sessions: usize,
    pub registered_agents: usize,
    pub average_load: f64,
    pub performance_metrics: CoordinationMetrics,
}

impl AgentCoordinationHub {
    /// Create new coordination hub with hybrid storage
    pub async fn new(
        storage: Arc<HybridStorageCoordinator>,
        config: CoordinationConfig,
    ) -> Result<Self, CoordinationError> {
        Ok(Self {
            storage,
            agents: Arc::new(RwLock::new(HashMap::new())),
            coordination_state: Arc::new(RwLock::new(CoordinationState::default())),
            config,
        })
    }

    /// Semantic task routing using vector similarity
    async fn route_task_by_similarity(&self, intent: &Intent) -> Result<Uuid, CoordinationError> {
        // Create search query from intent
        let search_query = format!("{} {}",
            intent.action_type,
            intent.parameters.values()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        );

        // Search for similar successful tasks
        let similar_tasks = self.storage
            .search_knowledge(&search_query, 5)
            .await
            .map_err(|e| CoordinationError::SemanticRoutingError(format!("Failed to search for similar tasks: {}", e)))?;

        if let Some(best_match) = similar_tasks.first() {
            // Extract agent ID from metadata
            if let Some(agent_id_value) = best_match.metadata.get("agent_id") {
                if let Some(agent_id_str) = agent_id_value.as_str() {
                    if let Ok(agent_id) = Uuid::parse_str(agent_id_str) {
                        return Ok(agent_id);
                    }
                }
            }
        }

        // Fallback to capability-based routing
        self.route_by_capabilities(&intent.action_type).await
    }

    /// Capability-based agent routing
    async fn route_by_capabilities(&self, action_type: &str) -> Result<Uuid, CoordinationError> {
        let agents = self.agents.read().await;

        for (agent_id, agent) in agents.iter() {
            let state = agent.get_state();
            if state.capabilities.iter().any(|cap| cap.contains(action_type)) {
                return Ok(*agent_id);
            }
        }

        Err(CoordinationError::NoCapableAgent(format!(
            "No agent found with capability for action: {}", action_type
        )))
    }

    /// Execute agent task with coordination tracking
    async fn execute_agent_task(
        &self,
        agent_id: Uuid,
        context: &AgentContext,
    ) -> Result<AgentAction, CoordinationError> {
        // Get agent from registry
        let agents = self.agents.read().await;
        let agent = agents
            .get(&agent_id)
            .ok_or_else(|| CoordinationError::AgentNotFound(agent_id))?
            .clone();
        drop(agents);

        // Update agent status
        {
            let mut state = self.coordination_state.write().await;
            state.agent_status.insert(agent_id, AgentStatus::Busy);
        }

        // Execute agent behavior
        let start_time = std::time::Instant::now();
        let result = agent.execute(context).await;
        let execution_time = start_time.elapsed();

        // Update agent status and metrics
        {
            let mut state = self.coordination_state.write().await;
            state.agent_status.insert(agent_id, AgentStatus::Available);

            // Update performance metrics
            state.performance_metrics.total_sessions += 1;
            if result.is_ok() {
                state.performance_metrics.successful_completions += 1;
            } else {
                state.performance_metrics.failed_operations += 1;
            }

            // Update average completion time
            let total_time = state.performance_metrics.average_completion_time_ms
                * (state.performance_metrics.total_sessions - 1) as f64
                + execution_time.as_millis() as f64;
            state.performance_metrics.average_completion_time_ms =
                total_time / state.performance_metrics.total_sessions as f64;
        }

        result.map_err(|e| CoordinationError::AgentExecutionError(format!("Agent execution failed: {}", e)))
    }

    /// Store task execution knowledge for future semantic routing
    async fn store_task_knowledge(
        &self,
        task: &CoordinationTask,
        agent_id: Uuid,
        result: &CoordinationResult,
    ) -> Result<(), CoordinationError> {
        let knowledge = KnowledgeEntity {
            id: Uuid::new_v4(),
            content: format!(
                "Task: {} executed by agent {} with status {:?}. Intent: {} {}",
                task.task_id,
                agent_id,
                result.status,
                task.intent.action_type,
                task.intent.context
            ),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("task_id".to_string(), task.task_id.to_string().into());
                metadata.insert("agent_id".to_string(), agent_id.to_string().into());
                metadata.insert("action_type".to_string(), task.intent.action_type.clone().into());
                metadata.insert("status".to_string(), serde_json::to_value(&result.status).unwrap());
                metadata.insert("execution_time_ms".to_string(), result.execution_time_ms.into());
                metadata
            },
            embeddings: None, // Will be generated by storage layer
            source: "coordination_hub".to_string(),
            credibility_rating: "A1".to_string(), // Internal system data
            created_at: chrono::Utc::now(),
        };

        self.storage
            .store_knowledge(&knowledge)
            .await
            .map_err(|e| CoordinationError::StorageError(format!("Failed to store task knowledge: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl AgentCoordination for AgentCoordinationHub {
    async fn register_agent(&self, agent: Arc<dyn AgentBehavior>) -> Result<Uuid, CoordinationError> {
        let agent_id = Uuid::new_v4();
        let agent_state = agent.get_state();

        // Store agent in registry
        {
            let mut agents = self.agents.write().await;
            agents.insert(agent_id, agent);
        }

        // Store agent entity in hybrid storage
        let agent_entity = AgentEntity {
            id: agent_id,
            agent_type: agent_state.capabilities.first()
                .unwrap_or(&"unknown".to_string())
                .clone(),
            state: serde_json::to_value(&agent_state)
                .map_err(|e| CoordinationError::SerializationError(format!("Failed to serialize agent state: {}", e)))?,
            capabilities: agent_state.capabilities,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.storage
            .store_agent(&agent_entity)
            .await
            .map_err(|e| CoordinationError::StorageError(format!("Failed to store agent: {}", e)))?;

        // Update coordination state
        {
            let mut state = self.coordination_state.write().await;
            state.agent_status.insert(agent_id, AgentStatus::Available);
        }

        Ok(agent_id)
    }

    async fn execute_coordinated_task(&self, task: &CoordinationTask) -> Result<CoordinationResult, CoordinationError> {
        let start_time = std::time::Instant::now();

        // Route task to appropriate agent
        let agent_id = if self.config.enable_semantic_routing {
            self.route_task_semantically(&task.intent).await?
        } else {
            self.route_by_capabilities(&task.intent.action_type).await?
        };

        // Create agent context
        let context = AgentContext {
            session_id: Uuid::new_v4(),
            user_intent: task.intent.clone(),
            available_resources: task.context.clone(),
            constraints: vec![], // Could be derived from task requirements
            timestamp: chrono::Utc::now(),
        };

        // Execute task with selected agent
        let agent_action = self.execute_agent_task(agent_id, &context).await?;

        let execution_time = start_time.elapsed();

        // Create coordination result
        let result = CoordinationResult {
            task_id: task.task_id,
            agent_id,
            status: match agent_action.status {
                crate::behavioral::ActionStatus::Completed => TaskStatus::Completed,
                crate::behavioral::ActionStatus::InProgress => TaskStatus::InProgress,
                crate::behavioral::ActionStatus::Failed => TaskStatus::Failed,
                crate::behavioral::ActionStatus::RequiresInput => TaskStatus::Pending,
                crate::behavioral::ActionStatus::Blocked => TaskStatus::Failed,
            },
            results: agent_action.results,
            execution_time_ms: execution_time.as_millis() as u64,
            evidence: agent_action.evidence.iter().map(|e| e.source_id.clone()).collect(),
        };

        // Store task knowledge for future semantic routing
        self.store_task_knowledge(task, agent_id, &result).await?;

        // Store coordination record
        let coordination_entity = CoordinationEntity {
            id: Uuid::new_v4(),
            session_id: context.session_id,
            operation_type: "task_execution".to_string(),
            status: format!("{:?}", result.status),
            data: serde_json::to_value(&result)
                .map_err(|e| CoordinationError::SerializationError(format!("Failed to serialize result: {}", e)))?,
            timestamp: chrono::Utc::now(),
        };

        self.storage
            .update_coordination(&coordination_entity)
            .await
            .map_err(|e| CoordinationError::StorageError(format!("Failed to store coordination record: {}", e)))?;

        Ok(result)
    }

    async fn route_task_semantically(&self, intent: &Intent) -> Result<Uuid, CoordinationError> {
        self.route_task_by_similarity(intent).await
    }

    async fn delegate_to_cloud(&self, task: &CloudDelegationTask) -> Result<serde_json::Value, CoordinationError> {
        if !self.config.enable_cloud_delegation {
            return Err(CoordinationError::CloudDelegationDisabled);
        }

        // This would integrate with actual cloud LLM APIs
        // For demonstration, we'll return a mock response
        Ok(serde_json::json!({
            "status": "completed",
            "response": format!("Cloud processing completed for task: {}", task.task_type),
            "model": task.model_preference.as_ref().unwrap_or(&"default".to_string()),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    async fn get_coordination_status(&self) -> CoordinationStatus {
        let state = self.coordination_state.read().await;
        let agents = self.agents.read().await;

        let available_agents = state.agent_status.values()
            .filter(|status| matches!(status, AgentStatus::Available))
            .count();

        let average_load = if agents.len() > 0 {
            (agents.len() - available_agents) as f64 / agents.len() as f64
        } else {
            0.0
        };

        CoordinationStatus {
            active_sessions: state.active_sessions.len(),
            registered_agents: agents.len(),
            average_load,
            performance_metrics: state.performance_metrics.clone(),
        }
    }
}

/// Coordination error types
#[derive(Debug, thiserror::Error)]
pub enum CoordinationError {
    #[error("Agent not found: {0}")]
    AgentNotFound(Uuid),

    #[error("No capable agent found: {0}")]
    NoCapableAgent(String),

    #[error("Agent execution error: {0}")]
    AgentExecutionError(String),

    #[error("Semantic routing error: {0}")]
    SemanticRoutingError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Cloud delegation is disabled")]
    CloudDelegationDisabled,

    #[error("Coordination timeout")]
    Timeout,

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            max_concurrent_agents: 10,
            coordination_timeout_ms: 30000, // 30 seconds
            consensus_threshold: 0.67,
            enable_semantic_routing: true,
            enable_cloud_delegation: false, // Local-first by default
        }
    }
}