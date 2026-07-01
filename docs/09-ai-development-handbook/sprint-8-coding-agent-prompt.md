# Sprint 8 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 8 after the backend MVP and Flutter MVP workflows are implemented.

Sprint 8 should stabilize the MVP for a controlled pilot.

## Prompt

```text
You are implementing Sprint 8 for Universal Platform.

Your task is to stabilize the MVP for a controlled first pilot.

Required reading before coding:
1. CLAUDE.md
2. docs/14-roadmap/mvp-launch-readiness-checklist.md
3. docs/14-roadmap/sprint-8-mvp-stabilization-plan.md
4. docs/16-quality/mvp-end-to-end-test-plan.md
5. docs/16-quality/user-acceptance-testing-checklist.md
6. docs/05-infrastructure/mvp-release-candidate-runbook.md
7. docs/16-quality/sprint-8-quality-gates.md
8. docs/12-api/frontend-backend-integration-checklist.md
9. docs/05-infrastructure/production-readiness-checklist.md
10. docs/05-infrastructure/support-operations-guide.md

Implement only:
- MVP bug fixes discovered during end-to-end testing
- missing loading/empty/error states
- API contract mismatches between Flutter and backend
- seed data fixes needed for pilot demo
- basic smoke test improvements
- release-candidate documentation updates
- pilot-readiness notes
- small UX fixes that do not expand MVP scope

Do not implement:
- new major modules
- advanced Religious features
- QR/GPS attendance
- full offline sync
- live provider integrations unless explicitly approved
- POS/Commerce/Finance/HR feature work
- large refactors not required for pilot stability

Recommended branch:
stabilize/mvp-release-candidate

Recommended PR title:
stabilize: prepare MVP release candidate

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- bugs fixed
- tests added or updated
- checks run
- pilot risks remaining
- architecture rules followed
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce a stabilization PR that improves readiness without expanding product scope.

## Final Rule

Sprint 8 is for stabilization, not feature ambition. Fix what blocks pilot confidence and defer what is not required for the first controlled pilot.
