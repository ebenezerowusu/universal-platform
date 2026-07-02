# Data Privacy Consent Governance API Contract

## Purpose

This document defines the MVP API contract for Data Privacy, Consent, and Sensitive Data Governance foundation.

The API should support privacy dashboards, data classifications, sensitive field catalog placeholders, consent purposes, consent records, withdrawal placeholders, access-purpose logs, masking/redaction placeholders, export reviews, subject request placeholders, and privacy audit history while preserving organization boundaries.

## Owner

```text
Privacy Governance Foundation
Permission Engine
Audit Engine
Audit/Compliance/Retention Foundation
Reporting/Export Foundation where exports are reviewed
```

## Base Path

```text
/api/v1/organizations/{organizationId}/privacy-governance
```

## Privacy Dashboard

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "classificationCount": 8,
    "sensitiveFieldCount": 34,
    "activeConsentRecords": 1200,
    "pendingExportReviews": 3,
    "openSubjectRequests": 2
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Data Classifications

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/classifications
POST /api/v1/organizations/{organizationId}/privacy-governance/classifications
GET /api/v1/organizations/{organizationId}/privacy-governance/classifications/{classificationKey}
PATCH /api/v1/organizations/{organizationId}/privacy-governance/classifications/{classificationKey}
```

## Sensitive Field Catalog Placeholder

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/sensitive-fields
POST /api/v1/organizations/{organizationId}/privacy-governance/sensitive-fields
GET /api/v1/organizations/{organizationId}/privacy-governance/sensitive-fields/{fieldId}
PATCH /api/v1/organizations/{organizationId}/privacy-governance/sensitive-fields/{fieldId}
```

## Consent Purposes

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/consent-purposes
POST /api/v1/organizations/{organizationId}/privacy-governance/consent-purposes
GET /api/v1/organizations/{organizationId}/privacy-governance/consent-purposes/{purposeKey}
PATCH /api/v1/organizations/{organizationId}/privacy-governance/consent-purposes/{purposeKey}
```

## Consent Records

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/consent-records
POST /api/v1/organizations/{organizationId}/privacy-governance/consent-records
GET /api/v1/organizations/{organizationId}/privacy-governance/consent-records/{consentId}
PATCH /api/v1/organizations/{organizationId}/privacy-governance/consent-records/{consentId}
```

### Create Consent Record Request

```json
{
  "subjectReference": {
    "domain": "religious",
    "type": "member",
    "id": "..."
  },
  "purposeKey": "communication_sms_placeholder",
  "sourceChannelPlaceholder": "mobile_app",
  "status": "granted"
}
```

## Consent Withdrawal Placeholder

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/withdrawals
POST /api/v1/organizations/{organizationId}/privacy-governance/consent-records/{consentId}/withdraw-placeholder
GET /api/v1/organizations/{organizationId}/privacy-governance/withdrawals/{withdrawalId}
POST /api/v1/organizations/{organizationId}/privacy-governance/withdrawals/{withdrawalId}/review-placeholder
```

## Access Purpose Logs

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/access-purpose-logs
POST /api/v1/organizations/{organizationId}/privacy-governance/access-purpose-logs
GET /api/v1/organizations/{organizationId}/privacy-governance/access-purpose-logs/{logId}
```

## Masking and Redaction Placeholders

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/masking-policies
POST /api/v1/organizations/{organizationId}/privacy-governance/masking-policies
PATCH /api/v1/organizations/{organizationId}/privacy-governance/masking-policies/{policyId}
```

## Export Review Placeholders

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/export-reviews
POST /api/v1/organizations/{organizationId}/privacy-governance/export-reviews
GET /api/v1/organizations/{organizationId}/privacy-governance/export-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/privacy-governance/export-reviews/{reviewId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/privacy-governance/export-reviews/{reviewId}/reject-placeholder
```

## Subject Request Placeholders

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/subject-requests
POST /api/v1/organizations/{organizationId}/privacy-governance/subject-requests
GET /api/v1/organizations/{organizationId}/privacy-governance/subject-requests/{requestId}
PATCH /api/v1/organizations/{organizationId}/privacy-governance/subject-requests/{requestId}
POST /api/v1/organizations/{organizationId}/privacy-governance/subject-requests/{requestId}/complete-placeholder
```

## Privacy Audit History

```text
GET /api/v1/organizations/{organizationId}/privacy-governance/audit-history
GET /api/v1/organizations/{organizationId}/privacy-governance/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- privacy governance feature availability
- required permission
- subject reference belongs to organization where applicable
- export reference belongs to organization where applicable
- sensitive field metadata is safe to display for actor

Privacy records must not cross organization boundaries.

## Status Direction

Suggested consent statuses:

```text
granted
withdrawn_placeholder
expired_placeholder
restricted_placeholder
manual_review
```

Suggested subject request statuses:

```text
open
in_review_placeholder
completed_placeholder
rejected_placeholder
manual_review
```

Suggested export review statuses:

```text
pending
approved_placeholder
rejected_placeholder
needs_redaction_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- classification created or updated
- sensitive field metadata changed
- consent purpose created or updated
- consent captured
- consent withdrawn placeholder
- access purpose logged
- masking/redaction policy changed
- export review completed placeholder
- subject request updated

## Error Direction

Use standard response shape.

Expected error areas:

- classification not found
- sensitive field metadata not found
- consent purpose not found
- consent record not found
- withdrawal not found
- export review not found
- subject request not found
- privacy permission denied
- cross-organization access denied
- organization not found

## Final Rule

Privacy governance APIs must be organization-scoped, permission-protected, consent-aware, export-aware, retention-aligned, and auditable.
