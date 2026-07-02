# Platform Support Operations API Contract

## Purpose

This document defines the MVP API contract for Platform Support and Organization Operations foundation.

The API should support support cases, case timelines, onboarding trackers, organization health reviews, escalation placeholders, operational checklists, and support notifications while preserving organization boundaries and privacy.

## Owner

```text
Platform Support Foundation
Permission Engine
Audit Engine
Notification Engine where support notifications are used
```

## Base Path

```text
/api/v1/organizations/{organizationId}/support
```

## Support Dashboard

```text
GET /api/v1/organizations/{organizationId}/support/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "openCases": 3,
    "escalatedCases": 1,
    "onboardingStatus": "in_progress",
    "healthStatus": "needs_attention_placeholder"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Support Cases

```text
GET /api/v1/organizations/{organizationId}/support/cases
POST /api/v1/organizations/{organizationId}/support/cases
GET /api/v1/organizations/{organizationId}/support/cases/{caseId}
PATCH /api/v1/organizations/{organizationId}/support/cases/{caseId}
```

### Create Support Case Request

```json
{
  "title": "Import validation issue",
  "category": "data_migration",
  "priority": "medium",
  "summary": "Safe summary of the issue",
  "sourceReference": {
    "type": "import_job",
    "id": "optional"
  }
}
```

## Case Timeline

```text
GET /api/v1/organizations/{organizationId}/support/cases/{caseId}/timeline
POST /api/v1/organizations/{organizationId}/support/cases/{caseId}/timeline
```

### Timeline Update Request

```json
{
  "updateType": "customer_note",
  "message": "Safe support update",
  "visibility": "organization_visible"
}
```

## Escalation Placeholder

```text
GET /api/v1/organizations/{organizationId}/support/escalations
POST /api/v1/organizations/{organizationId}/support/cases/{caseId}/escalations
POST /api/v1/organizations/{organizationId}/support/escalations/{escalationId}/resolve-placeholder
```

## Onboarding Tracker

```text
GET /api/v1/organizations/{organizationId}/support/onboarding
POST /api/v1/organizations/{organizationId}/support/onboarding
PATCH /api/v1/organizations/{organizationId}/support/onboarding/{trackerId}
```

## Organization Health Reviews

```text
GET /api/v1/organizations/{organizationId}/support/health-reviews
POST /api/v1/organizations/{organizationId}/support/health-reviews
GET /api/v1/organizations/{organizationId}/support/health-reviews/{reviewId}
PATCH /api/v1/organizations/{organizationId}/support/health-reviews/{reviewId}
```

## Operational Checklist

```text
GET /api/v1/organizations/{organizationId}/support/checklist
PATCH /api/v1/organizations/{organizationId}/support/checklist/{itemId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership or permitted support role
- support feature availability
- required permission
- organization visibility scope
- source reference access before exposing source details

Support case source references must not cross organization boundaries.

## Status Direction

Suggested support case statuses:

```text
open
in_progress
waiting_on_customer_placeholder
waiting_on_platform_placeholder
escalated
resolved
closed
manual_review
```

Suggested priorities:

```text
low
medium
high
urgent_placeholder
```

Suggested health statuses:

```text
healthy
needs_attention_placeholder
at_risk_placeholder
blocked
manual_review
```

## Result Safety Direction

Support responses should expose safe labels and summaries by default.

Internal notes should require explicit internal-note permission.

## Audit Direction

Audit should be written for:

- support case created
- priority changed
- status changed
- internal support note added
- escalation added
- escalation resolved placeholder
- onboarding blocker changed
- organization health review updated

## Error Direction

Use standard response shape.

Expected error areas:

- support case not found
- timeline update not found
- escalation not found
- onboarding tracker not found
- health review not found
- internal note permission denied
- source reference unavailable
- organization not found

## Final Rule

Support APIs must preserve organization scope, safe visibility, source-module boundaries, and audit history for sensitive support actions.
