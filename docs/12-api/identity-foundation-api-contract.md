# Identity Foundation API Contract

## Purpose

This document records the Sprint 2 Identity foundation API direction.

Identity is a platform-level capability. It must stay domain-agnostic and must not contain member, customer, staff, partner, or religious-specific behavior.

## Owner

```text
Identity Foundation
Security Controls Foundation
Permission Engine later
Audit Engine later
```

## Current Sprint 2 Route

```text
GET /api/v1/identity/status
```

Purpose:

- expose safe identity foundation readiness metadata
- confirm identity primitives are linked into the API
- avoid exposing real auth flows before repository, password hashing, sessions, rate limiting, and audit are implemented

## Planned Future Routes

Do not implement these until their dependencies exist:

```text
POST /api/v1/auth/register
POST /api/v1/auth/login
POST /api/v1/auth/logout
POST /api/v1/auth/password-reset/request
POST /api/v1/auth/password-reset/confirm
GET /api/v1/identity/users/{userId}
PATCH /api/v1/identity/users/{userId}/status
GET /api/v1/identity/profile
PATCH /api/v1/identity/profile
```

## Required Future Checks

Real identity routes must enforce:

- input validation
- password hashing
- secure password verification
- session/token lifecycle
- rate limiting on login and password reset
- audit for sensitive identity actions
- safe error messages
- no plain-text password persistence
- no domain-specific profile coupling

## Response Direction

```json
{
  "success": true,
  "data": {
    "foundation": "identity-foundation-placeholder",
    "implemented_flows": [],
    "planned_user_statuses": []
  },
  "meta": {
    "service": "platform-api",
    "trace_id": "not-wired-yet"
  }
}
```

## Events Direction

Future events may include:

```text
identity.user.created
identity.user.activated
identity.user.suspended
identity.user.deactivated
identity.password_reset.requested_placeholder
identity.password.changed_placeholder
identity.session.created_placeholder
identity.session.revoked_placeholder
```

## Audit Direction

Future audit events may include:

```text
audit.identity.user_created
audit.identity.user_updated
audit.identity.user_deactivated
audit.identity.login_failed_placeholder
audit.identity.password_reset_requested_placeholder
audit.identity.password_changed_placeholder
```

## Final Rule

Identity owns accounts. Organization membership owns tenant access. Domains own business profiles.
