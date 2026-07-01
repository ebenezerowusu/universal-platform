# Sprint 3 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 3 after Sprint 2 is merged.

Sprint 3 should implement Permission and Audit foundations only.

## Prompt

```text
You are implementing Sprint 3 for Universal Platform.

Your task is to implement the Permission Engine foundation and Audit Engine foundation only.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/04-capability-engines/permission-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/04-capability-engines/initial-permission-catalog.md
6. docs/04-capability-engines/audit-event-catalog.md
7. docs/06-development-standards/application-service-pattern.md
8. docs/06-development-standards/repository-pattern.md
9. docs/07-database/initial-migration-order.md
10. docs/14-roadmap/sprint-3-issue-breakdown.md
11. docs/06-development-standards/permission-audit-acceptance-criteria.md
12. docs/16-quality/sprint-3-quality-gates.md

Implement only:
- Permission migrations
- role and permission repository foundation
- permission seed foundation
- role assignment foundation
- permission check service foundation
- Audit migration
- audit repository foundation
- audit service foundation
- selected tests for permission allow/deny behavior
- selected tests for audit write behavior

Do not implement:
- Module Registry
- Subscription Engine
- Religious Domain
- payment integration
- SMS integration
- Flutter UI
- advanced permission scopes beyond the documented foundation

Recommended branch:
feature/permission-audit-foundation

Recommended PR title:
feat: add permission and audit foundations

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

The coding agent should produce a focused PR that establishes authorization and accountability foundations without implementing modules, subscriptions, or domain behavior.

## Final Rule

Sprint 3 proves what users can do and what important actions are recorded. Do not mix module or domain behavior into this sprint.
