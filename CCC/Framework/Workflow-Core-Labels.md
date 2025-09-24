# Workflow Core Labels Reference
*Systematic Tagging Framework for CCC Organization and Discovery*

---

## Core Tag Categories

### 🔍 **Research & Search Tags**
- **[SEARCH-###]** - Research and information gathering activities
  - Format: Sequential numbering within research waves/projects
  - Example: `[SEARCH-001]` CLI Tools Foundation Layer Synthesis
  - Usage: Multi-agent research tasks, literature reviews, systematic investigations

- **[WAVE-###]** - Research wave groupings and parallel execution phases
  - Format: Sequential numbering across major research initiatives
  - Example: `[WAVE-001]` Foundation Research & Core Applications
  - Usage: Organizing related research tasks into logical execution groups

### ✅ **Task & Action Tags**
- **[TASK-###]** - Specific actionable items requiring completion
  - Format: Sequential numbering within projects or workflows
  - Example: `[TASK-001]` Deploy Obsidian vault structure with security controls
  - Usage: Concrete deliverables, implementation steps, action items

- **[ACTION-###]** - Immediate actions requiring execution
  - Format: Priority-based numbering (001-099: Critical, 100-199: High, etc.)
  - Example: `[ACTION-001]` Execute automated validation checklist
  - Usage: Time-sensitive activities, rapid response items

### 🔍 **Validation & Quality Tags**
- **[VERIFY-###]** - Verification and validation activities
  - Format: Sequential numbering within validation phases
  - Example: `[VERIFY-001]` Cross-validate source credibility using Admiralty Code
  - Usage: Quality assurance checkpoints, evidence validation, compliance verification

- **[VALID-###]** - Validation checkpoint identifiers
  - Format: Alignment with validation tier requirements
  - Example: `[VALID-001]` Enhanced PRISMA 15-item validation complete
  - Usage: Systematic validation milestones, quality gate completions

### 🚪 **Phase & Gateway Tags**
- **[PHASE-###]** - Major workflow phases and lifecycle stages
  - Format: Aligned with 4-phase CCC lifecycle (001-004 for standard phases)
  - Example: `[PHASE-001]` Content Initiation and Classification
  - Usage: High-level workflow organization, milestone tracking

- **[GATE-###]** - Quality gates and approval checkpoints
  - Format: Sequential with phase alignment (001-004 for standard gates)
  - Example: `[GATE-001]` Content Initiation Authorization Required
  - Usage: Decision points, approval requirements, quality thresholds

### ⚠️ **Risk & Security Tags**
- **[RISK-###]** - Risk identification and assessment items
  - Format: Risk category coding (001-099: Critical, 100-199: High, 200-299: Medium, 300+: Low)
  - Example: `[RISK-001]` Information security risk - unauthorized access potential
  - Usage: ISO 31000 risk management, threat identification, mitigation planning

- **[SEC-###]** - Security controls and compliance items
  - Format: Aligned with CIS Controls numbering where applicable
  - Example: `[SEC-001]` CIS Control 1.1 - Asset inventory implementation
  - Usage: Security framework implementation, compliance tracking

### 📊 **Quality & Evidence Tags**
- **[EVID-###]** - Evidence collection and source tracking
  - Format: Include Admiralty Code rating in description
  - Example: `[EVID-001]` Source validation: Technical documentation [A1-1]
  - Usage: Source management, evidence preservation, credibility tracking

- **[QUAL-###]** - Quality assessment and improvement items
  - Format: Sequential numbering with quality tier indication
  - Example: `[QUAL-001]` Content accuracy verification against source material
  - Usage: Quality monitoring, improvement tracking, standard compliance

### 🔄 **Process & Integration Tags**
- **[PROC-###]** - Process documentation and workflow items
  - Format: Process category alignment (001-099: Core, 100-199: Advanced, etc.)
  - Example: `[PROC-001]` AI-human collaboration checkpoint procedure
  - Usage: Standard operating procedures, workflow documentation

- **[INTEG-###]** - Integration and cross-system coordination
  - Format: Integration scope numbering
  - Example: `[INTEG-001]` Obsidian-CCC framework integration requirements
  - Usage: System integration, cross-platform coordination, compatibility

---

## Implementation Standards

### 📋 **Tag Format Requirements**
```
[CATEGORY-###]: Brief descriptive title
- Objective: Clear statement of purpose and expected outcome
- Context: Relevant background and dependencies
- Validation: Required validation tier and criteria
- Status: Current completion status with evidence
```

### 🔢 **Numbering Conventions**
- **Sequential Numbering**: Within category and scope (001, 002, 003...)
- **Priority Indicators**: Critical (001-099), High (100-199), Medium (200-299), Low (300+)
- **Phase Alignment**: Tags aligned with CCC 4-phase lifecycle where applicable
- **Wave Coordination**: Research waves use consistent numbering across related searches

### 🔗 **Cross-Reference Integration**
- **Bidirectional Linking**: Tags must link to related items using `[[Document#Tag]]` format
- **Dependency Tracking**: Prerequisites noted with related tag references
- **Completion Chains**: Sequential tags showing workflow progression
- **Quality Inheritance**: Child tags inherit parent validation requirements

---

## Search & Discovery Patterns

### ⚡ **Rapid Search Examples**
```
Global Search Patterns:
- "[TASK-" → All actionable tasks across vault
- "[SEARCH-001" → Specific research item with wave context
- "[GATE-" + "authorization" → All authorization checkpoints
- "[RISK-" + "security" → All security-related risks
```

### 🔍 **Advanced Query Combinations**
```
Complex Discovery:
- "[PHASE-001" AND "[TASK-" → All tasks in Content Initiation phase
- "[VERIFY-" AND "B3" → All verification items with B3+ evidence standards
- "[WAVE-001" AND "[SEARCH-" → All research items in foundation wave
```

### 🌳 **Tag Hierarchies for Navigation**
```
Hierarchical Organization:
[PHASE-001] Content Initiation
├── [TASK-001] Content classification
├── [TASK-002] Resource assessment
├── [GATE-001] Initiation authorization
└── [RISK-001] Information security assessment
```

---

## Validation Integration

### 📊 **Validation Tier Alignment**
- **Essential (10-item)**: Standard tags with basic validation requirements
- **Extended (15-item)**: Enhanced tags with bias assessment and cross-validation
- **Comprehensive (27-item)**: Critical tags with full PRISMA compliance

### 🎯 **Evidence Standards Integration**
- **Minimum B3 Rating**: All tagged items must meet minimum evidence standards
- **Source Attribution**: Tags include source quality indicators where applicable
- **Cross-Validation**: Related tags require independent verification for critical items

### 🔒 **Security Classification Compatibility**
- **PUBLIC**: Standard tags with open access
- **INTERNAL**: Organization-specific tags with access controls
- **CONFIDENTIAL**: Sensitive tags requiring authorization
- **SECRET**: Highly restricted tags with need-to-know access

---

**Version**: 1.0.0 | **Framework**: CCC Labeling System | **Updated**: 2025-09-23