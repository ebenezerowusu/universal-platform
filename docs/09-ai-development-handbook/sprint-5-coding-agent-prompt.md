# Sprint 5 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 5 after Sprint 4 is merged.

Sprint 5 should implement the Religious MVP foundation only.

## Prompt

```text
You are implementing Sprint 5 for Universal Platform.

Your task is to implement the Religious MVP foundation only.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/03-domains/religious-domain.md
4. docs/03-domains/religious-mvp-build-sequence.md
5. docs/03-domains/religious-member-management.md
6. docs/03-domains/religious-visitors-and-follow-up.md
7. docs/03-domains/religious-congregations-services-groups.md
8. docs/03-domains/religious-attendance-module.md
9. docs/07-database/religious-domain-data-model.md
10. docs/07-database/religious-mvp-migration-plan.md
11. docs/12-api/religious-mvp-api-contracts.md
12. docs/14-roadmap/sprint-5-issue-breakdown.md
13. docs/06-development-standards/religious-mvp-acceptance-criteria.md
14. docs/16-quality/sprint-5-quality-gates.md

Implement only:
- Religious module registration checks
- Religious settings foundation
- Religious member foundation
- Religious visitor foundation
- Religious congregations, services, and groups foundation
- Religious manual attendance foundation
- basic Religious report foundation
- permission/module/feature checks through existing engines
- audit writes for important changes where foundation exists
- tests for core Religious MVP flows

Do not implement:
- QR attendance
- GPS attendance
- live SMS provider integration
- live payment provider integration
- full giving module
- full family/household module
- full care/counseling/welfare module
- advanced branch governance
- Flutter UI
- offline sync

Recommended branch:
feature/religious-mvp-foundation

Recommended PR title:
feat: add religious MVP foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- seed data added
- tests added
- checks run
- architecture rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused PR that proves a real domain can be built on top of Platform Core, Module Registry, Permission Engine, Audit Engine, and shared engine boundaries.

## Final Rule

Sprint 5 proves the platform architecture through the first real domain. Do not bypass Core, Engines, modules, permissions, or audit to make the domain work faster.
