---
allowed-tools: [Read, Write, Edit, MultiEdit, Bash, Glob, TodoWrite, mcp__sequential-thinking__sequentialthinking, mcp__context7__resolve-library-id, mcp__context7__get-library-docs, WebSearch, WebFetch]
description: "Intelligent research planning and scope clarification system for optimized deep-research preparation"
---

# /research-plan - Systematic Research Planning & Scope Clarification Command

## Purpose
Generate comprehensive research planning documents that break down complex research queries into systematic [TOPIC-###] frameworks, providing detailed scope clarification and research proposal development to optimize subsequent `/deep-research` execution. Integrates with CCC vault architecture and follows Enhanced PRISMA systematic planning protocols.

## BEHAVIORAL EXECUTION FLOW

**CRITICAL**: This command defines how Claude executes research planning when the user runs `/research-plan [query]` on Claude.

### **Claude's Direct Responsibilities:**
1. **[PHASE-001]**: Claude analyzes user query and identifies research domains
2. **[PHASE-002]**: Claude generates [TOPIC-###] breakdown with priority classification
3. **[PHASE-003]**: Claude performs gap analysis and scope clarification
4. **[PHASE-004]**: Claude creates structured research proposal document

### **Planning Sequence:**
```
User: /research-plan [research query] [args]
  ↓
Claude: Query analysis and domain identification ([PHASE-001])
  ↓
Claude: [TOPIC-###] generation with priority matrix ([PHASE-002])
  ↓
Claude: Gap analysis and scope validation ([PHASE-003])
  ↓
Claude: Research proposal finalization ([PHASE-004])
```

## Usage
```
/research-plan [Research Query] [--priority=high|medium|comprehensive] [--depth=overview|detailed|exhaustive] [--focus=academic|technical|practical] [--integration=standalone|deep-research]
```

## Arguments
- `Research Query` - Complex research question or topic requiring systematic breakdown and planning
- `--priority` - Planning depth level (default: detailed) determining [TOPIC-###] granularity and analysis depth
- `--depth` - Analysis comprehensiveness (default: detailed) controlling investigation scope per topic
- `--focus` - Research orientation (default: technical) influencing topic categorization and methodology
- `--integration` - Integration strategy (default: deep-research) for subsequent workflow coordination
  - `standalone`: Independent research planning document
  - `deep-research`: Optimized for `/deep-research` command integration with compatible formatting

## CCC Vault Integration

### **Research Domain Structure**
This command creates planning documents within the CCC vault's Research domain structure:
- **Planning Location**: `/Research/Active-Projects/Deep-Research/`
- **Integration Path**: Creates comprehensive research-proposal.md ready for `/deep-research` execution
- **Template Compliance**: Uses [[Templates/Documents/Research-Report-Template]] for structured output
- **Framework Alignment**: Full CCC framework compliance with systematic validation

### **Framework Compliance**
- **Enhanced PRISMA**: Planning-phase validation protocols from [[CCC/Standards/Enhanced-PRISMA]]
- **AI Workflow**: Implements [[CCC/AI-Workflows/AI-Standards]] for systematic planning
- **Security**: Applies [[CCC/Security/]] classification for sensitive research topics
- **Architecture**: Aligned with [[CCC/Architecture]] systematic planning principles

## Research Planning Architecture Framework

### **[PHASE-001]: Query Analysis & Domain Identification [FOUNDATION]**
**Duration**: 3-5 minutes | **Validation Tier**: Essential

**Objectives**:
1. **Query Decomposition**: Break down complex research question into core components
2. **Domain Classification**: Identify primary and secondary research domains (Technical, Academic, Practical, etc.)
3. **Scope Boundary Definition**: Establish clear research boundaries and limitations
4. **Context Assessment**: Analyze hardware, software, and environmental constraints
5. **Integration Requirements**: Determine compatibility with existing research and systems

**[PHASE-001] Protocol**:
```
📋 Query Analysis Checklist:
[ ] **MANDATORY**: Current datetime obtained and recorded in 'YYYY-MM-DD HH:MM:SS CST' format
[ ] Research question core components identified and documented
[ ] Primary research domain classified (Technical, Academic, Literature, Survival, etc.)
[ ] Secondary/supporting domains identified with relevance assessment
[ ] Scope boundaries clearly defined with inclusion/exclusion criteria
[ ] Hardware/software context constraints documented (RTX 4070, 20-core CPU, 32GB RAM, etc.)
[ ] Integration requirements with existing CCC research assessed
[ ] Research complexity level determined (Basic, Intermediate, Advanced, Expert)
[ ] Expected research timeline and resource requirements estimated
```

**Query Analysis Framework**:
```markdown
# Research Query Analysis: [Topic]
*Created: [YYYY-MM-DD HH:MM:SS CST] - MANDATORY: Insert actual current datetime*

## Query Decomposition
**Primary Research Question**: [Core research objective]
**Sub-Questions**: [Supporting questions that address different aspects]

## Domain Classification
**Primary Domain**: [Technical/Academic/Literature/Survival/etc.]
**Supporting Domains**: [Related areas requiring investigation]
**Complexity Level**: [Basic/Intermediate/Advanced/Expert]

## Scope Definition
**Included**: [What this research will cover]
**Excluded**: [What is outside scope with rationale]
**Constraints**: [Hardware, software, time, resource limitations]

## Context Assessment
**Hardware Context**: [Specific hardware requirements/optimizations]
**Software Context**: [Required tools, platforms, frameworks]
**Integration Context**: [Relationship to existing research/systems]
```

### **[PHASE-002]: [TOPIC-###] Generation & Priority Classification [SYSTEMATIC]**
**Duration**: 10-15 minutes | **Validation Tier**: Extended

**Topic Generation Framework**:
1. **High-Priority Topics ([TOPIC-001] to [TOPIC-006])**: Core research areas critical to primary objective
2. **Medium-Priority Topics ([TOPIC-007] to [TOPIC-012])**: Supporting research areas enhancing understanding
3. **Low-Priority Topics ([TOPIC-013] to [TOPIC-018])**: Extended research areas for comprehensive coverage
4. **Future Research Topics ([TOPIC-019]+)**: Identified gaps for subsequent research phases

**[TOPIC-###] Structure Template**:
```markdown
### **[TOPIC-001]: [Descriptive Title]**
**Research Question**: [Specific, measurable research question]

**Specific Investigation Targets**:
- [Detailed investigation point 1]
- [Detailed investigation point 2]
- [Detailed investigation point 3]
- [Performance/compatibility/integration considerations]
- [Alternative approaches and fallback strategies]

**Expected Outcomes**: [Specific deliverables and measurable results]

**Priority Rationale**: [Why this topic receives its priority classification]

**Integration Dependencies**: [How this topic relates to other [TOPIC-###] items]

**Resource Requirements**: [Estimated research time, tools, expertise needed]

**Validation Criteria**: [How success will be measured]
```

**Priority Classification Matrix**:
```
High Priority ([TOPIC-001] to [TOPIC-006]):
- Directly addresses primary research objective
- Critical for subsequent decision making
- High impact on overall research success
- Required for foundational understanding

Medium Priority ([TOPIC-007] to [TOPIC-012]):
- Enhances understanding of primary objective
- Provides supporting evidence and context
- Moderate impact on research comprehensiveness
- Valuable for optimization and improvement

Low Priority ([TOPIC-013] to [TOPIC-018]):
- Extends research coverage beyond core requirements
- Provides comprehensive understanding
- Lower immediate impact but valuable for completeness
- Future research foundation
```

### **[PHASE-003]: Gap Analysis & Scope Validation [CRITICAL]**
**Duration**: 5-10 minutes | **Systematic Validation**

**Gap Analysis Protocol**:
1. **Coverage Assessment**: Verify [TOPIC-###] items comprehensively address research question
2. **Dependency Validation**: Ensure proper relationships between topics
3. **Resource Feasibility**: Validate research scope against available resources
4. **Integration Compatibility**: Confirm compatibility with existing research and CCC framework
5. **Risk Assessment**: Identify potential research obstacles and mitigation strategies

**Scope Validation Framework**:
```markdown
## Gap Analysis & Validation

### Coverage Assessment
**Research Question Coverage**: [Percentage of original question addressed by [TOPIC-###] items]
**Identified Gaps**: [Areas not covered by current topic breakdown]
**Gap Mitigation**: [How gaps will be addressed - additional topics, scope adjustment, etc.]

### Dependency Analysis
**Critical Dependencies**: [Which [TOPIC-###] items depend on others]
**Research Sequence**: [Recommended order for topic investigation]
**Parallel Opportunities**: [Which topics can be researched simultaneously]

### Resource Validation
**Time Requirements**: [Total estimated research time across all topics]
**Expertise Requirements**: [Required knowledge areas and potential expert consultation]
**Tool Requirements**: [Software, hardware, access requirements]
**Feasibility Assessment**: [Overall project feasibility with recommendations]

### Integration Assessment
**CCC Framework Alignment**: [How research aligns with existing CCC content]
**Deep-Research Compatibility**: [Preparation for subsequent /deep-research execution]
**Template Compatibility**: [Template assignments for each priority tier]
```

### **[PHASE-004]: Research Proposal Finalization [DELIVERY]**
**Duration**: 5-10 minutes | **Quality Assurance**

**Final Documentation Protocol**:
1. **Comprehensive Research Proposal**: Single complete document with all [TOPIC-###] items organized by priority
2. **Methodology Integration**: CCC framework compliance verification embedded within proposal
3. **Quality Standards Application**: Enhanced PRISMA planning validation integrated throughout
4. **Deep-Research Optimization**: Formatting optimized for direct `/deep-research` execution
5. **Integrated Summary**: Key recommendations and executive overview included within main document

## Research Planning Quality Framework

### **Enhanced PRISMA Planning Integration**
```
📋 Essential Planning Validation (10-Item Core):
[ ] 01: Research objectives clearly articulated with measurable outcomes
[ ] 02: Systematic methodology documented for topic identification
[ ] 03: Scope boundaries explicitly defined with inclusion/exclusion criteria
[ ] 04: [TOPIC-###] classification completed with priority rationale
[ ] 05: Resource requirements assessed with feasibility validation
[ ] 06: Integration procedures documented for deep-research compatibility
[ ] 07: Gap analysis completed with coverage assessment
[ ] 08: Dependency relationships mapped between topics
[ ] 09: Quality criteria established for topic validation
[ ] 10: Documentation validation with CCC framework compliance
```

### **Planning Quality Standards**
- **Topic Granularity**: Each [TOPIC-###] addresses specific, measurable research question
- **Priority Accuracy**: Classification based on impact, urgency, and dependency analysis
- **Scope Clarity**: Clear boundaries prevent research drift and scope creep
- **Integration Readiness**: Formatted for seamless `/deep-research` consumption
- **Resource Realism**: Feasible within available time, tools, and expertise constraints

## Output Structure & Integration

### **Research Planning Document Structure**
```
/Research/Active-Projects/Deep-Research/[stub]/
└── research-proposal.md           # Comprehensive planning document ([PHASE-004] output)
                                   # Ready for direct /deep-research execution
```

### **Deep-Research Integration Format**
The comprehensive `research-proposal.md` serves as the complete planning document and input for `/deep-research` command execution.

**Comprehensive Research Proposal Structure (research-proposal.md)**:
```markdown
# Research Proposal: [Topic]
*Created: [YYYY-MM-DD HH:MM:SS CST] - MANDATORY: Insert actual current datetime*

## Executive Summary
[Brief overview of research objectives, approach, and expected outcomes]

## Research Objectives
[Clear, measurable research goals with success criteria]

## Query Analysis
**Primary Research Question**: [Core research objective]
**Sub-Questions**: [Supporting questions that address different aspects]
**Domain Classification**: [Technical/Academic/Literature/Survival/etc.]
**Complexity Level**: [Basic/Intermediate/Advanced/Expert]

## Scope Definition
**Included**: [What this research will cover]
**Excluded**: [What is outside scope with rationale]
**Constraints**: [Hardware, software, time, resource limitations]

## [TOPIC-###] Investigation Framework

### High-Priority Research Areas ([TOPIC-001] to [TOPIC-006])

#### **[TOPIC-001]: [Descriptive Title]**
**Research Question**: [Specific, measurable research question]

**Specific Investigation Targets**:
- [Detailed investigation point 1]
- [Detailed investigation point 2]
- [Detailed investigation point 3]
- [Performance/compatibility/integration considerations]
- [Alternative approaches and fallback strategies]

**Expected Outcomes**: [Specific deliverables and measurable results]
**Priority Rationale**: [Why this topic receives its priority classification]
**Integration Dependencies**: [How this topic relates to other [TOPIC-###] items]
**Resource Requirements**: [Estimated research time, tools, expertise needed]
**Validation Criteria**: [How success will be measured]

[Continue with TOPIC-002 through TOPIC-006 following same structure]

### Medium-Priority Research Areas ([TOPIC-007] to [TOPIC-012])
[Continue with TOPIC-007 through TOPIC-012 following same structure]

### Low-Priority Research Areas ([TOPIC-013] to [TOPIC-018])
[Continue with TOPIC-013 through TOPIC-018 following same structure]

## Gap Analysis & Scope Validation

### Coverage Assessment
**Research Question Coverage**: [Percentage of original question addressed by [TOPIC-###] items]
**Identified Gaps**: [Areas not covered by current topic breakdown]
**Gap Mitigation**: [How gaps will be addressed - additional topics, scope adjustment, etc.]

### Dependency Analysis
**Critical Dependencies**: [Which [TOPIC-###] items depend on others]
**Research Sequence**: [Recommended order for topic investigation]
**Parallel Opportunities**: [Which topics can be researched simultaneously]

### Resource Validation
**Time Requirements**: [Total estimated research time across all topics]
**Expertise Requirements**: [Required knowledge areas and potential expert consultation]
**Tool Requirements**: [Software, hardware, access requirements]
**Feasibility Assessment**: [Overall project feasibility with recommendations]

### Integration Assessment
**CCC Framework Alignment**: [How research aligns with existing CCC content]
**Deep-Research Compatibility**: [Preparation for /deep-research execution]
**Quality Standards**: [Validation tier assignments for each priority level]

## Implementation Strategy

### Research Methodology
[Systematic approach for conducting the research]

### Quality Assurance Framework
**Minimum Source Rating**: B3 Admiralty Code
**Validation Tier**: [Essential|Extended|Comprehensive]
**Cross-validation Requirements**: [Multi-source verification criteria]

### Risk Assessment
[Potential obstacles and mitigation strategies]

## Deep-Research Execution Readiness

### Search Strategy Framework
[How topics will be converted to [SEARCH-###] tasks for /deep-research]

### Validation Requirements
[Quality standards and validation tiers for research execution]

### Success Metrics
[Measurable criteria for research completion]
```

## Research Planning Patterns

### **Academic Research Focus**
```
/research-plan "systematic review of [topic]" --focus academic --depth exhaustive --priority comprehensive
```
- Enhanced focus on peer-reviewed sources and academic methodologies
- Comprehensive literature review preparation
- Systematic validation protocols

### **Technical Implementation Research**
```
/research-plan "optimal implementation of [technology]" --focus technical --depth detailed --integration deep-research
```
- Technical specification and implementation focus
- Performance optimization and compatibility analysis
- Practical validation requirements

### **Practical Application Research**
```
/research-plan "best practices for [domain]" --focus practical --depth overview --priority high
```
- Real-world application emphasis
- User experience and workflow optimization
- Implementation complexity assessment

## Integration with Deep-Research Command

### **Seamless Workflow Integration**
1. **Research Planning**: `/research-plan [query]` generates comprehensive research-proposal.md in Deep-Research directory
2. **Review & Refinement**: Human validation and scope adjustment of research-proposal.md
3. **Deep Research Execution**: `/deep-research` directly uses research-proposal.md for optimized execution
4. **Iterative Improvement**: Results feed back into planning for future research

### **Optimization Benefits**
- **Reduced Planning Overhead**: Pre-structured research approach saves 40-60% planning time
- **Enhanced Focus**: Clear [TOPIC-###] breakdown prevents research drift
- **Better Resource Allocation**: Realistic scope and priority assessment
- **Improved Quality**: Systematic validation from planning through execution
- **Stakeholder Clarity**: Clear research proposal for approval and resource allocation

## Implementation Success Criteria

### **Planning Completion Requirements**
```
📋 Research Planning Validation:
[ ] **MANDATORY**: research-proposal.md includes actual current datetime in 'YYYY-MM-DD HH:MM:SS CST' format
[ ] Complete [TOPIC-###] breakdown with priority classification
[ ] Gap analysis completed with coverage validation
[ ] Resource requirements assessed with feasibility confirmation
[ ] Direct deep-research integration preparation completed
[ ] CCC framework compliance verified and integrated
[ ] Enhanced PRISMA planning validation complete
[ ] Scope boundaries clearly defined with validation criteria
[ ] Quality standards established for subsequent research execution
[ ] Single comprehensive document ready for /deep-research execution
```

### **Quality Assurance Metrics**
- **Topic Coverage**: 100% of research question addressed by [TOPIC-###] items
- **Priority Accuracy**: Logical priority classification with clear rationale
- **Resource Feasibility**: Realistic scope within available constraints
- **Integration Readiness**: Formatted for immediate `/deep-research` consumption
- **Framework Compliance**: Full adherence to CCC systematic planning standards

## Advanced Planning Patterns

### **Multi-Domain Research Planning**
```
/research-plan "integration of [technology A] with [technology B] for [use case]" --depth exhaustive --focus technical
```
- Cross-domain integration analysis
- Compatibility assessment and risk evaluation
- Multi-stakeholder requirement coordination

### **Comparative Analysis Planning**
```
/research-plan "comparison of [approach A] vs [approach B] for [objective]" --priority comprehensive --integration deep-research
```
- Systematic comparison framework development
- Evaluation criteria establishment
- Bias mitigation strategy development

### **Implementation Strategy Planning**
```
/research-plan "deployment strategy for [system] in [environment]" --focus practical --depth detailed
```
- Implementation roadmap development
- Risk assessment and mitigation planning
- Resource and timeline estimation

## Framework Compliance

**CCC Integration**: Full compatibility with Context Command Center framework
**Enhanced PRISMA**: Systematic planning validation with evidence-based methodology
**ISO 31000**: Risk management integration for research planning and scope assessment
**CIS Controls**: Security considerations for sensitive research planning
**Behavioral Specifications**: Complete adherence to CCC behavioral protocols

---

**Command Version**: 1.0.0 | **Framework**: CCC-Compatible
**Evidence Rating**: A1 (Framework standards with systematic validation)
**Compliance**: Enhanced PRISMA + ISO 31000 + CIS Controls + CCC Behavioral Specifications
**Vault Integration**: Compatible with CCC Research domain structure
**Deep-Research Integration**: Optimized for seamless workflow transition

*Systematic research planning excellence through evidence-based methodology and comprehensive scope clarification integrated with the Context Command Center framework.*