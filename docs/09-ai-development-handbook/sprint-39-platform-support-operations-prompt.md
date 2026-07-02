# Sprint 39 Platform Support Operations Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 39.

Sprint 39 prepares the Platform Support and Organization Operations foundation after monitoring, incidents, backup/restore, audit, reporting, workflow, imports, commerce, finance, HR, and religious modules have introduced operational support needs.

## Prompt

```text
You are implementing Sprint 39 for Universal Platform.

Your task is to implement the Platform Support and Organization Operations foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/05-infrastructure/incident-response-playbook.md
3. docs/05-infrastructure/pilot-support-playbook.md
4. docs/05-infrastructure/organization-onboarding-guide.md
5. docs/04-capability-engines/audit-engine.md
6. docs/04-capability-engines/notification-engine.md
7. docs/14-roadmap/sprint-39-platform-support-operations-roadmap.md
8. docs/13-module-sdk/platform-support-operations-readiness.md
9. docs/12-api/platform-support-operations-api-contract.md
10. docs/11-ui-ux/platform-support-operations-ui-contract.md
11. docs/05-infrastructure/platform-support-operations-guide.md
12. docs/16-quality/sprint-39-platform-support-operations-review.md

Implement only:
- platform support feature registration foundation
- support case metadata foundation
- support case timeline/update foundation
- organization health review placeholder where practical
- onboarding status tracker foundation
- escalation placeholder foundation
- support note foundation with safe visibility
- organization operational checklist placeholder where practical
- support notification placeholder where practical
- permission and feature checks
- audit records for sensitive support actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- direct tenant data browsing tools
- hidden support access to organization data
- remote control of user accounts
- external helpdesk provider integration
- live chat provider integration
- AI support agent automation
- cross-organization support visibility
- unrelated Commerce/POS expansion

Recommended branch:
feature/platform-support-operations-foundation

Recommended PR title:
feat: add platform support operations foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Support/Audit/Permission boundaries followed
- organization visibility rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused support operations foundation for support cases, timelines, onboarding status, organization health reviews, escalation placeholders, operational checklists, support notifications, and audit-safe support actions.

## Final Rule

Support operations must be organization-scoped, permission-protected, auditable, and privacy-preserving. Support users should see operational context without bypassing tenant boundaries or source-module permissions.
