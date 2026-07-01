# Authentication, Tenant, and Permission Flow

## Purpose

This document explains how authentication, organization context, and permission checks work together.

These three concerns must remain separate but coordinated.

## Three Questions

Every protected request must answer:

1. Who is the user?
2. Which organization context is active?
3. Is the user allowed to perform this action in that context?

## Responsibility Split

### Identity Engine

Answers:

```text
Who is this user?
```

### Organization Engine

Answers:

```text
Does this user belong to this organization?
```

### Permission Engine

Answers:

```text
Can this user perform this action on this resource within this scope?
```

## Request Flow

```text
Request received
  -> Authenticate user
  -> Resolve organization context
  -> Verify organization membership
  -> Check module availability
  -> Check subscription feature access
  -> Check permission and scope
  -> Call application service
  -> Return response
```

## Organization Context

Most organization APIs should include organization context in the route.

Example:

```text
/api/v1/organizations/{organizationId}/members
```

The backend must verify that the authenticated user has access to that organization.

## Permission Check Inputs

A permission check may require:

- user_id
- organization_id
- permission_key
- resource_type
- resource_id
- scope
- module_key

## Example

A branch leader tries to mark attendance.

The platform must verify:

- The user is authenticated.
- The user belongs to the organization.
- The attendance module is active.
- The subscription allows attendance feature use.
- The user has `religious.attendance.mark`.
- The user's scope allows the selected group/session.

## Failure Responses

Use stable error codes:

- AUTHENTICATION_REQUIRED
- TENANT_ACCESS_DENIED
- PERMISSION_DENIED
- MODULE_NOT_INSTALLED
- FEATURE_NOT_AVAILABLE
- SUBSCRIPTION_LIMIT_REACHED

Do not reveal sensitive internal details.

## Testing Requirements

Tests must cover:

- Unauthenticated request denied
- User outside organization denied
- User without permission denied
- User with wrong scope denied
- Disabled module blocks action
- Subscription feature blocks action
- Authorized request succeeds

## Final Rule

Authentication is not authorization. Organization membership is not permission. All three must be checked correctly.
