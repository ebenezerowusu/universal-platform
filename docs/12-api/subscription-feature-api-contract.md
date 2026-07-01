# Subscription and Feature API Contract

## Purpose

This document defines the MVP Subscription and Feature API contract.

Subscription and feature checks answer:

```text
Is this organization commercially allowed to use this feature or capacity?
```

## Owner

```text
Subscription Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}
```

## Get Current Subscription

```text
GET /api/v1/organizations/{organizationId}/subscription
```

### Success Response

```json
{
  "success": true,
  "data": {
    "subscription": {
      "planCode": "starter",
      "status": "active",
      "startedAt": "...",
      "expiresAt": null
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## List Organization Features

```text
GET /api/v1/organizations/{organizationId}/features
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "key": "religious.members.manage",
        "available": true,
        "source": "plan"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## List Subscription Plans

```text
GET /api/v1/platform/subscription-plans
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "code": "starter",
        "name": "Starter",
        "features": [],
        "limits": []
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Internal Feature Check Contract

Application services should use an internal service contract similar to:

```text
is_feature_available(organization_id, feature_key) -> available/unavailable
```

## Internal Limit Check Contract

Application services should use an internal service contract similar to:

```text
check_limit(organization_id, limit_key, requested_quantity) -> allowed/denied
```

## Availability Inputs

Feature availability may later combine:

- organization subscription
- plan features
- module state
- feature flag
- license rules
- usage limits
- permission checks

Sprint 4 should implement only the foundation needed for MVP use.

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- organization not found
- permission denied
- subscription not found
- feature unavailable
- limit reached

## Final Rule

Subscription and feature checks must be centralized. Domains should not invent their own commercial access logic.
