# Audit Compliance Retention API Contract

## Purpose

This document defines the MVP API contract for Audit, Compliance, and Data Retention foundation.

The API should support audit event queries, sensitive-action categories, access review placeholders, retention policy metadata, safe archive/delete request placeholders, and compliance checklist placeholders while preserving organization boundaries.

## Owner

```text
Audit Foundation
Retention Foundation
Permission Engine
Workflow Engine where retention approvals are used later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/governance
```

## Audit Events

```text
GET /api/v1/organizations/{organizationId}/governance/audit-events
GET /api/v1/organizations/{organizationId}/governance/audit-events/{auditEventId}
```

### Query Parameters

```text
startDate
endDate
actorId
action
targetType
targetId
category
severity
module
page
perPage
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "events": [
      {
        "id": "...",
        "actorLabel": "Admin User",
        "action": "export_requested",
        "targetType": "report_export",
        "targetId": "...",
        "category": "report_export",
        "createdAt": "..."
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

## Sensitive Action Categories

```text
GET /api/v1/organizations/{organizationId}/governance/sensitive-action-categories
POST /api/v1/organizations/{organizationId}/governance/sensitive-action-categories
PATCH /api/v1/organizations/{organizationId}/governance/sensitive-action-categories/{categoryId}
```

Admin/maintenance only for create/update.

## Access Review Placeholder

```text
GET /api/v1/organizations/{organizationId}/governance/access-reviews
POST /api/v1/organizations/{organizationId}/governance/access-reviews
GET /api/v1/organizations/{organizationId}/governance/access-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/governance/access-reviews/{reviewId}/complete-placeholder
```

## Retention Policies

```text
GET /api/v1/organizations/{organizationId}/governance/retention-policies
POST /api/v1/organizations/{organizationId}/governance/retention-policies
GET /api/v1/organizations/{organizationId}/governance/retention-policies/{policyId}
PATCH /api/v1/organizations/{organizationId}/governance/retention-policies/{policyId}
```

### Create Retention Policy Request

```json
{
  "policyKey": "finance_receipts_retention",
  "module": "finance",
  "targetRecordType": "finance_receipt",
  "retentionPeriodPlaceholder": "7_years_placeholder",
  "archiveBehavior": "manual_review_placeholder",
  "deleteBehavior": "disabled_in_mvp",
  "status": "active"
}
```

## Archive/Delete Request Placeholder

```text
GET /api/v1/organizations/{organizationId}/governance/retention-requests
POST /api/v1/organizations/{organizationId}/governance/retention-requests
GET /api/v1/organizations/{organizationId}/governance/retention-requests/{requestId}
POST /api/v1/organizations/{organizationId}/governance/retention-requests/{requestId}/resolve-placeholder
```

### Create Retention Request

```json
{
  "requestType": "archive_request",
  "targetType": "document",
  "targetId": "...",
  "reason": "Optional reason"
}
```

## Compliance Checklist Placeholder

```text
GET /api/v1/organizations/{organizationId}/governance/compliance-checklist
PATCH /api/v1/organizations/{organizationId}/governance/compliance-checklist/{itemId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- governance feature availability
- required permission
- sensitive audit permission where needed
- organization ownership

Audit event routes must not expose records from another organization.

## Status Direction

Suggested retention policy statuses:

```text
active
inactive
draft
manual_review
```

Suggested retention request statuses:

```text
submitted
reviewing_placeholder
approved_placeholder
rejected_placeholder
completed_placeholder
cancelled
manual_review
```

Suggested compliance checklist statuses:

```text
not_started
in_progress
completed_placeholder
not_applicable
manual_review
```

## Result Safety Direction

Audit event details should expose safe labels by default.

Sensitive metadata should require sensitive audit permission.

## Audit Direction

Audit should be written for:

- audit event viewed where policy requires it
- sensitive audit query executed where policy requires it
- retention policy changed
- retention request created
- retention request resolved placeholder
- access review completed placeholder
- compliance checklist updated

## Error Direction

Use standard response shape.

Expected error areas:

- audit event not found
- sensitive permission denied
- retention policy not found
- retention request not found
- access review not found
- invalid retention action
- organization not found

## Final Rule

Governance APIs must preserve organization scope, sensitive permission checks, non-destructive retention behavior, and auditable action history.
