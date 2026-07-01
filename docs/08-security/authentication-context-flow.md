# Authentication Context Flow

## Purpose

This document defines the MVP authentication context flow for Universal Platform.

It explains how the backend should move from request credentials to a safe authenticated user context.

## Principle

Authentication context should be resolved once near the API boundary and passed into application services safely.

Application services should not parse raw headers or raw tokens directly.

## Flow

```text
Incoming Request
  -> API middleware / extractor
    -> session or token validation
      -> user lookup
        -> authenticated context
          -> application service
```

## Authenticated Context

The authenticated context should contain safe fields such as:

```text
user_id
session_id where applicable
trace_id
```

It should not contain:

```text
password values
raw credential values
provider secrets
unvalidated authorization claims
```

## Organization Context

Organization context is separate from authentication context.

Authentication answers:

```text
Who is the user?
```

Organization context answers:

```text
Which organization is the user operating in?
```

## Combined Request Context Later

Protected organization routes may use a combined context:

```text
authenticated_user
selected_organization
organization_membership
```

Permission Engine will later use that context to answer:

```text
What can the user do here?
```

## Sprint 2 Requirement

Sprint 2 should establish:

- current user context
- basic organization membership lookup
- selected organization context for organization routes where needed

Sprint 2 should not implement full Permission Engine behavior.

## Error Direction

Authentication failures should return safe standard errors.

Do not expose whether a credential field, lookup path, or internal validation step failed.

## Final Rule

Authentication context must be safe, minimal, and reusable. Do not mix it with domain logic or full authorization rules.
