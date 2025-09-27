//! Systematic Researcher Example
//!
//! This example demonstrates how agent.md behavioral descriptions translate
//! to algorithmic agent.rs implementations using the ACS framework.
//!
//! # Behavioral Translation Example
//!
//! ## Original agent.md description:
//! ```markdown
//! # Agent Behavioral Specifications
//!
//! ### Systematic Research Behavior
//! - Be systematic and evidence-based in research methodology
//! - Apply Enhanced PRISMA validation with B3+ source credibility
//! - Cross-validate findings across multiple independent sources
//! - Generate synthesis with confidence scoring and evidence hierarchy
//! ```
//!
//! ## Algorithmic agent.rs implementation:
//! - `SystematicResearcher` trait with concrete algorithmic methods
//! - `execute_systematic_search()` with rule-based search strategies
//! - `validate_evidence_quality()` with Admiralty Code algorithms
//! - `apply_prisma_methodology()` with automated compliance checking
//! - `cross_validate_findings()` with consistency scoring algorithms

use acs_example::{
    ACSFramework, ACSFrameworkOperations, ACSTask, ACSKnowledge,
    TaskType, TaskPriority, create_research_framework,
};
use std::collections::HashMap;
use tokio;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting ACS Framework Systematic Researcher Example");

    // Initialize the ACS framework with research configuration
    let framework = create_research_framework().await?;
    info!("ACS Framework initialized successfully");

    // Demonstrate storing knowledge for semantic understanding
    info!("Storing sample knowledge for semantic routing...");

    let sample_knowledge = vec![
        ACSKnowledge {
            title: "Rust Performance Characteristics".to_string(),
            content: "Rust provides zero-cost abstractions and memory safety without garbage collection, making it ideal for high-performance systems programming.".to_string(),
            source: "rust-lang.org".to_string(),
            credibility_rating: "A1".to_string(),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("domain".to_string(), "programming".into());
                metadata.insert("language".to_string(), "rust".into());
                metadata
            },
            tags: vec!["rust".to_string(), "performance".to_string(), "systems".to_string()],
        },
        ACSKnowledge {
            title: "Vector Database Integration Patterns".to_string(),
            content: "Qdrant provides native Rust integration with excellent performance characteristics for semantic search and embedding storage.".to_string(),
            source: "qdrant.tech".to_string(),
            credibility_rating: "B2".to_string(),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("domain".to_string(), "databases".into());
                metadata.insert("type".to_string(), "vector".into());
                metadata
            },
            tags: vec!["qdrant".to_string(), "vector".to_string(), "semantic".to_string()],
        },
        ACSKnowledge {
            title: "Hybrid AI Architecture Benefits".to_string(),
            content: "Hybrid symbolic-neural architectures combine the best of deterministic reasoning with semantic understanding capabilities.".to_string(),
            source: "research-paper.pdf".to_string(),
            credibility_rating: "A2".to_string(),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("domain".to_string(), "ai-architecture".into());
                metadata.insert("type".to_string(), "research".into());
                metadata
            },
            tags: vec!["ai".to_string(), "hybrid".to_string(), "architecture".to_string()],
        },
    ];

    for knowledge in sample_knowledge {
        framework.store_knowledge(knowledge).await?;
    }

    info!("Sample knowledge stored successfully");

    // Example 1: Research Task with Algorithmic Systematic Behavior
    info!("\n=== Example 1: Systematic Research Task ===");

    let research_task = ACSTask {
        description: "Research the performance characteristics of Rust for AI applications".to_string(),
        task_type: TaskType::Research {
            query: "Rust AI performance characteristics".to_string(),
            domain: Some("programming".to_string()),
            sources_required: 3,
        },
        priority: TaskPriority::High,
        parameters: {
            let mut params = HashMap::new();
            params.insert("query".to_string(), "Rust AI performance characteristics".into());
            params.insert("domain".to_string(), "programming".into());
            params.insert("max_sources".to_string(), 10.into());
            params
        },
        timeout_ms: Some(30000),
    };

    match framework.execute_task(research_task).await {
        Ok(result) => {
            info!("Research task completed successfully!");
            info!("Task ID: {}", result.task_id);
            info!("Agent: {} ({})", result.agent_info.agent_id, result.agent_info.agent_type);
            info!("Status: {:?}", result.status);
            info!("Confidence: {:.2}", result.confidence);
            info!("Execution time: {}ms", result.execution_time_ms);
            info!("Evidence sources: {}", result.evidence.len());

            // Display evidence with credibility ratings (algorithmic validation)
            for (i, evidence) in result.evidence.iter().enumerate() {
                info!("  Evidence {}: {} (Credibility: {}, Status: {})",
                    i + 1, evidence.source, evidence.credibility, evidence.validation_status);
            }

            info!("Results: {}", serde_json::to_string_pretty(&result.content)?);
        }
        Err(e) => {
            error!("Research task failed: {}", e);
        }
    }

    // Example 2: Semantic Knowledge Search
    info!("\n=== Example 2: Semantic Knowledge Search ===");

    let search_results = framework.search_knowledge("vector database performance", 3).await?;

    info!("Found {} semantically similar knowledge items:", search_results.len());
    for (i, knowledge) in search_results.iter().enumerate() {
        info!("  {}. {} (Source: {}, Credibility: {})",
            i + 1, knowledge.title, knowledge.source, knowledge.credibility_rating);
        info!("     Content: {}",
            if knowledge.content.len() > 100 {
                format!("{}...", &knowledge.content[..100])
            } else {
                knowledge.content.clone()
            }
        );
    }

    // Example 3: Analysis Task with Evidence Validation
    info!("\n=== Example 3: Analysis Task with Algorithmic Validation ===");

    let analysis_task = ACSTask {
        description: "Analyze the feasibility of hybrid AI architectures".to_string(),
        task_type: TaskType::Analysis {
            content: "Hybrid symbolic-neural AI architectures combining Rust algorithmic intelligence with vector databases for semantic understanding".to_string(),
            analysis_type: "feasibility".to_string(),
        },
        priority: TaskPriority::Medium,
        parameters: {
            let mut params = HashMap::new();
            params.insert("analysis_type".to_string(), "feasibility".into());
            params.insert("evidence_threshold".to_string(), "B3".into());
            params.insert("validation_tier".to_string(), "essential".into());
            params
        },
        timeout_ms: Some(25000),
    };

    match framework.execute_task(analysis_task).await {
        Ok(result) => {
            info!("Analysis task completed!");
            info!("Status: {:?}, Confidence: {:.2}", result.status, result.confidence);
            info!("Algorithmic validation applied to {} evidence sources", result.evidence.len());

            // Show algorithmic validation results
            for evidence in &result.evidence {
                info!("  Source: {} -> Validation: {} (Credibility: {})",
                    evidence.source, evidence.validation_status, evidence.credibility);
            }
        }
        Err(e) => {
            error!("Analysis task failed: {}", e);
        }
    }

    // Example 4: Framework Status and Metrics
    info!("\n=== Example 4: Framework Status ===");

    let status = framework.get_status().await;
    info!("Framework Status:");
    info!("  Version: {}", status.framework_version);
    info!("  Registered Agents: {}", status.registered_agents);
    info!("  Active Tasks: {}", status.active_tasks);
    info!("  Knowledge Base Size: {}", status.knowledge_base_size);
    info!("  Storage Status: {}", status.storage_status);
    info!("  Coordination Status: {}", status.coordination_status);

    info!("Performance Metrics:");
    info!("  Tasks Completed: {}", status.performance_metrics.tasks_completed);
    info!("  Average Execution Time: {:.2}ms", status.performance_metrics.average_execution_time_ms);
    info!("  Success Rate: {:.2}%", status.performance_metrics.success_rate * 100.0);
    info!("  Agent Utilization: {:.2}%", status.performance_metrics.agent_utilization * 100.0);

    // Example 5: Demonstrating Key Behavioral Translation Concepts
    info!("\n=== Example 5: Behavioral Translation Demonstration ===");

    info!("Key Behavioral Translations Demonstrated:");
    info!("1. agent.md: 'Be systematic and evidence-based'");
    info!("   -> agent.rs: SystematicResearcher::execute_systematic_search()");
    info!("   -> Algorithmic search strategies with rule-based source selection");

    info!("2. agent.md: 'Apply Enhanced PRISMA validation'");
    info!("   -> agent.rs: SystematicResearcher::apply_prisma_methodology()");
    info!("   -> Automated compliance checking with structured validation phases");

    info!("3. agent.md: 'Cross-validate findings across sources'");
    info!("   -> agent.rs: SystematicResearcher::cross_validate_findings()");
    info!("   -> Consistency scoring algorithms with conflict detection");

    info!("4. agent.md: 'Use Admiralty Code for source credibility'");
    info!("   -> agent.rs: CredibilityRating::meets_threshold()");
    info!("   -> Algorithmic credibility assessment with numeric scoring");

    info!("5. Hybrid Storage: REDB (structured state) + Qdrant (semantic understanding)");
    info!("   -> Coordinated transactions with async-sync bridge patterns");

    info!("6. Local-First: Algorithmic intelligence runs locally");
    info!("   -> Optional cloud delegation for complex linguistic tasks only");

    // Graceful shutdown
    info!("\n=== Shutting Down Framework ===");
    framework.shutdown().await?;
    info!("ACS Framework shutdown completed successfully");

    info!("\n🎯 ACS Framework Example Completed Successfully! 🎯");
    info!("This demonstrates how agent.md behavioral descriptions");
    info!("translate to algorithmic agent.rs implementations with:");
    info!("  ✅ Deterministic, auditable behavior");
    info!("  ✅ Hybrid storage (REDB + Qdrant)");
    info!("  ✅ Semantic understanding and routing");
    info!("  ✅ Local-first with optional cloud augmentation");
    info!("  ✅ Performance optimization and resource efficiency");

    Ok(())
}

/// Additional helper function to demonstrate behavioral component composition
#[allow(dead_code)]
async fn demonstrate_behavioral_composition() -> Result<(), Box<dyn std::error::Error>> {
    use acs_example::{
        SystematicResearchAgent, ValidationConfig, CredibilityRating,
        behavioral::systematic_research::*,
    };

    info!("=== Behavioral Component Composition Demo ===");

    // Create a systematic research agent with specific behavioral configuration
    let validation_config = ValidationConfig {
        minimum_sources: 5,
        required_credibility: CredibilityRating::B2, // Higher threshold
        enable_bias_detection: true,
        cross_validation_threshold: 0.80, // Stricter cross-validation
    };

    let research_agent = SystematicResearchAgent::new(validation_config);

    // Demonstrate the algorithmic behavioral implementation
    let search_query = SearchQuery {
        text: "hybrid AI architecture performance".to_string(),
        domain: "artificial-intelligence".to_string(),
        scope: "comprehensive".to_string(),
        max_sources: 10,
    };

    info!("Executing systematic search with algorithmic methodology...");

    // This would normally execute the full search, but for demo we'll show the pattern
    match research_agent.execute_systematic_search(&search_query).await {
        Ok(results) => {
            info!("Systematic search completed with {} findings", results.findings.len());
            info!("Quality metrics: {} high-quality sources out of {} total",
                results.quality_metrics.high_quality_sources,
                results.quality_metrics.total_sources);
            info!("Average credibility: {:.2}", results.quality_metrics.average_credibility);
            info!("Bias score: {:.3}", results.quality_metrics.bias_score);
        }
        Err(e) => {
            error!("Systematic search failed: {}", e);
        }
    }

    info!("Behavioral composition demonstration completed");
    Ok(())
}