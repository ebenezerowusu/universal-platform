# Audit API Contract

## Purpose

This document defines the MVP Audit API contract.

Audit answers:

```text
What important action happened, who did it, and where did it happen?
```

Audit is for accountability. It is different from general application logs.

## Owner

```text
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/audit
```

## List Audit Records

```text
GET /api/v1/organizations/{organizationId}/audit/logs
```

## Query Parameters

Supported MVP query parameters may include:

```text
action
entityType
entityId
actorUserId
createdFrom
createdTo
page
perPage
```

Do not overbuild advanced audit search in the MVP.

## Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "...",
        "organizationId": "...",
        "actorUserId": "...",
        "action": "organization.updated",
        "entityType": "organization",
        "entityId": "...",
        "severity": "info",
        "metadata": {},
        "createdAt": "..."
      }
    ],
    "pagination": {
      "page": 1,
      "perPage": 20,
      "total": 0,
      "totalPages": 0
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Get Audit Record

```text
GET /api/v1/organizations/{organizationId}/audit/logs/{auditLogId}
```

### Success Response

```json
{
  "success": true,
  "data": {
    "auditLog": {
      "id": "...",
      "organizationId": "...",
      "actorUserId": "...",
      "action": "organization.updated",
      "entityType": "organization",
      "entityId": "...",
      "severity": "info",
      "before": {},
      "after": {},
      "metadata": {},
      "createdAt": "..."
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Internal Write Contract

Application services should write audit records through Audit Engine.

Suggested internal contract:

```text
record_audit_event(actor, organization_context, action, entity, changes, metadata)
```

API handlers should not manually insert audit records.

## Sensitivity Direction

Audit records should not contain unnecessary sensitive values.

For sensitive changes, store enough context for accountability without exposing private data in general audit views.

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- organization not found
- user is not part of organization
- permission denied
- audit record not found

## Sprint 3 Scope Note

Sprint 3 should implement audit write foundation.

Audit read APIs may be implemented in Sprint 3 or deferred if documented, but the write service should exist for later application services.

## Final Rule

Audit should be written through the Audit Engine and read through controlled APIs. Do not confuse audit logs with general application logs.
