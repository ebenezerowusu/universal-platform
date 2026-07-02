# Configuration Feature Flags Rules API Contract

## Purpose

This document defines the MVP API contract for Configuration, Feature Flags, and Runtime Rules foundation.

The API should support configuration dashboards, namespaces, keys, defaults, tenant overrides, feature flags, rollout placeholders, runtime rule placeholders, evaluation logs, and change history while preserving tenant boundaries.

## Owner

```text
Configuration Foundation
Permission Engine
Audit Engine
Module Registry Engine
Subscription/Feature Entitlement Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/configuration
```

Platform-admin global configuration routes may be introduced later with stricter controls.

## Configuration Dashboard

```text
GET /api/v1/organizations/{organizationId}/configuration/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "namespaces": 12,
    "tenantOverrides": 7,
    "activeFlags": 18,
    "rolloutWarnings": 2,
    "recentChanges": 5
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Namespaces

```text
GET /api/v1/organizations/{organizationId}/configuration/namespaces
POST /api/v1/organizations/{organizationId}/configuration/namespaces
GET /api/v1/organizations/{organizationId}/configuration/namespaces/{namespaceKey}
PATCH /api/v1/organizations/{organizationId}/configuration/namespaces/{namespaceKey}
```

Namespace create/update should be platform-admin or configuration-admin only where applicable.

## Configuration Keys

```text
GET /api/v1/organizations/{organizationId}/configuration/keys
POST /api/v1/organizations/{organizationId}/configuration/keys
GET /api/v1/organizations/{organizationId}/configuration/keys/{keyId}
PATCH /api/v1/organizations/{organizationId}/configuration/keys/{keyId}
```

### Create Configuration Key Request

```json
{
  "namespaceKey": "notifications",
  "key": "sms.enabled_placeholder",
  "valueType": "boolean",
  "defaultValuePlaceholder": false,
  "validationPlaceholder": "optional"
}
```

## Tenant Overrides

```text
GET /api/v1/organizations/{organizationId}/configuration/overrides
POST /api/v1/organizations/{organizationId}/configuration/overrides
GET /api/v1/organizations/{organizationId}/configuration/overrides/{overrideId}
PATCH /api/v1/organizations/{organizationId}/configuration/overrides/{overrideId}
POST /api/v1/organizations/{organizationId}/configuration/overrides/{overrideId}/disable-placeholder
```

## Feature Flags

```text
GET /api/v1/organizations/{organizationId}/configuration/feature-flags
POST /api/v1/organizations/{organizationId}/configuration/feature-flags
GET /api/v1/organizations/{organizationId}/configuration/feature-flags/{flagKey}
PATCH /api/v1/organizations/{organizationId}/configuration/feature-flags/{flagKey}
POST /api/v1/organizations/{organizationId}/configuration/feature-flags/{flagKey}/enable-placeholder
POST /api/v1/organizations/{organizationId}/configuration/feature-flags/{flagKey}/disable-placeholder
```

## Rollout Rule Placeholders

```text
GET /api/v1/organizations/{organizationId}/configuration/rollout-rules
POST /api/v1/organizations/{organizationId}/configuration/rollout-rules
GET /api/v1/organizations/{organizationId}/configuration/rollout-rules/{ruleId}
PATCH /api/v1/organizations/{organizationId}/configuration/rollout-rules/{ruleId}
```

## Runtime Rule Placeholders

```text
GET /api/v1/organizations/{organizationId}/configuration/runtime-rules
POST /api/v1/organizations/{organizationId}/configuration/runtime-rules
GET /api/v1/organizations/{organizationId}/configuration/runtime-rules/{ruleId}
PATCH /api/v1/organizations/{organizationId}/configuration/runtime-rules/{ruleId}
POST /api/v1/organizations/{organizationId}/configuration/runtime-rules/{ruleId}/evaluate-placeholder
```

Runtime rules must be declarative placeholders only in this sprint.

## Evaluation Logs

```text
GET /api/v1/organizations/{organizationId}/configuration/evaluation-logs
GET /api/v1/organizations/{organizationId}/configuration/evaluation-logs/{evaluationId}
```

## Change History

```text
GET /api/v1/organizations/{organizationId}/configuration/change-history
GET /api/v1/organizations/{organizationId}/configuration/change-history/{changeId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- configuration feature availability
- required permission
- organization ownership for tenant overrides
- entitlement compatibility where flags depend on plans/modules
- safe value type validation where practical

Configuration records must not cross organization boundaries.

## Status Direction

Suggested configuration statuses:

```text
active
inactive
deprecated_placeholder
manual_review
```

Suggested rollout statuses:

```text
draft
active
paused_placeholder
completed_placeholder
manual_review
```

Suggested rule statuses:

```text
draft
active
disabled_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- namespace created or updated
- configuration key created or updated
- tenant override changed
- feature flag changed
- rollout rule placeholder changed
- runtime rule placeholder changed
- default/fallback changed

## Error Direction

Use standard response shape.

Expected error areas:

- namespace not found
- configuration key not found
- tenant override not found
- feature flag not found
- rollout rule not found
- runtime rule not found
- invalid value type
- entitlement conflict
- configuration permission denied
- organization not found

## Final Rule

Configuration APIs must be organization-scoped where relevant, permission-protected, entitlement-aware, declarative, default-safe, and auditable.
