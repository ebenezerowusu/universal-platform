# Organization/Tenant Foundation API Contract

## Purpose

This document defines the Sprint 3 API direction for the Organization/Tenant foundation.

Organizations are generic tenant containers. They are not churches, shops, schools, partners, or any domain-specific concept.

## Owner

```text
Organization/Tenant Foundation
Identity Foundation
Permission Engine later
Audit Engine later
```

## Implemented In Sprint 3

```text
GET /api/v1/organizations/status
```

This route is a non-mutating foundation status route. It confirms that organization and membership primitives have been introduced without exposing real tenant creation yet.

## Implemented Response Direction

```json
{
  "success": true,
  "data": {
    "foundation": "organization-tenant-foundation-placeholder",
    "implementedFlows": [
      "organization tenant model primitives",
      "organization membership model primitives",
      "repository boundary",
      "organization service boundary"
    ],
    "plannedOrganizationTypes": [
      "generic",
      "religious",
      "commerce",
      "non_profit",
      "education",
      "other"
    ],
    "plannedOrganizationStatuses": [
      "pending_setup",
      "active",
      "suspended",
      "deactivated"
    ],
    "plannedMembershipStatuses": [
      "invited",
      "active",
      "suspended",
      "revoked"
    ]
  },
  "meta": {
    "service": "platform-api",
    "traceId": "not-wired-yet"
  }
}
```

## Future Routes

Do not implement these until authentication, permission, and audit foundations are ready.

```text
GET /api/v1/organizations
POST /api/v1/organizations
GET /api/v1/organizations/{organizationId}
PATCH /api/v1/organizations/{organizationId}
GET /api/v1/organizations/{organizationId}/memberships
POST /api/v1/organizations/{organizationId}/memberships
PATCH /api/v1/organizations/{organizationId}/memberships/{membershipId}
POST /api/v1/organizations/{organizationId}/memberships/{membershipId}/suspend-placeholder
POST /api/v1/organizations/{organizationId}/memberships/{membershipId}/revoke-placeholder
```

## Required Future Checks

Future mutating routes must check:

- authenticated user
- organization context where applicable
- permission checks
- membership status
- tenant isolation
- feature/module availability where applicable
- audit logging for sensitive changes

## Organization Type Direction

Initial generic organization types:

```text
generic
religious
commerce
non_profit
education
other
```

These are platform-level classification hints, not domain-specific profiles.

Domain modules must still attach their own domain records later.

## Organization Status Direction

```text
pending_setup
active
suspended
deactivated
```

## Membership Status Direction

```text
invited
active
suspended
revoked
```

## Out Of Scope In Sprint 3

```text
real organization creation endpoint
organization switching
invitation flow
permission enforcement
audit persistence
organization hierarchy/network
branch relationship management
subscription/module entitlement checks
frontend screens
```

## Final Rule

Organization/Tenant foundation owns tenant boundaries. Domain modules must build on top of this foundation and must not redefine tenant ownership independently.
