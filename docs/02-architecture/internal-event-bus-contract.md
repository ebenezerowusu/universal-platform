# Internal Event Bus Contract

## Purpose

This document defines the internal event bus contract for Universal Platform.

The event bus allows Platform Core, capability engines, and domain modules to communicate through events without creating direct module coupling.

## Principle

Events describe facts that already happened.

Commands ask for action. Events report completed facts.

## Event Envelope

All internal events should follow a common envelope.

Recommended fields:

```text
event_id
event_type
version
occurred_at
organization_id nullable
actor_user_id nullable
source
correlation_id nullable
causation_id nullable
payload
metadata
```

## Event Type Naming

Use past-tense names.

Good examples:

```text
OrganizationCreated
MemberCreated
AttendanceMarked
PaymentCompleted
SmsDelivered
```

Avoid command-style names such as:

```text
CreateMember
SendSms
ProcessPayment
```

## Event Versioning

Events should include a version field.

When payload structure changes, update the version and keep consumers compatible where practical.

## Publishing Rules

Publish events from application services or engines after the relevant action succeeds.

Do not publish success events before the data change is committed.

## Consumption Rules

Consumers should:

- validate event type and version
- ignore events they do not handle
- avoid assuming only one consumer exists
- handle duplicate events where practical
- log failures safely

## Transaction Direction

For MVP, start simple.

Where event reliability becomes critical, introduce an outbox-style pattern later so database changes and event recording stay consistent.

## Common Event Sources

Examples:

```text
core.identity
core.organizations
engine.payments
engine.sms
domain.religious.members
domain.religious.attendance
```

## Testing Requirements

Tests should cover:

- event envelope creation
- expected event published after action
- event not published after failed action
- consumer handles expected event
- duplicate-safe behavior where needed

## Final Rule

The event bus is a decoupling mechanism. It must not become a hidden shortcut around permissions, tenant context, or application services.
