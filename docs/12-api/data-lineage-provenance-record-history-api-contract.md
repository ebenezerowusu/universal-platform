# Data Lineage Provenance Record History API Contract

## Purpose

This document defines the MVP API contract for Data Lineage, Provenance, and Record History foundation.

The API should support lineage dashboards, source systems, provenance records, record history placeholders, lineage links, transformation steps, derived record relationships, lineage graph summaries, correction/merge lineage, and lineage audit history while preserving organization boundaries.

## Owner

```text
Data Lineage Foundation
Permission Engine
Audit Engine
Privacy Governance Foundation
Import/Export Foundation
Data Quality Foundation
Reporting/Search Foundations where lineage is consumed
```

## Base Path

```text
/api/v1/organizations/{organizationId}/data-lineage
```

## Lineage Dashboard

```text
GET /api/v1/organizations/{organizationId}/data-lineage/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "sourceSystems": 6,
    "provenanceRecords": 1200,
    "lineageLinks": 3400,
    "recordsWithUnknownSource": 32,
    "recentLineageEvents": 12
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Source Systems

```text
GET /api/v1/organizations/{organizationId}/data-lineage/source-systems
POST /api/v1/organizations/{organizationId}/data-lineage/source-systems
GET /api/v1/organizations/{organizationId}/data-lineage/source-systems/{sourceKey}
PATCH /api/v1/organizations/{organizationId}/data-lineage/source-systems/{sourceKey}
```

## Provenance Records

```text
GET /api/v1/organizations/{organizationId}/data-lineage/provenance
POST /api/v1/organizations/{organizationId}/data-lineage/provenance
GET /api/v1/organizations/{organizationId}/data-lineage/provenance/{provenanceId}
```

### Create Provenance Record Request

```json
{
  "entityReference": {
    "domain": "religious",
    "type": "member",
    "id": "..."
  },
  "sourceSystemKey": "manual_entry_placeholder",
  "sourceRecordReferencePlaceholder": "optional",
  "createdThrough": "manual_placeholder",
  "processReferencePlaceholder": "optional"
}
```

## Record History Placeholders

```text
GET /api/v1/organizations/{organizationId}/data-lineage/record-history
POST /api/v1/organizations/{organizationId}/data-lineage/record-history
GET /api/v1/organizations/{organizationId}/data-lineage/record-history/{historyId}
GET /api/v1/organizations/{organizationId}/data-lineage/entities/{entityType}/{entityId}/history
```

## Lineage Links

```text
GET /api/v1/organizations/{organizationId}/data-lineage/links
POST /api/v1/organizations/{organizationId}/data-lineage/links
GET /api/v1/organizations/{organizationId}/data-lineage/links/{linkId}
PATCH /api/v1/organizations/{organizationId}/data-lineage/links/{linkId}
```

## Transformation Steps

```text
GET /api/v1/organizations/{organizationId}/data-lineage/transformation-steps
POST /api/v1/organizations/{organizationId}/data-lineage/transformation-steps
GET /api/v1/organizations/{organizationId}/data-lineage/transformation-steps/{stepId}
```

## Derived Record Relationships

```text
GET /api/v1/organizations/{organizationId}/data-lineage/derived-relationships
POST /api/v1/organizations/{organizationId}/data-lineage/derived-relationships
GET /api/v1/organizations/{organizationId}/data-lineage/derived-relationships/{relationshipId}
```

## Lineage Graph Summary Placeholder

```text
GET /api/v1/organizations/{organizationId}/data-lineage/entities/{entityType}/{entityId}/graph-summary-placeholder
GET /api/v1/organizations/{organizationId}/data-lineage/reports/{reportSnapshotId}/graph-summary-placeholder
GET /api/v1/organizations/{organizationId}/data-lineage/exports/{exportRequestId}/graph-summary-placeholder
```

## Correction and Merge Lineage

```text
GET /api/v1/organizations/{organizationId}/data-lineage/corrections
POST /api/v1/organizations/{organizationId}/data-lineage/corrections
GET /api/v1/organizations/{organizationId}/data-lineage/merge-reviews/{mergeReviewId}/lineage
GET /api/v1/organizations/{organizationId}/data-lineage/correction-requests/{correctionRequestId}/lineage
```

## Lineage Audit History

```text
GET /api/v1/organizations/{organizationId}/data-lineage/audit-history
GET /api/v1/organizations/{organizationId}/data-lineage/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- data lineage feature availability
- required permission
- entity references belong to organization where applicable
- lineage graph summaries apply privacy masking rules
- source references are safe to display for the actor

Lineage records must not cross organization boundaries.

## Status Direction

Suggested source statuses:

```text
active
inactive
deprecated_placeholder
manual_review
```

Suggested lineage link statuses:

```text
active
superseded_placeholder
disputed_placeholder
manual_review
```

Suggested provenance quality statuses:

```text
known
partial_placeholder
unknown_placeholder
disputed_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- source system created or updated
- provenance record created placeholder
- lineage link created or updated
- transformation metadata changed
- derived relationship changed
- record history reviewed placeholder
- correction/merge lineage reviewed placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- source system not found
- provenance record not found
- lineage link not found
- transformation step not found
- derived relationship not found
- graph summary unavailable
- entity reference not found
- privacy masking required
- data lineage permission denied
- organization not found

## Final Rule

Data lineage APIs must be organization-scoped, privacy-aware, permission-protected, auditable, and honest about placeholder-level lineage rather than full historical reconstruction.
