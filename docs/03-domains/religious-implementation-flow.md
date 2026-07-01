# Religious Implementation Flow

## Purpose

This document defines the MVP implementation flow for the Religious Domain.

It explains how Religious Domain use cases should pass through Platform Core and Engines instead of bypassing them.

## Principle

Religious Domain owns religious business meaning.

Platform Core owns generic platform capabilities.

Capability Engines provide shared services.

## Standard Use Case Flow

Religious MVP use cases should generally follow this flow:

```text
API Route
  -> Authentication Context
    -> Organization Context
      -> Module Registry Check
        -> Feature Availability Check
          -> Permission Check
            -> Religious Application Service
              -> Religious Repository
                -> Audit Important Change
```

## Module Check

Before Religious features run, application services should confirm the required Religious module is available for the organization.

Examples:

```text
religious.members
religious.visitors
religious.structure
religious.attendance
religious.reports
```

## Feature Check

Where feature checks exist, Religious use cases should ask the feature availability foundation.

Examples:

```text
religious.members.manage
religious.visitors.manage
religious.attendance.manage
```

## Permission Check

Religious use cases should ask Permission Engine for protected actions.

Examples:

```text
religious.members.view
religious.members.create
religious.visitors.convert
religious.attendance.mark
```

## Audit Direction

Important Religious Domain changes should write audit records through Audit Engine.

Examples:

- member created
- member status changed
- visitor converted
- group created
- attendance submitted

## Repository Direction

Religious repositories must scope data by organization context.

Do not create repository methods that return organization-owned records without organization filtering.

## Error Direction

Religious use cases should return platform-standard errors.

Do not expose internal database or provider details.

## MVP Build Order

Recommended build order:

1. Religious module keys and seed data
2. Religious settings
3. Members
4. Visitors
5. Congregations
6. Services
7. Groups
8. Group membership
9. Attendance sessions
10. Attendance records
11. Basic reports

## Final Rule

Religious Domain should prove the architecture. If a Religious feature bypasses Core or Engines, it should be rejected in review.
