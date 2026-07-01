# Sprint 11 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 11 after post-pilot hardening is complete and the platform is ready for a wider pilot or limited rollout.

Sprint 11 should prepare controlled multi-organization rollout operations.

## Prompt

```text
You are implementing Sprint 11 for Universal Platform.

Your task is to prepare limited rollout operations for multiple organizations while keeping the MVP stable and supportable.

Required reading before coding or documentation updates:
1. CLAUDE.md
2. docs/14-roadmap/wider-rollout-readiness-checklist.md
3. docs/14-roadmap/sprint-11-limited-rollout-plan.md
4. docs/05-infrastructure/limited-rollout-runbook.md
5. docs/05-infrastructure/incident-response-playbook.md
6. docs/16-quality/production-smoke-test-checklist.md
7. docs/05-infrastructure/customer-onboarding-playbook.md
8. docs/16-quality/sprint-11-readiness-checks.md
9. docs/05-infrastructure/support-operations-guide.md
10. docs/05-infrastructure/production-readiness-checklist.md

Implement only:
- limited rollout setup improvements
- repeatable organization onboarding improvements
- operational smoke test improvements
- support and incident workflow improvements
- rollout reporting templates
- small stability fixes required for multiple pilot organizations
- documentation updates required by rollout operations

Do not implement:
- broad new modules
- unrelated product expansion
- advanced Religious features unless approved as rollout-critical
- provider integrations unless approved for limited rollout
- large refactors unrelated to rollout stability
- public launch marketing features

Recommended branch:
rollout/limited-organization-readiness

Recommended PR title:
rollout: prepare limited organization rollout

Before finishing, run the relevant checks available in the project.

Final response must include:
- files changed
- rollout workflows added or updated
- operational risks reduced
- tests/checks run
- known rollout limitations
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce focused changes that prepare the platform for multiple pilot organizations without expanding beyond the approved MVP.

## Final Rule

Sprint 11 is about repeatability and operational control. Make rollout safer before making it bigger.
