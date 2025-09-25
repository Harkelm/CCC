---
allowed-tools: [Read, Write, Edit, MultiEdit, Bash, Glob, TodoWrite, Task, mcp__sequential-thinking__sequentialthinking, mcp__context7__resolve-library-id, mcp__context7__get-library-docs, WebSearch, WebFetch]
description: "Product research workflow with ACS component selection and explicit file responsibility matrix"
---

# /ccc-product-research - ACS-Driven Product Research Command

## Purpose
Execute focused product research using Agent Component System (ACS) architecture to produce concise, actionable product comparisons and recommendations. Outputs structured product guides rather than comprehensive research reports.

## BEHAVIORAL EXECUTION FLOW

### **Phase 1: ACS Component Selection [CLAUDE DIRECT EXECUTION]**
**Duration**: 2-5 minutes | **No Agent Delegation**

**Claude Direct Responsibilities:**
```
📋 ACS Component Selection Matrix:
[ ] **MANDATORY**: Current datetime recorded in 'YYYY-MM-DD HH:MM:SS CST' format
[ ] **CLAUDE DIRECTLY**: Check existing research at `Research/Active-Projects/Deep-Research/` for overlap
[ ] **CLAUDE DIRECTLY**: Select ACS components based on product research requirements:

Component Selection Framework:
├── Behavioral [REQUIRED]: systematic-product-researcher
├── Procedural: product-research-strategy (Templates/Search-Strategies/Product-Research-Strategy.md)
├── Format: product-analysis-template (Templates/Documents/Product-Analysis-Template.md)
└── Validation [SELECT ONE]:
    ├── enhanced-PRISMA (safety-critical products)
    ├── CCC-standard (general validation)
    └── minimal (low-stakes consumer items)
```

**File Creation**: Claude creates `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/research-planning.md`
**Content Structure**:
```markdown
# Product Research Planning: [Topic]
*Created: [YYYY-MM-DD HH:MM:SS CST]*

## ACS Component Selection
**Behavioral**: systematic-product-researcher
**Procedural**: product-research-strategy (Templates/Search-Strategies/Product-Research-Strategy.md)
**Format**: product-analysis-template (Templates/Documents/Product-Analysis-Template.md)
**Validation**: [selected-validation-tier]

## Search Task Allocation
### [SEARCH-001]: Market Foundation & Product Discovery
### [SEARCH-002]: Comparative Analysis & Deep Evaluation
### [SEARCH-003]: Validation & Gap Resolution
### [Additional SEARCH-### tasks based on complexity]

## File ID Assignment
**Research ID**: PR-001_[topic-stub]
**Results File**: `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/results.md`
```

### **Phase 2: Agent-Executed Research [TACTICAL]**
**Duration**: 15-30 minutes

**Agent Deployment Protocol:**
```
Task: CCC-Web-Researcher
Description: "Execute [SEARCH-00X] product research with ACS components"
Context: "Product research using pre-selected ACS components:

ACS INTEGRATION (PRE-DETERMINED BY CLAUDE):
- Behavioral: systematic-product-researcher
- Procedural: product-research-strategy (Templates/Search-Strategies/Product-Research-Strategy.md)
- Format: product-analysis-template (Templates/Documents/Product-Analysis-Template.md)
- Validation: [already-selected-by-claude]

CRITICAL: This is PRODUCT research - focus on WHAT to buy/choose, NOT implementation.
Use language for purchasing decisions: recommendations, cost-benefit, purchasing strategy.

RESEARCH EXECUTION:
- Source standards: CCC/Framework/Admiralty-Rating-Codes.md (B3+ minimum)
- Research strategy: Follow Product-Research-Strategy.md source hierarchy and bias detection
- Focus: Product options, pricing, key differentiators, purchasing recommendations
- Output: Structured findings for Product-Analysis-Template compilation

FILE RESPONSIBILITY: Agent writes to `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/research/SEARCH-[number].md`"
```

**Agent File Creation Pattern**:
- **Location**: `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/research/SEARCH-[number].md`
- **Content**: Raw research findings, source links, product specifications
- **Format**: Structured data following Product-Research-Strategy.md methodology

### **Phase 3: Product Analysis Compilation [CLAUDE DIRECT]**
**Duration**: 5-10 minutes

**Claude Responsibilities:**
1. **Compile agent research** from `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/research/SEARCH-*.md` files
2. **Apply Product-Analysis-Template.md structure**
3. **Create final analysis** at `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/results.md`

**Final Output File**: `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/results.md`
**Template**: `Templates/Documents/Product-Analysis-Template.md`


## File Responsibility Matrix

### **Directory Structure**:
```
/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/
├── research-planning.md          [CLAUDE CREATES - Phase 1]
├── results.md                    [CLAUDE CREATES - Phase 3]
└── research/
    ├── SEARCH-001.md             [AGENT CREATES - Phase 2]
    ├── SEARCH-002.md             [AGENT CREATES - Phase 2]
    └── SEARCH-003.md             [AGENT CREATES - Phase 2]
```

### **Information Flow**:
1. **User Input** → Claude selects ACS components → `research-planning.md`
2. **Claude** deploys agents with component context → Agents create `research/SEARCH-*.md`
3. **Agents** return research findings → Claude compiles into `results.md` using Product-Analysis-Template.md
4. **Claude** returns file path and brief summary to user

### **Return Format to User**:
```
Product research completed: `/Research/Active-Projects/Product-Research/PR-001_[topic-stub]/results.md`

Summary:
- [X] products evaluated in [category]
- Price range: $[low] - $[high]
- Best Overall: [Product Name] - [Brief reason]
- File ID: PR-001_[topic-stub]
```

## ACS Component References

### **Existing Templates (Direct References)**:
- **Procedural**: `Templates/Search-Strategies/Product-Research-Strategy.md` - Source hierarchy and bias detection methodology
- **Format**: `Templates/Documents/Product-Analysis-Template.md` - Complete analysis structure with comparison matrices

### **ACS Components (To Be Created)**:
- **Behavioral**: `CCC/AI-Workflows/systematic-product-researcher.md` - Methodical product evaluation patterns
- **Validation Templates**: Enhanced PRISMA, CCC-standard, minimal validation tiers

## Usage Examples

### **Tactical Gear Research**:
```
/ccc-product-research "mens hiking outdoor clothing, tactical / minimal preference"
```
- **File ID**: PR-001_tactical-hiking-gear
- **Output**: `/Research/Active-Projects/Product-Research/PR-001_tactical-hiking-gear/results.md`
- **Format**: Product-Analysis-Template.md with tactical focus

### **Tech Product Research**:
```
/ccc-product-research "best laptops for software development under $2000"
```
- **File ID**: PR-002_dev-laptops-2k
- **Output**: `/Research/Active-Projects/Product-Research/PR-002_dev-laptops-2k/results.md`
- **Format**: Product-Analysis-Template.md with feature comparison tables

### **General Consumer Research**:
```
/ccc-product-research "family camping tents for 4 people"
```
- **File ID**: PR-003_family-camping-tents
- **Output**: `/Research/Active-Projects/Product-Research/PR-003_family-camping-tents/results.md`
- **Format**: Product-Analysis-Template.md with consumer focus

## Quality Control Standards

### **Completion Criteria**:
```
📋 Product Research Completion Validation:
[ ] **MANDATORY**: All files include actual datetime stamps
[ ] ACS components properly selected and documented
[ ] Agent research files created following Product-Research-Strategy.md
[ ] Final results.md compiled using Product-Analysis-Template.md structure
[ ] Product recommendations include pricing and availability
[ ] Sources meet minimum B3 Admiralty Code rating
[ ] Output focused on purchasing decisions, not implementation
[ ] File ID properly assigned (PR-001_stub format)
```

### **Output Quality Standards**:
- **Template Compliance**: Follow Product-Analysis-Template.md structure exactly
- **Clear Differentiation**: Comparative analysis with feature tables
- **Actionable Recommendations**: Best Overall, Best Budget, Best Premium
- **No Implementation Details**: Focus on WHAT to buy, not HOW to use
- **Source Quality Summary**: Include source ratings and review count

---

**Command Version**: 1.0.0 | **Framework**: ACS Product Research System
**Evidence Rating**: A1 (Explicit ACS component integration with defined file responsibilities)
**Compliance**: CCC Agent Component System + Admiralty Code Standards
**Architecture**: ACS-Driven Product Research with Clear File Management

*Streamlined product research through Agent Component System architecture with explicit file creation responsibilities and focused purchasing guidance.*