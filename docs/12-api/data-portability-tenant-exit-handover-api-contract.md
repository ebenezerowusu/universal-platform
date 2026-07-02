# Data Portability Tenant Exit Handover API Contract

## Purpose

This document defines the MVP API contract for Data Portability, Tenant Exit, and Organization Data Handover foundation.

The API should support portability dashboards, export package requests, scope manifests, handover checklists, exit readiness reviews, ownership confirmations, privacy/lineage linkage, archive/retention linkage, billing/support/offboarding linkage, and portability audit history while preserving organization boundaries.

## Owner

```text
Data Portability Foundation
Permission Engine
Audit Engine
Import/Export Foundation
Privacy Governance Foundation
Data Lineage Foundation
Audit/Compliance/Retention Foundation
Billing Foundation
Support Operations Foundation
```

## Base Path

```text
/api/v1/organizations/{organizationId}/data-portability
```

## Portability Dashboard

```text
GET /api/v1/organizations/{organizationId}/data-portability/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "openExportPackages": 2,
    "pendingHandoverTasks": 5,
    "exitReadinessStatus": "in_review_placeholder",
    "privacyReviewsPending": 1,
    "retentionWarnings": 1
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Export Package Requests

```text
GET /api/v1/organizations/{organizationId}/data-portability/export-packages
POST /api/v1/organizations/{organizationId}/data-portability/export-packages
GET /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}
PATCH /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}
POST /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/reject-placeholder
```

### Create Export Package Request

```json
{
  "packagePurpose": "tenant_handover_placeholder",
  "scopeSummary": "All selected organization data with privacy review",
  "requestedModules": ["religious", "finance", "documents"],
  "notes": "Optional safe note"
}
```

## Export Scope Manifests

```text
GET /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/manifest
POST /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/manifest/generate-placeholder
PATCH /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/manifest
```

## Handover Checklists

```text
GET /api/v1/organizations/{organizationId}/data-portability/handover-checklists
POST /api/v1/organizations/{organizationId}/data-portability/handover-checklists
GET /api/v1/organizations/{organizationId}/data-portability/handover-checklists/{checklistId}
PATCH /api/v1/organizations/{organizationId}/data-portability/handover-checklists/{checklistId}
POST /api/v1/organizations/{organizationId}/data-portability/handover-checklists/{checklistId}/complete-item-placeholder
```

## Exit Readiness Reviews

```text
GET /api/v1/organizations/{organizationId}/data-portability/exit-readiness-reviews
POST /api/v1/organizations/{organizationId}/data-portability/exit-readiness-reviews
GET /api/v1/organizations/{organizationId}/data-portability/exit-readiness-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/data-portability/exit-readiness-reviews/{reviewId}/mark-reviewed-placeholder
```

## Ownership Confirmations

```text
GET /api/v1/organizations/{organizationId}/data-portability/ownership-confirmations
POST /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/ownership-confirmations
GET /api/v1/organizations/{organizationId}/data-portability/ownership-confirmations/{confirmationId}
```

## Privacy and Lineage Linkage

```text
GET /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/privacy-lineage-links
POST /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/privacy-review-link-placeholder
POST /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/lineage-manifest-link-placeholder
```

## Archive and Retention Linkage

```text
GET /api/v1/organizations/{organizationId}/data-portability/archive-retention-links
POST /api/v1/organizations/{organizationId}/data-portability/export-packages/{packageId}/archive-retention-link-placeholder
```

## Billing Support Offboarding Linkage

```text
GET /api/v1/organizations/{organizationId}/data-portability/operational-links
POST /api/v1/organizations/{organizationId}/data-portability/operational-links
PATCH /api/v1/organizations/{organizationId}/data-portability/operational-links/{linkId}
```

## Portability Audit History

```text
GET /api/v1/organizations/{organizationId}/data-portability/audit-history
GET /api/v1/organizations/{organizationId}/data-portability/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- data portability feature availability
- required permission
- export package belongs to organization
- ownership confirmation actor is permitted
- privacy/export review status where required
- archive/retention linkage status where required
- billing/support/offboarding warning status where applicable

Data portability records must not cross organization boundaries.

## Status Direction

Suggested package statuses:

```text
requested
in_review_placeholder
approved_placeholder
rejected_placeholder
packaging_placeholder
ready_placeholder
handed_over_placeholder
manual_review
```

Suggested exit readiness statuses:

```text
not_started
in_review_placeholder
blocked_placeholder
ready_placeholder
manual_review
```

Suggested checklist statuses:

```text
open
in_progress_placeholder
completed_placeholder
blocked_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- export package requested
- export package approved/rejected placeholder
- manifest generated placeholder
- ownership confirmation captured placeholder
- handover checklist updated
- exit readiness reviewed placeholder
- privacy review linked
- lineage manifest linked
- archive/retention linkage changed

## Error Direction

Use standard response shape.

Expected error areas:

- export package not found
- manifest not found
- handover checklist not found
- exit readiness review not found
- ownership confirmation not found
- privacy review required
- retention review required
- data portability permission denied
- organization not found

## Final Rule

Data portability APIs must be organization-scoped, privacy-aware, lineage-aware, retention-aligned, permission-protected, auditable, and non-destructive by default.
