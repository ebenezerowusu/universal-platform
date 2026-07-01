# Sprint 9 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 9 after the MVP release candidate is stable enough for controlled pilot testing.

Sprint 9 should support controlled pilot launch and feedback handling.

## Prompt

```text
You are implementing Sprint 9 for Universal Platform.

Your task is to prepare controlled pilot launch operations and feedback handling.

Required reading before coding or documentation updates:
1. CLAUDE.md
2. docs/14-roadmap/sprint-9-controlled-pilot-plan.md
3. docs/05-infrastructure/pilot-launch-runbook.md
4. docs/05-infrastructure/pilot-support-playbook.md
5. docs/16-quality/pilot-feedback-and-issue-triage.md
6. docs/17-implementation-notes/pilot-readiness-report-template.md
7. docs/17-implementation-notes/post-pilot-review-template.md
8. docs/16-quality/sprint-9-quality-gates.md
9. docs/14-roadmap/mvp-scope-lock.md
10. docs/14-roadmap/mvp-launch-readiness-checklist.md

Implement only:
- pilot launch checklist improvements
- pilot support workflow improvements
- feedback capture workflow improvements
- issue triage labels or templates if needed
- small fixes required for controlled pilot launch
- documentation updates required by pilot operations
- pilot reporting templates

Do not implement:
- new major modules
- new advanced Religious features
- QR/GPS attendance unless already approved outside this sprint
- full offline sync
- provider integrations unless approved for the pilot
- POS/Commerce/Finance/HR feature work
- broad refactors unrelated to pilot safety

Recommended branch:
pilot/controlled-launch-readiness

Recommended PR title:
pilot: prepare controlled launch readiness

Before finishing, run the relevant checks available in the project.

Final response must include:
- files changed
- pilot workflows added or updated
- operational risks reduced
- checks run
- known pilot limitations
- out-of-scope items avoided
- follow-up work
```

## Expected Output

The coding agent should produce a focused PR that prepares the MVP for controlled real-world pilot usage without expanding product scope.

## Final Rule

Sprint 9 is about pilot discipline. Keep the pilot controlled, observable, and honest about limitations.
