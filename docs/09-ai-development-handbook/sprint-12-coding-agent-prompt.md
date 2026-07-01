# Sprint 12 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 12 after limited rollout has proven repeatable.

Sprint 12 should prepare the platform for broader launch readiness without expanding the MVP randomly.

## Prompt

```text
You are implementing Sprint 12 for Universal Platform.

Your task is to prepare launch-readiness operations after limited rollout has shown stable usage across multiple organizations.

Required reading before coding or documentation updates:
1. CLAUDE.md
2. docs/14-roadmap/sprint-12-launch-readiness-roadmap.md
3. docs/14-roadmap/public-launch-readiness-checklist.md
4. docs/16-quality/limited-rollout-metrics-and-success-criteria.md
5. docs/05-infrastructure/backup-restore-drill-playbook.md
6. docs/05-infrastructure/tenant-growth-operations-guide.md
7. docs/16-quality/sprint-12-readiness-checks.md
8. docs/05-infrastructure/production-smoke-test-checklist.md
9. docs/05-infrastructure/incident-response-playbook.md
10. docs/14-roadmap/wider-rollout-readiness-checklist.md

Implement only:
- launch-readiness documentation updates
- rollout metrics and reporting improvements
- backup and restore verification improvements
- organization growth operations improvements
- support and smoke test improvements
- small stability fixes needed for broader rollout
- test coverage for rollout-discovered gaps

Do not implement:
- broad new modules
- public marketing features
- unrelated product expansion
- advanced Religious features unless approved as launch-critical
- provider integrations unless approved for launch readiness
- large refactors unrelated to rollout evidence

Recommended branch:
launch/readiness-foundation

Recommended PR title:
launch: prepare broader rollout readiness

Before finishing, run the relevant checks available in the project.

Final response must include:
- files changed
- rollout issues addressed
- operations improvements added
- tests/checks run
- remaining launch risks
- accepted limitations
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce focused changes that help the platform move from limited rollout to launch readiness through evidence, operations, and stability.

## Final Rule

Sprint 12 is about launch discipline. Prove the system can support growth before inviting wider usage.
