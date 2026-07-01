# Sprint 1 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 1.

Use this prompt only after the documentation phase has been reviewed.

## Prompt

```text
You are implementing Sprint 1 for Universal Platform.

Your task is to initialize the backend scaffold only.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/02-architecture/02-platform-architecture.md
4. docs/06-development-standards/repository-structure-target.md
5. docs/06-development-standards/backend-bootstrap-guide.md
6. docs/06-development-standards/backend-scaffold-acceptance-criteria.md
7. docs/14-roadmap/sprint-1-issue-breakdown.md
8. docs/14-roadmap/sprint-1-pr-checklist.md
9. docs/09-ai-development-handbook/first-implementation-task.md

Implement only:
- backend workspace structure
- Rust workspace or crate structure
- Axum API shell
- runtime configuration foundation
- standard response model
- standard error model
- health endpoint
- first tests
- local setup notes if needed

Do not implement:
- Identity
- login
- organizations
- permissions
- audit
- module registry
- subscriptions
- Religious Domain
- payment integration
- SMS integration
- Flutter UI

Recommended branch:
chore/backend-scaffold

Recommended PR title:
chore: initialize backend scaffold

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- tests added
- checks run
- architecture rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a small PR focused only on backend foundation.

## Final Rule

Do not give the coding agent a broad product prompt for Sprint 1. Use this constrained prompt to prevent scope creep.
