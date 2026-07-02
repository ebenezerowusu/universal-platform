# Workflow Automation API Contract

## Purpose

This document defines the MVP API contract for Workflow Automation and Approval foundation.

The API should support workflow definitions, workflow instances, task inbox, decision actions, status history, reminders/escalation placeholders, and domain trigger references while preserving organization boundaries.

## Owner

```text
Workflow Foundation
Permission Engine
Notification Engine where workflow notifications are used
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/workflows
```

## Workflow Definitions

```text
GET /api/v1/organizations/{organizationId}/workflows/definitions
POST /api/v1/organizations/{organizationId}/workflows/definitions
GET /api/v1/organizations/{organizationId}/workflows/definitions/{workflowKey}
PATCH /api/v1/organizations/{organizationId}/workflows/definitions/{workflowKey}
```

Admin/maintenance only for create/update.

## Workflow Steps

```text
GET /api/v1/organizations/{organizationId}/workflows/definitions/{workflowKey}/steps
POST /api/v1/organizations/{organizationId}/workflows/definitions/{workflowKey}/steps
PATCH /api/v1/organizations/{organizationId}/workflows/definitions/{workflowKey}/steps/{stepId}
```

## Start Workflow Instance

```text
POST /api/v1/organizations/{organizationId}/workflows/instances
```

### Request Direction

```json
{
  "workflowKey": "expense_approval",
  "domainReference": {
    "type": "expense_request",
    "id": "..."
  },
  "summary": "Expense request approval",
  "notes": "Optional note"
}
```

## Workflow Instances

```text
GET /api/v1/organizations/{organizationId}/workflows/instances
GET /api/v1/organizations/{organizationId}/workflows/instances/{instanceId}
POST /api/v1/organizations/{organizationId}/workflows/instances/{instanceId}/cancel
```

## Task Inbox

```text
GET /api/v1/organizations/{organizationId}/workflows/tasks
GET /api/v1/organizations/{organizationId}/workflows/tasks/{taskId}
```

### Query Parameters

```text
status
workflowKey
domainType
assignedToMe
page
perPage
```

## Decision Actions

```text
POST /api/v1/organizations/{organizationId}/workflows/tasks/{taskId}/approve
POST /api/v1/organizations/{organizationId}/workflows/tasks/{taskId}/reject
POST /api/v1/organizations/{organizationId}/workflows/tasks/{taskId}/return
```

### Request Direction

```json
{
  "decisionNote": "Approved",
  "metadata": {
    "reason": "optional"
  }
}
```

## Status History

```text
GET /api/v1/organizations/{organizationId}/workflows/instances/{instanceId}/history
```

## Reminder/Escalation Placeholder

```text
POST /api/v1/organizations/{organizationId}/workflows/tasks/{taskId}/reminder-placeholder
POST /api/v1/organizations/{organizationId}/workflows/tasks/{taskId}/escalation-placeholder
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- workflow feature availability
- required permission
- task assignment or admin permission
- source domain access where domain reference is shown

Domain trigger references must not cross organization boundaries.

## Status Direction

Suggested workflow instance statuses:

```text
started
pending
approved
rejected
returned_placeholder
cancelled
expired_placeholder
manual_review
```

Suggested task statuses:

```text
pending
approved
rejected
returned_placeholder
cancelled
expired_placeholder
```

## Result Safety Direction

Workflow summaries and task labels must be safe.

Do not expose restricted source-record details unless the user has source access.

## Audit Direction

Audit should be written for:

- workflow definition created or updated
- workflow instance started
- task assigned
- decision recorded
- workflow cancelled
- status changed
- reminder/escalation triggered where supported

## Error Direction

Use standard response shape.

Expected error areas:

- workflow definition not found
- workflow instance not found
- task not found
- invalid decision action
- invalid status transition
- source domain unavailable
- permission denied
- organization not found

## Final Rule

Workflow APIs must preserve organization scope, permission checks, source-domain boundaries, and auditable decision history.
