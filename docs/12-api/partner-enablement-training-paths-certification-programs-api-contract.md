# Partner Enablement Training Paths Certification Programs API Contract

## Purpose

This document defines the MVP API contract for Partner Enablement, Training Paths, and Certification Programs foundation.

The API should support partner enablement dashboards, enablement programs, training paths, content links, certification programs, assessments/checkpoints, certification attempts/results, renewals/expiry reviews, partner readiness links, and enablement audit history while preserving partner and tenant boundaries.

## Owner

```text
Partner Enablement Foundation
Permission Engine
Audit Engine
Partner Ecosystem Foundation
Knowledge Base Foundation
Policy Library Foundation
Solution Blueprint Foundation
Implementation Delivery Foundation
Customer Success Foundation
Notification and Workflow Foundations where reminders/reviews are consumed later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/partner-enablement
```

Platform-level enablement program authoring may later use stricter platform-admin routes.

## Partner Enablement Dashboard

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activePrograms": 5,
    "activeTrainingPaths": 9,
    "certificationPrograms": 7,
    "pendingCertificationReviews": 4,
    "expiringCertifications": 3
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Enablement Programs

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/programs
POST /api/v1/organizations/{organizationId}/partner-enablement/programs
GET /api/v1/organizations/{organizationId}/partner-enablement/programs/{programId}
PATCH /api/v1/organizations/{organizationId}/partner-enablement/programs/{programId}
POST /api/v1/organizations/{organizationId}/partner-enablement/programs/{programId}/retire-placeholder
```

## Training Paths

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/training-paths
POST /api/v1/organizations/{organizationId}/partner-enablement/programs/{programId}/training-paths
GET /api/v1/organizations/{organizationId}/partner-enablement/training-paths/{pathId}
PATCH /api/v1/organizations/{organizationId}/partner-enablement/training-paths/{pathId}
```

## Training Content Links

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/content-links
POST /api/v1/organizations/{organizationId}/partner-enablement/training-paths/{pathId}/content-links
GET /api/v1/organizations/{organizationId}/partner-enablement/content-links/{linkId}
PATCH /api/v1/organizations/{organizationId}/partner-enablement/content-links/{linkId}
DELETE /api/v1/organizations/{organizationId}/partner-enablement/content-links/{linkId}/remove-placeholder
```

## Certification Programs

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/certification-programs
POST /api/v1/organizations/{organizationId}/partner-enablement/certification-programs
GET /api/v1/organizations/{organizationId}/partner-enablement/certification-programs/{certificationProgramId}
PATCH /api/v1/organizations/{organizationId}/partner-enablement/certification-programs/{certificationProgramId}
POST /api/v1/organizations/{organizationId}/partner-enablement/certification-programs/{certificationProgramId}/retire-placeholder
```

## Assessments and Checkpoints

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/checkpoints
POST /api/v1/organizations/{organizationId}/partner-enablement/certification-programs/{certificationProgramId}/checkpoints
GET /api/v1/organizations/{organizationId}/partner-enablement/checkpoints/{checkpointId}
PATCH /api/v1/organizations/{organizationId}/partner-enablement/checkpoints/{checkpointId}
POST /api/v1/organizations/{organizationId}/partner-enablement/checkpoints/{checkpointId}/mark-reviewed-placeholder
```

## Certification Attempts and Results

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/certification-attempts
POST /api/v1/organizations/{organizationId}/partner-enablement/certification-programs/{certificationProgramId}/attempts
GET /api/v1/organizations/{organizationId}/partner-enablement/certification-attempts/{attemptId}
PATCH /api/v1/organizations/{organizationId}/partner-enablement/certification-attempts/{attemptId}
POST /api/v1/organizations/{organizationId}/partner-enablement/certification-attempts/{attemptId}/record-result-placeholder
POST /api/v1/organizations/{organizationId}/partner-enablement/certification-attempts/{attemptId}/mark-reviewed-placeholder
```

## Renewals and Expiry Reviews

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/renewals
POST /api/v1/organizations/{organizationId}/partner-enablement/certification-attempts/{attemptId}/renewal-review-placeholder
GET /api/v1/organizations/{organizationId}/partner-enablement/renewals/{renewalId}
PATCH /api/v1/organizations/{organizationId}/partner-enablement/renewals/{renewalId}
POST /api/v1/organizations/{organizationId}/partner-enablement/renewals/{renewalId}/mark-reviewed-placeholder
```

## Partner Readiness Links

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/readiness-links
POST /api/v1/organizations/{organizationId}/partner-enablement/readiness-links
PATCH /api/v1/organizations/{organizationId}/partner-enablement/readiness-links/{linkId}
```

## Enablement Audit History

```text
GET /api/v1/organizations/{organizationId}/partner-enablement/audit-history
GET /api/v1/organizations/{organizationId}/partner-enablement/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- partner enablement feature availability
- required permission
- linked content is visible to actor
- certification result does not automatically grant tenant access
- partner readiness links respect partner ecosystem visibility
- expired certification warnings are visible where practical
- certification review is audit logged

Enablement records must not expose unauthorized partner or tenant details.

## Status Direction

Suggested program statuses:

```text
draft
active_placeholder
retired_placeholder
manual_review
```

Suggested certification result statuses:

```text
pending_review
passed_placeholder
not_passed_placeholder
certified_placeholder
expired_placeholder
revoked_placeholder
manual_review
```

Suggested renewal statuses:

```text
not_required
required_placeholder
in_progress_placeholder
renewed_placeholder
expired_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- enablement program created or updated
- training path created or updated
- content link added or removed placeholder
- certification program created or updated
- assessment/checkpoint created or reviewed placeholder
- certification attempt recorded placeholder
- certification result reviewed placeholder
- renewal/expiry status changed placeholder
- partner readiness link changed

## Error Direction

Use standard response shape.

Expected error areas:

- enablement program not found
- training path not found
- content link not found
- certification program not found
- checkpoint not found
- certification attempt not found
- renewal not found
- readiness link not found
- partner enablement permission denied
- organization not found

## Final Rule

Partner enablement APIs must be permission-protected, content-aware, review-aware, certification-boundary-safe, renewal-aware, and auditable.
