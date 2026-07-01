# Religious Member and Visitor API Contract

## Purpose

This document defines the MVP Religious Member and Visitor API contract.

Members and visitors are Religious Domain records scoped to an organization.

## Owner

```text
Religious Domain
```

## Member Base Path

```text
/api/v1/organizations/{organizationId}/religious/members
```

## List Members

```text
GET /api/v1/organizations/{organizationId}/religious/members
```

### Query Parameters

```text
status
search
page
perPage
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "...",
        "memberCode": "MEM-0001",
        "displayName": "...",
        "phone": "...",
        "email": "...",
        "status": "active"
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

## Create Member

```text
POST /api/v1/organizations/{organizationId}/religious/members
```

### Request

```json
{
  "firstName": "Ama",
  "lastName": "Mensah",
  "phone": "+233000000000",
  "email": "ama@example.com",
  "gender": "female",
  "dateOfBirth": "1990-01-01",
  "status": "active"
}
```

## Get Member Detail

```text
GET /api/v1/organizations/{organizationId}/religious/members/{memberId}
```

## Update Member

```text
PATCH /api/v1/organizations/{organizationId}/religious/members/{memberId}
```

## Visitor Base Path

```text
/api/v1/organizations/{organizationId}/religious/visitors
```

## List Visitors

```text
GET /api/v1/organizations/{organizationId}/religious/visitors
```

### Query Parameters

```text
status
search
page
perPage
```

## Create Visitor

```text
POST /api/v1/organizations/{organizationId}/religious/visitors
```

### Request

```json
{
  "firstName": "Kofi",
  "lastName": "Boateng",
  "phone": "+233000000001",
  "email": "kofi@example.com",
  "visitDate": "2026-07-01",
  "source": "sunday_service",
  "notes": "First-time visitor"
}
```

## Get Visitor Detail

```text
GET /api/v1/organizations/{organizationId}/religious/visitors/{visitorId}
```

## Update Visitor

```text
PATCH /api/v1/organizations/{organizationId}/religious/visitors/{visitorId}
```

## Convert Visitor To Member

```text
POST /api/v1/organizations/{organizationId}/religious/visitors/{visitorId}/convert
```

### Success Response

```json
{
  "success": true,
  "data": {
    "member": {
      "id": "...",
      "memberCode": "MEM-0002",
      "displayName": "Kofi Boateng",
      "status": "active"
    },
    "visitor": {
      "id": "...",
      "status": "converted"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Required Checks

Member and visitor routes should check:

- authenticated user
- organization membership
- Religious module availability
- required permission
- feature availability where applicable

## Audit Direction

Audit should be written for important changes such as:

- member created
- member status changed
- visitor created
- visitor converted to member

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- organization not found
- Religious module unavailable
- permission denied
- member not found
- visitor not found
- visitor already converted
- invalid input

## Final Rule

Member and visitor behavior belongs to Religious Domain. It must use Platform Core and Engines instead of bypassing them.
