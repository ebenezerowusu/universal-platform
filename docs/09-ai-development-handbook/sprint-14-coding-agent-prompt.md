# Sprint 14 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 14 after the launch window has completed.

Sprint 14 should focus on post-launch stabilization, adoption learning, and the next product decision.

## Prompt

```text
You are implementing Sprint 14 for Universal Platform.

Your task is to stabilize the platform after launch and convert early usage evidence into clear next actions.

Required reading before coding or documentation updates:
1. CLAUDE.md
2. docs/05-infrastructure/post-launch-monitoring-playbook.md
3. docs/14-roadmap/sprint-14-post-launch-stabilization-roadmap.md
4. docs/16-quality/post-launch-adoption-metrics.md
5. docs/05-infrastructure/post-launch-support-review-guide.md
6. docs/17-implementation-notes/post-launch-review-template.md
7. docs/16-quality/sprint-14-stabilization-checks.md
8. docs/14-roadmap/public-launch-readiness-checklist.md
9. docs/16-quality/limited-rollout-metrics-and-success-criteria.md
10. docs/05-infrastructure/incident-response-playbook.md

Implement only:
- post-launch bug fixes based on real evidence
- support process improvements
- monitoring and reporting improvements
- adoption metric reporting improvements
- documentation updates from launch learning
- small UX fixes that improve core MVP workflows
- test coverage for launch-discovered gaps

Do not implement:
- broad new modules
- unrelated product expansion
- major new Religious features unless approved as urgent
- provider integrations unless approved as launch follow-up
- large refactors without launch evidence

Recommended branch:
stabilize/post-launch-mvp

Recommended PR title:
stabilize: apply post-launch MVP improvements

Before finishing, run the relevant checks available in the project.

Final response must include:
- files changed
- launch issues addressed
- support improvements added
- metrics/reporting improvements added
- tests/checks run
- remaining risks
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce focused post-launch improvements based on actual usage, support, and workflow evidence.

## Final Rule

Sprint 14 should learn from launch before expanding features. Stabilize, measure, then decide.
