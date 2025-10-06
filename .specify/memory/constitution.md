<!--
Sync Impact Report:
Version change: N/A (initial) → 1.0.0
Modified principles: N/A (initial version)
Added sections: All sections (initial constitution)
Removed sections: None
Templates requiring updates:
  ✅ .specify/templates/plan-template.md - verified alignment
  ✅ .specify/templates/spec-template.md - verified alignment
  ✅ .specify/templates/tasks-template.md - verified alignment
Follow-up TODOs: None
-->

# Rust Game Constitution

## Core Principles

### I. Code Quality First
All code MUST adhere to Rust best practices and pass automated quality gates:
- **Rustfmt Compliance**: Code MUST pass `cargo fmt --check` (non-negotiable)
- **Clippy Standards**: Code MUST pass `cargo clippy -- -D warnings` with zero warnings
- **Memory Safety**: Leverage Rust's ownership system; `unsafe` code requires explicit
  justification and documentation
- **Error Handling**: Use `Result<T, E>` and `Option<T>`; production code MUST NOT use
  `unwrap()` or `expect()` without documented justification
- **Type Safety**: Prefer newtype patterns over primitives (e.g., `PlayerId(u32)` not `u32`)
- **Documentation**: All public APIs MUST have rustdoc comments with examples

**Rationale**: Automated quality gates catch issues early and ensure consistent code style.
Memory safety prevents entire classes of bugs. Strong typing makes errors impossible to compile.

### II. Testing Discipline (NON-NEGOTIABLE)
Comprehensive testing MUST precede and validate all implementations:
- **Minimum Coverage**: 80% line coverage for business logic and game systems (measured,
  enforced)
- **Deterministic Tests**: All tests MUST be deterministic; flaky tests are blocked until fixed
- **Fast Execution**: Unit test suite MUST complete under 30 seconds
- **Test Quality**: Follow Arrange-Act-Assert pattern; test names MUST clearly describe
  behavior being tested
- **Integration Tests**: Critical game flows MUST have end-to-end integration tests
- **CI/CD Gates**: All tests MUST pass in CI before merge (non-negotiable)

**Rationale**: High test coverage prevents regressions. Deterministic tests enable reliable
development. Fast tests encourage frequent runs. Integration tests validate real-world behavior.

### III. User Experience Consistency
Player-facing systems MUST maintain consistent, responsive, and accessible experiences:
- **Input Responsiveness**: Input lag MUST NOT exceed 16ms (1 frame at 60 FPS)
- **Configurable Controls**: All key bindings MUST be user-configurable
- **Accessibility**: Support colorblind-friendly palettes and text scaling
- **Multi-Input Support**: Keyboard, mouse, and gamepad inputs MUST work consistently
- **UI Consistency**: Use unified UI framework and component library throughout
- **Feedback Systems**: Provide clear feedback for all user actions
- **Error Messages**: User-friendly error messages with actionable solutions

**Rationale**: Consistent UX builds player trust. Responsive controls are critical for gameplay
feel. Accessibility expands player base. Clear feedback reduces frustration.

### IV. Performance Requirements
Game MUST meet strict performance standards on target hardware:
- **Target Frame Rate**: Maintain 60 FPS on target hardware (non-negotiable)
- **Minimum Frame Rate**: MUST NEVER drop below 30 FPS during normal gameplay
- **Frame Time Consistency**: Frame time variance MUST NOT exceed 16.67ms
- **Memory Management**: Zero tolerance for memory leaks; use sanitizers in CI
- **Startup Time**: Game MUST start within 5 seconds on target hardware
- **Level Loading**: Level transitions MUST complete within 3 seconds
- **Performance Profiling**: Regular profiling required to identify bottlenecks

**Rationale**: Smooth frame rates are essential for player experience. Memory leaks degrade
performance over time. Fast loading maintains immersion. Profiling prevents performance debt.

### V. ECS Architecture Adherence
Follow Bevy's Entity-Component-System patterns consistently:
- **Single Responsibility**: Each system MUST have one clear purpose
- **Modular Design**: Game systems organized into logical modules (input, rendering, physics)
- **ECS Patterns**: Use Bevy ECS idioms consistently (queries, resources, events)
- **Resource Management**: Clear ownership patterns for game assets
- **System Ordering**: Explicit system ordering where execution order matters

**Rationale**: ECS architecture provides clear separation of concerns. Consistent patterns
improve maintainability. Proper resource management prevents asset-related bugs.

## Technical Standards

### Code Organization
- **Naming Conventions**: Follow Rust conventions (snake_case for variables/functions,
  PascalCase for types)
- **Maximum Line Length**: 100 characters per line for readability
- **Module Structure**: Logical grouping by feature/system, not technical layer
- **Dependency Management**: Audit dependencies for security and maintenance status

### Development Workflow
- **Version Control**: Conventional commit format required for clear change tracking
- **Branch Strategy**: Feature branches with descriptive names
- **Code Reviews**: All changes MUST be reviewed before merging to main
- **Semantic Versioning**: Use semantic versioning for all releases

## Testing Requirements

### Test Categories
- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test system interactions and game flows
- **Contract Tests**: Test public API boundaries remain stable
- **Property-Based Tests**: Use for mathematical operations (physics, animations)
- **Performance Tests**: Benchmark tests for critical performance paths

### Test Organization
- **Parallel Structure**: Tests organized in modules parallel to source code
- **Test Isolation**: Each test MUST be independent with no shared state
- **Test Data**: Keep test assets minimal and organized in dedicated directories
- **Cross-Platform**: Tests MUST pass on Windows, macOS, and Linux

## Quality Assurance

### Automated Checks
- **Pre-commit Hooks**: Tests and formatting checks before commits
- **CI/CD Pipeline**: Automated testing, linting, and building on all commits
- **Security Audits**: Regular dependency security audits via `cargo audit`
- **Performance Monitoring**: Track performance metrics across builds

### Manual Verification
- **Code Review**: Human review focusing on constitutional principles
- **Manual Testing**: Regular testing of critical user journeys
- **Usability Testing**: Periodic player feedback integration

## Governance

### Amendment Procedure
This constitution can be updated through:
1. **Proposal**: Any contributor proposes changes via pull request
2. **Discussion**: Open discussion period for review and feedback
3. **Consensus**: Changes require consensus from core development team
4. **Documentation**: All changes clearly documented with rationale and version increment

### Versioning Policy
Constitution version follows semantic versioning:
- **MAJOR**: Backward incompatible governance or principle removals/redefinitions
- **MINOR**: New principles added or materially expanded guidance
- **PATCH**: Clarifications, wording refinements, typo fixes

### Compliance Review
- **Pull Request Gate**: All PRs reviewed for constitutional compliance
- **Complexity Justification**: Deviations from principles MUST be explicitly justified
- **Regular Audits**: Periodic review of adherence to standards
- **Training**: Onboarding for new contributors on constitutional principles

### Technical Decision Guidance
When making technical decisions, evaluate against:
1. **Code Quality**: Does this maintain or improve code quality standards?
2. **Testing**: Can this be thoroughly tested? Does it add test debt?
3. **User Experience**: How does this affect player experience and consistency?
4. **Performance**: What are the performance implications? Are they acceptable?
5. **Architecture**: Does this align with ECS patterns and system boundaries?

Violations of core principles MUST be justified with:
- **Problem Statement**: What specific problem requires deviation?
- **Alternatives Considered**: Why simpler compliant approaches are insufficient
- **Mitigation Plan**: How to minimize impact and eventual alignment

**Version**: 1.0.0 | **Ratified**: 2025-10-03 | **Last Amended**: 2025-10-03
