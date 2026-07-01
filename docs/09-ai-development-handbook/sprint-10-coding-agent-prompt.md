# Sprint 10 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 10 after a controlled pilot has produced real feedback.

Sprint 10 should turn pilot learning into post-pilot hardening and wider rollout readiness.

## Prompt

```text
You are implementing Sprint 10 for Universal Platform.

Your task is to harden the MVP after the controlled pilot and prepare for a wider pilot or limited rollout.

Required reading before coding or documentation updates:
1. CLAUDE.md
2. docs/17-implementation-notes/post-pilot-review-template.md
3. docs/14-roadmap/sprint-10-post-pilot-hardening-plan.md
4. docs/16-quality/post-pilot-issue-prioritization-model.md
5. docs/05-infrastructure/data-correction-playbook.md
6. docs/05-infrastructure/pilot-pause-and-rollback-playbook.md
7. docs/14-roadmap/wider-rollout-readiness-checklist.md
8. docs/16-quality/sprint-10-readiness-checks.md
9. docs/14-roadmap/mvp-scope-lock.md
10. docs/14-roadmap/mvp-launch-readiness-checklist.md

Implement only:
- fixes accepted from post-pilot review
- workflow improvements required for wider pilot readiness
- data correction support improvements
- clearer error/empty/unavailable states
- support and operations improvements
- test coverage for pilot-discovered gaps
- documentation updates based on pilot learning
- small UX improvements that support core MVP workflows

Do not implement:
- broad new modules
- unrelated feature expansion
- advanced Religious features unless accepted as post-pilot must-fix
- full offline sync unless accepted as required for rollout
- provider integrations unless approved for rollout
- large refactors unrelated to post-pilot findings

Recommended branch:
harden/post-pilot-mvp

Recommended PR title:
harden: apply post-pilot MVP improvements

Before finishing, run the relevant checks available in the project.

Final response must include:
- files changed
- pilot issues addressed
- tests added or updated
- checks run
- remaining rollout risks
- accepted limitations
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce focused post-pilot improvements that strengthen the MVP before expanding usage.

## Final Rule

Sprint 10 should convert pilot evidence into focused hardening. Do not use it as a reason to expand the platform randomly.
