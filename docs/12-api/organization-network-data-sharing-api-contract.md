# Organization Network Data Sharing API Contract

## Purpose

This document defines the MVP API contract for Organization Network, Branch Relationships, and Data Sharing Governance foundation.

The API should support organization network dashboards, relationships, connection requests, disconnect/remove requests, relationship history, hierarchy metadata, data sharing policies, historical access policies, network reporting visibility, and network audit history while preserving tenant ownership.

## Owner

```text
Organization Network Foundation
Organization Foundation
Permission Engine
Audit Engine
Privacy Governance Foundation
Reporting Foundation where network reporting is consumed
```

## Base Path

```text
/api/v1/organizations/{organizationId}/organization-network
```

## Network Dashboard

```text
GET /api/v1/organizations/{organizationId}/organization-network/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "parentRelationships": 1,
    "childRelationships": 12,
    "pendingConnectionRequests": 2,
    "pendingDisconnectRequests": 1,
    "dataSharingWarnings": 3
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Relationships

```text
GET /api/v1/organizations/{organizationId}/organization-network/relationships
POST /api/v1/organizations/{organizationId}/organization-network/relationships
GET /api/v1/organizations/{organizationId}/organization-network/relationships/{relationshipId}
PATCH /api/v1/organizations/{organizationId}/organization-network/relationships/{relationshipId}
```

### Create Relationship Request

```json
{
  "parentOrganizationId": "...",
  "childOrganizationId": "...",
  "relationshipType": "branch_placeholder",
  "levelLabelPlaceholder": "district",
  "notes": "Optional safe note"
}
```

## Connection Requests

```text
GET /api/v1/organizations/{organizationId}/organization-network/connection-requests
POST /api/v1/organizations/{organizationId}/organization-network/connection-requests
GET /api/v1/organizations/{organizationId}/organization-network/connection-requests/{requestId}
POST /api/v1/organizations/{organizationId}/organization-network/connection-requests/{requestId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/organization-network/connection-requests/{requestId}/reject-placeholder
```

## Disconnect and Remove Requests

```text
GET /api/v1/organizations/{organizationId}/organization-network/disconnect-requests
POST /api/v1/organizations/{organizationId}/organization-network/disconnect-requests
GET /api/v1/organizations/{organizationId}/organization-network/disconnect-requests/{requestId}
POST /api/v1/organizations/{organizationId}/organization-network/disconnect-requests/{requestId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/organization-network/disconnect-requests/{requestId}/reject-placeholder
```

## Relationship History

```text
GET /api/v1/organizations/{organizationId}/organization-network/relationships/{relationshipId}/history
POST /api/v1/organizations/{organizationId}/organization-network/relationships/{relationshipId}/history
```

## Hierarchy Metadata

```text
GET /api/v1/organizations/{organizationId}/organization-network/hierarchy
GET /api/v1/organizations/{organizationId}/organization-network/hierarchy/path-placeholder
GET /api/v1/organizations/{organizationId}/organization-network/hierarchy/children-placeholder
GET /api/v1/organizations/{organizationId}/organization-network/hierarchy/ancestors-placeholder
```

## Data Sharing Policies

```text
GET /api/v1/organizations/{organizationId}/organization-network/data-sharing-policies
POST /api/v1/organizations/{organizationId}/organization-network/data-sharing-policies
GET /api/v1/organizations/{organizationId}/organization-network/data-sharing-policies/{policyId}
PATCH /api/v1/organizations/{organizationId}/organization-network/data-sharing-policies/{policyId}
```

## Historical Access Policies

```text
GET /api/v1/organizations/{organizationId}/organization-network/historical-access-policies
POST /api/v1/organizations/{organizationId}/organization-network/historical-access-policies
GET /api/v1/organizations/{organizationId}/organization-network/historical-access-policies/{policyId}
PATCH /api/v1/organizations/{organizationId}/organization-network/historical-access-policies/{policyId}
```

## Network Reporting Visibility

```text
GET /api/v1/organizations/{organizationId}/organization-network/reporting-visibility
POST /api/v1/organizations/{organizationId}/organization-network/reporting-visibility
PATCH /api/v1/organizations/{organizationId}/organization-network/reporting-visibility/{visibilityId}
```

## Network Audit History

```text
GET /api/v1/organizations/{organizationId}/organization-network/audit-history
GET /api/v1/organizations/{organizationId}/organization-network/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- organization network feature availability
- required permission
- actor can act for parent or child organization as required
- child organization ownership is preserved
- data sharing policy permits requested visibility
- privacy rules apply to network reporting

Organization network records must not create hidden cross-tenant access.

## Status Direction

Suggested relationship statuses:

```text
pending
active
rejected
disconnect_requested_placeholder
disconnected_placeholder
suspended_placeholder
revoked_placeholder
manual_review
```

Suggested sharing policy statuses:

```text
active
inactive
inherited_placeholder
restricted_placeholder
manual_review
```

Suggested historical access statuses:

```text
none
summary_only_placeholder
audit_only_placeholder
retained_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- relationship created or updated
- connection requested
- connection approved/rejected placeholder
- disconnect requested
- disconnect approved/rejected placeholder
- data sharing policy changed
- historical access policy changed
- network reporting visibility changed

## Error Direction

Use standard response shape.

Expected error areas:

- relationship not found
- connection request not found
- disconnect request not found
- sharing policy not found
- historical access policy not found
- organization network permission denied
- parent/child organization mismatch
- hidden cross-tenant access denied
- organization not found

## Final Rule

Organization network APIs must be tenant-safe, permission-protected, privacy-aware, auditable, and explicit about all parent/child visibility.
