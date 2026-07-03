# Implementation Projects Onboarding Delivery Migration Runbooks API Contract

## Purpose

This document defines the MVP API contract for Implementation Projects, Onboarding Delivery, and Migration Runbooks foundation.

The API should support implementation dashboards, implementation projects, onboarding milestones, migration runbooks, setup tasks, go-live readiness reviews, implementation issues/risks, delivery links, implementation notes, and implementation audit history while preserving tenant boundaries.

## Owner

```text
Implementation Delivery Foundation
Permission Engine
Audit Engine
Organization Lifecycle Foundation
Import/Export and Data Migration Foundation
Data Quality Foundation
Customer Success Foundation
Support Operations Foundation
Knowledge Base Foundation
Notification and Workflow Foundations where tasks/alerts are consumed later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/implementation-delivery
```

Platform-level delivery views may later use stricter platform-admin routes.

## Implementation Dashboard

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activeProjects": 6,
    "projectsAtRisk": 2,
    "openSetupTasks": 24,
    "openMigrationRunbooks": 3,
    "goLiveReadyProjects": 1
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Implementation Projects

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/projects
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects
GET /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}/mark-reviewed-placeholder
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}/close-placeholder
```

## Onboarding Milestones

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/milestones
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}/milestones
GET /api/v1/organizations/{organizationId}/implementation-delivery/milestones/{milestoneId}
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/milestones/{milestoneId}
POST /api/v1/organizations/{organizationId}/implementation-delivery/milestones/{milestoneId}/complete-placeholder
```

## Migration Runbooks

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/migration-runbooks
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}/migration-runbooks
GET /api/v1/organizations/{organizationId}/implementation-delivery/migration-runbooks/{runbookId}
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/migration-runbooks/{runbookId}
POST /api/v1/organizations/{organizationId}/implementation-delivery/migration-runbooks/{runbookId}/mark-reviewed-placeholder
```

## Setup Tasks

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/setup-tasks
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}/setup-tasks
GET /api/v1/organizations/{organizationId}/implementation-delivery/setup-tasks/{taskId}
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/setup-tasks/{taskId}
POST /api/v1/organizations/{organizationId}/implementation-delivery/setup-tasks/{taskId}/complete-placeholder
```

## Go-Live Readiness Reviews

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/go-live-readiness-reviews
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}/go-live-readiness-reviews
GET /api/v1/organizations/{organizationId}/implementation-delivery/go-live-readiness-reviews/{reviewId}
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/go-live-readiness-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/implementation-delivery/go-live-readiness-reviews/{reviewId}/mark-ready-placeholder
POST /api/v1/organizations/{organizationId}/implementation-delivery/go-live-readiness-reviews/{reviewId}/mark-not-ready-placeholder
```

## Implementation Issues and Risks

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/issues-risks
POST /api/v1/organizations/{organizationId}/implementation-delivery/projects/{projectId}/issues-risks
GET /api/v1/organizations/{organizationId}/implementation-delivery/issues-risks/{issueRiskId}
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/issues-risks/{issueRiskId}
POST /api/v1/organizations/{organizationId}/implementation-delivery/issues-risks/{issueRiskId}/resolve-placeholder
```

## Delivery Links

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/related-links
POST /api/v1/organizations/{organizationId}/implementation-delivery/related-links
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/related-links/{linkId}
```

## Implementation Notes

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/implementation-notes
POST /api/v1/organizations/{organizationId}/implementation-delivery/implementation-notes
GET /api/v1/organizations/{organizationId}/implementation-delivery/implementation-notes/{noteId}
PATCH /api/v1/organizations/{organizationId}/implementation-delivery/implementation-notes/{noteId}
```

## Implementation Audit History

```text
GET /api/v1/organizations/{organizationId}/implementation-delivery/audit-history
GET /api/v1/organizations/{organizationId}/implementation-delivery/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- implementation delivery feature availability
- required permission
- project belongs to visible organization scope
- migration links are visible and do not execute destructive migration directly
- data quality status is checked before readiness where practical
- go-live readiness is reviewed and not automatically approved

Implementation delivery records must not expose unauthorized cross-tenant implementation details.

## Status Direction

Suggested project statuses:

```text
draft
active_placeholder
at_risk_placeholder
go_live_ready_placeholder
closed_placeholder
manual_review
```

Suggested milestone statuses:

```text
not_started
in_progress_placeholder
completed_placeholder
blocked_placeholder
manual_review
```

Suggested readiness statuses:

```text
not_ready_placeholder
ready_with_risks_placeholder
ready_placeholder
blocked_placeholder
manual_review
```

Suggested issue/risk statuses:

```text
open
in_progress_placeholder
resolved_placeholder
accepted_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- implementation project created or updated
- onboarding milestone updated
- migration runbook created or updated
- migration task link changed
- setup task created or completed placeholder
- readiness review recorded
- implementation issue/risk created or resolved placeholder
- go-live readiness status changed

## Error Direction

Use standard response shape.

Expected error areas:

- implementation project not found
- onboarding milestone not found
- migration runbook not found
- setup task not found
- readiness review not found
- implementation issue/risk not found
- related link not found
- implementation note not found
- implementation delivery permission denied
- organization not found

## Final Rule

Implementation delivery APIs must be tenant-safe, permission-protected, migration-aware, data-quality-aware, support-linked, customer-success-linked, and auditable.
