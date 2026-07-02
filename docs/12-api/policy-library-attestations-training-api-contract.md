# Policy Library Attestations Training API Contract

## Purpose

This document defines the MVP API contract for Policy Library, Attestations, and Training Acknowledgement foundation.

The API should support policy dashboards, policy documents, policy versions, publication reviews, attestation requests, acknowledgement records, training assignments, audience targets, evidence links, and policy audit history while preserving organization boundaries.

## Owner

```text
Policy Library Foundation
Permission Engine
Audit Engine
Document/Media Foundation
Risk and Controls Foundation
HR/Workforce Foundation
Notification and Workflow Foundations where reminders/reviews are consumed later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/policy-library
```

## Policy Library Dashboard

```text
GET /api/v1/organizations/{organizationId}/policy-library/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activePolicies": 14,
    "pendingAttestations": 42,
    "overdueAcknowledgements": 5,
    "activeTrainingAssignments": 8,
    "policiesDueForReview": 2
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Policies

```text
GET /api/v1/organizations/{organizationId}/policy-library/policies
POST /api/v1/organizations/{organizationId}/policy-library/policies
GET /api/v1/organizations/{organizationId}/policy-library/policies/{policyId}
PATCH /api/v1/organizations/{organizationId}/policy-library/policies/{policyId}
POST /api/v1/organizations/{organizationId}/policy-library/policies/{policyId}/retire-placeholder
```

## Policy Versions

```text
GET /api/v1/organizations/{organizationId}/policy-library/policies/{policyId}/versions
POST /api/v1/organizations/{organizationId}/policy-library/policies/{policyId}/versions
GET /api/v1/organizations/{organizationId}/policy-library/policy-versions/{versionId}
PATCH /api/v1/organizations/{organizationId}/policy-library/policy-versions/{versionId}
```

## Publication Reviews

```text
GET /api/v1/organizations/{organizationId}/policy-library/publication-reviews
POST /api/v1/organizations/{organizationId}/policy-library/policy-versions/{versionId}/publication-reviews
GET /api/v1/organizations/{organizationId}/policy-library/publication-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/policy-library/publication-reviews/{reviewId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/policy-library/publication-reviews/{reviewId}/publish-placeholder
```

## Attestation Requests

```text
GET /api/v1/organizations/{organizationId}/policy-library/attestation-requests
POST /api/v1/organizations/{organizationId}/policy-library/attestation-requests
GET /api/v1/organizations/{organizationId}/policy-library/attestation-requests/{requestId}
PATCH /api/v1/organizations/{organizationId}/policy-library/attestation-requests/{requestId}
POST /api/v1/organizations/{organizationId}/policy-library/attestation-requests/{requestId}/close-placeholder
```

### Create Attestation Request

```json
{
  "policyVersionId": "...",
  "targetAudiencePlaceholder": {
    "type": "role_placeholder",
    "id": "branch-admin"
  },
  "dueDatePlaceholder": "optional",
  "requiresTrainingPlaceholder": false
}
```

## Acknowledgement Records

```text
GET /api/v1/organizations/{organizationId}/policy-library/acknowledgements
POST /api/v1/organizations/{organizationId}/policy-library/attestation-requests/{requestId}/acknowledge-placeholder
GET /api/v1/organizations/{organizationId}/policy-library/acknowledgements/{acknowledgementId}
```

## Training Assignments

```text
GET /api/v1/organizations/{organizationId}/policy-library/training-assignments
POST /api/v1/organizations/{organizationId}/policy-library/training-assignments
GET /api/v1/organizations/{organizationId}/policy-library/training-assignments/{assignmentId}
PATCH /api/v1/organizations/{organizationId}/policy-library/training-assignments/{assignmentId}
POST /api/v1/organizations/{organizationId}/policy-library/training-assignments/{assignmentId}/complete-placeholder
```

## Audience Targets

```text
GET /api/v1/organizations/{organizationId}/policy-library/audience-targets
POST /api/v1/organizations/{organizationId}/policy-library/audience-targets
GET /api/v1/organizations/{organizationId}/policy-library/audience-targets/{targetId}
PATCH /api/v1/organizations/{organizationId}/policy-library/audience-targets/{targetId}
```

## Evidence Links

```text
GET /api/v1/organizations/{organizationId}/policy-library/evidence-links
POST /api/v1/organizations/{organizationId}/policy-library/evidence-links
GET /api/v1/organizations/{organizationId}/policy-library/evidence-links/{evidenceLinkId}
DELETE /api/v1/organizations/{organizationId}/policy-library/evidence-links/{evidenceLinkId}/remove-placeholder
```

## Policy Audit History

```text
GET /api/v1/organizations/{organizationId}/policy-library/audit-history
GET /api/v1/organizations/{organizationId}/policy-library/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- policy library feature availability
- required permission
- policy document reference is visible to actor
- audience target belongs to the same organization where applicable
- acknowledgement actor matches expected target where required
- evidence links respect risk/control and document permissions

Policy records must not cross organization boundaries.

## Status Direction

Suggested policy statuses:

```text
draft
in_review_placeholder
published_placeholder
retired_placeholder
manual_review
```

Suggested attestation statuses:

```text
open
in_progress_placeholder
completed_placeholder
overdue_placeholder
closed_placeholder
manual_review
```

Suggested acknowledgement statuses:

```text
pending
acknowledged_placeholder
declined_placeholder
overdue_placeholder
manual_review
```

Suggested training assignment statuses:

```text
assigned
in_progress_placeholder
completed_placeholder
overdue_placeholder
cancelled_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- policy created or updated
- policy version created or retired placeholder
- policy published placeholder
- attestation request created
- acknowledgement captured
- training assignment created
- training completion captured placeholder
- evidence link added or removed placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- policy not found
- policy version not found
- publication review not found
- attestation request not found
- acknowledgement not found
- training assignment not found
- audience target not found
- policy document permission denied
- policy library permission denied
- organization not found

## Final Rule

Policy library APIs must be organization-scoped, document-aware, audience-aware, permission-protected, and auditable.
