//! Behavioral Component System - Algorithmic Intelligence Traits
//!
//! This module demonstrates how agent.md behavioral descriptions translate
//! to concrete Rust trait implementations with algorithmic intelligence.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod systematic_research;
pub mod evidence_validation;
pub mod decision_making;

/// Core behavioral trait that all agents must implement
#[async_trait]
pub trait AgentBehavior: Send + Sync {
    /// Execute the primary behavioral pattern for this agent
    async fn execute(&self, context: &AgentContext) -> Result<AgentAction, AgentError>;

    /// Validate that the agent can perform the requested action
    fn can_handle(&self, intent: &Intent) -> bool;

    /// Get the agent's current state and capabilities
    fn get_state(&self) -> AgentState;
}

/// Agent execution context containing state and environment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub session_id: Uuid,
    pub user_intent: Intent,
    pub available_resources: ResourceMap,
    pub constraints: Vec<Constraint>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// User intent representation for semantic understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub action_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub priority: Priority,
    pub context: String,
}

/// Agent action result with deterministic outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub action_id: Uuid,
    pub action_type: String,
    pub status: ActionStatus,
    pub results: serde_json::Value,
    pub evidence: Vec<Evidence>,
    pub next_actions: Vec<String>,
}

/// Evidence structure for systematic validation (replaces text-based validation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub source_id: String,
    pub source_type: SourceType,
    pub credibility_rating: CredibilityRating,
    pub content: String,
    pub validation_checks: Vec<ValidationCheck>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Admiralty Code-based credibility rating (algorithmic implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredibilityRating {
    A1, A2, A3, A4, A5, A6,  // Completely reliable
    B1, B2, B3, B4, B5, B6,  // Usually reliable
    C1, C2, C3, C4, C5, C6,  // Fairly reliable
    D1, D2, D3, D4, D5, D6,  // Not usually reliable
    E1, E2, E3, E4, E5, E6,  // Unreliable
    F1, F2, F3, F4, F5, F6,  // Cannot judge
}

impl CredibilityRating {
    /// Algorithmic implementation of credibility assessment
    pub fn meets_threshold(&self, minimum: &CredibilityRating) -> bool {
        self.numeric_value() >= minimum.numeric_value()
    }

    pub fn numeric_value(&self) -> u8 {
        match self {
            Self::A1 => 61, Self::A2 => 62, Self::A3 => 63, Self::A4 => 64, Self::A5 => 65, Self::A6 => 66,
            Self::B1 => 51, Self::B2 => 52, Self::B3 => 53, Self::B4 => 54, Self::B5 => 55, Self::B6 => 56,
            Self::C1 => 41, Self::C2 => 42, Self::C3 => 43, Self::C4 => 44, Self::C5 => 45, Self::C6 => 46,
            Self::D1 => 31, Self::D2 => 32, Self::D3 => 33, Self::D4 => 34, Self::D5 => 35, Self::D6 => 36,
            Self::E1 => 21, Self::E2 => 22, Self::E3 => 23, Self::E4 => 24, Self::E5 => 25, Self::E6 => 26,
            Self::F1 => 11, Self::F2 => 12, Self::F3 => 13, Self::F4 => 14, Self::F5 => 15, Self::F6 => 16,
        }
    }
}

/// Validation check results (algorithmic implementation of validation tiers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    pub check_type: ValidationType,
    pub status: ValidationStatus,
    pub details: String,
    pub automated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    SourceCredibility,
    ContentAccuracy,
    BiasAssessment,
    CrossValidation,
    MethodologicalSoundness,
    EvidenceQuality,
    ConsistencyCheck,
    TemporalRelevance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Passed,
    Failed,
    Warning,
    RequiresHumanReview,
}

/// Additional supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    PeerReviewed,
    OfficialDocumentation,
    ExpertOpinion,
    CommunitySource,
    CommercialContent,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionStatus {
    InProgress,
    Completed,
    Failed,
    RequiresInput,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub agent_id: Uuid,
    pub status: AgentStatus,
    pub capabilities: Vec<String>,
    pub current_task: Option<String>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Processing,
    WaitingForInput,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub tasks_completed: u64,
    pub average_processing_time_ms: f64,
    pub success_rate: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

pub type ResourceMap = HashMap<String, serde_json::Value>;
pub type Constraint = String;

/// Agent error types with specific handling patterns
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Storage error: {0}")]
    StorageError(#[from] redb::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Insufficient evidence quality: required {required:?}, found {found:?}")]
    InsufficientEvidence {
        required: CredibilityRating,
        found: CredibilityRating,
    },

    #[error("Agent capability mismatch: {0}")]
    CapabilityMismatch(String),

    #[error("Internal error: {0}")]
    Internal(String),
}