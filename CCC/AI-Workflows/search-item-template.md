# Search Item Template - ACS Component Structure
*Agent Component System Template for Search Task Definition*

**Last Updated**: 2025-09-25 | **Framework**: ACS Search Architecture

---

## Search Task Definition Framework

### **[SEARCH-###] Template Structure**
```markdown
# [SEARCH-###]: [Task Title]
*Created: [YYYY-MM-DD HH:MM:SS CST]*

## ACS Component Integration
**Behavioral**: [component-name] - [brief description]
**Procedural**: [component-name] - [methodology reference]
**Format**: [component-name] - [output structure reference]
**Validation**: [validation-tier] - [quality standards]

## Search Objective
[Clear statement of what this search task aims to accomplish]

## Scope Definition
**Included**:
- [Specific areas to research]
- [Information types to gather]
- [Source categories to prioritize]

**Excluded**:
- [Areas to avoid to prevent overlap]
- [Information types not relevant]
- [Out-of-scope considerations]

## Information Requirements
### **Primary Information**:
- [Key data points needed]
- [Essential facts to establish]
- [Critical comparisons required]

### **Secondary Information**:
- [Supporting details]
- [Context information]
- [Background research]

## Source Strategy
**Source Priority** (based on selected procedural component):
1. [High-priority source types]
2. [Medium-priority source types]
3. [Low-priority source types]

**Quality Standards**: [Minimum Admiralty Code rating]
**Bias Detection**: [Specific bias risks to watch for]

## Output Structure
**File Location**: [Specific path where results will be saved]
**Template Compliance**: [Which template structure to follow]
**Key Deliverables**:
- [Specific outputs required]
- [Data formats needed]
- [Integration requirements for compilation phase]

## Success Criteria
[ ] [Specific measurable outcomes]
[ ] [Quality thresholds met]
[ ] [Coverage requirements satisfied]
[ ] [Source quality standards achieved]

## Dependencies
**Prerequisite Searches**: [Previous SEARCH-### that must be completed first]
**Related Searches**: [Concurrent SEARCH-### that complement this task]
**Information Inputs**: [External data or context required]

## Integration Notes
**Compilation Phase**: [How results will be used in final document]
**Cross-References**: [Links to related search tasks]
**Quality Handoff**: [Specific information to pass to next phase]
```

---

## ACS Component Categories

### **Behavioral Components** (Research Intelligence Patterns)

#### **systematic-product-researcher**
```yaml
component_type: behavioral
description: "Methodical product evaluation with systematic comparison methodology"
decision_patterns:
  - comparative_analysis: "Evaluate products against clear criteria matrices"
  - evidence_prioritization: "Weight sources by reliability and independence"
  - bias_detection: "Identify and mitigate commercial and selection bias"
  - option_discovery: "Comprehensive market scanning for alternatives"
intelligence_focus:
  - "WHAT to buy/choose, not HOW to implement"
  - "Purchasing decisions with cost-benefit analysis"
  - "Market positioning and competitive advantages"
  - "User experience and practical considerations"
```

#### **systematic-researcher** (General)
```yaml
component_type: behavioral
description: "Methodical investigation with comprehensive analysis patterns"
decision_patterns:
  - information_hierarchy: "Establish source credibility and evidence quality"
  - gap_identification: "Systematic discovery of knowledge gaps"
  - cross_validation: "Independent source verification protocols"
  - synthesis_methodology: "Integration of findings across domains"
```

### **Procedural Components** (Research Methodologies)

#### **product-research-strategy**
```yaml
component_type: procedural
reference_template: "Templates/Search-Strategies/Product-Research-Strategy.md"
methodology:
  source_hierarchy:
    - "Independent testing organizations (A1-A2 rating)"
    - "Professional reviews (B1-B2 rating)"
    - "Aggregated user reviews (B2-B3 rating)"
    - "Manufacturer information (C2-C3 rating)"
  bias_detection:
    - "Affiliate marketing disclaimers"
    - "Single-source bias warnings"
    - "Commercial motivation assessment"
  research_process:
    - "Specification baseline establishment"
    - "Independent testing verification"
    - "Professional review compilation"
    - "User pattern analysis (not individual reviews)"
    - "Alternative comparison framework"
    - "Price verification across retailers"
```

### **Format Components** (Output Structure Templates)

#### **product-analysis-template**
```yaml
component_type: format
reference_template: "Templates/Documents/Product-Analysis-Template.md"
structure:
  - "Executive Summary (2-3 sentences)"
  - "Research Objective (clear evaluation statement)"
  - "Options Evaluated (product list with descriptions)"
  - "Comparison Analysis (feature tables, cost analysis, pros/cons)"
  - "Recommendations (Best Overall/Budget/Premium, what to avoid)"
  - "Key Factors to Consider"
  - "Source Quality Summary"
output_characteristics:
  - "Comparative matrices with weighted criteria"
  - "Clear purchasing recommendations"
  - "Cost-benefit analysis integration"
  - "Source credibility documentation"
```

### **Validation Components** (Quality Control Tiers)

#### **enhanced-PRISMA**
```yaml
component_type: validation
validation_tier: "Comprehensive (27-item)"
application: "Safety-critical products, medical devices, high-stakes purchases"
standards:
  - "A2+ source requirement for critical claims"
  - "Independent verification mandatory"
  - "Systematic bias assessment"
  - "Cross-validation across multiple domains"
```

#### **CCC-standard**
```yaml
component_type: validation
validation_tier: "Essential (10-item)"
application: "General product research, standard consumer items"
standards:
  - "B3+ source requirement minimum"
  - "Basic bias detection protocols"
  - "Template compliance verification"
  - "Source attribution documentation"
```

#### **minimal**
```yaml
component_type: validation
validation_tier: "Basic (5-item)"
application: "Low-stakes consumer items, budget purchases"
standards:
  - "C3+ source acceptable with caveats"
  - "Basic fact-checking protocols"
  - "Simplified validation checklist"
```

---

## Search Task Allocation Patterns

### **Non-Overlapping Search Strategy**
```
[SEARCH-001]: Market Foundation & Product Discovery
- Objective: Establish market landscape and identify primary options
- Focus: Product categories, market segments, major players
- Avoids: Detailed feature comparison (reserved for SEARCH-002)

[SEARCH-002]: Comparative Analysis & Deep Evaluation
- Objective: Detailed product comparison and evaluation
- Focus: Feature analysis, performance data, user experience
- Avoids: Basic market overview (covered in SEARCH-001)

[SEARCH-003]: Validation & Gap Resolution
- Objective: Verify findings and address information gaps
- Focus: Cross-validation, missing information, final verification
- Avoids: Duplicate research from previous searches
```

### **Scope Boundary Management**
Each search task includes explicit **Excluded** sections to prevent overlap:
- Previous search coverage (avoid duplication)
- Out-of-scope domains (maintain focus)
- Information types handled by other searches

---

## Integration with Command Structure

### **File Responsibility Chain**
```
1. Claude (Phase 1): Creates research-planning.md with ACS component selection
2. Agent (Phase 2): Uses search-item-template.md to create SEARCH-###.md files
3. Claude (Phase 3): Compiles agent outputs using format component template
```

### **ACS Component Selection Matrix**
```
User Query → Domain Analysis → Component Selection:

Product Research:
├── Behavioral: systematic-product-researcher
├── Procedural: product-research-strategy
├── Format: product-analysis-template
└── Validation: [enhanced-PRISMA|CCC-standard|minimal]
```

### **Quality Inheritance**
- Search tasks inherit validation tier from ACS selection
- Format components determine final compilation structure
- Procedural components guide research methodology
- Behavioral components shape information processing patterns

---

**Version**: 1.0.0 | **Architecture**: ACS Search Task Framework
**Evidence Rating**: A1 (Comprehensive component structure with clear integration)
**Compliance**: CCC Agent Component System + Template Integration
**Usage**: Command-driven research with modular component architecture

*Systematic search task definition through Agent Component System architecture enabling consistent, high-quality research execution across domains.*