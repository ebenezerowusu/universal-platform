# Sprint 29 Calendar Scheduling Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 29.

Sprint 29 prepares the Organization Calendar and Scheduling foundation after staff attendance, leave, events, fulfillment, and operations foundations.

## Prompt

```text
You are implementing Sprint 29 for Universal Platform.

Your task is to implement the Organization Calendar and Scheduling foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/workflow-engine.md
3. docs/04-capability-engines/notification-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/14-roadmap/sprint-29-calendar-scheduling-roadmap.md
6. docs/13-module-sdk/calendar-scheduling-readiness.md
7. docs/12-api/calendar-scheduling-api-contract.md
8. docs/11-ui-ux/calendar-scheduling-ui-contract.md
9. docs/05-infrastructure/calendar-scheduling-governance-playbook.md
10. docs/16-quality/sprint-29-calendar-scheduling-review.md

Implement only:
- calendar feature registration foundation
- organization calendar foundation
- scheduled item/event foundation
- recurring schedule placeholder where practical
- participant/resource placeholder foundation
- reminder/notification trigger foundation where practical
- conflict detection placeholder where practical
- domain link/reference foundation for events, attendance, HR, commerce, and religious workflows
- permission and feature checks
- audit records for important schedule actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full external calendar sync
- Google Calendar or Outlook integration
- complex recurrence engine beyond placeholder rules
- resource booking marketplace
- public ticketing
- advanced appointment automation
- payroll scheduling
- unrelated Commerce/POS expansion

Recommended branch:
feature/calendar-scheduling-foundation

Recommended PR title:
feat: add calendar scheduling foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- scheduling/domain boundaries followed
- notification and audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused organization calendar and scheduling foundation that can be reused by Religious events, HR attendance, leave, fulfillment, commerce, and future modules.

## Final Rule

Scheduling must be reusable, organization-scoped, timezone-aware, permission-protected, and auditable. Domain modules should link to schedule records instead of inventing separate calendars.
