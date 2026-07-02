# Workflow Automation Operations Guide

## Purpose

This document defines operating guidance for workflow definitions, instances, tasks, decisions, status history, and notifications.

The goal is to make approval and task-flow behavior explainable across domains.

## Principle

Workflow records should explain who requested an action, who reviewed it, what decision was made, and how the source domain record should respond.

## Data Sources

Workflow operations may use:

- workflow definition
- workflow step
- workflow trigger/reference
- workflow instance
- approval task
- decision record
- status history
- notification trigger placeholder
- audit records

## Domain Reference Direction

Domain references should capture:

```text
domain_type
domain_record_id
organization_id
started_by
started_at
```

Domain references must not cross organization boundaries.

## Decision Direction

Every approval decision should capture:

```text
task_id
instance_id
decision
actor
reason optional
created_at
```

Decisions should not silently change source records without a clear application service boundary.

## Status History Direction

Status changes should capture:

```text
old_status
new_status
actor
reason optional
created_at
```

## Notification Direction

Workflow notifications may be triggered for:

- new task assigned
- decision recorded
- workflow approved
- workflow rejected
- reminder placeholder
- escalation placeholder

Use Notification Engine where practical.

## Duplicate Prevention

Before starting a workflow, check:

- domain reference
- workflow key
- existing active instance
- idempotency key where available

## Cancellation Direction

Cancellation should capture:

- actor
- reason
- timestamp
- impact on pending tasks

Pending tasks should not remain active after cancellation.

## Audit Direction

Audit should record:

- workflow definition changed
- workflow instance started
- task assigned
- decision recorded
- workflow cancelled
- reminder or escalation triggered where supported

## Data Quality Warnings

Warn or flag when:

- active instance has no active task
- task references missing instance
- instance references missing domain record
- source module is disabled
- task is assigned to inactive user

## Final Rule

Workflow operations must preserve decision history and must not hide approvals inside domain-specific status changes.
