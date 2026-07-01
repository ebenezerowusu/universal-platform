# Religious Settings API Contract

## Purpose

This document defines the MVP Religious Settings API contract.

Religious settings control Religious Domain behavior for a specific organization.

## Owner

```text
Religious Domain
```

## Base Path

```text
/api/v1/organizations/{organizationId}/religious/settings
```

## Get Religious Settings

```text
GET /api/v1/organizations/{organizationId}/religious/settings
```

### Success Response

```json
{
  "success": true,
  "data": {
    "settings": {
      "memberCodePrefix": "MEM",
      "defaultMemberStatus": "active",
      "allowVisitorConversion": true,
      "attendanceDefaultStatus": "present"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Update Religious Settings

```text
PATCH /api/v1/organizations/{organizationId}/religious/settings
```

### Request

```json
{
  "memberCodePrefix": "MEM",
  "defaultMemberStatus": "active",
  "allowVisitorConversion": true,
  "attendanceDefaultStatus": "present"
}
```

### Success Response

```json
{
  "success": true,
  "data": {
    "settings": {
      "memberCodePrefix": "MEM",
      "defaultMemberStatus": "active",
      "allowVisitorConversion": true,
      "attendanceDefaultStatus": "present"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Required Checks

Religious settings routes should check:

- authenticated user
- organization membership
- Religious module availability
- required permission
- feature availability where applicable

## Audit Direction

Updating Religious settings should write an audit record.

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- organization not found
- Religious module unavailable
- permission denied
- invalid settings value

## Final Rule

Religious settings belong to Religious Domain. Platform Core must not know Religious settings or religious terminology.
