---
allowed-tools: [Read, Write, Edit, MultiEdit, Bash, Glob, TodoWrite, Task, mcp__sequential-thinking__sequentialthinking, mcp__context7__resolve-library-id, mcp__context7__get-library-docs, WebSearch, WebFetch]
description: "Comprehensive research workflow with systematic validation and Enhanced PRISMA compliance"
---

# /deep-research - Systematic Research Command

## Purpose
Execute comprehensive technical research workflows to provide structured, actionable technical information. Focuses on delivering technical blueprints, implementation patterns, and comparative analysis to enable informed decision-making. Produces concise research reports with practical technical content and clear implementation guidance.

## BEHAVIORAL EXECUTION FLOW

**CRITICAL**: This command defines how Gemini executes research when the user runs `/deep-research [topic]`.

### **Gemini's Responsibilities:**
1. **Phase 1**: Create research-planning.md and directory structure.
2. **Phase 2**: Perform web searches for individual [SEARCH-###] tasks.
3. **Phase 3**: Perform synthesis and validation of research outputs.
4. **Phase 4**: Generate final research report and executive summary.

### **EXECUTION SEQUENCE:**
```
User: /deep-research [topic] [args]
  ↓
Gemini: Creates research-planning.md + directory structure (Phase 1)
  ↓
Gemini: Executes searches for [WAVE-001]
  ↓
Gemini: Synthesizes [WAVE-001] → Executes searches for [WAVE-002]
  ↓
Gemini: Synthesizes [WAVE-002] → Executes searches for [WAVE-003]
  ↓
Gemini: Final synthesis and report generation (Phase 3-4)
```

## Usage
```
/deep-research [Research Prompt] [--output-format=md|pdf] [--scope=web|library|comprehensive] [--validation=standard|rigorous]
```

## Arguments
- `Research Prompt` - Detailed research question or topic specification with clear scope and objective definition
- `--output-format` - Results document format (default: md) with comprehensive formatting and source documentation integration
- `--scope` - Research domain focus (default: comprehensive) determining resource allocation protocols
- `--validation` - Quality control level (default: standard) with enhanced validation protocols and evidence verification requirements

## CCC Vault Integration

### **Research Domain Structure**
This command creates projects within the CCC vault's Research domain structure:
- **Base Location**: `/Research/Active-Projects/Deep-Research/`
- **Results Integration**: Final reports can be moved to `/Research/Findings/` upon completion
- **Source Archive**: Source materials archived in `/Research/References/` for future use
- **Template Compliance**: Uses [[Templates/Documents/Research-Report-Template]] for consistency

### **Framework Compliance**
- **Enhanced PRISMA**: Follows [[CCC/Standards/Enhanced-PRISMA]] validation protocols
- **AI Workflow**: Implements [[CCC/AI-Workflows/AI-Standards]] for human-AI collaboration
- **Security**: Applies [[CCC/Security/]] classification and access controls
- **Architecture**: Aligned with [[CCC/Architecture]] design principles

## Research Architecture Framework

### **Phase 1: Research Planning & Infrastructure [CRITICAL]**
**Duration**: 5-10 minutes | **Validation Tier**: Essential

**Objectives**:
1. **Research Scope Analysis**: Parse research question for objectives, deliverables, and success criteria
2. **Infrastructure Creation**: Automated workspace creation with organized structure
3. **Template Strategy Planning**: Review [[Templates/Template-Guide]] and assign appropriate templates to [SEARCH-###] tasks
4. **Strategy Development**: Multi-wave coordination protocols
5. **Quality Gate Establishment**: Validation criteria definition with evidence requirements

**Phase 1 Protocol**:
```
📋 Research Planning Checklist:
[ ] **MANDATORY**: Current datetime obtained and recorded in '''YYYY-MM-DD HH:MM:SS CST''' format
[ ] Research scope clearly defined with measurable objectives
[ ] Directory structure created at /Research/Active-Projects/Deep-Research/[stub]/
[ ] Template guide reviewed ([[Templates/Template-Guide]]) for appropriate template selection
[ ] [SEARCH-###] tasks assigned with specific templates (Research-Report-Template, Technical-Guide-Template, etc.)
[ ] Research strategy documented with [SEARCH-###] task assignment plan
[ ] Quality criteria established with Enhanced PRISMA compliance requirements
```

**Research Planning Structure (research-planning.md)**:
```markdown
# Research Planning: [Topic]
*Created: [YYYY-MM-DD HH:MM:SS CST] - MANDATORY: Insert actual current datetime*

## Research Objectives
[Clear, measurable research goals]

## Search Task Breakdown

### [WAVE-001]: Foundation Research
- **[SEARCH-001]**: Broad topic exploration - academic papers, authoritative sources
  - **Template**: [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Academic-Research-Strategy]]
  - **Validation**: Extended (15-item)
- **[SEARCH-002]**: Technical documentation - official docs, standards, specifications
  - **Template**: [[Templates/Documents/Technical-Guide-Template]] or [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Technical-Documentation-Strategy]]
  - **Validation**: Essential (10-item) to Extended (15-item)
- **[SEARCH-003]**: Community insights - expert discussions, implementation experiences
  - **Template**: [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Modern-Technology-Strategy]]
  - **Validation**: Essential (10-item) to Extended (15-item)

### [WAVE-002]: Deep Dive Investigation
- **[SEARCH-004]**: Gap-focused research - address deficiencies from [WAVE-001]
  - **Template**: [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Academic-Research-Strategy]]
  - **Validation**: Extended (15-item)
- **[SEARCH-005]**: Implementation analysis - detailed technical approaches
  - **Template**: [[Templates/Documents/Technical-Guide-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Technical-Documentation-Strategy]]
  - **Validation**: Essential (10-item) to Extended (15-item)
- **[SEARCH-006]**: Expert perspectives - professional opinions, case studies
  - **Template**: [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Modern-Technology-Strategy]]
  - **Validation**: Extended (15-item)

### [WAVE-003]: Technical Comparison & Implementation Patterns
- **[SEARCH-007]**: Comparative analysis - detailed comparison of alternative approaches with implementation examples
  - **Template**: [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Modern-Technology-Strategy]]
  - **Validation**: Extended (15-item)
- **[SEARCH-008]**: Implementation patterns - concrete code examples, best practices, and integration details
  - **Template**: [[Templates/Documents/Technical-Guide-Template]] or [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Technical-Documentation-Strategy]]
  - **Validation**: Essential (10-item) to Extended (15-item)
- **[SEARCH-009]**: Expert implementation insights - professional implementation patterns and technical recommendations
  - **Template**: [[Templates/Documents/Research-Report-Template]]
  - **Search Strategy**: [[Templates/Search-Strategies/Academic-Research-Strategy]]
  - **Validation**: Extended (15-item)

## Quality Standards
- **Minimum Source Rating**: B3 Admiralty Code
- **Validation Tier**: [Essential|Extended|Comprehensive]
- **Cross-validation Requirements**: [Multi-source verification criteria]

## Task Dependencies
### [WAVE-001] Dependencies
- **[SEARCH-001] → [SEARCH-004]**: Foundation research scope influences gap-focused research direction
- **[SEARCH-002] → [SEARCH-005]**: Technical documentation analysis informs implementation approach
- **[SEARCH-003] → [SEARCH-006]**: Community insights inform expert perspective focus areas

### [WAVE-002] Dependencies
- **[SEARCH-004] → [SEARCH-007]**: Gap analysis identifies areas requiring alternative approach research
- **[SEARCH-005] → [SEARCH-008]**: Implementation analysis informs version/compatibility validation scope
- **[SEARCH-006] → [SEARCH-009]**: Expert perspectives inform assumption challenge methodology

### Cross-Wave Information Flow
- **Prerequisite Information**: [Document specific information dependencies between waves]
- **Context Sharing**: [Define how previous wave findings inform subsequent research]
- **Integration Points**: [Identify where cross-task validation is required]
```

**Directory Structure Creation**:
```
/Research/Active-Projects/Deep-Research/[stub]/
├── research-planning.md          # Initial planning and scope (Phase 1)
├── research/                     # Workspace directory
│   ├── wave-001/                 # Foundation research wave
│   │   ├── SEARCH-001.md         # Broad topic exploration
│   │   ├── SEARCH-002.md         # Technical documentation focus
│   │   ├── SEARCH-003.md         # Community insights and implementation experiences
│   │   └── wave-synthesis.md     # [WAVE-001] findings compilation
│   ├── wave-002/                 # Deep dive investigation wave
│   │   ├── SEARCH-004.md         # Gap-focused research
│   │   ├── SEARCH-005.md         # Implementation analysis
│   │   ├── SEARCH-006.md         # Expert perspectives
│   │   └── wave-synthesis.md     # [WAVE-002] findings compilation
│   └── wave-003/                 # Technical comparison & implementation patterns
│       ├── SEARCH-007.md         # Comparative analysis with implementation examples
│       ├── SEARCH-008.md         # Implementation patterns and technical details
│       ├── SEARCH-009.md         # Expert implementation insights
│       └── wave-synthesis.md     # [WAVE-003] findings compilation
├── results.md                    # Main research report (Phase 4)
└── executive-summary.md          # Condensed findings summary (Phase 4)
```

### **Phase 2: Research Execution [TACTICAL]**
**Duration**: 15-45 minutes

**Multi-Wave Research Strategy**:

#### **[WAVE-001]: Foundation Research**
- **[SEARCH-001]**: Broad topic exploration, academic sources
- **[SEARCH-002]**: Technical documentation, official sources
- **[SEARCH-003]**: Library/framework research

#### **[WAVE-002]: Deep Dive Investigation**
- **[SEARCH-004]**: Gap-focused research addressing [WAVE-001] deficiencies
- **[SEARCH-005]**: Implementation analysis, technical deep dive
- **[SEARCH-006]**: Expert perspectives, specialized sources

#### **[WAVE-003]: Technical Comparison & Implementation Patterns**
- **[SEARCH-007]**: Comparative analysis with implementation examples
- **[SEARCH-008]**: Implementation patterns and technical details
- **[SEARCH-009]**: Expert implementation insights and professional patterns

**Context Package Requirements**:
- **Timestamp Protocol**: MANDATORY requirement to include actual current datetime in '''YYYY-MM-DD HH:MM:SS CST''' format in all [SEARCH-###] documents
- **Enhanced PRISMA Template**: Systematic validation framework from [[CCC/Standards/Enhanced-PRISMA]]
- **Research Planning Context**: Current research-planning.md contents with [SEARCH-###] task assignments
- **Template Specifications**: Specific template link for [SEARCH-###] task (e.g., [[Templates/Documents/Research-Report-Template]])
- **Search Strategy Templates**: Assigned search strategy template for [SEARCH-###] task (e.g., [[Templates/Search-Strategies/Academic-Research-Strategy]])
- **Admiralty Code Standards**: Source credibility assessment requirements (B3+ minimum)
- **CCC Behavioral Specifications**: Quality and validation standards from [[GEMINI]]
- **Wave-Specific Objectives**: Targeted research goals and gap identification
- **Template Validation Requirements**: Quality standards from assigned template (Essential/Extended validation tier)

**Inter-Wave Validation Checkpoints**:
```
📋 Wave Completion Validation:
[ ] [WAVE-001]: Initial findings documented with source quality assessment and template compliance
[ ] [WAVE-002]: Deep dive completed with targeted gap resolution and template validation
[ ] [WAVE-003]: Comprehensive validation with alternative perspective analysis and template standards
[ ] Cross-Validation: Findings consistency analysis completed
[ ] Template Compliance: All [SEARCH-###] outputs follow assigned templates (Research-Report-Template, Technical-Guide-Template)
[ ] Validation Tier Compliance: All [SEARCH-###] tasks meet assigned validation requirements (Essential/Extended)
[ ] Quality Control: All sources meet minimum B3 Admiralty Code rating
```


### **Phase 3: Technical Synthesis & Report Generation [FOCUS]**
**Duration**: 10-20 minutes | **Technical Content Focus**

**Synthesis Protocol**:
1. **Technical Findings Compilation**: Organize all research into actionable technical content
2. **Implementation Pattern Analysis**: Extract concrete implementation patterns and code examples
3. **Comparative Analysis**: Synthesize comparisons between different approaches
4. **Technical Decision Framework**: Provide structured information for technical decision-making

**Quality Standards**:
```
📋 Research Quality Checklist:
[ ] Technical information clearly organized and actionable
[ ] Implementation patterns include concrete examples
[ ] Comparative analysis provides structured decision-making information
[ ] Sources meet minimum credibility standards (B3+ when applicable)
[ ] Results focus on technical content rather than process documentation
```

### **Phase 4: Final Report Generation [DELIVERY]**
**Duration**: 5-10 minutes | **Technical Content Focus**

**Report Generation Protocol**:
1. **Technical Content Organization**: Structure all research into clear, actionable format
2. **Results Document Creation**: Comprehensive technical report with implementation guidance
3. **Executive Summary**: Concise summary with key technical findings and recommendations
4. **Implementation Focus**: Ensure results provide technical blueprints rather than process documentation

## Technical Quality Framework

### **Source Quality Guidelines**
```
Technical Source Priority:
- Official documentation and specifications (highest priority)
- Professional implementation examples and patterns
- Expert technical opinions and recommendations
- Community validation and real-world usage examples
- Avoid marketing materials and unsubstantiated claims
```

### **Research Output Standards**
```
📋 Technical Content Requirements:
[ ] Technical accuracy with credible sources
[ ] Concrete implementation examples where applicable
[ ] Clear comparison matrices for alternative approaches
[ ] Actionable technical information for decision-making
[ ] Focus on "how to build" rather than "why you might fail"
```

### **Sequential Thinking Integration**
**Use Cases**: Complex research logic, assumption challenge, evidence synthesis
**Context Package Requirements**:
- Multi-step research problems requiring systematic analysis
- Evidence-based decision making with assumption challenge
- Complex synthesis requiring logical progression

## Quality Control & Synthesis Protocols

### **Wave Synthesis Protocols**

#### **[SEARCH-###] Results Compilation**
```markdown
# Wave Synthesis Process
1. **Individual Task Review**: Validate each [SEARCH-###] file for completeness and quality
2. **Source Quality Assessment**: Aggregate Admiralty Code ratings across all wave tasks
3. **Finding Integration**: Combine discoveries into coherent wave summary
4. **Gap Identification**: Document areas requiring additional research
5. **Quality Metrics**: Calculate wave-level quality indicators

# Wave Synthesis File Structure (/research/wave-X/wave-synthesis.md)
## Wave [X] Summary
### Completed Tasks
- **[SEARCH-00X]**: [Brief summary] | **Quality**: [Average rating] | **Status**: [Complete/Partial]

### Key Findings
[Synthesized discoveries organized by theme/relevance]

### Source Quality Matrix
| Task | Sources | Avg Rating | Quality Notes |
|------|---------|------------|---------------|
| SEARCH-001| 5       | B2         | Strong academic sources |

### Research Gaps Identified
[Areas requiring follow-up research]

### Next Wave Recommendations
[Specific objectives for subsequent research waves]
```

#### **Quality Assurance Checkpoints**
```
📋 Wave Completion Validation:
[ ] All [SEARCH-###] tasks completed with evidence documentation
[ ] Template compliance verified for all [SEARCH-###] outputs (Research-Report-Template, Technical-Guide-Template)
[ ] Validation tier requirements met for all [SEARCH-###] tasks (Essential/Extended as specified)
[ ] Source quality meets minimum B3 threshold across all tasks
[ ] No critical research gaps remain unaddressed within wave scope
[ ] Wave synthesis document complete with quality metrics
[ ] Context prepared for subsequent wave or final synthesis
```

### **Final Synthesis Integration**

#### **Multi-Wave Consolidation**
- **Evidence Triangulation**: Validate findings across multiple waves and sources
- **Conflict Resolution**: Address contradictory findings with systematic analysis
- **Confidence Assessment**: Assign confidence levels based on source quality and consensus
- **Comprehensive Gap Analysis**: Identify overall research limitations and areas for future work

#### **Quality Control Dashboard**
```markdown
# Research Quality Dashboard
## Overall Metrics
- **Total [SEARCH-###] Tasks**: [X] | **Completed**: [Y] | **Success Rate**: [Z%]
- **Average Source Quality**: [Rating] | **Total Sources**: [Count]
- **Cross-Validation Rate**: [Percentage] | **Conflict Resolution**: [Count]

## Wave Performance
| Wave | Tasks | Completion | Avg Quality | Key Issues |
|------|-------|------------|-------------|------------|
| 1    | 3     | 100%       | A2          | None       |
| 2    | 3     | 100%       | B3          | Version conflicts |
| 3    | 3     | 67%        | B2          | Expert source limited |

## Quality Flags
[ ] All waves meet minimum quality thresholds
[ ] No unresolved source conflicts
[ ] Research objectives fully addressed
[ ] Expert review completed where required
```

## Error Handling & Recovery Protocols

### **Source Validation Failure Protocols**
- **Alternative Identification**: Source validation with quality assessment
- **Expert Consultation**: Escalation for critical information gaps
- **Confidence Adjustment**: Level modification with limitation documentation
- **Scope Modification**: Research boundary adjustment with stakeholder approval

## Implementation Success Criteria

### **Technical Completion Requirements**
```
📋 Research Completion Checklist:
[ ] **MANDATORY**: All documents include actual current datetime in '''YYYY-MM-DD HH:MM:SS CST''' format
[ ] Complete technical research report with actionable implementation guidance
[ ] All [SEARCH-###] tasks completed with technical focus
[ ] Comparative analysis includes concrete examples and implementation patterns
[ ] Technical decision-making information clearly structured
[ ] Executive summary highlights key technical findings and recommendations
[ ] Results focus on technical blueprints rather than validation processes
```

### **Technical Quality Metrics**
- **Implementation Focus**: Research provides concrete technical guidance
- **Comparative Analysis**: Clear comparison of alternative approaches with examples
- **Source Credibility**: Technical information from reliable sources
- **Actionable Content**: Results enable informed technical decision-making

## Advanced Deployment Patterns

### **Academic Research Focus**
```
/deep-research "systematic review of [topic]" --scope comprehensive --validation rigorous
```
- Enhanced focus on peer-reviewed sources (A1-A2 preference)
- Extended validation with 15-item Enhanced PRISMA compliance
- Expert consultation for critical findings

### **Technical Implementation Research**
```
/deep-research "best practices for [technology]" --scope library --validation standard
```
- Working code examples with validation requirements
- Implementation comparison with decision matrix

### **Industry Analysis Research**
```
/deep-research "market trends in [domain]" --scope web --validation standard
```
- Expert opinion focus with credential verification
- Trend analysis with multiple perspective validation
- Professional source preference with bias assessment

## Framework Integration

**CCC Integration**: Compatible with Context Command Center framework and research domain structure
**Technical Focus**: Emphasizes actionable technical content over process validation
**Implementation Guidance**: Provides concrete technical blueprints for informed decision-making
**Quality Standards**: Maintains source credibility while prioritizing technical utility

---

**Command Version**: 1.0.0 | **Framework**: CCC-Compatible
**Evidence Rating**: A1 (Framework standards with systematic validation)
**Compliance**: Enhanced PRISMA + ISO 31000 + CIS Controls + CCC Behavioral Specifications
**Vault Integration**: Compatible with CCC Research domain structure

*Technical research excellence through systematic research focused on actionable implementation guidance and comparative analysis integrated with the Context Command Center framework.*
