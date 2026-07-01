# Event-Driven Architecture Guide

## Purpose

This document defines how Universal Platform should use events internally.

Events help domains and engines communicate without creating tight dependencies.

## Event Principle

Modules should publish events when important business or platform actions happen.

Other modules may react to those events without the publisher knowing who is listening.

## Why Events Matter

Events help the platform:

- Reduce direct module coupling
- Support background processing
- Support notifications
- Support reporting updates
- Support workflows
- Support future service extraction

## Event Examples

Platform events:

- UserCreated
- OrganizationCreated
- ModuleInstalled
- SubscriptionChanged
- PermissionAssigned

Engine events:

- PaymentCompleted
- PaymentFailed
- SmsSent
- SmsFailed
- FileUploaded
- WorkflowTaskAssigned
- ReportExportCompleted

Domain events:

- ReligiousMemberCreated
- ReligiousAttendanceMarked
- CommerceOrderCreated
- PosSaleCompleted
- HrLeaveRequested
- FinanceExpenseRecorded

## Event Structure

A standard event should include:

- event_id
- event_type
- occurred_at
- organization_id nullable
- actor_user_id nullable
- source
- correlation_id nullable
- payload

## Synchronous vs Asynchronous

Not every action needs asynchronous handling.

Use synchronous application calls when an immediate result is required.

Use events when other parts of the system should react independently.

## Event Flow Example

```text
Payment Engine verifies payment
  -> PaymentCompleted event published
  -> Notification Engine sends receipt
  -> Reporting Engine updates summaries
  -> Domain module updates related record
```

## Event Naming

Use past-tense names for completed facts.

Good:

```text
PaymentCompleted
MemberCreated
ModuleInstalled
```

Avoid command-style event names such as:

```text
SendSms
CreateMember
```

Commands request action. Events describe something that happened.

## Event Payload Rules

Payloads should include enough data for consumers to react, but should not expose secrets.

Do not include:

- Passwords
- Provider secrets
- Full confidential notes
- Raw payment credentials

Use IDs and safe metadata where possible.

## Reliability Direction

The MVP may start with an in-process event bus and Redis-backed jobs where needed.

As the platform grows, events may move to a more robust message broker.

The architecture should keep event publishers and consumers behind interfaces.

## Idempotency

Event consumers should be designed to handle duplicate event delivery where practical.

This is especially important for payments, SMS, notifications, imports, and reports.

## Testing Requirements

Tests must cover:

- Event is published after important action
- Consumer reacts correctly
- Unauthorized actions do not publish success events
- Duplicate events do not create duplicate critical effects where relevant

## Final Rule

Events describe facts that already happened. Use them to reduce coupling and prepare the platform for future scale.
