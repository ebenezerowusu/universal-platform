# Authorization Decision Flow

## Purpose

This document defines the MVP authorization decision flow for Universal Platform.

Authorization answers:

```text
What can the authenticated user do in the selected organization?
```

## Principle

Authorization must be enforced by the backend.

Flutter may hide or show UI actions, but backend application services must make the final decision.

## Flow

```text
Incoming Request
  -> Authentication Context
    -> Organization Context
      -> Organization Membership Check
        -> Module/Feature Checks later
          -> Permission Check
            -> Application Service Action
              -> Audit Important Change
```

## Authentication vs Authorization

Authentication answers:

```text
Who is the user?
```

Authorization answers:

```text
What can the user do?
```

Organization context answers:

```text
Where is the user operating?
```

## Required Context

A permission decision should receive:

```text
user_id
organization_id
permission_key
scope_context optional
```

## Permission Result

A permission decision should return a clear result:

```text
allow
deny
```

The result may also include a safe reason code for logging or API error mapping.

## Application Service Rule

Application services should call Permission Engine for protected actions.

API handlers should not contain authorization rules beyond routing and context extraction.

## Denied Action Behavior

When access is denied:

- return standard error shape
- use a safe message
- do not expose internal role or permission internals unnecessarily
- log enough operational context where appropriate

## Audit Direction

Important successful changes should be auditable.

Denied attempts may be logged operationally and may later become auditable depending on policy.

## Sprint 3 Scope

Sprint 3 should implement the Permission Engine foundation.

Advanced scope behavior such as branch, group, own record, and subtree checks may be prepared but should not be overbuilt.

## Final Rule

Every protected action must have one backend authorization decision path. Do not duplicate permission logic across handlers, domains, or Flutter clients.
