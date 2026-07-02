# Sprint 34 Workflow Automation Review

## Purpose

This document defines review checks for Sprint 34.

Sprint 34 implements Workflow Automation and Approval foundation.

## Review 1: Scope

Sprint 34 may include:

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
- API and UI foundations

Sprint 34 should not include:

- full no-code automation builder
- BPMN engine
- arbitrary script execution
- external automation provider integration
- AI workflow generation
- unrestricted auto-approval rules
- provider-specific notification calls inside domain modules
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Workflow Foundation owns definitions, instances, tasks, and decisions
- domain modules own source records and request workflow decisions through Workflow Engine
- Permission Engine controls access decisions
- Notification Engine handles workflow notifications where used
- Platform Core does not contain domain-specific workflow rules

## Review 3: Organization Boundaries

Confirm:

- workflow definitions are organization-aware or platform-defined safely
- workflow instances are organization-scoped
- task records are organization-scoped
- domain references cannot cross organization boundaries

## Review 4: Decision Rules

Confirm:

- decisions require permission
- task assignment or admin permission is checked
- invalid status transitions are rejected
- decision notes are stored where supplied
- decisions create status history and audit records

## Review 5: Source Domain Rules

Confirm:

- source-record details are hidden unless user has source access
- source domain unavailable state is handled safely
- workflow status does not silently diverge from source status
- duplicate active instances are prevented where required

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/workflow-automation-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/task-flow-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- workflow definition retrieval
- workflow instance creation
- duplicate active instance prevention
- task inbox filtering
- approve decision
- reject decision
- invalid transition rejection
- organization boundary enforcement
- permission denied behavior
- status history creation

## Final Rule

Sprint 34 is complete when organizations can start workflow instances, assign tasks, record decisions, and preserve auditable decision history through reusable workflow APIs and UI foundations.
