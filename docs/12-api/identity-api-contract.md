# Identity API Contract

## Purpose

This document defines the MVP Identity API contract.

Identity answers:

```text
Who is the user?
```

Authorization is handled later by the Permission Engine.

## Owner

```text
Identity Engine
```

## Base Path

```text
/api/v1/auth
```

## Login

```text
POST /api/v1/auth/login
```

### Request

```json
{
  "identifier": "user@example.com",
  "password": "example-password"
}
```

`identifier` may represent email, phone, or username depending on implementation direction.

### Success Response

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "...",
      "displayName": "...",
      "email": "..."
    },
    "session": {
      "accessToken": "...",
      "expiresAt": "..."
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

### Notes

- Do not return credential internals.
- Do not return password-related values.
- Keep response safe and minimal.

## Current User

```text
GET /api/v1/auth/me
```

### Success Response

```json
{
  "success": true,
  "data": {
    "user": {
      "id": "...",
      "displayName": "...",
      "email": "..."
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Logout

```text
POST /api/v1/auth/logout
```

### Success Response

```json
{
  "success": true,
  "data": {
    "loggedOut": true
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Refresh Session

```text
POST /api/v1/auth/refresh
```

This route may be implemented in Sprint 2 or deferred if documented.

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- invalid input
- invalid credentials
- inactive account
- expired session
- missing session context

## Security Direction

Identity routes must follow:

- `docs/08-security/auth-session-standards.md`
- `docs/08-security/password-and-token-standards.md`
- `docs/08-security/mvp-security-checklist.md`

## Final Rule

Identity APIs should return safe user/session summaries only. They must not leak credential internals or authorization decisions.
