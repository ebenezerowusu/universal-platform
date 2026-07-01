# Sprint 4 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 4 after Sprint 3 is merged.

Sprint 4 should implement Module Registry, Subscription/Plan foundation, and shared engine skeletons.

## Prompt

```text
You are implementing Sprint 4 for Universal Platform.

Your task is to implement the Module Registry foundation, Subscription/Plan foundation, and shared capability engine skeletons only.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/04-capability-engines/module-registry-engine.md
4. docs/04-capability-engines/subscription-engine.md
5. docs/04-capability-engines/provider-adapter-contracts.md
6. docs/04-capability-engines/configuration-engine.md
7. docs/04-capability-engines/feature-flag-license-engine.md
8. docs/13-module-sdk/module-manifest-specification.md
9. docs/13-module-sdk/module-lifecycle.md
10. docs/07-database/initial-migration-order.md
11. docs/14-roadmap/sprint-4-module-registry-tasks.md
12. docs/14-roadmap/sprint-4-plan-and-engine-tasks.md
13. docs/16-quality/sprint-4-quality-gates.md

Implement only:
- Module Registry migrations
- module catalog repository foundation
- organization module repository foundation
- module seed foundation
- module install/activate/deactivate use cases
- module availability check foundation
- Subscription/Plan migrations
- plan feature and limit repository foundation
- feature availability check foundation
- shared engine interface skeletons for Payment, SMS, Storage, Notification, Reporting, and Workflow
- local/mock engine implementations where useful
- tests for module availability and feature checks

Do not implement:
- Religious Domain features
- live payment provider integration
- live SMS provider integration
- full billing lifecycle
- full workflow builder
- Flutter UI
- advanced provider configuration UI

Recommended branch:
feature/module-subscription-engine-foundations

Recommended PR title:
feat: add module registry subscription and engine foundations

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

The coding agent should produce a focused PR that prepares modules, plan-based feature availability, and engine boundaries before Religious Domain implementation begins.

## Final Rule

Sprint 4 prepares domain enablement. Do not implement Religious Domain behavior or live providers in this sprint.
