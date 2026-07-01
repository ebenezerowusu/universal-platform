# Workflow Engine Specification

## Purpose

The Workflow Engine provides configurable process flows, approvals, tasks, reminders, and automation across domains.

It helps the platform avoid hardcoding process rules inside domain modules.

## Responsibilities

The Workflow Engine is responsible for:

- Workflow definitions
- Workflow instances
- Approval steps
- Task assignment
- Due dates
- Reminders
- Status transitions
- Workflow events
- Workflow audit support

## Non-Responsibilities

The Workflow Engine does not own domain business records.

A visitor belongs to the Religious Domain. A leave request belongs to the HR Domain. A payment belongs to the Payment Engine.

The Workflow Engine manages the process around those records.

## Example Use Cases

Religious Domain:

- Visitor follow-up
- Branch connection approval
- Branch disconnect approval
- Counseling assignment
- Welfare request review
- New member onboarding

HR Domain:

- Leave approval
- Staff onboarding

Finance Domain:

- Expense approval
- Budget approval

Commerce Domain:

- Order fulfillment steps
- Return review

## Workflow Structure

A workflow may include:

- Trigger
- Steps
- Assigned role or user
- Due date rules
- Conditions
- Actions
- Notifications
- Completion criteria

## Workflow Instance Flow

```text
Trigger occurs
  -> Workflow Engine starts workflow instance
  -> First task assigned
  -> Notification sent
  -> User completes task
  -> Next step evaluated
  -> Workflow reaches final state
  -> WorkflowCompleted event published
```

## Suggested Entities

Candidate entities:

- workflow_definitions
- workflow_steps
- workflow_instances
- workflow_tasks
- workflow_task_comments
- workflow_transitions
- workflow_events

## Task Statuses

Possible statuses:

- pending
- in_progress
- approved
- returned
- completed
- cancelled
- overdue

## Workflow Statuses

Possible statuses:

- draft
- active
- paused
- completed
- cancelled

## Configuration

Workflows should eventually be configurable per organization.

Examples:

- require approval for member registration
- assign visitor follow-up to group leader
- require parent approval for branch disconnect
- require finance officer approval for expenses above threshold

## Events Published

- WorkflowStarted
- WorkflowTaskAssigned
- WorkflowTaskCompleted
- WorkflowTaskReturned
- WorkflowCompleted
- WorkflowCancelled
- WorkflowOverdue

## Events Consumed

Possible consumed events:

- VisitorRegistered
- MemberCreated
- BranchConnectionRequested
- PaymentCompleted
- WelfareRequestCreated
- LeaveRequestSubmitted

## Security Requirements

- Only authorized users can act on workflow tasks.
- Task visibility must respect organization and scope.
- Approval actions must be audited.
- Confidential workflows require extra permission checks.

## Tenant Isolation

Workflow definitions and instances must be scoped to the organization unless platform-level.

Organization users must not view or act on another organization's workflows.

## Testing Requirements

Tests must cover:

- Workflow starts from trigger
- Task assigned correctly
- Unauthorized user cannot complete task
- Approved path works
- Returned path works
- Tenant isolation
- Notification event emitted
- Audit log created for approval action

## Final Rule

Domains own business meaning. The Workflow Engine owns configurable process execution.
