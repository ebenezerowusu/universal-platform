# Experimentation Controlled Rollouts Product Validation API Contract

## Purpose

This document defines the MVP API contract for Experimentation, Controlled Rollouts, and Product Validation foundation.

The API should support experimentation dashboards, experiments, variants, rollout cohorts, success metrics, guardrail metrics, decision reviews, feature/configuration links, analytics/feedback/support links, product communication links, and experiment audit history while preserving privacy and organization boundaries.

## Owner

```text
Experimentation Foundation
Permission Engine
Audit Engine
Configuration/Feature Flag Foundation
Product Analytics Foundation
Feedback Intake Foundation
Support Operations Foundation
Product Communication Foundation
Knowledge Base Foundation
Monitoring/Incident Foundation
Privacy Governance Foundation
```

## Base Path

```text
/api/v1/organizations/{organizationId}/experimentation
```

Platform-level experiments may later use stricter platform-admin routes.

## Experimentation Dashboard

```text
GET /api/v1/organizations/{organizationId}/experimentation/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activeExperiments": 3,
    "pausedExperiments": 1,
    "rolloutCohorts": 7,
    "openDecisionReviews": 2,
    "guardrailWarnings": 4
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Experiments

```text
GET /api/v1/organizations/{organizationId}/experimentation/experiments
POST /api/v1/organizations/{organizationId}/experimentation/experiments
GET /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}
PATCH /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}
POST /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/pause-placeholder
POST /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/complete-placeholder
```

## Variants

```text
GET /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/variants
POST /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/variants
GET /api/v1/organizations/{organizationId}/experimentation/variants/{variantId}
PATCH /api/v1/organizations/{organizationId}/experimentation/variants/{variantId}
```

## Rollout Cohorts

```text
GET /api/v1/organizations/{organizationId}/experimentation/rollout-cohorts
POST /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/rollout-cohorts
GET /api/v1/organizations/{organizationId}/experimentation/rollout-cohorts/{cohortId}
PATCH /api/v1/organizations/{organizationId}/experimentation/rollout-cohorts/{cohortId}
```

## Success Metrics

```text
GET /api/v1/organizations/{organizationId}/experimentation/success-metrics
POST /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/success-metrics
GET /api/v1/organizations/{organizationId}/experimentation/success-metrics/{metricId}
PATCH /api/v1/organizations/{organizationId}/experimentation/success-metrics/{metricId}
```

## Guardrail Metrics

```text
GET /api/v1/organizations/{organizationId}/experimentation/guardrail-metrics
POST /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/guardrail-metrics
GET /api/v1/organizations/{organizationId}/experimentation/guardrail-metrics/{metricId}
PATCH /api/v1/organizations/{organizationId}/experimentation/guardrail-metrics/{metricId}
```

## Decision Reviews

```text
GET /api/v1/organizations/{organizationId}/experimentation/decision-reviews
POST /api/v1/organizations/{organizationId}/experimentation/experiments/{experimentId}/decision-reviews
GET /api/v1/organizations/{organizationId}/experimentation/decision-reviews/{reviewId}
PATCH /api/v1/organizations/{organizationId}/experimentation/decision-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/experimentation/decision-reviews/{reviewId}/mark-reviewed-placeholder
```

## Feature and Configuration Links

```text
GET /api/v1/organizations/{organizationId}/experimentation/feature-configuration-links
POST /api/v1/organizations/{organizationId}/experimentation/feature-configuration-links
PATCH /api/v1/organizations/{organizationId}/experimentation/feature-configuration-links/{linkId}
```

## Analytics Feedback Support Links

```text
GET /api/v1/organizations/{organizationId}/experimentation/analytics-feedback-support-links
POST /api/v1/organizations/{organizationId}/experimentation/analytics-feedback-support-links
PATCH /api/v1/organizations/{organizationId}/experimentation/analytics-feedback-support-links/{linkId}
```

## Product Communication Links

```text
GET /api/v1/organizations/{organizationId}/experimentation/product-communication-links
POST /api/v1/organizations/{organizationId}/experimentation/product-communication-links
PATCH /api/v1/organizations/{organizationId}/experimentation/product-communication-links/{linkId}
```

## Experiment Audit History

```text
GET /api/v1/organizations/{organizationId}/experimentation/audit-history
GET /api/v1/organizations/{organizationId}/experimentation/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- experimentation feature availability
- required permission
- cohort visibility does not expose user-level cross-tenant data
- experiment links do not mutate feature flag state directly
- success and guardrail metric links are visible to actor
- privacy/consent restrictions apply to cohort and metric summaries

Experiments must not expose cross-organization user-level behavior.

## Status Direction

Suggested experiment statuses:

```text
draft
active_placeholder
paused_placeholder
completed_placeholder
stopped_placeholder
manual_review
```

Suggested decision statuses:

```text
pending_review
continue_placeholder
pause_placeholder
expand_rollout_placeholder
rollback_placeholder
stop_placeholder
manual_review
```

Suggested guardrail statuses:

```text
healthy_placeholder
warning_placeholder
breached_placeholder
restricted_by_privacy_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- experiment created or updated
- variant created or updated
- rollout cohort changed
- success metric link changed
- guardrail metric link changed
- decision review recorded
- feature/configuration link changed
- experiment status changed

## Error Direction

Use standard response shape.

Expected error areas:

- experiment not found
- variant not found
- rollout cohort not found
- success metric not found
- guardrail metric not found
- decision review not found
- linked feature/configuration not found
- privacy restriction applied
- experimentation permission denied
- organization not found

## Final Rule

Experimentation APIs must be privacy-aware, feature-flag-aware, analytics-linked, permission-protected, tenant-safe, and auditable.
