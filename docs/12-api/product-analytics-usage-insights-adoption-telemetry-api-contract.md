# Product Analytics Usage Insights Adoption Telemetry API Contract

## Purpose

This document defines the MVP API contract for Product Analytics, Usage Insights, and Adoption Telemetry foundation.

The API should support product analytics dashboards, usage event definitions, feature adoption metrics, module engagement summaries, onboarding funnels, activation/retention summaries, rollout adoption links, feedback/support correlations, aggregation rules, and analytics audit history while preserving privacy and organization boundaries.

## Owner

```text
Product Analytics Foundation
Permission Engine
Audit Engine
Reporting/Analytics Foundation
Privacy Governance Foundation
Configuration/Feature Flag Foundation
Feedback Intake Foundation
Support Operations Foundation
Knowledge Base Foundation
Product Communication Foundation
```

## Base Path

```text
/api/v1/organizations/{organizationId}/product-analytics
```

Platform-level analytics may later use stricter platform-admin routes with privacy-safe aggregation.

## Product Analytics Dashboard

```text
GET /api/v1/organizations/{organizationId}/product-analytics/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "trackedFeatures": 32,
    "activeModules": 8,
    "onboardingCompletionRatePlaceholder": 0.72,
    "featureAdoptionWarnings": 3,
    "supportCorrelationWarnings": 4
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Usage Event Definitions

```text
GET /api/v1/organizations/{organizationId}/product-analytics/usage-events
POST /api/v1/organizations/{organizationId}/product-analytics/usage-events
GET /api/v1/organizations/{organizationId}/product-analytics/usage-events/{eventKey}
PATCH /api/v1/organizations/{organizationId}/product-analytics/usage-events/{eventKey}
POST /api/v1/organizations/{organizationId}/product-analytics/usage-events/{eventKey}/disable-placeholder
```

## Feature Adoption Metrics

```text
GET /api/v1/organizations/{organizationId}/product-analytics/feature-adoption
GET /api/v1/organizations/{organizationId}/product-analytics/feature-adoption/{featureKey}
POST /api/v1/organizations/{organizationId}/product-analytics/feature-adoption/{featureKey}/refresh-summary-placeholder
```

## Module Engagement Summaries

```text
GET /api/v1/organizations/{organizationId}/product-analytics/module-engagement
GET /api/v1/organizations/{organizationId}/product-analytics/module-engagement/{moduleKey}
POST /api/v1/organizations/{organizationId}/product-analytics/module-engagement/{moduleKey}/refresh-summary-placeholder
```

## Onboarding Funnels

```text
GET /api/v1/organizations/{organizationId}/product-analytics/onboarding-funnels
POST /api/v1/organizations/{organizationId}/product-analytics/onboarding-funnels
GET /api/v1/organizations/{organizationId}/product-analytics/onboarding-funnels/{funnelId}
PATCH /api/v1/organizations/{organizationId}/product-analytics/onboarding-funnels/{funnelId}
```

## Activation and Retention Summaries

```text
GET /api/v1/organizations/{organizationId}/product-analytics/activation-retention
GET /api/v1/organizations/{organizationId}/product-analytics/activation-retention/{summaryId}
POST /api/v1/organizations/{organizationId}/product-analytics/activation-retention/refresh-summary-placeholder
```

## Rollout Adoption Links

```text
GET /api/v1/organizations/{organizationId}/product-analytics/rollout-adoption-links
POST /api/v1/organizations/{organizationId}/product-analytics/rollout-adoption-links
GET /api/v1/organizations/{organizationId}/product-analytics/rollout-adoption-links/{linkId}
PATCH /api/v1/organizations/{organizationId}/product-analytics/rollout-adoption-links/{linkId}
```

## Feedback Support Correlations

```text
GET /api/v1/organizations/{organizationId}/product-analytics/feedback-support-correlations
POST /api/v1/organizations/{organizationId}/product-analytics/feedback-support-correlations
GET /api/v1/organizations/{organizationId}/product-analytics/feedback-support-correlations/{correlationId}
POST /api/v1/organizations/{organizationId}/product-analytics/feedback-support-correlations/{correlationId}/mark-reviewed-placeholder
```

## Aggregation Rules

```text
GET /api/v1/organizations/{organizationId}/product-analytics/aggregation-rules
POST /api/v1/organizations/{organizationId}/product-analytics/aggregation-rules
GET /api/v1/organizations/{organizationId}/product-analytics/aggregation-rules/{ruleId}
PATCH /api/v1/organizations/{organizationId}/product-analytics/aggregation-rules/{ruleId}
```

## Analytics Audit History

```text
GET /api/v1/organizations/{organizationId}/product-analytics/audit-history
GET /api/v1/organizations/{organizationId}/product-analytics/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- product analytics feature availability
- required permission
- aggregation threshold rules before returning analytics
- privacy/consent restrictions where usage signals are sensitive
- analytics view does not expose raw audit/event replay
- platform-level analytics remains aggregated and tenant-safe

Analytics records must not expose cross-organization user-level behavior.

## Status Direction

Suggested usage event statuses:

```text
active
inactive
deprecated_placeholder
manual_review
```

Suggested summary statuses:

```text
pending_placeholder
ready_placeholder
restricted_by_privacy_placeholder
insufficient_sample_placeholder
manual_review
```

Suggested correlation statuses:

```text
open
reviewed_placeholder
actioned_placeholder
ignored_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- usage event definition created or updated
- aggregation rule changed
- analytics view accessed placeholder
- feature adoption summary generated placeholder
- rollout/adoption link changed
- feedback/support correlation reviewed placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- usage event not found
- feature adoption summary not found
- module engagement summary not found
- onboarding funnel not found
- activation/retention summary unavailable
- aggregation threshold not met
- privacy restriction applied
- product analytics permission denied
- organization not found

## Final Rule

Product analytics APIs must be aggregation-aware, privacy-aware, permission-protected, tenant-safe, and auditable.
