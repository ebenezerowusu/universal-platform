# Import Export Data Migration API Contract

## Purpose

This document defines the MVP API contract for Import, Export, and Data Migration foundation.

The API should support import templates, import jobs, validation previews, execution placeholders, export requests, job history, and data-quality warnings while preserving organization boundaries.

## Owner

```text
Import Export Foundation
Document Media Library
Storage Engine
Permission Engine
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/data-movement
```

## Import Templates

```text
GET /api/v1/organizations/{organizationId}/data-movement/import-templates
GET /api/v1/organizations/{organizationId}/data-movement/import-templates/{templateKey}
POST /api/v1/organizations/{organizationId}/data-movement/import-templates
PATCH /api/v1/organizations/{organizationId}/data-movement/import-templates/{templateKey}
```

Admin/maintenance only for create/update.

## Import Jobs

```text
GET /api/v1/organizations/{organizationId}/data-movement/import-jobs
POST /api/v1/organizations/{organizationId}/data-movement/import-jobs
GET /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}
POST /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}/cancel
```

### Create Import Job Request

```json
{
  "templateKey": "member_import",
  "sourceDocumentId": "...",
  "notes": "Optional note"
}
```

## Validation Preview

```text
POST /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}/validate
GET /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}/validation-results
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "summary": {
      "totalRows": 100,
      "validRows": 80,
      "warningRows": 15,
      "errorRows": 5
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Field Mapping Placeholder

```text
GET /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}/field-mapping
PATCH /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}/field-mapping
```

## Execute Import Placeholder

```text
POST /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}/execute
```

Execution should only be allowed after successful validation or explicit permitted override.

## Rollback Note Placeholder

```text
POST /api/v1/organizations/{organizationId}/data-movement/import-jobs/{jobId}/rollback-note
```

This records a rollback note. It does not implement automatic destructive rollback.

## Export Requests

```text
GET /api/v1/organizations/{organizationId}/data-movement/export-requests
POST /api/v1/organizations/{organizationId}/data-movement/export-requests
GET /api/v1/organizations/{organizationId}/data-movement/export-requests/{exportRequestId}
POST /api/v1/organizations/{organizationId}/data-movement/export-requests/{exportRequestId}/cancel
```

### Create Export Request

```json
{
  "exportKey": "finance_summary_export",
  "format": "xlsx_placeholder",
  "filters": {
    "startDate": "2026-07-01",
    "endDate": "2026-07-31"
  }
}
```

## Data-Quality Warnings

```text
GET /api/v1/organizations/{organizationId}/data-movement/data-quality-warnings
```

### Query Parameters

```text
jobId
severity
status
module
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- import/export feature availability
- required permission
- document/file access where source or output files are referenced
- source domain availability
- organization ownership

Exports should require explicit export permission.

## Status Direction

Suggested import job statuses:

```text
draft
submitted
validating
validated
validation_failed
ready_to_execute
executing_placeholder
completed_placeholder
cancelled
manual_review
```

Suggested export request statuses:

```text
requested
processing_placeholder
completed_placeholder
failed
cancelled
manual_review
```

## Validation Result Direction

Suggested validation levels:

```text
info
warning
error
blocked
manual_review
```

## Audit Direction

Audit should be written for:

- import job submitted
- validation completed
- import execution requested
- import cancelled
- rollback note added
- export requested
- export cancelled
- template changed

## Error Direction

Use standard response shape.

Expected error areas:

- template not found
- job not found
- source document not found
- validation failed
- execution not allowed
- export request not found
- export permission denied
- organization not found

## Final Rule

Import/export APIs must be validation-first, permission-aware, file-boundary-safe, and auditable.
