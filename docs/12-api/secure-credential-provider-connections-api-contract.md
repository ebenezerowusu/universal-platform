# Secure Credential Provider Connections API Contract

## Purpose

This document defines the MVP API contract for Secure Credential References, Provider Connections, and Environment Profiles foundation.

The API should support provider connection dashboards, provider access references, provider connections, environment profiles, rotation placeholders, health checks, capability mappings, access review placeholders, and usage audit history while preserving tenant boundaries.

## Owner

```text
Provider Connection Foundation
Permission Engine
Audit Engine
Configuration Foundation
Regional Operations Foundation
Monitoring/Incident Foundation where health is surfaced
```

## Base Path

```text
/api/v1/organizations/{organizationId}/provider-connections
```

Platform-admin global provider routes may be introduced later with stricter controls.

## Provider Connection Dashboard

```text
GET /api/v1/organizations/{organizationId}/provider-connections/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "providerConnections": 6,
    "activeConnections": 5,
    "healthWarnings": 1,
    "rotationReviewsDue": 2,
    "environmentProfiles": 3
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Provider Access References

```text
GET /api/v1/organizations/{organizationId}/provider-connections/access-references
POST /api/v1/organizations/{organizationId}/provider-connections/access-references
GET /api/v1/organizations/{organizationId}/provider-connections/access-references/{referenceId}
PATCH /api/v1/organizations/{organizationId}/provider-connections/access-references/{referenceId}
POST /api/v1/organizations/{organizationId}/provider-connections/access-references/{referenceId}/disable-placeholder
```

### Create Provider Access Reference Request

```json
{
  "referenceKey": "payment-gh-production-placeholder",
  "providerType": "payment",
  "providerKey": "hubtel_placeholder",
  "environmentProfileKey": "production",
  "notes": "Optional safe note"
}
```

Responses must return metadata only, never sensitive access material.

## Provider Connections

```text
GET /api/v1/organizations/{organizationId}/provider-connections/connections
POST /api/v1/organizations/{organizationId}/provider-connections/connections
GET /api/v1/organizations/{organizationId}/provider-connections/connections/{connectionId}
PATCH /api/v1/organizations/{organizationId}/provider-connections/connections/{connectionId}
POST /api/v1/organizations/{organizationId}/provider-connections/connections/{connectionId}/disable-placeholder
```

## Environment Profiles

```text
GET /api/v1/organizations/{organizationId}/provider-connections/environment-profiles
POST /api/v1/organizations/{organizationId}/provider-connections/environment-profiles
GET /api/v1/organizations/{organizationId}/provider-connections/environment-profiles/{profileKey}
PATCH /api/v1/organizations/{organizationId}/provider-connections/environment-profiles/{profileKey}
```

## Rotation Placeholders

```text
GET /api/v1/organizations/{organizationId}/provider-connections/rotation-requests
POST /api/v1/organizations/{organizationId}/provider-connections/access-references/{referenceId}/rotation-requests
GET /api/v1/organizations/{organizationId}/provider-connections/rotation-requests/{rotationId}
POST /api/v1/organizations/{organizationId}/provider-connections/rotation-requests/{rotationId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/provider-connections/rotation-requests/{rotationId}/complete-placeholder
```

## Connection Health Placeholders

```text
GET /api/v1/organizations/{organizationId}/provider-connections/health-checks
POST /api/v1/organizations/{organizationId}/provider-connections/connections/{connectionId}/health-check-placeholder
GET /api/v1/organizations/{organizationId}/provider-connections/health-checks/{healthCheckId}
```

## Capability Mappings

```text
GET /api/v1/organizations/{organizationId}/provider-connections/capability-mappings
POST /api/v1/organizations/{organizationId}/provider-connections/capability-mappings
PATCH /api/v1/organizations/{organizationId}/provider-connections/capability-mappings/{mappingId}
```

## Access Review Placeholders

```text
GET /api/v1/organizations/{organizationId}/provider-connections/access-reviews
POST /api/v1/organizations/{organizationId}/provider-connections/access-reviews
POST /api/v1/organizations/{organizationId}/provider-connections/access-reviews/{reviewId}/complete-placeholder
```

## Usage Audit History

```text
GET /api/v1/organizations/{organizationId}/provider-connections/usage-history
GET /api/v1/organizations/{organizationId}/provider-connections/connections/{connectionId}/usage-history
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- provider connection feature availability
- required permission
- provider connection belongs to organization where organization-scoped
- environment profile is valid
- regional provider availability compatibility where practical
- sensitive access material is never returned by metadata routes

Provider connection records must not cross organization boundaries.

## Status Direction

Suggested provider connection statuses:

```text
active
inactive
pending_review_placeholder
health_warning_placeholder
disabled_placeholder
manual_review
```

Suggested environment profile statuses:

```text
active
inactive
restricted_placeholder
manual_review
```

Suggested rotation statuses:

```text
requested
approved_placeholder
completed_placeholder
cancelled
manual_review
```

## Audit Direction

Audit should be written for:

- provider access reference created or changed
- provider connection created or changed
- environment profile changed
- rotation requested placeholder
- rotation approved/completed placeholder
- health check run placeholder
- access review completed placeholder
- provider connection disabled

## Error Direction

Use standard response shape.

Expected error areas:

- provider access reference not found
- provider connection not found
- environment profile not found
- rotation request not found
- provider capability mapping not found
- unsupported provider/country/currency combination
- provider connection permission denied
- sensitive material unavailable by design
- organization not found

## Final Rule

Provider connection APIs must return safe metadata, enforce organization boundaries, and route provider access through controlled references and adapter boundaries.
