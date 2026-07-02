# Data Quality Master Data API Contract

## Purpose

This document defines the MVP API contract for Data Quality, Validation, Duplicate Detection, and Master Data Governance foundation.

The API should support data quality dashboards, quality rules, validation results, duplicate detection rules, duplicate candidates, merge review placeholders, master data registry placeholders, canonical markers, correction requests, and data quality audit history while preserving organization boundaries.

## Owner

```text
Data Quality Foundation
Permission Engine
Audit Engine
Privacy Governance Foundation
Import/Export/Data Migration Foundation
Reporting/Search Foundations where quality warnings are consumed
```

## Base Path

```text
/api/v1/organizations/{organizationId}/data-quality
```

## Data Quality Dashboard

```text
GET /api/v1/organizations/{organizationId}/data-quality/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activeQualityRules": 24,
    "openValidationIssues": 57,
    "duplicateCandidates": 14,
    "pendingMergeReviews": 3,
    "openCorrectionRequests": 8
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Data Quality Rules

```text
GET /api/v1/organizations/{organizationId}/data-quality/rules
POST /api/v1/organizations/{organizationId}/data-quality/rules
GET /api/v1/organizations/{organizationId}/data-quality/rules/{ruleKey}
PATCH /api/v1/organizations/{organizationId}/data-quality/rules/{ruleKey}
```

## Validation Results

```text
GET /api/v1/organizations/{organizationId}/data-quality/validation-results
POST /api/v1/organizations/{organizationId}/data-quality/validation-results
GET /api/v1/organizations/{organizationId}/data-quality/validation-results/{resultId}
POST /api/v1/organizations/{organizationId}/data-quality/validation-results/{resultId}/mark-reviewed-placeholder
```

## Duplicate Detection Rules

```text
GET /api/v1/organizations/{organizationId}/data-quality/duplicate-rules
POST /api/v1/organizations/{organizationId}/data-quality/duplicate-rules
GET /api/v1/organizations/{organizationId}/data-quality/duplicate-rules/{ruleId}
PATCH /api/v1/organizations/{organizationId}/data-quality/duplicate-rules/{ruleId}
```

## Duplicate Candidates

```text
GET /api/v1/organizations/{organizationId}/data-quality/duplicate-candidates
POST /api/v1/organizations/{organizationId}/data-quality/duplicate-candidates
GET /api/v1/organizations/{organizationId}/data-quality/duplicate-candidates/{candidateId}
POST /api/v1/organizations/{organizationId}/data-quality/duplicate-candidates/{candidateId}/mark-not-duplicate-placeholder
POST /api/v1/organizations/{organizationId}/data-quality/duplicate-candidates/{candidateId}/send-to-merge-review-placeholder
```

## Merge Review Placeholders

```text
GET /api/v1/organizations/{organizationId}/data-quality/merge-reviews
POST /api/v1/organizations/{organizationId}/data-quality/merge-reviews
GET /api/v1/organizations/{organizationId}/data-quality/merge-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/data-quality/merge-reviews/{reviewId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/data-quality/merge-reviews/{reviewId}/reject-placeholder
```

Merge approval should not automatically delete source records in MVP.

## Master Data Registry Placeholders

```text
GET /api/v1/organizations/{organizationId}/data-quality/master-data-registry
POST /api/v1/organizations/{organizationId}/data-quality/master-data-registry
GET /api/v1/organizations/{organizationId}/data-quality/master-data-registry/{registryId}
PATCH /api/v1/organizations/{organizationId}/data-quality/master-data-registry/{registryId}
```

## Canonical Record Markers

```text
GET /api/v1/organizations/{organizationId}/data-quality/canonical-markers
POST /api/v1/organizations/{organizationId}/data-quality/canonical-markers
GET /api/v1/organizations/{organizationId}/data-quality/canonical-markers/{markerId}
PATCH /api/v1/organizations/{organizationId}/data-quality/canonical-markers/{markerId}
```

## Correction Requests

```text
GET /api/v1/organizations/{organizationId}/data-quality/correction-requests
POST /api/v1/organizations/{organizationId}/data-quality/correction-requests
GET /api/v1/organizations/{organizationId}/data-quality/correction-requests/{requestId}
PATCH /api/v1/organizations/{organizationId}/data-quality/correction-requests/{requestId}
POST /api/v1/organizations/{organizationId}/data-quality/correction-requests/{requestId}/complete-placeholder
```

## Data Quality Audit History

```text
GET /api/v1/organizations/{organizationId}/data-quality/audit-history
GET /api/v1/organizations/{organizationId}/data-quality/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- data quality feature availability
- required permission
- entity references belong to organization where applicable
- privacy masking rules are respected for sensitive fields
- merge review does not perform destructive action automatically

Data quality records must not cross organization boundaries.

## Status Direction

Suggested validation statuses:

```text
open
reviewed_placeholder
resolved_placeholder
ignored_placeholder
manual_review
```

Suggested duplicate candidate statuses:

```text
open
not_duplicate_placeholder
sent_to_merge_review_placeholder
resolved_placeholder
manual_review
```

Suggested merge review statuses:

```text
pending
approved_placeholder
rejected_placeholder
needs_privacy_review_placeholder
manual_review
```

Suggested correction request statuses:

```text
open
in_review_placeholder
completed_placeholder
rejected_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- data quality rule created or updated
- validation result reviewed
- duplicate candidate reviewed
- merge review approved/rejected placeholder
- canonical record marker changed
- correction request created or completed
- bulk correction placeholder reviewed

## Error Direction

Use standard response shape.

Expected error areas:

- quality rule not found
- validation result not found
- duplicate candidate not found
- merge review not found
- master data registry item not found
- canonical marker not found
- correction request not found
- privacy masking required
- data quality permission denied
- organization not found

## Final Rule

Data quality APIs must be organization-scoped, privacy-aware, permission-protected, auditable, and non-destructive by default.
