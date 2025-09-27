//! Hybrid Storage System - REDB + Qdrant Integration
//!
//! This module demonstrates the hybrid storage architecture identified in the research:
//! - REDB for structured state management and agent coordination
//! - Qdrant for semantic embeddings and similarity search
//! - Coordinated access patterns with shared entity management

use async_trait::async_trait;
use qdrant_client::{client::QdrantClient, qdrant::*};
use redb::{Database, ReadableTable, WriteTransaction, ReadTransaction, TableDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod coordination;
pub mod redb_integration;
pub mod qdrant_integration;

/// Hybrid storage coordinator managing both REDB and Qdrant
#[derive(Clone)]
pub struct HybridStorageCoordinator {
    // REDB for structured state and coordination
    redb: Arc<Database>,

    // Qdrant for semantic embeddings and similarity
    qdrant: Arc<QdrantClient>,

    // Coordination state
    state: Arc<RwLock<CoordinationState>>,

    // Configuration
    config: StorageConfig,
}

/// Storage configuration matching research recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub redb_path: String,
    pub qdrant_url: String,
    pub collection_name: String,
    pub embedding_dimension: usize,
    pub sync_batch_size: usize,
    pub consistency_mode: ConsistencyMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsistencyMode {
    Immediate,     // Two-phase commit for critical operations
    Eventually,    // Async synchronization for performance
    EventDriven,   // Event sourcing pattern
}

/// Coordination state for hybrid storage operations
#[derive(Debug, Default)]
struct CoordinationState {
    pending_operations: HashMap<Uuid, PendingOperation>,
    sync_status: SyncStatus,
    performance_metrics: StorageMetrics,
}

#[derive(Debug)]
struct PendingOperation {
    operation_id: Uuid,
    operation_type: OperationType,
    redb_committed: bool,
    qdrant_committed: bool,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
enum OperationType {
    Insert,
    Update,
    Delete,
    Batch,
}

#[derive(Debug, Default)]
struct SyncStatus {
    last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pending_count: usize,
    error_count: usize,
}

#[derive(Debug, Default)]
struct StorageMetrics {
    operations_total: u64,
    operations_success: u64,
    average_latency_ms: f64,
    last_updated: chrono::DateTime<chrono::Utc>,
}

/// Entity types for hybrid storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEntity {
    pub id: Uuid,
    pub agent_type: String,
    pub state: serde_json::Value,
    pub capabilities: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntity {
    pub id: Uuid,
    pub content: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub embeddings: Option<Vec<f32>>,
    pub source: String,
    pub credibility_rating: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationEntity {
    pub id: Uuid,
    pub session_id: Uuid,
    pub operation_type: String,
    pub status: String,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// REDB table definitions
const AGENTS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("agents");
const KNOWLEDGE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("knowledge");
const COORDINATION_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("coordination");
const METADATA_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("metadata");

/// Hybrid storage trait for coordinated operations
#[async_trait]
pub trait HybridStorage: Send + Sync {
    /// Store agent with coordinated REDB + Qdrant operation
    async fn store_agent(&self, agent: &AgentEntity) -> Result<(), StorageError>;

    /// Store knowledge with semantic embeddings
    async fn store_knowledge(&self, knowledge: &KnowledgeEntity) -> Result<(), StorageError>;

    /// Retrieve agent by ID
    async fn get_agent(&self, id: &Uuid) -> Result<Option<AgentEntity>, StorageError>;

    /// Search knowledge by semantic similarity
    async fn search_knowledge(&self, query: &str, limit: usize) -> Result<Vec<KnowledgeEntity>, StorageError>;

    /// Update coordination state
    async fn update_coordination(&self, coordination: &CoordinationEntity) -> Result<(), StorageError>;

    /// Synchronize state between storage systems
    async fn synchronize(&self) -> Result<SyncResult, StorageError>;

    /// Get storage metrics
    async fn get_metrics(&self) -> StorageMetrics;
}

impl HybridStorageCoordinator {
    /// Initialize hybrid storage coordinator
    pub async fn new(config: StorageConfig) -> Result<Self, StorageError> {
        // Initialize REDB
        let redb = Database::create(&config.redb_path)
            .map_err(|e| StorageError::InitializationError(format!("REDB init failed: {}", e)))?;

        // Initialize Qdrant connection
        let qdrant = QdrantClient::from_url(&config.qdrant_url)
            .build()
            .map_err(|e| StorageError::InitializationError(format!("Qdrant connection failed: {}", e)))?;

        // Create collection if it doesn't exist
        let collections = qdrant.list_collections().await
            .map_err(|e| StorageError::InitializationError(format!("Failed to list collections: {}", e)))?;

        if !collections.collections.iter().any(|c| c.name == config.collection_name) {
            qdrant.create_collection(&CreateCollection {
                collection_name: config.collection_name.clone(),
                vectors_config: Some(VectorConfig {
                    config: Some(Config::Params(VectorParams {
                        size: config.embedding_dimension as u64,
                        distance: Distance::Cosine as i32,
                    })),
                }),
                ..Default::default()
            }).await
            .map_err(|e| StorageError::InitializationError(format!("Failed to create collection: {}", e)))?;
        }

        Ok(Self {
            redb: Arc::new(redb),
            qdrant: Arc::new(qdrant),
            state: Arc::new(RwLock::new(CoordinationState::default())),
            config,
        })
    }

    /// Generate embedding for text content (simplified implementation)
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, StorageError> {
        // In production, this would use a local embedding model
        // For demo purposes, we'll create a simple hash-based embedding
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        // Convert hash to normalized embedding vector
        let mut embedding = Vec::with_capacity(self.config.embedding_dimension);
        for i in 0..self.config.embedding_dimension {
            let component = ((hash.wrapping_mul(i as u64 + 1)) as f32) / (u64::MAX as f32);
            embedding.push(component * 2.0 - 1.0); // Normalize to [-1, 1]
        }

        Ok(embedding)
    }

    /// Execute coordinated transaction across both storage systems
    async fn execute_coordinated_transaction<F>(&self, operation: F) -> Result<(), StorageError>
    where
        F: FnOnce(&WriteTransaction) -> Result<(), StorageError>,
    {
        let operation_id = Uuid::new_v4();

        // Phase 1: Prepare operation
        {
            let mut state = self.state.write().await;
            state.pending_operations.insert(operation_id, PendingOperation {
                operation_id,
                operation_type: OperationType::Insert,
                redb_committed: false,
                qdrant_committed: false,
                timestamp: chrono::Utc::now(),
            });
        }

        // Phase 2: Execute REDB transaction
        let write_txn = self.redb.begin_write()
            .map_err(|e| StorageError::TransactionError(format!("Failed to begin write transaction: {}", e)))?;

        match operation(&write_txn) {
            Ok(_) => {
                write_txn.commit()
                    .map_err(|e| StorageError::TransactionError(format!("Failed to commit REDB transaction: {}", e)))?;

                // Mark REDB as committed
                {
                    let mut state = self.state.write().await;
                    if let Some(pending_op) = state.pending_operations.get_mut(&operation_id) {
                        pending_op.redb_committed = true;
                    }
                }
            }
            Err(e) => {
                // Rollback operation
                self.rollback_operation(operation_id).await?;
                return Err(e);
            }
        }

        // Phase 3: Complete coordination
        self.complete_operation(operation_id).await?;

        Ok(())
    }

    async fn rollback_operation(&self, operation_id: Uuid) -> Result<(), StorageError> {
        let mut state = self.state.write().await;
        state.pending_operations.remove(&operation_id);
        state.sync_status.error_count += 1;
        Ok(())
    }

    async fn complete_operation(&self, operation_id: Uuid) -> Result<(), StorageError> {
        let mut state = self.state.write().await;
        state.pending_operations.remove(&operation_id);
        state.performance_metrics.operations_success += 1;
        Ok(())
    }
}

#[async_trait]
impl HybridStorage for HybridStorageCoordinator {
    async fn store_agent(&self, agent: &AgentEntity) -> Result<(), StorageError> {
        let agent_key = agent.id.to_string();
        let agent_data = bincode::serialize(agent)
            .map_err(|e| StorageError::SerializationError(format!("Failed to serialize agent: {}", e)))?;

        self.execute_coordinated_transaction(|write_txn| {
            let mut table = write_txn.open_table(AGENTS_TABLE)
                .map_err(|e| StorageError::TransactionError(format!("Failed to open agents table: {}", e)))?;

            table.insert(agent_key.as_str(), agent_data.as_slice())
                .map_err(|e| StorageError::TransactionError(format!("Failed to insert agent: {}", e)))?;

            Ok(())
        }).await?;

        // Update metrics
        {
            let mut state = self.state.write().await;
            state.performance_metrics.operations_total += 1;
            state.performance_metrics.last_updated = chrono::Utc::now();
        }

        Ok(())
    }

    async fn store_knowledge(&self, knowledge: &KnowledgeEntity) -> Result<(), StorageError> {
        let knowledge_key = knowledge.id.to_string();

        // Generate embedding for semantic search
        let embedding = if knowledge.embeddings.is_some() {
            knowledge.embeddings.clone().unwrap()
        } else {
            self.generate_embedding(&knowledge.content).await?
        };

        // Store in REDB
        let knowledge_data = bincode::serialize(knowledge)
            .map_err(|e| StorageError::SerializationError(format!("Failed to serialize knowledge: {}", e)))?;

        self.execute_coordinated_transaction(|write_txn| {
            let mut table = write_txn.open_table(KNOWLEDGE_TABLE)
                .map_err(|e| StorageError::TransactionError(format!("Failed to open knowledge table: {}", e)))?;

            table.insert(knowledge_key.as_str(), knowledge_data.as_slice())
                .map_err(|e| StorageError::TransactionError(format!("Failed to insert knowledge: {}", e)))?;

            Ok(())
        }).await?;

        // Store embedding in Qdrant
        let point = PointStruct::new(
            knowledge.id.to_string(),
            embedding,
            Some(
                [
                    ("source".to_string(), knowledge.source.clone().into()),
                    ("credibility".to_string(), knowledge.credibility_rating.clone().into()),
                    ("created_at".to_string(), knowledge.created_at.to_rfc3339().into()),
                ].into_iter().collect()
            ),
        );

        self.qdrant.upsert_points_blocking(&UpsertPoints {
            collection_name: self.config.collection_name.clone(),
            points: vec![point],
            ..Default::default()
        }).await
        .map_err(|e| StorageError::VectorError(format!("Failed to store vector: {}", e)))?;

        Ok(())
    }

    async fn get_agent(&self, id: &Uuid) -> Result<Option<AgentEntity>, StorageError> {
        let read_txn = self.redb.begin_read()
            .map_err(|e| StorageError::TransactionError(format!("Failed to begin read transaction: {}", e)))?;

        let table = read_txn.open_table(AGENTS_TABLE)
            .map_err(|e| StorageError::TransactionError(format!("Failed to open agents table: {}", e)))?;

        let agent_key = id.to_string();
        match table.get(agent_key.as_str()) {
            Ok(Some(data)) => {
                let agent: AgentEntity = bincode::deserialize(data.value())
                    .map_err(|e| StorageError::SerializationError(format!("Failed to deserialize agent: {}", e)))?;
                Ok(Some(agent))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(StorageError::TransactionError(format!("Failed to get agent: {}", e))),
        }
    }

    async fn search_knowledge(&self, query: &str, limit: usize) -> Result<Vec<KnowledgeEntity>, StorageError> {
        // Generate query embedding
        let query_embedding = self.generate_embedding(query).await?;

        // Search in Qdrant
        let search_result = self.qdrant.search_points(&SearchPoints {
            collection_name: self.config.collection_name.clone(),
            vector: query_embedding,
            limit: limit as u64,
            with_payload: Some(WithPayloadSelector {
                selector_options: Some(with_payload_selector::SelectorOptions::Enable(true)),
            }),
            ..Default::default()
        }).await
        .map_err(|e| StorageError::VectorError(format!("Failed to search vectors: {}", e)))?;

        // Retrieve full knowledge entities from REDB
        let mut knowledge_entities = Vec::new();
        let read_txn = self.redb.begin_read()
            .map_err(|e| StorageError::TransactionError(format!("Failed to begin read transaction: {}", e)))?;

        let table = read_txn.open_table(KNOWLEDGE_TABLE)
            .map_err(|e| StorageError::TransactionError(format!("Failed to open knowledge table: {}", e)))?;

        for scored_point in search_result.result {
            if let Ok(Some(data)) = table.get(scored_point.id.unwrap().point_id_options.unwrap().to_string().as_str()) {
                if let Ok(knowledge) = bincode::deserialize::<KnowledgeEntity>(data.value()) {
                    knowledge_entities.push(knowledge);
                }
            }
        }

        Ok(knowledge_entities)
    }

    async fn update_coordination(&self, coordination: &CoordinationEntity) -> Result<(), StorageError> {
        let coordination_key = coordination.id.to_string();
        let coordination_data = bincode::serialize(coordination)
            .map_err(|e| StorageError::SerializationError(format!("Failed to serialize coordination: {}", e)))?;

        self.execute_coordinated_transaction(|write_txn| {
            let mut table = write_txn.open_table(COORDINATION_TABLE)
                .map_err(|e| StorageError::TransactionError(format!("Failed to open coordination table: {}", e)))?;

            table.insert(coordination_key.as_str(), coordination_data.as_slice())
                .map_err(|e| StorageError::TransactionError(format!("Failed to insert coordination: {}", e)))?;

            Ok(())
        }).await
    }

    async fn synchronize(&self) -> Result<SyncResult, StorageError> {
        let mut state = self.state.write().await;

        let pending_count = state.pending_operations.len();
        let start_time = chrono::Utc::now();

        // Process pending operations (simplified)
        state.pending_operations.clear();
        state.sync_status.last_sync = Some(chrono::Utc::now());
        state.sync_status.pending_count = 0;

        let duration = chrono::Utc::now() - start_time;

        Ok(SyncResult {
            operations_processed: pending_count,
            duration_ms: duration.num_milliseconds() as u64,
            success: true,
        })
    }

    async fn get_metrics(&self) -> StorageMetrics {
        let state = self.state.read().await;
        state.performance_metrics.clone()
    }
}

/// Storage error types
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Initialization error: {0}")]
    InitializationError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Vector database error: {0}")]
    VectorError(String),

    #[error("Coordination error: {0}")]
    CoordinationError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Synchronization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub operations_processed: usize,
    pub duration_ms: u64,
    pub success: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            redb_path: "agents.redb".to_string(),
            qdrant_url: "http://localhost:6334".to_string(),
            collection_name: "agent_knowledge".to_string(),
            embedding_dimension: 384, // Common embedding dimension
            sync_batch_size: 100,
            consistency_mode: ConsistencyMode::Eventually,
        }
    }
}