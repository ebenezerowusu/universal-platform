# Reporting Analytics API Contract

## Purpose

This document defines the MVP API contract for Reporting and Analytics foundation.

The API should support dashboard summaries, report definitions, metric cards, report execution, filters, export request placeholders, and data-quality warnings while preserving organization boundaries.

## Owner

```text
Reporting Foundation
Permission Engine
Import/Export Engine where exports are used
Audit Engine where policy requires it
```

## Base Path

```text
/api/v1/organizations/{organizationId}/reports
```

## Dashboard Summary

```text
GET /api/v1/organizations/{organizationId}/reports/dashboard-summary
```

### Query Parameters

```text
startDate
endDate
module
status
```

## Metric Cards

```text
GET /api/v1/organizations/{organizationId}/reports/metrics
GET /api/v1/organizations/{organizationId}/reports/metrics/{metricKey}
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "metrics": [
      {
        "key": "orders_pending",
        "title": "Pending Orders",
        "value": 12,
        "module": "commerce",
        "dataQuality": "ok"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Report Definitions

```text
GET /api/v1/organizations/{organizationId}/reports/definitions
POST /api/v1/organizations/{organizationId}/reports/definitions
GET /api/v1/organizations/{organizationId}/reports/definitions/{reportKey}
PATCH /api/v1/organizations/{organizationId}/reports/definitions/{reportKey}
```

Admin/maintenance only for create/update.

## Execute Report

```text
POST /api/v1/organizations/{organizationId}/reports/execute
```

### Request Direction

```json
{
  "reportKey": "finance_summary",
  "filters": {
    "startDate": "2026-07-01",
    "endDate": "2026-07-31",
    "status": "active"
  }
}
```

## Filter Metadata

```text
GET /api/v1/organizations/{organizationId}/reports/definitions/{reportKey}/filters
```

## Result Snapshot Placeholder

```text
GET /api/v1/organizations/{organizationId}/reports/snapshots/{snapshotId}
POST /api/v1/organizations/{organizationId}/reports/snapshots
```

## Export Request Placeholder

```text
POST /api/v1/organizations/{organizationId}/reports/exports
GET /api/v1/organizations/{organizationId}/reports/exports
GET /api/v1/organizations/{organizationId}/reports/exports/{exportRequestId}
```

### Request Direction

```json
{
  "reportKey": "finance_summary",
  "format": "xlsx_placeholder",
  "filters": {
    "startDate": "2026-07-01",
    "endDate": "2026-07-31"
  }
}
```

## Data-Quality Warnings

```text
GET /api/v1/organizations/{organizationId}/reports/data-quality-warnings
```

### Query Parameters

```text
module
severity
status
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- reporting feature availability
- required report permission
- source module availability
- source-record access where drill-down is supported

Exports should require explicit export permission.

## Status Direction

Suggested report definition statuses:

```text
active
inactive
deprecated_placeholder
manual_review
```

Suggested data-quality statuses:

```text
ok
warning
partial
unavailable
manual_review
```

## Result Safety Direction

Report responses must not include records from disabled modules or modules the user cannot access.

Aggregates should clearly indicate when data is partial due to permission or availability limits.

## Audit Direction

Audit may be written for:

- sensitive report opened where policy requires it
- export requested
- report definition changed
- admin dashboard configuration changed
- data-quality warning manually resolved

## Error Direction

Use standard response shape.

Expected error areas:

- report definition not found
- metric not found
- invalid filters
- source module unavailable
- export permission denied
- report unavailable
- organization not found

## Final Rule

Reporting APIs must be permission-aware, organization-scoped, export-safe, and clear about partial or weak data.
