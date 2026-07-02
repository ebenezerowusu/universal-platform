# Organization Lifecycle Provisioning API Contract

## Purpose

This document defines the MVP API contract for Organization Lifecycle, Provisioning, and Tenant Operations foundation.

The API should support provisioning requests, lifecycle statuses, activation/suspension/reactivation placeholders, admin bootstrap, regional bootstrap, module bootstrap, billing linkage, onboarding linkage, and tenant isolation validation placeholders while preserving tenant boundaries.

## Owner

```text
Organization Lifecycle Foundation
Identity Foundation
Module Registry Engine
Subscription/Feature Entitlement Engine
Regional Operations Foundation
Support Operations Foundation
Permission Engine
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/lifecycle
```

## Lifecycle Dashboard

```text
GET /api/v1/organizations/{organizationId}/lifecycle/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "lifecycleStatus": "active",
    "regionalBootstrapStatus": "completed_placeholder",
    "moduleBootstrapStatus": "in_progress_placeholder",
    "billingLinkageStatus": "linked_placeholder",
    "tenantValidationStatus": "passed_placeholder"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Provisioning Requests

```text
GET /api/v1/organizations/{organizationId}/lifecycle/provisioning-requests
POST /api/v1/organizations/{organizationId}/lifecycle/provisioning-requests
GET /api/v1/organizations/{organizationId}/lifecycle/provisioning-requests/{requestId}
PATCH /api/v1/organizations/{organizationId}/lifecycle/provisioning-requests/{requestId}
POST /api/v1/organizations/{organizationId}/lifecycle/provisioning-requests/{requestId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/lifecycle/provisioning-requests/{requestId}/reject-placeholder
```

### Create Provisioning Request

```json
{
  "organizationName": "Example Organization",
  "requestedCountryCode": "GH",
  "requestedModules": ["religious", "communication_sms"],
  "ownerUserId": "optional",
  "sourceChannelPlaceholder": "admin_console",
  "notes": "Optional note"
}
```

## Lifecycle Actions

```text
GET /api/v1/organizations/{organizationId}/lifecycle/actions
POST /api/v1/organizations/{organizationId}/lifecycle/actions/activate-placeholder
POST /api/v1/organizations/{organizationId}/lifecycle/actions/suspend-placeholder
POST /api/v1/organizations/{organizationId}/lifecycle/actions/reactivate-placeholder
POST /api/v1/organizations/{organizationId}/lifecycle/actions/archive-placeholder
```

No destructive organization deletion should be exposed in MVP.

## Admin Bootstrap

```text
GET /api/v1/organizations/{organizationId}/lifecycle/admin-bootstrap
POST /api/v1/organizations/{organizationId}/lifecycle/admin-bootstrap
PATCH /api/v1/organizations/{organizationId}/lifecycle/admin-bootstrap/{bootstrapId}
```

## Regional Bootstrap

```text
GET /api/v1/organizations/{organizationId}/lifecycle/regional-bootstrap
POST /api/v1/organizations/{organizationId}/lifecycle/regional-bootstrap
PATCH /api/v1/organizations/{organizationId}/lifecycle/regional-bootstrap/{bootstrapId}
```

## Module Bootstrap

```text
GET /api/v1/organizations/{organizationId}/lifecycle/module-bootstrap
POST /api/v1/organizations/{organizationId}/lifecycle/module-bootstrap
PATCH /api/v1/organizations/{organizationId}/lifecycle/module-bootstrap/{bootstrapId}
```

## Billing Linkage Placeholder

```text
GET /api/v1/organizations/{organizationId}/lifecycle/billing-linkage
POST /api/v1/organizations/{organizationId}/lifecycle/billing-linkage
PATCH /api/v1/organizations/{organizationId}/lifecycle/billing-linkage/{linkageId}
```

## Onboarding Linkage Placeholder

```text
GET /api/v1/organizations/{organizationId}/lifecycle/onboarding-linkage
POST /api/v1/organizations/{organizationId}/lifecycle/onboarding-linkage
PATCH /api/v1/organizations/{organizationId}/lifecycle/onboarding-linkage/{linkageId}
```

## Tenant Isolation Validation Placeholder

```text
GET /api/v1/organizations/{organizationId}/lifecycle/tenant-validation
POST /api/v1/organizations/{organizationId}/lifecycle/tenant-validation/run-placeholder
GET /api/v1/organizations/{organizationId}/lifecycle/tenant-validation/{validationId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership or permitted platform lifecycle role
- lifecycle feature availability
- required permission
- organization ownership
- regional settings compatibility
- module entitlement compatibility where practical
- billing/subscription organization match

Lifecycle records must not cross organization boundaries.

## Status Direction

Suggested lifecycle statuses:

```text
draft
provisioning_requested
provisioning_in_progress
active
suspended
reactivation_requested
archived_placeholder
manual_review
```

Suggested bootstrap statuses:

```text
not_started
in_progress_placeholder
completed_placeholder
failed
manual_review
```

Suggested tenant validation statuses:

```text
not_run
running_placeholder
passed_placeholder
failed
warning
manual_review
```

## Audit Direction

Audit should be written for:

- provisioning request created
- provisioning approved/rejected placeholder
- organization activated
- organization suspended
- organization reactivated
- admin bootstrap changed
- regional bootstrap changed
- module bootstrap changed
- billing linkage changed
- tenant validation run placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- provisioning request not found
- organization lifecycle action invalid
- admin bootstrap invalid
- regional settings invalid
- module entitlement unavailable
- billing linkage invalid
- tenant validation failed
- organization not found

## Final Rule

Organization lifecycle APIs must be tenant-safe, permission-protected, auditable, and integrated with regional settings, module registry, billing, and onboarding without exposing cross-tenant data.
