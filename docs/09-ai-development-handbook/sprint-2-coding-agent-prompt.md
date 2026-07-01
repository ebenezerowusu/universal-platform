# Sprint 2 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 2 after Sprint 1 is merged.

Sprint 2 should implement Identity and Organization foundations only.

## Prompt

```text
You are implementing Sprint 2 for Universal Platform.

Your task is to implement the Identity and Organization foundations only.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/04-capability-engines/identity-engine.md
4. docs/04-capability-engines/organization-engine.md
5. docs/06-development-standards/application-service-pattern.md
6. docs/06-development-standards/repository-pattern.md
7. docs/06-development-standards/rust-error-handling-standards.md
8. docs/07-database/initial-migration-order.md
9. docs/14-roadmap/sprint-2-issue-breakdown.md
10. docs/06-development-standards/identity-organization-acceptance-criteria.md

Implement only:
- Identity migrations
- user repository foundation
- credential/session foundation
- login use case foundation
- current user endpoint
- Organization migrations
- organization repository foundation
- create organization use case
- list current user organizations use case
- get organization detail use case
- first Identity and Organization tests

Do not implement:
- Permission Engine
- Audit Engine
- Module Registry
- Subscription Engine
- Religious Domain
- payment integration
- SMS integration
- Flutter UI

Recommended branch:
feature/identity-organization-foundation

Recommended PR title:
feat: add identity and organization foundations

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- tests added
- checks run
- architecture rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused PR that establishes user identity and organization context without implementing full authorization or domain behavior.

## Final Rule

Sprint 2 proves who the user is and where they operate. Do not mix authorization or domain features into this sprint.
