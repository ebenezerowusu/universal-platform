# Organization API Contract

## Purpose

This document defines the MVP Organization API contract.

Organization answers:

```text
Where is the user operating?
```

A user may belong to many organizations.

## Owner

```text
Organization Engine
```

## Base Path

```text
/api/v1/organizations
```

## List Current User Organizations

```text
GET /api/v1/organizations
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "...",
        "name": "...",
        "slug": "...",
        "status": "active",
        "roleSummary": []
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Create Organization

```text
POST /api/v1/organizations
```

### Request

```json
{
  "name": "Example Organization",
  "country": "GH",
  "currency": "GHS",
  "timezone": "Africa/Accra",
  "defaultLanguage": "en"
}
```

### Success Response

```json
{
  "success": true,
  "data": {
    "organization": {
      "id": "...",
      "name": "Example Organization",
      "status": "active"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

### Notes

Creating an organization should also create the owner's organization membership.

## Get Organization Detail

```text
GET /api/v1/organizations/{organizationId}
```

### Success Response

```json
{
  "success": true,
  "data": {
    "organization": {
      "id": "...",
      "name": "...",
      "country": "GH",
      "currency": "GHS",
      "timezone": "Africa/Accra",
      "defaultLanguage": "en",
      "status": "active"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Update Organization Foundation

```text
PATCH /api/v1/organizations/{organizationId}
```

This may be implemented in Sprint 2 or deferred until Permission Engine foundation is available.

If implemented before Permission Engine, keep update scope minimal and document the rule.

## Organization Users Foundation

These routes may be planned but not fully implemented in Sprint 2:

```text
GET /api/v1/organizations/{organizationId}/users
POST /api/v1/organizations/{organizationId}/invitations
```

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- invalid input
- organization not found
- user is not part of the organization
- organization inactive

## Final Rule

Organization APIs must establish clean organization context without implementing full authorization or domain behavior in Sprint 2.
