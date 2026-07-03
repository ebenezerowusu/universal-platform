# Solution Blueprints Configuration Templates Deployment Packages API Contract

## Purpose

This document defines the MVP API contract for Solution Blueprints, Configuration Templates, and Deployment Packages foundation.

The API should support solution blueprint dashboards, blueprints, blueprint versions, configuration templates, deployment packages, component manifests, compatibility/preflight checks, preview/apply plans, rollback/change summaries, related implementation/customer success links, and blueprint audit history while preserving tenant boundaries.

## Owner

```text
Solution Blueprint Foundation
Permission Engine
Audit Engine
Configuration/Feature Flag Foundation
Module Registry Foundation
Workflow Engine
Localization Foundation
Reporting/Analytics Foundation
Policy Library Foundation
Knowledge Base Foundation
Implementation Delivery Foundation
Customer Success Foundation
```

## Base Path

```text
/api/v1/organizations/{organizationId}/solution-blueprints
```

Platform-level blueprint authoring may later use stricter platform-admin routes.

## Solution Blueprint Dashboard

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "availableBlueprints": 12,
    "activeDeploymentPackages": 4,
    "failedPreflightChecks": 2,
    "pendingApplyPlans": 3,
    "recentPackageApplies": 5
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Blueprints

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/blueprints
POST /api/v1/organizations/{organizationId}/solution-blueprints/blueprints
GET /api/v1/organizations/{organizationId}/solution-blueprints/blueprints/{blueprintId}
PATCH /api/v1/organizations/{organizationId}/solution-blueprints/blueprints/{blueprintId}
POST /api/v1/organizations/{organizationId}/solution-blueprints/blueprints/{blueprintId}/retire-placeholder
```

## Blueprint Versions

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/blueprints/{blueprintId}/versions
POST /api/v1/organizations/{organizationId}/solution-blueprints/blueprints/{blueprintId}/versions
GET /api/v1/organizations/{organizationId}/solution-blueprints/versions/{versionId}
PATCH /api/v1/organizations/{organizationId}/solution-blueprints/versions/{versionId}
POST /api/v1/organizations/{organizationId}/solution-blueprints/versions/{versionId}/publish-placeholder
```

## Configuration Templates

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/configuration-templates
POST /api/v1/organizations/{organizationId}/solution-blueprints/configuration-templates
GET /api/v1/organizations/{organizationId}/solution-blueprints/configuration-templates/{templateId}
PATCH /api/v1/organizations/{organizationId}/solution-blueprints/configuration-templates/{templateId}
```

## Deployment Packages

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages
POST /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages
GET /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages/{packageId}
PATCH /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages/{packageId}
POST /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages/{packageId}/archive-placeholder
```

## Component Manifests

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/component-manifests
POST /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages/{packageId}/component-manifests
GET /api/v1/organizations/{organizationId}/solution-blueprints/component-manifests/{manifestId}
PATCH /api/v1/organizations/{organizationId}/solution-blueprints/component-manifests/{manifestId}
```

## Compatibility and Preflight Checks

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/preflight-checks
POST /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages/{packageId}/run-preflight-placeholder
GET /api/v1/organizations/{organizationId}/solution-blueprints/preflight-checks/{checkId}
POST /api/v1/organizations/{organizationId}/solution-blueprints/preflight-checks/{checkId}/mark-reviewed-placeholder
```

## Preview and Apply Plans

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/apply-plans
POST /api/v1/organizations/{organizationId}/solution-blueprints/deployment-packages/{packageId}/generate-apply-plan-placeholder
GET /api/v1/organizations/{organizationId}/solution-blueprints/apply-plans/{planId}
POST /api/v1/organizations/{organizationId}/solution-blueprints/apply-plans/{planId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/solution-blueprints/apply-plans/{planId}/record-apply-placeholder
```

## Rollback and Change Summaries

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/change-summaries
POST /api/v1/organizations/{organizationId}/solution-blueprints/apply-plans/{planId}/change-summary-placeholder
GET /api/v1/organizations/{organizationId}/solution-blueprints/change-summaries/{summaryId}
```

## Related Implementation Customer Success Links

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/related-links
POST /api/v1/organizations/{organizationId}/solution-blueprints/related-links
PATCH /api/v1/organizations/{organizationId}/solution-blueprints/related-links/{linkId}
```

## Blueprint Audit History

```text
GET /api/v1/organizations/{organizationId}/solution-blueprints/audit-history
GET /api/v1/organizations/{organizationId}/solution-blueprints/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- solution blueprint feature availability
- required permission
- module entitlement and package compatibility
- feature flag/configuration rule compatibility where practical
- preview/apply plan approval before apply placeholder is recorded
- package components do not execute arbitrary scripts
- Core remains domain-agnostic

Blueprint records must not expose unauthorized tenant configuration details.

## Status Direction

Suggested blueprint statuses:

```text
draft
published_placeholder
retired_placeholder
manual_review
```

Suggested package statuses:

```text
draft
ready_placeholder
preflight_failed_placeholder
apply_plan_pending_placeholder
applied_placeholder
archived_placeholder
manual_review
```

Suggested preflight statuses:

```text
pending_placeholder
passed_placeholder
failed_placeholder
warning_placeholder
manual_review
```

Suggested apply plan statuses:

```text
generated_placeholder
approved_placeholder
recorded_applied_placeholder
cancelled_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- blueprint created or updated
- blueprint version created or published placeholder
- configuration template created or updated
- deployment package created or updated
- component manifest changed
- preflight validation executed placeholder
- preview/apply plan generated placeholder
- package apply placeholder recorded
- rollback/change summary recorded

## Error Direction

Use standard response shape.

Expected error areas:

- blueprint not found
- blueprint version not found
- configuration template not found
- deployment package not found
- component manifest not found
- preflight check failed
- apply plan not approved
- module entitlement missing
- solution blueprint permission denied
- organization not found

## Final Rule

Solution blueprint APIs must be tenant-safe, entitlement-aware, preview-first, permission-protected, Core-safe, and auditable.
