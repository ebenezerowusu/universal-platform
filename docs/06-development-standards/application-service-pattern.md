# Application Service Pattern

## Purpose

This document defines the application service pattern for Universal Platform.

Application services coordinate use cases. They sit between API handlers and domain/core logic.

## Principle

API handlers should not contain business logic.

Use application services to execute use cases consistently.

## Handler Responsibility

A route handler may:

- extract authenticated user context
- extract organization context
- parse request data
- call an application service
- convert the service result into an API response

A route handler must not:

- perform complex business decisions
- run direct database queries
- call external providers directly
- bypass permission checks

## Application Service Responsibility

An application service may:

- validate use-case rules
- call permission checks
- call module and subscription checks
- coordinate repositories
- call engines
- publish events
- write audit records
- manage transactions

## Example Flow

```text
Handler receives request
  -> Application service called
  -> Permission checked
  -> Module availability checked
  -> Repository operation performed
  -> Event published
  -> Audit recorded where needed
  -> Result returned
```

## Naming Direction

Use clear use-case names.

Examples:

```text
CreateOrganizationService
InviteOrganizationUserService
CreateMemberService
MarkAttendanceService
InstallModuleService
```

Exact Rust naming can follow project style during implementation.

## Transaction Boundary

Application services are often the correct place to define transaction boundaries.

If a use case writes multiple records that must stay consistent, wrap them in one transaction where appropriate.

## Testing

Application service tests should cover:

- successful use case
- validation failure
- permission failure
- organization boundary failure
- module or subscription failure where relevant
- event publication where relevant
- audit record where relevant

## Final Rule

Handlers expose the API. Application services execute the use case.
