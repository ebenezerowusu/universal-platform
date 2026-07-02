# Feedback Feature Requests Roadmap Intake API Contract

## Purpose

This document defines the MVP API contract for Feedback, Feature Requests, and Product Roadmap Intake foundation.

The API should support feedback dashboards, feedback items, feature requests, categories, duplicate/grouping review, prioritization summaries, roadmap candidates, support/help links, announcement/release links, status responses, and feedback audit history while preserving organization and visibility boundaries.

## Owner

```text
Feedback Intake Foundation
Permission Engine
Audit Engine
Support Operations Foundation
Knowledge Base Foundation
Product Communication Foundation
Search/Discovery Foundation
Data Quality Foundation where grouping support is reused later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/feedback-intake
```

## Feedback Dashboard

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "openFeedbackItems": 24,
    "openFeatureRequests": 13,
    "duplicateGroups": 6,
    "roadmapCandidates": 5,
    "itemsAwaitingResponse": 8
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Feedback Items

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/feedback-items
POST /api/v1/organizations/{organizationId}/feedback-intake/feedback-items
GET /api/v1/organizations/{organizationId}/feedback-intake/feedback-items/{feedbackId}
PATCH /api/v1/organizations/{organizationId}/feedback-intake/feedback-items/{feedbackId}
POST /api/v1/organizations/{organizationId}/feedback-intake/feedback-items/{feedbackId}/triage-placeholder
POST /api/v1/organizations/{organizationId}/feedback-intake/feedback-items/{feedbackId}/close-placeholder
```

### Create Feedback Item Request

```json
{
  "sourceChannel": "help_center_placeholder",
  "feedbackType": "feature_request_placeholder",
  "summary": "Short safe summary",
  "detailsPlaceholder": "Optional longer description",
  "relatedModulePlaceholder": "finance"
}
```

## Feature Requests

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/feature-requests
POST /api/v1/organizations/{organizationId}/feedback-intake/feature-requests
GET /api/v1/organizations/{organizationId}/feedback-intake/feature-requests/{requestId}
PATCH /api/v1/organizations/{organizationId}/feedback-intake/feature-requests/{requestId}
POST /api/v1/organizations/{organizationId}/feedback-intake/feature-requests/{requestId}/convert-to-roadmap-candidate-placeholder
```

## Categories

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/categories
POST /api/v1/organizations/{organizationId}/feedback-intake/categories
GET /api/v1/organizations/{organizationId}/feedback-intake/categories/{categoryId}
PATCH /api/v1/organizations/{organizationId}/feedback-intake/categories/{categoryId}
```

## Duplicate and Grouping Review

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/groups
POST /api/v1/organizations/{organizationId}/feedback-intake/groups
GET /api/v1/organizations/{organizationId}/feedback-intake/groups/{groupId}
PATCH /api/v1/organizations/{organizationId}/feedback-intake/groups/{groupId}
POST /api/v1/organizations/{organizationId}/feedback-intake/groups/{groupId}/add-item-placeholder
POST /api/v1/organizations/{organizationId}/feedback-intake/groups/{groupId}/remove-item-placeholder
```

## Prioritization Summaries

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/prioritization-summaries
POST /api/v1/organizations/{organizationId}/feedback-intake/feature-requests/{requestId}/prioritization-summary-placeholder
GET /api/v1/organizations/{organizationId}/feedback-intake/prioritization-summaries/{summaryId}
PATCH /api/v1/organizations/{organizationId}/feedback-intake/prioritization-summaries/{summaryId}
```

## Roadmap Candidates

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/roadmap-candidates
POST /api/v1/organizations/{organizationId}/feedback-intake/roadmap-candidates
GET /api/v1/organizations/{organizationId}/feedback-intake/roadmap-candidates/{candidateId}
PATCH /api/v1/organizations/{organizationId}/feedback-intake/roadmap-candidates/{candidateId}
POST /api/v1/organizations/{organizationId}/feedback-intake/roadmap-candidates/{candidateId}/mark-reviewed-placeholder
```

## Support and Knowledge Base Links

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/support-help-links
POST /api/v1/organizations/{organizationId}/feedback-intake/support-help-links
PATCH /api/v1/organizations/{organizationId}/feedback-intake/support-help-links/{linkId}
```

## Announcement and Release Links

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/release-communication-links
POST /api/v1/organizations/{organizationId}/feedback-intake/release-communication-links
PATCH /api/v1/organizations/{organizationId}/feedback-intake/release-communication-links/{linkId}
```

## Status Responses

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/status-responses
POST /api/v1/organizations/{organizationId}/feedback-intake/status-responses
GET /api/v1/organizations/{organizationId}/feedback-intake/status-responses/{responseId}
POST /api/v1/organizations/{organizationId}/feedback-intake/status-responses/{responseId}/send-placeholder
```

## Feedback Audit History

```text
GET /api/v1/organizations/{organizationId}/feedback-intake/audit-history
GET /api/v1/organizations/{organizationId}/feedback-intake/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- feedback intake feature availability
- required permission
- feedback item belongs to organization where organization-scoped
- cross-organization grouping uses safe summary metadata only
- support/help/release links are visible to actor
- roadmap candidate does not imply delivery commitment

Feedback records must not expose private details across organizations.

## Status Direction

Suggested feedback statuses:

```text
submitted
triaged_placeholder
linked_to_request_placeholder
closed_placeholder
manual_review
```

Suggested feature request statuses:

```text
open
under_review_placeholder
grouped_placeholder
roadmap_candidate_placeholder
addressed_placeholder
closed_placeholder
manual_review
```

Suggested roadmap candidate statuses:

```text
candidate
reviewed_placeholder
accepted_placeholder
deferred_placeholder
rejected_placeholder
released_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- feedback submitted
- feedback updated or triaged
- feature request created or updated
- duplicate/grouping changed
- prioritization placeholder changed
- roadmap candidate created or updated
- support/help link changed
- release/announcement link changed
- status response sent placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- feedback item not found
- feature request not found
- category not found
- duplicate/group not found
- prioritization summary not found
- roadmap candidate not found
- linked support/help item not found
- feedback intake permission denied
- organization not found

## Final Rule

Feedback intake APIs must be organization-aware, privacy-safe, support-linked, non-committal by default, permission-protected, and auditable.
