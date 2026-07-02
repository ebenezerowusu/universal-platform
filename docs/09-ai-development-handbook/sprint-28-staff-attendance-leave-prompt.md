# Sprint 28 Staff Attendance and Leave Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 28.

Sprint 28 prepares staff attendance, check-in/out, and leave request foundations after the HR workforce foundation.

## Prompt

```text
You are implementing Sprint 28 for Universal Platform.

Your task is to implement the Staff Attendance and Leave foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/hr-domain.md
3. docs/04-capability-engines/workflow-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/14-roadmap/sprint-28-staff-attendance-leave-roadmap.md
6. docs/13-module-sdk/staff-attendance-leave-readiness.md
7. docs/12-api/staff-attendance-leave-api-contract.md
8. docs/11-ui-ux/staff-attendance-leave-ui-contract.md
9. docs/05-infrastructure/staff-attendance-leave-governance-playbook.md
10. docs/16-quality/sprint-28-staff-attendance-leave-review.md
11. docs/12-api/workforce-api-contract.md

Implement only:
- staff attendance feature registration foundation
- work schedule/session foundation where practical
- check-in/check-out record foundation
- manual attendance adjustment foundation
- leave request placeholder foundation
- leave approval placeholder where practical
- attendance summary foundation
- permission and feature checks
- audit records for important attendance/leave actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- payroll
- overtime payment calculation
- biometric attendance
- facial recognition
- complex shift planning
- full leave entitlement engine
- tax or benefits features
- direct payment provider calls inside HR Domain
- unrelated Commerce/POS expansion

Recommended branch:
feature/staff-attendance-leave-foundation

Recommended PR title:
feat: add staff attendance and leave foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Attendance/Workflow boundaries followed
- privacy and audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused attendance and leave foundation linked to workforce profiles without becoming payroll, biometrics, or advanced HR automation.

## Final Rule

Staff attendance and leave records must be organization-scoped, permission-protected, auditable, and decoupled from payroll until payroll is explicitly planned.
