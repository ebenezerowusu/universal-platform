# Risk Register Controls Compliance Evidence API Contract

## Purpose

This document defines the MVP API contract for Risk Register, Internal Controls, and Compliance Evidence foundation.

The API should support risk/control dashboards, risk registers, risk assessments, control objectives, control checks, evidence links, exceptions, remediation actions, compliance checklist mappings, review cycles, and risk/control audit history while preserving organization boundaries.

## Owner

```text
Risk and Controls Foundation
Permission Engine
Audit Engine
Monitoring/Incident Foundation
Privacy Governance Foundation
Backup/Restore Foundation
Governance Authority Foundation
Document/Media Foundation where evidence files are linked
```

## Base Path

```text
/api/v1/organizations/{organizationId}/risk-controls
```

## Risk and Controls Dashboard

```text
GET /api/v1/organizations/{organizationId}/risk-controls/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "openRisks": 12,
    "highRisks": 3,
    "activeControls": 18,
    "openExceptions": 4,
    "overdueRemediations": 2
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Risk Register

```text
GET /api/v1/organizations/{organizationId}/risk-controls/risks
POST /api/v1/organizations/{organizationId}/risk-controls/risks
GET /api/v1/organizations/{organizationId}/risk-controls/risks/{riskId}
PATCH /api/v1/organizations/{organizationId}/risk-controls/risks/{riskId}
POST /api/v1/organizations/{organizationId}/risk-controls/risks/{riskId}/close-placeholder
```

## Risk Assessments

```text
GET /api/v1/organizations/{organizationId}/risk-controls/assessments
POST /api/v1/organizations/{organizationId}/risk-controls/risks/{riskId}/assessments
GET /api/v1/organizations/{organizationId}/risk-controls/assessments/{assessmentId}
PATCH /api/v1/organizations/{organizationId}/risk-controls/assessments/{assessmentId}
```

## Control Objectives

```text
GET /api/v1/organizations/{organizationId}/risk-controls/control-objectives
POST /api/v1/organizations/{organizationId}/risk-controls/control-objectives
GET /api/v1/organizations/{organizationId}/risk-controls/control-objectives/{controlId}
PATCH /api/v1/organizations/{organizationId}/risk-controls/control-objectives/{controlId}
```

## Control Checks

```text
GET /api/v1/organizations/{organizationId}/risk-controls/control-checks
POST /api/v1/organizations/{organizationId}/risk-controls/control-objectives/{controlId}/checks
GET /api/v1/organizations/{organizationId}/risk-controls/control-checks/{checkId}
POST /api/v1/organizations/{organizationId}/risk-controls/control-checks/{checkId}/complete-placeholder
POST /api/v1/organizations/{organizationId}/risk-controls/control-checks/{checkId}/mark-failed-placeholder
```

## Evidence Links

```text
GET /api/v1/organizations/{organizationId}/risk-controls/evidence-links
POST /api/v1/organizations/{organizationId}/risk-controls/evidence-links
GET /api/v1/organizations/{organizationId}/risk-controls/evidence-links/{evidenceLinkId}
DELETE /api/v1/organizations/{organizationId}/risk-controls/evidence-links/{evidenceLinkId}/remove-placeholder
```

## Control Exceptions

```text
GET /api/v1/organizations/{organizationId}/risk-controls/exceptions
POST /api/v1/organizations/{organizationId}/risk-controls/exceptions
GET /api/v1/organizations/{organizationId}/risk-controls/exceptions/{exceptionId}
PATCH /api/v1/organizations/{organizationId}/risk-controls/exceptions/{exceptionId}
POST /api/v1/organizations/{organizationId}/risk-controls/exceptions/{exceptionId}/mark-reviewed-placeholder
```

## Remediation Actions

```text
GET /api/v1/organizations/{organizationId}/risk-controls/remediations
POST /api/v1/organizations/{organizationId}/risk-controls/remediations
GET /api/v1/organizations/{organizationId}/risk-controls/remediations/{remediationId}
PATCH /api/v1/organizations/{organizationId}/risk-controls/remediations/{remediationId}
POST /api/v1/organizations/{organizationId}/risk-controls/remediations/{remediationId}/complete-placeholder
```

## Compliance Checklist Mappings

```text
GET /api/v1/organizations/{organizationId}/risk-controls/checklist-mappings
POST /api/v1/organizations/{organizationId}/risk-controls/checklist-mappings
GET /api/v1/organizations/{organizationId}/risk-controls/checklist-mappings/{mappingId}
PATCH /api/v1/organizations/{organizationId}/risk-controls/checklist-mappings/{mappingId}
```

## Review Cycles

```text
GET /api/v1/organizations/{organizationId}/risk-controls/review-cycles
POST /api/v1/organizations/{organizationId}/risk-controls/review-cycles
GET /api/v1/organizations/{organizationId}/risk-controls/review-cycles/{cycleId}
PATCH /api/v1/organizations/{organizationId}/risk-controls/review-cycles/{cycleId}
```

## Risk Control Audit History

```text
GET /api/v1/organizations/{organizationId}/risk-controls/audit-history
GET /api/v1/organizations/{organizationId}/risk-controls/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- risk and controls feature availability
- required permission
- evidence references are safe for actor to view
- evidence belongs to the same organization where applicable
- exception and remediation actions are audit logged
- checklist mappings do not claim legal certification

Risk/control records must not cross organization boundaries.

## Status Direction

Suggested risk statuses:

```text
open
in_review_placeholder
accepted_placeholder
mitigating_placeholder
closed_placeholder
manual_review
```

Suggested control check statuses:

```text
pending
passed_placeholder
failed_placeholder
waived_placeholder
manual_review
```

Suggested exception statuses:

```text
open
reviewed_placeholder
accepted_placeholder
remediation_required_placeholder
closed_placeholder
manual_review
```

Suggested remediation statuses:

```text
open
in_progress_placeholder
completed_placeholder
overdue_placeholder
cancelled_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- risk created or updated
- risk assessment updated
- control objective created or updated
- control check completed placeholder
- evidence linked or removed placeholder
- exception created or reviewed
- remediation action created or completed
- checklist mapping changed

## Error Direction

Use standard response shape.

Expected error areas:

- risk not found
- assessment not found
- control objective not found
- control check not found
- evidence link not found
- exception not found
- remediation action not found
- checklist mapping not found
- evidence permission denied
- risk controls permission denied
- organization not found

## Final Rule

Risk and controls APIs must be organization-scoped, evidence-aware, permission-protected, audit-safe, and explicit that checklist mappings are operational readiness tools rather than legal certifications.
