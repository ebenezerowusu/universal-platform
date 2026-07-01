# Sprint 13 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 13 after broader launch readiness has been approved.

Sprint 13 should prepare launch-day execution and immediate post-launch operations.

## Prompt

```text
You are implementing Sprint 13 for Universal Platform.

Your task is to prepare launch-day execution and immediate post-launch operations.

Required reading before coding or documentation updates:
1. CLAUDE.md
2. docs/14-roadmap/public-launch-readiness-checklist.md
3. docs/14-roadmap/sprint-13-launch-operations-roadmap.md
4. docs/05-infrastructure/public-launch-runbook.md
5. docs/05-infrastructure/post-launch-monitoring-playbook.md
6. docs/16-quality/launch-day-smoke-test-checklist.md
7. docs/17-implementation-notes/launch-decision-record-template.md
8. docs/16-quality/sprint-13-launch-checks.md
9. docs/05-infrastructure/incident-response-playbook.md
10. docs/05-infrastructure/tenant-growth-operations-guide.md

Implement only:
- launch-day runbook updates
- launch smoke test improvements
- post-launch monitoring improvements
- support readiness updates
- launch decision records
- rollout reporting updates
- small stability fixes required for launch-day confidence
- documentation updates required by launch operations

Do not implement:
- broad new modules
- unrelated product expansion
- public marketing features inside product code
- advanced Religious features unless approved as launch-critical
- provider integrations unless approved for launch
- large refactors unrelated to launch safety

Recommended branch:
launch/day-operations-readiness

Recommended PR title:
launch: prepare launch-day operations

Before finishing, run the relevant checks available in the project.

Final response must include:
- files changed
- launch workflows added or updated
- operational risks reduced
- tests/checks run
- known launch limitations
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce focused changes that support launch-day execution and immediate post-launch confidence.

## Final Rule

Sprint 13 is about operational launch control. Do not use launch preparation as an excuse to add new features.
