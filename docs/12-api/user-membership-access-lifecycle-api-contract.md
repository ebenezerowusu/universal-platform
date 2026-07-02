# User Membership Access Lifecycle API Contract

## Purpose

This document defines the MVP API contract for User Invitation, Membership, Role Assignment, and Access Lifecycle foundation.

The API should support invitations, membership statuses, role assignments, permission group placeholders, access history, offboarding placeholders, privileged access review summaries, and safe audit behavior while preserving organization boundaries.

## Owner

```text
Identity Foundation
Organization Foundation
Permission Engine
Access Lifecycle Foundation
Audit Engine
Governance/Access Review Foundation where reviews are linked
Notification Engine where invitations are sent later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/user-access
```

## Access Lifecycle Dashboard

```text
GET /api/v1/organizations/{organizationId}/user-access/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activeMembers": 24,
    "pendingInvitations": 3,
    "privilegedUsers": 5,
    "offboardingUsers": 1,
    "accessReviewStatus": "due_placeholder"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Invitations

```text
GET /api/v1/organizations/{organizationId}/user-access/invitations
POST /api/v1/organizations/{organizationId}/user-access/invitations
GET /api/v1/organizations/{organizationId}/user-access/invitations/{invitationId}
POST /api/v1/organizations/{organizationId}/user-access/invitations/{invitationId}/cancel
POST /api/v1/organizations/{organizationId}/user-access/invitations/{invitationId}/resend-placeholder
POST /api/v1/organizations/{organizationId}/user-access/invitations/{invitationId}/accept-placeholder
```

### Create Invitation Request

```json
{
  "email": "user@example.com",
  "phonePlaceholder": "optional",
  "roleKeys": ["organization_admin"],
  "expiresAt": "optional",
  "message": "Optional safe message"
}
```

## Memberships

```text
GET /api/v1/organizations/{organizationId}/user-access/memberships
GET /api/v1/organizations/{organizationId}/user-access/memberships/{membershipId}
PATCH /api/v1/organizations/{organizationId}/user-access/memberships/{membershipId}
POST /api/v1/organizations/{organizationId}/user-access/memberships/{membershipId}/activate-placeholder
POST /api/v1/organizations/{organizationId}/user-access/memberships/{membershipId}/suspend-placeholder
POST /api/v1/organizations/{organizationId}/user-access/memberships/{membershipId}/remove-placeholder
```

## Role Assignments

```text
GET /api/v1/organizations/{organizationId}/user-access/role-assignments
POST /api/v1/organizations/{organizationId}/user-access/role-assignments
GET /api/v1/organizations/{organizationId}/user-access/role-assignments/{assignmentId}
POST /api/v1/organizations/{organizationId}/user-access/role-assignments/{assignmentId}/remove
```

### Role Assignment Request

```json
{
  "userId": "...",
  "roleKey": "finance_admin",
  "reason": "Optional reason"
}
```

## Permission Group Placeholder

```text
GET /api/v1/organizations/{organizationId}/user-access/permission-groups
POST /api/v1/organizations/{organizationId}/user-access/permission-groups/{groupKey}/assign-placeholder
POST /api/v1/organizations/{organizationId}/user-access/permission-groups/{groupKey}/remove-placeholder
```

## Access History

```text
GET /api/v1/organizations/{organizationId}/user-access/history
GET /api/v1/organizations/{organizationId}/user-access/users/{userId}/history
```

## Offboarding Placeholder

```text
GET /api/v1/organizations/{organizationId}/user-access/offboarding
POST /api/v1/organizations/{organizationId}/user-access/users/{userId}/offboarding/start-placeholder
POST /api/v1/organizations/{organizationId}/user-access/users/{userId}/offboarding/complete-placeholder
```

## Privileged Access Review Summary

```text
GET /api/v1/organizations/{organizationId}/user-access/privileged-access-summary
POST /api/v1/organizations/{organizationId}/user-access/users/{userId}/link-access-review-placeholder
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- user access lifecycle feature availability
- required permission
- target user belongs to the same organization where required
- role is valid for organization/module context
- actor cannot grant roles beyond allowed scope

Membership and role records must not cross organization boundaries.

## Status Direction

Suggested invitation statuses:

```text
pending
accepted_placeholder
expired_placeholder
cancelled
manual_review
```

Suggested membership statuses:

```text
invited
active
inactive
suspended_placeholder
removed
offboarding_placeholder
manual_review
```

Suggested access history event types:

```text
invitation_created
invitation_accepted_placeholder
membership_activated
membership_suspended
membership_removed
role_assigned
role_removed
permission_group_changed_placeholder
offboarding_started_placeholder
offboarding_completed_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- invitation created/cancelled/accepted placeholder
- membership activated/suspended/removed
- role assigned/removed
- permission group changed placeholder
- offboarding started/completed placeholder
- access review linked placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- invitation not found
- membership not found
- role assignment not found
- user not found
- role not found
- user already invited
- user already member
- insufficient permission to assign role
- cross-organization access denied
- organization not found

## Final Rule

User access APIs must be organization-scoped, permission-protected, auditable, and routed through Identity, Organization, and Permission boundaries.
