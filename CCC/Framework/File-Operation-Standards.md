# File Operation Standards Reference
*Systematic File Management Protocols for AI Assistant Operations*

---

## Path & Reference Requirements

### 📁 File Path Standards
**📋 Absolute Path Protocol:**
- [ ] All file operations use absolute paths only - no relative paths
- [ ] File references include line numbers where applicable: `/path/to/file:123`
- [ ] Include readable stubs for context: `[validation_criteria]`
- [ ] Document cross-references with related procedure IDs
- [ ] **MANDATORY**: Include actual current datetime in 'YYYY-MM-DD HH:MM:SS CST' format for all document creation and updates

### File Modification Protocol
**📋 Safe File Operations:**
- [ ] **Read-Before-Write**: Mandatory file content verification before modifications
- [ ] **Batch Optimization**: Parallel operations when dependencies allow
- [ ] **Transaction Safety**: Implement atomic-like operations for modifications
- [ ] **Format Compliance**: All documents follow parent template and style guide

---

## Documentation Synchronization

### 📝 Documentation Truth Protocol
**📋 Synchronization Requirements:**
- [ ] All documentation must reflect implementation reality
- [ ] Tracking documents verified to match current state
- [ ] Same-session updates mandatory for all documentation changes
- [ ] Verification before summary completion

---

**Version**: 1.0.0 | **Framework**: CCC File Operations | **Updated**: 2025-09-23