---
allowed-tools: [Read, Write, Edit, MultiEdit, Bash, Glob, TodoWrite, Task, mcp__sequential-thinking__sequentialthinking, mcp__context7__resolve-library-id, mcp__context7__get-library-docs, WebSearch, WebFetch]
description: "Comprehensive multi-domain research workflow with adaptive framework integration and comparative analysis focus"
---

# /deep-research - Universal Multi-Domain Research Command

## Purpose
Execute comprehensive research across any domain using adaptive agent orchestration to provide structured, evidence-based analysis focusing on understanding options, recommendations, and the 'why' behind choices. Produces systematic research reports with comparative analysis to enable informed decision-making across academic, technical, product, historical, literary, scientific, and practical domains.

## BEHAVIORAL EXECUTION FLOW

**CRITICAL**: This command defines how Claude executes research when the user runs `/deep-research [topic]` on Claude.

**DOMAIN LANGUAGE WARNING**: Do NOT use implementation/technical language unless the research is specifically about HOW to build/implement something technical. Product research is about WHAT to buy, not how to implement. Historical research is about WHAT happened, not how to implement. Match your language to the research domain appropriately.

### **Claude's Direct Responsibilities:**
1. **Phase 1**: Domain classification and adaptive planning with CCC framework integration
2. **Phase 2**: Multi-agent research execution with flexible wave structure
3. **Phase 3**: Comparative analysis synthesis with evidence evaluation
4. **Phase 4**: Domain-appropriate research report generation with contextually relevant recommendations

### **Agent Integration:**
- **CCC-Web-Researcher**: Primary research agent for systematic investigation
- **general-purpose**: Supporting analysis and specialized research tasks
- **Context7 integration**: Library/technical documentation when applicable
- **Component System**: Modular CCC framework components loaded as needed

## Usage
```
/deep-research [Research Topic] [--domain=auto|academic|technical|product|practical|creative] [--scope=focused|comprehensive] [--output=analysis|recommendations|comparison]
```

## Arguments
- `Research Topic` - Any research question across domains requiring systematic investigation
- `--domain` - Research domain classification (default: auto-detect)
  - `auto`: Intelligent domain detection and template selection
  - `academic`: Academic papers, literature, scientific research
  - `technical`: Technology analysis, implementation options
  - `product`: Product comparisons, market analysis, recommendations
  - `practical`: How-to guides, skill development, procedural knowledge
  - `creative`: Literary analysis, cultural studies, artistic research
- `--scope` - Research comprehensiveness (default: comprehensive)
- `--output` - Primary focus (default: analysis with recommendations)

## CCC Framework Integration

### **Modular Component System**
This command leverages the CCC Agent Component System with dynamic loading:

#### **Core Components (Always Loaded)**
- **CCC/Agents/Agent.md**: Universal behavioral core and quality standards
- **CCC/Framework/Status-Tag-Codes.md**: Progress tracking and completion validation
- **CCC/Framework/Admiralty-Rating-Codes.md**: Source credibility assessment
- **CCC/Framework/Workflow-Core-Labels.md**: Systematic organization and discovery

#### **Contextual Components (Domain-Dependent)**
- **@CCC/Framework/Domain-Detection-Quick-Reference.md**: Auto-detection logic for domain classification
- **@CCC/Framework/Validation-Tier-Selection-Quick-Reference.md**: Smart validation tier assignment
- **@CCC/Framework/Comparative-Analysis-Quick-Reference.md**: Systematic comparison framework
- **@CCC/Framework/Validation-Quick-Reference.md**: Applied for academic/safety-critical research
- **@CCC/Framework/Risk-Quick-Reference.md**: Loaded for practical/safety research
- **@Templates/Template-Guide.md**: Smart template selection based on domain
- **Templates/Search-Strategies/Product-Research-Strategy.md**: Product domain research methodology

### **Research Domain Structure**
- **Base Location**: `/Research/Active-Projects/Deep-Research/[research-stub]/`
- **Framework Compliance**: Full CCC behavioral specifications
- **Template Integration**: Dynamic template selection from Template-Guide.md
- **Quality Standards**: Adaptive validation based on research domain

## Research Architecture Framework

### **Phase 1: Domain Analysis & Adaptive Planning [FOUNDATION]**
**Duration**: 5-10 minutes | **Framework Integration**: Essential

**Domain Classification Protocol**:
```
📋 Research Domain Analysis:
[ ] **MANDATORY**: Current datetime recorded in 'YYYY-MM-DD HH:MM:SS CST' format
[ ] Research domain identified using @CCC/Framework/Domain-Detection-Quick-Reference.md
[ ] Template selection from @Templates/Template-Guide.md based on domain
[ ] Validation tier assignment using @CCC/Framework/Validation-Tier-Selection-Quick-Reference.md
[ ] Component loading decisions based on research requirements
[ ] Wave structure adaptation based on topic complexity
[ ] Quality standards establishment per @CCC/Agents/Agent.md
```

**Domain Classification Matrix**:
- **Academic**: Peer-reviewed research, literature analysis, scientific investigation
- **Technical**: Technology evaluation, system analysis, implementation options
- **Product**: Market research, product comparisons, buying guides
- **Practical**: How-to analysis, skill development, procedural knowledge
- **Creative**: Literary analysis, cultural studies, artistic research
- **Historical**: Event analysis, timeline research, contextual studies
- **Scientific**: Hypothesis investigation, data analysis, methodology research

**Adaptive Template Selection**:
```markdown
Domain → Template Mapping (from @Templates/Template-Guide.md):
- Academic: Research-Report-Template (Extended validation)
- Technical: Technical-Guide-Template (Essential validation)
- Product: Product-Analysis-Template (Essential validation)
- Practical: Technical-Guide-Template (Essential + Safety if applicable)
- Creative: Literature-Analysis-Template (Essential validation)
- Multi-domain: Research-Report-Template (Extended validation)
```

**Planning Document Structure (research-planning.md)**:
```markdown
# Research Planning: [Topic]
*Created: [YYYY-MM-DD HH:MM:SS CST] - MANDATORY datetime*

## Domain Analysis
**Primary Domain**: [Classification result]
**Research Objective**: [Clear goal focusing on options/recommendations/why]
**Scope Definition**: [Boundaries and constraints]

## Component Integration
**Templates**: [Selected from Template-Guide.md]
**Validation**: [Essential/Extended/Comprehensive]
**Components**: [Loaded framework components]

## Wave Structure
### [WAVE-001]: Foundation Research
### [WAVE-002]: Deep Investigation
### [WAVE-003]: Comparative Analysis
### [Additional waves as needed based on complexity]

## Search Task Framework
[Dynamic SEARCH-### allocation - not limited to 3 per wave]
```

### **Phase 2: Multi-Agent Research Execution [TACTICAL]**
**Duration**: 20-60 minutes | **Adaptive Scaling**

**Flexible Wave Architecture**:

#### **[WAVE-001]: Foundation Research**
**Objective**: Establish foundational understanding and identify key areas
**Search Tasks**: [SEARCH-001] through [SEARCH-00X] - scalable based on complexity

**Agent Deployment Pattern**:
```
Task: CCC-Web-Researcher
Context: "Execute [SEARCH-00X]: [specific research objective].

FRAMEWORK INTEGRATION:
- Apply @CCC/Framework/Admiralty-Rating-Codes.md (minimum B3 rating)
- Use template: [domain-appropriate template from Template-Guide.md]
- Search strategy: [domain-specific strategy from Search-Strategies/]
- Validation tier: [Essential/Extended/Comprehensive from Validation-Tier-Selection]

RESEARCH FOCUS:
- Understand 'why' options exist and their comparative advantages
- Evidence-based analysis with source quality assessment
- Gap identification for subsequent wave planning

OUTPUT REQUIREMENTS:
- MANDATORY: Include current datetime in YYYY-MM-DD HH:MM:SS CST format
- Write to /research/wave-001/SEARCH-[number].md
- Follow CCC-Web-Researcher methodology with structured documentation"
```

#### **[WAVE-002]: Deep Investigation**
**Objective**: Detailed analysis of specific areas identified in WAVE-001
**Search Tasks**: Gap-focused research addressing foundation limitations
**Scaling**: Additional search tasks as needed based on WAVE-001 findings

#### **[WAVE-003]: Comparative Analysis [MANDATORY]**
**Objective**: Systematic comparison of alternatives with decision framework
**Search Tasks**: Focused on comparative evaluation and recommendation synthesis using @CCC/Framework/Comparative-Analysis-Quick-Reference.md
**Output**: Clear comparison matrices and decision-making guidance following systematic comparison structure

**Additional Waves**: Scale beyond 3 waves for complex research requiring specialized investigation phases

**Agent Context Package Requirements**:
- **@CCC/Agents/Agent.md**: Core behavioral specifications and quality standards
- **Research Planning Context**: Current wave objectives and findings from previous waves
- **Template Specifications**: Domain-appropriate template from Template-Guide.md
- **Search Strategy**: Domain-specific methodology from Search-Strategies/ directory
- **Validation Requirements**: Tier from Validation-Tier-Selection-Quick-Reference.md
- **Admiralty Standards**: Source credibility requirements from Admiralty-Rating-Codes.md
- **Comparative Framework**: Comparative-Analysis-Quick-Reference.md for Wave 3 tasks
- **Progress Tracking**: Status indicators from Status-Tag-Codes.md

### **Phase 3: Comparative Synthesis & Analysis [FOCUS]**
**Duration**: 10-25 minutes | **Decision-Oriented**

**Synthesis Protocol**:
1. **Evidence Integration**: Systematic compilation using @CCC/Framework/Admiralty-Rating-Codes.md
2. **Comparative Framework**: Structured comparison of alternatives with pros/cons analysis
3. **Recommendation Development**: Evidence-based recommendations with rationale
4. **Quality Validation**: Application of appropriate validation tier

**Comparative Analysis Standards**:
```
📋 Comparative Analysis Checklist (from @CCC/Framework/Comparative-Analysis-Quick-Reference.md):
[ ] Options identification: 3-5 main alternatives including status quo
[ ] Evaluation criteria: 4-6 most important factors (cost, complexity, performance, risk, time, compatibility)
[ ] Comparison matrix: Systematic scoring with evidence backing
[ ] Recommendation logic: Best for specific contexts with clear reasoning
[ ] Decision framework: High/medium/low stakes mapping to evidence requirements
[ ] Uncertainty factors: What could change the recommendation
```

### **Phase 4: Domain-Appropriate Research Report [DELIVERY]**
**Duration**: 10-15 minutes | **Contextually Relevant Content**

**CRITICAL**: Report structure must match research domain. Do NOT use implementation language unless the research is about HOW to build/implement something technical.

**Domain-Specific Report Structures**:

#### **Product Domain** (What to buy/choose):
1. **Executive Summary**: Key findings and primary recommendations
2. **Comparative Analysis**: Clear option comparison with decision matrix
3. **Purchase Recommendations**: Specific product/service recommendations with rationale
4. **Cost-Benefit Analysis**: Value propositions and budget considerations
5. **Purchasing Strategy**: Timing, sourcing, and acquisition guidance

#### **Academic/Scientific Domain** (What research shows):
1. **Executive Summary**: Key findings and scholarly consensus
2. **Literature Synthesis**: Integration of research findings across sources
3. **Evidence Analysis**: Source quality assessment and confidence levels
4. **Research Gaps & Limitations**: Areas needing further investigation
5. **Further Research Directions**: Suggested areas for continued study

#### **Historical/Cultural Domain** (What happened/what it means):
1. **Executive Summary**: Key events and historical significance
2. **Historical Analysis**: Context, causes, and consequences
3. **Evidence Assessment**: Source reliability and historical interpretation
4. **Significance & Impact**: Long-term effects and contemporary relevance
5. **Related Topics**: Connected events, figures, or themes for further exploration

#### **Technical Domain** (How to implement/build) - ONLY when user asks HOW:
1. **Executive Summary**: Key technical findings and recommendations
2. **Technical Analysis**: System requirements and architectural considerations
3. **Implementation Considerations**: Practical deployment guidance
4. **Integration Requirements**: Dependencies and compatibility factors
5. **Deployment Strategy**: Step-by-step implementation approach

#### **Practical Domain** (How to do/accomplish):
1. **Executive Summary**: Key methods and recommended approaches
2. **Method Comparison**: Systematic evaluation of different approaches
3. **Application Guidance**: Step-by-step practical instructions
4. **Resource Requirements**: Tools, skills, and materials needed
5. **Success Factors**: Critical elements for effective execution

#### **Creative/Literary Domain** (What it means/significance):
1. **Executive Summary**: Key themes and analytical insights
2. **Critical Analysis**: Interpretation and significance across sources
3. **Evidence Synthesis**: Scholarly consensus and debate areas
4. **Cultural Context**: Historical and contemporary relevance
5. **Related Works**: Connected literature, art, or cultural phenomena

## Quality Framework Integration

### **Source Quality Standards (from @CCC/Framework/Admiralty-Rating-Codes.md)**
- **Minimum Standard**: B3 rating for most research
- **Academic Research**: A2 minimum for critical claims
- **Safety-Critical**: A1 preferred with independent verification
- **Product Research**: B2 minimum with bias assessment

### **Validation Integration (from @CCC/Framework/Validation-Quick-Reference.md)**
```
Domain-Based Validation Assignment:
- Academic/Scientific: Extended (15-item) validation
- Technical/Product: Essential (10-item) validation
- Safety/Medical: Comprehensive (27-item) validation
- Creative/Cultural: Essential (10-item) validation
```

## Agent Deployment Patterns

### **Smart Agent Selection**
**Primary Pattern - CCC-Web-Researcher**:
```
Task: CCC-Web-Researcher
Description: "Comprehensive [domain] research with structured methodology"
Context: "Execute systematic research using CCC-Web-Researcher methodology:

DOMAIN INTEGRATION:
- Domain: [detected from Domain-Detection-Quick-Reference.md]
- Template: [selected from Template-Guide.md mapping]
- Search Strategy: [appropriate strategy from Search-Strategies/]
- Validation Tier: [selected from Validation-Tier-Selection-Quick-Reference.md]

CRITICAL DOMAIN GUIDANCE:
- Product Domain: Focus on WHAT to buy/choose, NOT how to implement
- Academic Domain: Focus on WHAT research shows, NOT how to implement findings
- Historical Domain: Focus on WHAT happened and significance, NOT implementation
- Technical Domain: ONLY use implementation language when user asks HOW to build/implement
- Use domain-appropriate language: purchase/acquisition (products), exploration (historical), further study (academic)

RESEARCH EXECUTION:
- Apply multi-phase methodology: Query Analysis → Strategic Gathering → Content Analysis → Documentation
- Source standards: @CCC/Framework/Admiralty-Rating-Codes.md (B3+ minimum)
- Focus: Understand options and WHY they exist, evidence-based recommendations
- Quality framework: Enhanced PRISMA validation as appropriate

Write structured documentation to designated wave/search file."
```

**Product Research Pattern**:
```
Task: CCC-Web-Researcher
Description: "Product research with specialized methodology"
Context: "Product domain research using @Templates/Search-Strategies/Product-Research-Strategy.md:

FRAMEWORK:
- Template: Product-Analysis-Template
- Search Strategy: Product-Research-Strategy (independent sources, bias detection)
- Validation: Essential tier from Validation-Tier-Selection-Quick-Reference.md

CRITICAL: This is PRODUCT research - focus on WHAT to buy/choose, NOT implementation.
Use language appropriate for purchasing decisions: recommendations, cost-benefit, purchasing strategy.
AVOID: Implementation readiness, deployment strategy, next steps for implementation.

FOCUS: Product comparisons, market analysis, buying recommendations with systematic comparison framework."
```

**Academic Research Pattern**:
```
Task: CCC-Web-Researcher
Description: "Academic research with Enhanced PRISMA compliance"
Context: "Academic investigation using comprehensive validation:

FRAMEWORK:
- Template: Research-Report-Template
- Validation: Extended tier, @CCC/Standards/Enhanced-PRISMA.md for critical findings
- Search Strategy: Academic-Research-Strategy (peer-reviewed focus)
- Source minimum: A2 rating for key claims

CRITICAL: This is ACADEMIC research - focus on WHAT research shows and scholarly consensus.
Use language appropriate for academic findings: literature synthesis, research gaps, further study directions.
AVOID: Implementation readiness, deployment strategy, next steps for implementation.

FOCUS: Systematic literature review methodology, evidence hierarchy, expert validation."
```

## Advanced Research Patterns

### **Cross-Domain Research**
```
/deep-research "integration of X and Y for Z purpose" --domain=auto --scope=comprehensive
```
- Automatic domain detection for complex topics
- Multi-template integration approach
- Enhanced comparative analysis across domains

### **Academic Literature Research**
```
/deep-research "systematic review of [academic topic]" --domain=academic --scope=comprehensive
```
- Enhanced PRISMA compliance loading
- A1-A2 source preference with peer-review focus
- Extended validation protocols

### **Product/Technology Analysis**
```
/deep-research "best options for [need/goal]" --domain=product --output=recommendations
```
- Market analysis with comparative matrices
- Cost-benefit evaluation frameworks
- User experience and practical considerations

### **Historical/Cultural Research**
```
/deep-research "analysis of [historical/cultural topic]" --domain=creative --scope=comprehensive
```
- Cultural context and significance analysis
- Multiple perspective integration
- Timeline and causation analysis

## Error Recovery & Quality Assurance

### **Agent Failure Protocols (from @CCC/Framework/Emergency-Escalation-Procedures.md)**
- **Retry with Modified Parameters**: Automatic adjustment for transient failures
- **Alternative Agent Deployment**: Backup strategy with general-purpose agents
- **Quality Degradation Documentation**: Clear impact assessment and limitations

### **Quality Control Integration**
```
📋 Research Quality Validation (from @CCC/Agents/Agent.md):
[ ] **MANDATORY**: All documents include actual current datetime
[ ] All sources meet minimum Admiralty Code ratings
[ ] Template compliance verified for all outputs
[ ] Comparative analysis completed with clear recommendations
[ ] Evidence hierarchy established with confidence levels
[ ] Cross-validation performed where applicable
[ ] Research objectives fully addressed with evidence
```

## Research Completion Standards

### **Quality Completion Criteria**
```
📋 Research Completion Validation:
[ ] **MANDATORY**: Datetime compliance across all documents
[ ] Foundation research establishes comprehensive knowledge base
[ ] Deep investigation addresses specific questions and gaps
[ ] Comparative analysis provides clear option evaluation
[ ] Recommendations supported by evidence with confidence levels
[ ] Source quality meets domain-appropriate standards
[ ] Template compliance maintained throughout research
[ ] CCC framework components properly integrated
[ ] Domain-appropriate report structure used (NOT implementation for non-technical domains)
```

### **Quality Metrics**
- **Domain Adaptation**: Appropriate template and validation selection
- **Comparative Analysis**: Clear option comparison with decision support
- **Evidence Quality**: Source credibility meeting minimum standards
- **Recommendation Focus**: Actionable insights with supporting rationale
- **Framework Integration**: Proper CCC component utilization

---

**Command Version**: 2.0.0 | **Framework**: CCC Universal Research System
**Evidence Rating**: A1 (Modular framework with comprehensive domain coverage)
**Compliance**: CCC Agent Component System + Enhanced PRISMA + ISO 31000 Integration
**Architecture**: Universal Domain Research with Adaptive Component Loading

*Comprehensive research excellence through adaptive framework integration, focusing on understanding options and providing evidence-based recommendations across all knowledge domains.*