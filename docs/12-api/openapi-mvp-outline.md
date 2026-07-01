# OpenAPI MVP Outline

## Purpose

This document defines the first OpenAPI outline for the Universal Platform MVP.

It is not the full generated OpenAPI specification. It is the planning outline for the first contract groups.

## API Base

```text
/api/v1
```

## Standard Response Shape

Success responses should follow:

```json
{
  "success": true,
  "data": {},
  "meta": {
    "traceId": "..."
  }
}
```

Error responses should follow:

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Safe error message",
    "details": {}
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Auth Routes

```text
POST /auth/login
GET  /auth/me
POST /auth/logout
POST /auth/refresh
```

Owned by:

```text
Identity Engine
```

## Organization Routes

```text
GET  /organizations
POST /organizations
GET  /organizations/{organizationId}
PATCH /organizations/{organizationId}
```

Owned by:

```text
Organization Engine
```

## Organization User Routes

```text
GET  /organizations/{organizationId}/users
POST /organizations/{organizationId}/invitations
DELETE /organizations/{organizationId}/users/{organizationUserId}
```

Owned by:

```text
Organization Engine
```

## Permission and Role Routes

```text
GET  /organizations/{organizationId}/permissions
GET  /organizations/{organizationId}/roles
POST /organizations/{organizationId}/roles
PATCH /organizations/{organizationId}/roles/{roleId}
POST /organizations/{organizationId}/roles/{roleId}/permissions
```

Owned by:

```text
Permission Engine
```

## Module Routes

```text
GET  /organizations/{organizationId}/modules
GET  /organizations/{organizationId}/modules/catalog
POST /organizations/{organizationId}/modules/{moduleKey}/install
POST /organizations/{organizationId}/modules/{moduleKey}/activate
POST /organizations/{organizationId}/modules/{moduleKey}/deactivate
```

Owned by:

```text
Module Registry Engine
```

## Subscription and Plan Routes

```text
GET /organizations/{organizationId}/subscription
GET /organizations/{organizationId}/features
GET /platform/subscription-plans
```

Owned by:

```text
Subscription Engine
```

## Religious Member Routes

```text
GET  /organizations/{organizationId}/religious/members
POST /organizations/{organizationId}/religious/members
GET  /organizations/{organizationId}/religious/members/{memberId}
PATCH /organizations/{organizationId}/religious/members/{memberId}
```

Owned by:

```text
Religious Domain
```

## Religious Visitor Routes

```text
GET  /organizations/{organizationId}/religious/visitors
POST /organizations/{organizationId}/religious/visitors
GET  /organizations/{organizationId}/religious/visitors/{visitorId}
PATCH /organizations/{organizationId}/religious/visitors/{visitorId}
POST /organizations/{organizationId}/religious/visitors/{visitorId}/convert
```

Owned by:

```text
Religious Domain
```

## Religious Structure Routes

```text
GET  /organizations/{organizationId}/religious/congregations
POST /organizations/{organizationId}/religious/congregations
GET  /organizations/{organizationId}/religious/services
POST /organizations/{organizationId}/religious/services
GET  /organizations/{organizationId}/religious/groups
POST /organizations/{organizationId}/religious/groups
```

Owned by:

```text
Religious Domain
```

## Religious Attendance Routes

```text
GET  /organizations/{organizationId}/religious/attendance/sessions
POST /organizations/{organizationId}/religious/attendance/sessions
GET  /organizations/{organizationId}/religious/attendance/sessions/{sessionId}
POST /organizations/{organizationId}/religious/attendance/sessions/{sessionId}/records
```

Owned by:

```text
Religious Domain
```

## Reporting Routes

```text
GET  /organizations/{organizationId}/reports
POST /organizations/{organizationId}/reports/{reportKey}/run
POST /organizations/{organizationId}/reports/{reportKey}/export
```

Owned by:

```text
Reporting Engine
```

## MVP Documentation Rule

Each route group should later receive a detailed OpenAPI schema with:

- request body
- response body
- query parameters
- error responses
- permission notes
- module notes

## Final Rule

The MVP should follow this API outline unless an ADR documents why the API shape changed.
