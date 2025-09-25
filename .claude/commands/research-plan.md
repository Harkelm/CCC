---
allowed-tools: [Read, Write, Edit, MultiEdit, Bash, Glob, TodoWrite, mcp__sequential-thinking__sequentialthinking, mcp__context7__resolve-library-id, mcp__context7__get-library-docs, WebSearch, WebFetch]
description: "Intelligent research planning and scope clarification system for optimized deep-research preparation"
---

# /research-plan - ACS-Driven Research Planning Command

## Purpose
Generate systematic [TOPIC-###] breakdowns with ACS component selection that optimize subsequent `/deep-research` execution. Creates structured research planning documents using Agent Component System integration for comprehensive research workflows.

## BEHAVIORAL EXECUTION FLOW

### **Claude Direct Execution [STREAMLINED]**
**Duration**: 10-15 minutes | **No Agent Delegation**

**Claude Direct Responsibilities:**
1. **ACS Component Selection**: Select appropriate behavioral, procedural, format, and validation components
2. **[TOPIC-###] Generation**: Break down query into prioritized research topics
3. **Research Planning**: Create structured planning document with [TOPIC-###] framework
4. **Deep-Research Integration**: Format output for seamless `/deep-research` consumption

**Execution Flow:**
```
User: /research-plan [research query]
  ↓
Claude: ACS component selection based on domain analysis
  ↓
Claude: [TOPIC-###] breakdown with priority classification
  ↓
Claude: Create research-planning.md ready for /deep-research
```

## Usage
```
/research-plan [Research Query]
```

**Simple Interface**: Claude automatically detects domain and selects appropriate ACS components based on query analysis.

### **Directory Structure**
```
/Research/Active-Projects/Deep-Research/[topic-stub]/
├── research-planning.md          [CLAUDE CREATES]
└── [Future deep-research files]  [DEEP-RESEARCH CREATES]
```

## ACS Component Selection Framework

### **Domain Analysis & Component Selection [CLAUDE DIRECT]**

**Claude Responsibilities:**
```
📋 ACS Component Selection Matrix:
[ ] **MANDATORY**: Current datetime recorded in 'YYYY-MM-DD HH:MM:SS CST' format
[ ] Research domain analysis (academic/technical/product/practical/creative)
[ ] ACS component selection based on research requirements:

Component Selection Framework:
├── Behavioral [SELECT ONE]:
│   ├── systematic-researcher (general research)
│   ├── systematic-product-researcher (product analysis)
│   └── comparative-researcher (comparison studies)
├── Procedural [SELECT ONE]:
│   ├── academic-research-strategy (academic focus)
│   ├── product-research-strategy (product focus)
│   └── technical-research-strategy (technical focus)
├── Format [SELECT ONE]:
│   ├── research-report-template (general research)
│   ├── product-analysis-template (product research)
│   └── technical-analysis-template (technical research)
└── Validation [SELECT ONE]:
    ├── enhanced-PRISMA (comprehensive validation)
    ├── CCC-standard (standard validation)
    └── essential (basic validation)
```

## [TOPIC-###] Generation Framework

### **Research Topic Breakdown [SYSTEMATIC]**

**[TOPIC-###] Structure** (feeds into deep-research [SEARCH-###] conversion):
```markdown
### **[TOPIC-001]: [Descriptive Title]**
**Research Question**: [Specific research question for investigation]

**Investigation Targets**:
- [Primary investigation focus]
- [Secondary investigation areas]
- [Integration/compatibility considerations]

**Expected Outcomes**: [What this research will deliver]
**Priority Level**: [High/Medium/Low]
**Search Integration**: [How this converts to SEARCH-### tasks]
```

**Priority Classification**:
- **High Priority ([TOPIC-001] to [TOPIC-003])**: Core research areas critical to primary objective
- **Medium Priority ([TOPIC-004] to [TOPIC-006])**: Supporting research enhancing understanding
- **Low Priority ([TOPIC-007]+)**: Extended coverage and future research foundation

## Research Planning Output

### **File Creation Protocol**
**Target File**: `/Research/Active-Projects/Deep-Research/[topic-stub]/research-planning.md`

**File Content Structure**:
```markdown
# Research Planning: [Topic]
*Created: [YYYY-MM-DD HH:MM:SS CST]*

## ACS Component Selection
**Behavioral**: [selected-component] - [description]
**Procedural**: [selected-component] - [methodology reference]
**Format**: [selected-component] - [template reference]
**Validation**: [selected-tier] - [quality standards]

## Research Objective
[Clear statement of research goals and expected outcomes]

## [TOPIC-###] Framework

### **High Priority Research Areas**

#### **[TOPIC-001]: [Title]**
**Research Question**: [Specific question]
**Investigation Targets**: [Key areas to research]
**Expected Outcomes**: [Deliverables]
**Search Integration**: [How this becomes SEARCH-### in deep-research]

#### **[TOPIC-002]: [Title]**
[Continue structure...]

### **Medium Priority Research Areas**
[TOPIC-004] through [TOPIC-006]...

### **Low Priority Research Areas**
[TOPIC-007]+ if applicable...

## Deep-Research Integration Notes
**Search Strategy**: [How TOPIC-### items will convert to SEARCH-### tasks]
**Validation Requirements**: [Quality standards for research execution]
**Expected Timeline**: [Estimated research completion timeline]
```

## Quality Standards & Completion Criteria

### **Research Planning Validation**
```
📋 Research Planning Completion Checklist:
[ ] **MANDATORY**: Current datetime included in research-planning.md
[ ] ACS components selected and documented
[ ] [TOPIC-###] breakdown with clear priority classification
[ ] Research objectives clearly stated with measurable outcomes
[ ] Deep-research integration notes included
[ ] File created at specified location with proper structure
[ ] Component-specified validation tier applied
```

### **Integration with Deep-Research**
- **[TOPIC-###] to [SEARCH-###] Conversion**: Deep-research takes TOPIC items and refines them into SEARCH tasks
- **ACS Component Inheritance**: Selected components flow through to deep-research execution
- **Search Strategy Reference**: CCC/AI-Workflows/search-item-template.md provides SEARCH structure
- **Quality Continuity**: Validation tier selections maintain consistency through workflow

## Usage Examples

### **Academic Research Planning**
```
/research-plan "systematic review of machine learning applications in healthcare"
```
- **ACS Selection**: systematic-researcher + academic-research-strategy + research-report-template + enhanced-PRISMA
- **Output**: `/Research/Active-Projects/Deep-Research/ml-healthcare-review/research-planning.md`

### **Technical Implementation Research**
```
/research-plan "optimal setup for high-performance computing cluster with RTX 4070"
```
- **ACS Selection**: systematic-researcher + technical-research-strategy + technical-analysis-template + CCC-standard
- **Output**: `/Research/Active-Projects/Deep-Research/hpc-cluster-setup/research-planning.md`

### **Product Research Planning**
```
/research-plan "best camping gear for family outdoor adventures"
```
- **ACS Selection**: systematic-product-researcher + product-research-strategy + product-analysis-template + CCC-standard
- **Output**: `/Research/Active-Projects/Deep-Research/family-camping-gear/research-planning.md`

## Deep-Research Workflow Integration

### **Research Planning → Deep-Research Flow**
1. **Research Planning**: `/research-plan [query]` generates [TOPIC-###] breakdown with ACS component selection
2. **Review & Refinement**: Human validation of research-planning.md structure and scope
3. **Deep Research Execution**: `/deep-research` consumes [TOPIC-###] items and refines them into [SEARCH-###] tasks
4. **Agent Execution**: CCC-Web-Researcher agents execute [SEARCH-###] tasks using inherited ACS components
5. **Compilation**: Results compiled using selected format component templates

### **ACS Component Flow**
- **Planning Phase**: Claude selects components based on domain analysis
- **Research Phase**: Components inherit through to deep-research and agent execution
- **Output Phase**: Format components determine final compilation structure

---

**Command Version**: 2.0.0 | **Framework**: ACS-Integrated Research Planning System
**Compliance**: CCC Behavioral Specifications + ACS Component System + Enhanced PRISMA
**Evidence Rating**: A1 (ACS-driven planning with [TOPIC-###] to [SEARCH-###] integration)

*Systematic research planning through ACS component integration with streamlined [TOPIC-###] generation for deep-research workflow optimization.*