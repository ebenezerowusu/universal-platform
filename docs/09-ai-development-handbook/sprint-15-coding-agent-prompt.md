# Sprint 15 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 15 after post-launch stabilization has been reviewed.

Sprint 15 should prepare the next feature wave using evidence from launch, support, adoption metrics, and platform strategy.

## Prompt

```text
You are implementing Sprint 15 for Universal Platform.

Your task is to prepare the next MVP feature wave without breaking platform discipline.

Required reading before coding or documentation updates:
1. CLAUDE.md
2. docs/14-roadmap/sprint-15-next-feature-wave-roadmap.md
3. docs/14-roadmap/next-feature-wave-selection-framework.md
4. docs/13-module-sdk/feature-wave-module-readiness-template.md
5. docs/14-roadmap/revenue-readiness-roadmap.md
6. docs/16-quality/sprint-15-feature-wave-checks.md
7. docs/16-quality/post-launch-adoption-metrics.md
8. docs/17-implementation-notes/post-launch-review-template.md
9. docs/01-platform-constitution/01-platform-constitution.md
10. docs/13-module-sdk/module-manifest-specification.md

Implement only:
- evidence-backed feature wave planning
- module-readiness documentation
- feature selection records
- revenue readiness direction
- architecture guardrail updates where needed
- small enabling fixes needed before the next feature wave
- tests or checks for platform extension points where practical

Do not implement:
- a new module without feature-wave approval
- random feature requests without evidence
- domain logic inside Platform Core
- provider-specific logic outside adapters
- broad refactors unrelated to the selected wave
- monetization behavior that bypasses subscription or feature foundations

Recommended branch:
planning/next-feature-wave

Recommended PR title:
planning: prepare next MVP feature wave

Before finishing, run the relevant checks available in the project.

Final response must include:
- files changed
- feature wave candidates reviewed
- selected or recommended next wave
- architecture risks identified
- revenue considerations identified
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce a focused planning PR that prepares the next approved feature wave using evidence and platform principles.

## Final Rule

Sprint 15 decides what to build next. It should not start building everything.
