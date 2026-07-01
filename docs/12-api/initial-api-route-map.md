# Initial API Route Map

## Purpose

This document defines the initial API route map for Universal Platform.

The route map is a planning document. Exact implementation may evolve, but API structure should remain consistent and predictable.

## Base Path

```text
/api/v1
```

## Health

```text
GET /health
```

## Authentication

```text
POST /api/v1/auth/login
POST /api/v1/auth/logout
POST /api/v1/auth/refresh
POST /api/v1/auth/password-reset/request
POST /api/v1/auth/password-reset/confirm
GET  /api/v1/auth/me
```

## Organizations

```text
GET  /api/v1/organizations
POST /api/v1/organizations
GET  /api/v1/organizations/{organizationId}
PATCH /api/v1/organizations/{organizationId}
GET  /api/v1/organizations/{organizationId}/users
POST /api/v1/organizations/{organizationId}/invitations
```

## Permissions and Roles

```text
GET  /api/v1/organizations/{organizationId}/roles
POST /api/v1/organizations/{organizationId}/roles
GET  /api/v1/organizations/{organizationId}/permissions
POST /api/v1/organizations/{organizationId}/users/{userId}/roles
```

## Modules

```text
GET  /api/v1/modules
GET  /api/v1/organizations/{organizationId}/modules
POST /api/v1/organizations/{organizationId}/modules/{moduleKey}/install
POST /api/v1/organizations/{organizationId}/modules/{moduleKey}/activate
POST /api/v1/organizations/{organizationId}/modules/{moduleKey}/deactivate
```

## Subscriptions

```text
GET  /api/v1/subscription-plans
GET  /api/v1/organizations/{organizationId}/subscription
POST /api/v1/organizations/{organizationId}/subscription/change-plan
```

## Audit

```text
GET /api/v1/organizations/{organizationId}/audit-logs
```

## Religious Members

```text
GET  /api/v1/organizations/{organizationId}/religious/members
POST /api/v1/organizations/{organizationId}/religious/members
GET  /api/v1/organizations/{organizationId}/religious/members/{memberId}
PATCH /api/v1/organizations/{organizationId}/religious/members/{memberId}
POST /api/v1/organizations/{organizationId}/religious/members/{memberId}/status
```

## Religious Visitors

```text
GET  /api/v1/organizations/{organizationId}/religious/visitors
POST /api/v1/organizations/{organizationId}/religious/visitors
PATCH /api/v1/organizations/{organizationId}/religious/visitors/{visitorId}
POST /api/v1/organizations/{organizationId}/religious/visitors/{visitorId}/convert
```

## Religious Attendance

```text
GET  /api/v1/organizations/{organizationId}/religious/attendance/sessions
POST /api/v1/organizations/{organizationId}/religious/attendance/sessions
POST /api/v1/organizations/{organizationId}/religious/attendance/sessions/{sessionId}/records
PATCH /api/v1/organizations/{organizationId}/religious/attendance/records/{recordId}
```

## Religious Structure

```text
GET  /api/v1/organizations/{organizationId}/religious/congregations
POST /api/v1/organizations/{organizationId}/religious/congregations
GET  /api/v1/organizations/{organizationId}/religious/services
POST /api/v1/organizations/{organizationId}/religious/services
GET  /api/v1/organizations/{organizationId}/religious/groups
POST /api/v1/organizations/{organizationId}/religious/groups
```

## Reports

```text
GET /api/v1/organizations/{organizationId}/reports
POST /api/v1/organizations/{organizationId}/reports/{reportKey}/export
```

## Route Rules

- Organization-owned routes must include organization context.
- Protected routes must enforce authentication and permission checks.
- Routes must return standard success/error responses.
- Sensitive exports must be audited.

## Final Rule

The route map should grow intentionally. Do not create inconsistent endpoint patterns for each module.
