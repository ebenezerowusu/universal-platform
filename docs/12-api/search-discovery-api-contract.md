# Search Discovery API Contract

## Purpose

This document defines the MVP API contract for Search and Discovery foundation.

The API should support organization-scoped search across approved searchable metadata while preserving permissions and module boundaries.

## Owner

```text
Search Foundation
Permission Engine
Audit Engine where policy requires it
```

## Base Path

```text
/api/v1/organizations/{organizationId}/search
```

## Global Search

```text
GET /api/v1/organizations/{organizationId}/search
```

### Query Parameters

```text
q
type
module
status
startDate
endDate
page
perPage
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "results": [
      {
        "id": "...",
        "entityType": "document",
        "entityId": "...",
        "title": "Receipt July 2026",
        "snippet": "Safe summary",
        "module": "documents",
        "status": "active",
        "routeHint": "optional"
      }
    ]
  },
  "meta": {
    "traceId": "...",
    "page": 1,
    "perPage": 20
  }
}
```

## Type-Filtered Search

```text
GET /api/v1/organizations/{organizationId}/search/types/{entityType}
```

## Searchable Entity Types

```text
GET /api/v1/organizations/{organizationId}/search/entity-types
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "types": [
      {
        "type": "document",
        "label": "Documents",
        "module": "documents",
        "enabled": true
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Search Metadata Management

```text
GET /api/v1/organizations/{organizationId}/search/index-records
POST /api/v1/organizations/{organizationId}/search/index-records
PATCH /api/v1/organizations/{organizationId}/search/index-records/{indexRecordId}
```

Admin/maintenance only.

## Indexing Job Placeholder

```text
POST /api/v1/organizations/{organizationId}/search/indexing-jobs
```

### Request Direction

```json
{
  "entityType": "document",
  "entityId": "optional",
  "mode": "reindex_placeholder"
}
```

## Recent Search Placeholder

```text
GET /api/v1/organizations/{organizationId}/search/recent
POST /api/v1/organizations/{organizationId}/search/recent
```

Recent search storage should be disabled unless policy allows it.

## Required Checks

Routes should check:

- authenticated user
- organization membership
- search feature availability
- required permission
- module availability for each result type
- source-record permission before returning result

Search results must not include records from disabled modules or restricted domains unless the user has access.

## Entity Type Direction

Suggested entity types:

```text
document
product
order
workforce_profile
calendar_item
finance_record
religious_member
religious_event
delivery_record
custom
```

## Result Safety Direction

Search result titles and snippets must be safe fields.

Do not include sensitive details in snippets unless the user has permission.

## Audit Direction

Audit may be written for:

- sensitive search performed where policy requires it
- index rebuild requested
- search metadata changed
- admin search over restricted entity types

## Error Direction

Use standard response shape.

Expected error areas:

- query missing
- entity type not supported
- search feature unavailable
- permission denied
- source module unavailable
- organization not found

## Final Rule

Search APIs must be permission-aware. A record must not appear in search if the user cannot access it through its source module.
