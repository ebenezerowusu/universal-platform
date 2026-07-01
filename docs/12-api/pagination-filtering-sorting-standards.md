# Pagination, Filtering, and Sorting Standards

## Purpose

This document defines standard pagination, filtering, and sorting behavior for Universal Platform APIs.

Consistent list APIs make backend implementation, Flutter clients, and OpenAPI contracts easier to maintain.

## Principle

List endpoints must be bounded, predictable, and tenant-safe.

Do not return large unbounded lists.

## Pagination Parameters

Recommended query parameters:

```text
page
perPage
```

Default behavior:

```text
page = 1
perPage = 20
```

The backend should enforce a maximum `perPage` value.

## Pagination Response Shape

```json
{
  "success": true,
  "data": {
    "items": [],
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

## Filtering

Common filters:

```text
search
status
dateFrom
dateTo
branchId
congregationId
groupId
moduleKey
```

Filters must respect permissions and organization scope.

A user should not be able to infer hidden data through filter behavior.

## Sorting

Recommended query parameters:

```text
sortBy
sortDirection
```

Allowed sort fields should be explicitly controlled per endpoint.

Do not allow arbitrary database column names from client input.

## Search

Search should be safe and bounded.

For MVP, simple search may cover common fields such as name, phone, email, code, or title depending on the resource.

Advanced search can be introduced later through the Search Engine.

## Date Filters

Use clear date filters:

```text
dateFrom
dateTo
```

Avoid ambiguous date parameter names.

## Tenant Safety

Every list query for organization-owned resources must include organization context.

Filtering and sorting must not remove tenant boundaries.

## Error Cases

Possible errors:

- VALIDATION_ERROR for invalid query values
- PERMISSION_DENIED for unauthorized list access
- TENANT_ACCESS_DENIED for organization boundary failure

## Testing Requirements

Tests should cover:

- Default pagination
- Maximum page size enforcement
- Status filtering
- Date filtering
- Search behavior
- Invalid sort field rejection
- Organization boundary enforcement

## Final Rule

Every list endpoint must be bounded, permission-aware, and organization-scoped.
