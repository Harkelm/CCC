//! Agent Component System (ACS) Example Implementation
//!
//! This library demonstrates the hybrid symbolic-neural AI architecture
//! identified in the research, showing how agent.md behavioral descriptions
//! translate to algorithmic agent.rs implementations.
//!
//! # Key Concepts Demonstrated
//!
//! 1. **Behavioral Component Translation**: How text-based agent descriptions
//!    become algorithmic Rust trait implementations
//!
//! 2. **Hybrid Storage Architecture**: REDB for structured state management
//!    combined with Qdrant for semantic understanding and similarity search
//!
//! 3. **Agent Coordination Patterns**: Multi-agent coordination through
//!    algorithmic decision-making and consensus mechanisms
//!
//! 4. **Local-First with Cloud Augmentation**: Local operation by default
//!    with optional cloud delegation for complex linguistic tasks
//!
//! # Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                    LINGUISTIC PROCESSING LAYER                     │
//! │  Qdrant Vector DB + Local Embeddings + Optional Cloud LLM          │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │                    ALGORITHMIC INTELLIGENCE LAYER                  │
//! │  Agent.rs Coordination + Decision Trees + Rule Engines + FSMs      │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │                       FOUNDATION STORAGE LAYER                     │
//! │  REDB State Management + Event Sourcing + Transaction Safety       │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```

pub mod behavioral;
pub mod storage;
pub mod coordination;

// Re-export key types for easy access
pub use behavioral::{
    AgentBehavior, AgentContext, AgentAction, AgentError, Intent,
    systematic_research::{SystematicResearcher, SystematicResearchAgent},
    CredibilityRating, Evidence, ValidationCheck,
};

pub use storage::{
    HybridStorage, HybridStorageCoordinator, StorageConfig, StorageError,
    AgentEntity, KnowledgeEntity, CoordinationEntity,
};

pub use coordination::{
    AgentCoordination, AgentCoordinationHub, CoordinationConfig, CoordinationError,
    CoordinationTask, CoordinationResult, CloudDelegationTask,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Main ACS Framework facade providing high-level operations
pub struct ACSFramework {
    coordination_hub: Arc<AgentCoordinationHub>,
    storage: Arc<HybridStorageCoordinator>,
    config: ACSConfig,
}

/// Framework configuration combining all component configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACSConfig {
    pub storage: storage::StorageConfig,
    pub coordination: coordination::CoordinationConfig,
    pub behavioral: BehavioralConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralConfig {
    pub enable_systematic_research: bool,
    pub evidence_threshold: String, // CredibilityRating as string
    pub validation_strictness: ValidationStrictness,
    pub enable_cross_validation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStrictness {
    Essential,   // 10-item validation
    Extended,    // 15-item validation
    Comprehensive, // 27-item validation
}

/// High-level framework operations
#[async_trait]
pub trait ACSFrameworkOperations {
    /// Initialize the complete ACS framework
    async fn initialize(config: ACSConfig) -> Result<Self, ACSError>
    where
        Self: Sized;

    /// Register a new agent with algorithmic behavioral implementation
    async fn register_agent(&self, agent: Arc<dyn AgentBehavior>) -> Result<Uuid, ACSError>;

    /// Execute a task using the best available agent
    async fn execute_task(&self, task: ACSTask) -> Result<ACSResult, ACSError>;

    /// Store knowledge for semantic understanding
    async fn store_knowledge(&self, knowledge: ACSKnowledge) -> Result<(), ACSError>;

    /// Search knowledge using semantic similarity
    async fn search_knowledge(&self, query: &str, limit: usize) -> Result<Vec<ACSKnowledge>, ACSError>;

    /// Get framework status and metrics
    async fn get_status(&self) -> ACSStatus;

    /// Shutdown framework gracefully
    async fn shutdown(&self) -> Result<(), ACSError>;
}

/// High-level task representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACSTask {
    pub description: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Research {
        query: String,
        domain: Option<String>,
        sources_required: usize,
    },
    Analysis {
        content: String,
        analysis_type: String,
    },
    Synthesis {
        inputs: Vec<String>,
        output_format: String,
    },
    Custom {
        action_type: String,
        context: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// High-level result representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACSResult {
    pub task_id: Uuid,
    pub status: ResultStatus,
    pub content: serde_json::Value,
    pub evidence: Vec<ACSEvidence>,
    pub confidence: f64,
    pub execution_time_ms: u64,
    pub agent_info: AgentInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultStatus {
    Success,
    Partial,
    Failed,
    RequiresInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACSEvidence {
    pub source: String,
    pub credibility: String,
    pub content: String,
    pub validation_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub agent_id: Uuid,
    pub agent_type: String,
    pub capabilities: Vec<String>,
}

/// High-level knowledge representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACSKnowledge {
    pub title: String,
    pub content: String,
    pub source: String,
    pub credibility_rating: String,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    pub tags: Vec<String>,
}

/// Framework status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACSStatus {
    pub framework_version: String,
    pub registered_agents: usize,
    pub active_tasks: usize,
    pub knowledge_base_size: usize,
    pub storage_status: String,
    pub coordination_status: String,
    pub performance_metrics: FrameworkMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkMetrics {
    pub tasks_completed: u64,
    pub average_execution_time_ms: f64,
    pub success_rate: f64,
    pub storage_utilization: f64,
    pub agent_utilization: f64,
}

impl ACSFramework {
    /// Create a new systematic research agent (demonstration of behavioral translation)
    pub fn create_systematic_research_agent(
        config: &BehavioralConfig,
    ) -> Arc<dyn AgentBehavior> {
        let validation_config = behavioral::systematic_research::ValidationConfig {
            minimum_sources: 3,
            required_credibility: match config.evidence_threshold.as_str() {
                "A1" => CredibilityRating::A1,
                "A2" => CredibilityRating::A2,
                "A3" => CredibilityRating::A3,
                "B1" => CredibilityRating::B1,
                "B2" => CredibilityRating::B2,
                "B3" => CredibilityRating::B3,
                _ => CredibilityRating::B3, // Default
            },
            enable_bias_detection: true,
            cross_validation_threshold: 0.75,
        };

        Arc::new(behavioral::systematic_research::SystematicResearchAgent::new(validation_config))
    }

    /// Convert high-level task to coordination task
    fn convert_task(&self, task: ACSTask) -> coordination::CoordinationTask {
        let intent = Intent {
            action_type: match &task.task_type {
                TaskType::Research { .. } => "research".to_string(),
                TaskType::Analysis { .. } => "analyze".to_string(),
                TaskType::Synthesis { .. } => "synthesize".to_string(),
                TaskType::Custom { action_type, .. } => action_type.clone(),
            },
            parameters: task.parameters,
            priority: match task.priority {
                TaskPriority::Critical => behavioral::Priority::Critical,
                TaskPriority::High => behavioral::Priority::High,
                TaskPriority::Medium => behavioral::Priority::Medium,
                TaskPriority::Low => behavioral::Priority::Low,
            },
            context: task.description,
        };

        coordination::CoordinationTask {
            task_id: Uuid::new_v4(),
            intent,
            required_capabilities: match &task.task_type {
                TaskType::Research { .. } => vec!["systematic_research".to_string()],
                TaskType::Analysis { .. } => vec!["analysis".to_string()],
                TaskType::Synthesis { .. } => vec!["synthesis".to_string()],
                TaskType::Custom { .. } => vec!["general".to_string()],
            },
            priority: match task.priority {
                TaskPriority::Critical => coordination::TaskPriority::Critical,
                TaskPriority::High => coordination::TaskPriority::High,
                TaskPriority::Medium => coordination::TaskPriority::Medium,
                TaskPriority::Low => coordination::TaskPriority::Low,
            },
            timeout_ms: task.timeout_ms,
            context: std::collections::HashMap::new(),
        }
    }

    /// Convert coordination result to high-level result
    fn convert_result(&self, result: coordination::CoordinationResult, agent_info: AgentInfo) -> ACSResult {
        ACSResult {
            task_id: result.task_id,
            status: match result.status {
                coordination::TaskStatus::Completed => ResultStatus::Success,
                coordination::TaskStatus::InProgress => ResultStatus::Partial,
                coordination::TaskStatus::Failed => ResultStatus::Failed,
                coordination::TaskStatus::Pending => ResultStatus::RequiresInput,
                coordination::TaskStatus::Delegated => ResultStatus::Partial,
            },
            content: result.results,
            evidence: result.evidence.into_iter().map(|source| ACSEvidence {
                source: source.clone(),
                credibility: "B3".to_string(), // Would be extracted from actual evidence
                content: "Evidence content".to_string(),
                validation_status: "Passed".to_string(),
            }).collect(),
            confidence: 0.85, // Would be calculated from actual results
            execution_time_ms: result.execution_time_ms,
            agent_info,
        }
    }
}

#[async_trait]
impl ACSFrameworkOperations for ACSFramework {
    async fn initialize(config: ACSConfig) -> Result<Self, ACSError> {
        // Initialize hybrid storage
        let storage = Arc::new(
            HybridStorageCoordinator::new(config.storage.clone())
                .await
                .map_err(|e| ACSError::InitializationError(format!("Storage initialization failed: {}", e)))?
        );

        // Initialize coordination hub
        let coordination_hub = Arc::new(
            AgentCoordinationHub::new(storage.clone(), config.coordination.clone())
                .await
                .map_err(|e| ACSError::InitializationError(format!("Coordination hub initialization failed: {}", e)))?
        );

        // Create framework instance
        let framework = Self {
            coordination_hub,
            storage,
            config,
        };

        // Register default agents if enabled
        if framework.config.behavioral.enable_systematic_research {
            let research_agent = Self::create_systematic_research_agent(&framework.config.behavioral);
            framework.register_agent(research_agent).await?;
        }

        Ok(framework)
    }

    async fn register_agent(&self, agent: Arc<dyn AgentBehavior>) -> Result<Uuid, ACSError> {
        self.coordination_hub
            .register_agent(agent)
            .await
            .map_err(|e| ACSError::AgentRegistrationError(format!("Failed to register agent: {}", e)))
    }

    async fn execute_task(&self, task: ACSTask) -> Result<ACSResult, ACSError> {
        // Convert high-level task to coordination task
        let coordination_task = self.convert_task(task);

        // Execute through coordination hub
        let result = self.coordination_hub
            .execute_coordinated_task(&coordination_task)
            .await
            .map_err(|e| ACSError::TaskExecutionError(format!("Task execution failed: {}", e)))?;

        // Get agent information
        let agent_info = AgentInfo {
            agent_id: result.agent_id,
            agent_type: "systematic_researcher".to_string(), // Would be retrieved from storage
            capabilities: vec!["research".to_string(), "analysis".to_string()],
        };

        // Convert to high-level result
        Ok(self.convert_result(result, agent_info))
    }

    async fn store_knowledge(&self, knowledge: ACSKnowledge) -> Result<(), ACSError> {
        let knowledge_entity = KnowledgeEntity {
            id: Uuid::new_v4(),
            content: knowledge.content,
            metadata: knowledge.metadata,
            embeddings: None, // Will be generated by storage layer
            source: knowledge.source,
            credibility_rating: knowledge.credibility_rating,
            created_at: chrono::Utc::now(),
        };

        self.storage
            .store_knowledge(&knowledge_entity)
            .await
            .map_err(|e| ACSError::StorageError(format!("Failed to store knowledge: {}", e)))
    }

    async fn search_knowledge(&self, query: &str, limit: usize) -> Result<Vec<ACSKnowledge>, ACSError> {
        let knowledge_entities = self.storage
            .search_knowledge(query, limit)
            .await
            .map_err(|e| ACSError::StorageError(format!("Failed to search knowledge: {}", e)))?;

        let acs_knowledge: Vec<ACSKnowledge> = knowledge_entities
            .into_iter()
            .map(|entity| ACSKnowledge {
                title: entity.metadata.get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled")
                    .to_string(),
                content: entity.content,
                source: entity.source,
                credibility_rating: entity.credibility_rating,
                metadata: entity.metadata,
                tags: vec![], // Would be extracted from metadata
            })
            .collect();

        Ok(acs_knowledge)
    }

    async fn get_status(&self) -> ACSStatus {
        let coordination_status = self.coordination_hub.get_coordination_status().await;
        let storage_metrics = self.storage.get_metrics().await;

        ACSStatus {
            framework_version: "0.1.0".to_string(),
            registered_agents: coordination_status.registered_agents,
            active_tasks: coordination_status.active_sessions,
            knowledge_base_size: 0, // Would be retrieved from storage
            storage_status: "Online".to_string(),
            coordination_status: "Active".to_string(),
            performance_metrics: FrameworkMetrics {
                tasks_completed: coordination_status.performance_metrics.total_sessions,
                average_execution_time_ms: coordination_status.performance_metrics.average_completion_time_ms,
                success_rate: if coordination_status.performance_metrics.total_sessions > 0 {
                    coordination_status.performance_metrics.successful_completions as f64
                        / coordination_status.performance_metrics.total_sessions as f64
                } else {
                    0.0
                },
                storage_utilization: 0.0, // Would be calculated from storage metrics
                agent_utilization: coordination_status.average_load,
            },
        }
    }

    async fn shutdown(&self) -> Result<(), ACSError> {
        // Graceful shutdown would involve:
        // 1. Completing active tasks
        // 2. Saving state
        // 3. Closing database connections
        // 4. Cleanup resources

        // For this example, we'll just synchronize storage
        self.storage
            .synchronize()
            .await
            .map_err(|e| ACSError::ShutdownError(format!("Failed to synchronize storage: {}", e)))?;

        Ok(())
    }
}

/// Framework error types
#[derive(Debug, thiserror::Error)]
pub enum ACSError {
    #[error("Initialization error: {0}")]
    InitializationError(String),

    #[error("Agent registration error: {0}")]
    AgentRegistrationError(String),

    #[error("Task execution error: {0}")]
    TaskExecutionError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Shutdown error: {0}")]
    ShutdownError(String),
}

impl Default for ACSConfig {
    fn default() -> Self {
        Self {
            storage: storage::StorageConfig::default(),
            coordination: coordination::CoordinationConfig::default(),
            behavioral: BehavioralConfig {
                enable_systematic_research: true,
                evidence_threshold: "B3".to_string(),
                validation_strictness: ValidationStrictness::Essential,
                enable_cross_validation: true,
            },
        }
    }
}

/// Helper function to create a default ACS framework instance
pub async fn create_default_framework() -> Result<ACSFramework, ACSError> {
    ACSFramework::initialize(ACSConfig::default()).await
}

/// Helper function to create a research-focused ACS framework
pub async fn create_research_framework() -> Result<ACSFramework, ACSError> {
    let mut config = ACSConfig::default();
    config.behavioral.enable_systematic_research = true;
    config.behavioral.evidence_threshold = "B3".to_string();
    config.behavioral.validation_strictness = ValidationStrictness::Extended;
    config.coordination.enable_semantic_routing = true;

    ACSFramework::initialize(config).await
}