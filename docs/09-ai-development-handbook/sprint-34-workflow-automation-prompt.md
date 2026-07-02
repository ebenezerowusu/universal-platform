# Sprint 34 Workflow Automation Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 34.

Sprint 34 prepares the Workflow Automation and Approval foundation after reporting, notifications, scheduling, HR, finance, commerce, religious, and operations modules have introduced many approval and task-flow needs.

## Prompt

```text
You are implementing Sprint 34 for Universal Platform.

Your task is to implement the Workflow Automation and Approval foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/workflow-engine.md
3. docs/04-capability-engines/notification-engine.md
4. docs/04-capability-engines/permission-engine.md
5. docs/04-capability-engines/audit-engine.md
6. docs/14-roadmap/sprint-34-workflow-automation-roadmap.md
7. docs/13-module-sdk/workflow-automation-readiness.md
8. docs/12-api/workflow-automation-api-contract.md
9. docs/11-ui-ux/workflow-automation-ui-contract.md
10. docs/05-infrastructure/workflow-automation-operations-guide.md
11. docs/16-quality/sprint-34-workflow-automation-review.md

Implement only:
- workflow feature registration foundation
- workflow definition metadata foundation
- workflow trigger/reference foundation
- approval step foundation
- workflow instance foundation
- task/approval inbox foundation
- status transition history foundation
- escalation/reminder placeholder where practical
- notification trigger placeholder where practical
- permission and feature checks
- audit records for workflow decisions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full no-code automation builder
- BPMN engine
- arbitrary script execution
- external automation provider integration
- AI workflow generation
- unrestricted auto-approval rules
- provider-specific notification calls inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/workflow-automation-foundation

Recommended PR title:
feat: add workflow automation foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Workflow/Permission/Notification boundaries followed
- approval and audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused workflow foundation for definitions, triggers, approval steps, workflow instances, task inbox, status history, reminders, and audit-safe decisions.

## Final Rule

Workflow must be organization-scoped, permission-aware, auditable, and reusable across domains. Domain modules should request workflow decisions through Workflow Engine instead of implementing isolated approval logic.
