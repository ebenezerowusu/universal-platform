# Sprint 34 Workflow Automation Roadmap

## Purpose

This document defines Sprint 34 for Workflow Automation and Approval foundation.

Sprint 34 should create a reusable workflow layer for approvals, task queues, domain-triggered decisions, reminders, escalation placeholders, and auditable status transitions.

## Sprint Goal

Enable organizations to define workflow metadata, start workflow instances from domain records, assign approval tasks, record decisions, track status history, and trigger safe notifications without building a full no-code automation builder too early.

## Why This Sprint

Many platform modules need controlled workflows:

- expense approvals
- leave approvals
- branch connection/disconnection approvals
- report export approvals later
- document access approvals later
- order cancellation/manual review
- delivery exception handling
- religious care/follow-up tasks
- finance reconciliation review

Without a shared workflow foundation, each module will build isolated approval behavior.

## Work Package 1: Workflow Definitions

Prepare workflow definition metadata with:

- workflow key
- title
- domain/module
- description optional
- status
- required permission

## Work Package 2: Workflow Triggers

Prepare trigger/domain references such as:

```text
expense_request
leave_request
branch_relation_request
order_manual_review
delivery_exception
finance_reconciliation
report_export_request
document_access_request
custom
```

## Work Package 3: Approval Steps

Prepare approval step foundation with:

- step order
- approver role/permission placeholder
- decision type
- status
- timeout/escalation placeholder

## Work Package 4: Workflow Instances

Prepare workflow instances linked to domain records.

Instances should track:

- workflow definition
- domain reference
- current status
- current step
- started by
- started at

## Work Package 5: Task and Approval Inbox

Prepare task records for assigned reviewers.

Tasks should support:

- pending
- approved
- rejected
- returned_placeholder
- cancelled
- expired_placeholder

## Work Package 6: Status Transition History

Every decision should create history.

Track:

- old status
- new status
- actor
- reason/note optional
- timestamp

## Work Package 7: Notifications and Escalation Placeholder

Prepare hooks for:

- new approval task
- approval completed
- workflow rejected
- reminder placeholder
- escalation placeholder

Use Notification Engine where practical.

## Work Package 8: Flutter Foundation

Prepare screens or placeholders for:

- workflow dashboard
- approval/task inbox
- workflow detail
- decision action
- status history
- workflow definition admin placeholder

## Out of Scope

Do not implement:

- full no-code automation builder
- BPMN engine
- arbitrary script execution
- external automation provider integration
- AI workflow generation
- unrestricted auto-approval rules

## Completion Standard

Sprint 34 is complete when organizations can define workflow metadata, start workflow instances, assign approval tasks, record decisions, and maintain auditable status history through reusable workflow APIs and UI foundations.

## Final Rule

Workflow must be the shared approval and task-flow layer for domain modules.
