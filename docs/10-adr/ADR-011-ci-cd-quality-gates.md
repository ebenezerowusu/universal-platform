# ADR-011: Use CI/CD Quality Gates

## Status

Accepted

## Context

Universal Platform will be developed by humans and AI coding agents.

Without quality gates, implementation can drift away from the architecture and introduce regressions.

## Decision

Use CI/CD quality gates to validate important checks before merging implementation work.

Initial checks should include formatting, linting, tests, and migration validation where practical.

## Consequences

### Positive

- Better code quality
- Safer collaboration
- More reliable AI-generated work
- Early detection of regressions
- Stronger confidence before deployment

### Cost

- Requires CI setup effort
- Slower feedback when checks are heavy
- Requires discipline to keep checks meaningful

## Implementation Direction

Start with simple checks and grow over time.

Recommended early checks:

- Rust formatting
- Rust linting
- Rust tests
- Migration validation
- Documentation review through PR process

Future checks:

- Flutter analysis
- Security scans
- Dependency checks
- Container checks

## Final Rule

Quality gates are part of the platform architecture, not an afterthought.
