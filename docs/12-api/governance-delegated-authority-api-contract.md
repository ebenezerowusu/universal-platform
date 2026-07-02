# Governance Delegated Authority API Contract

## Purpose

This document defines the MVP API contract for Governance Policy, Delegated Authority, and Approval Mandates foundation.

The API should support governance dashboards, policies, authority mandates, delegated approver assignments, approval thresholds, domain action mappings, temporary delegations, conflict warnings, authority decision logs, and governance audit history while preserving organization boundaries.

## Owner

```text
Governance Authority Foundation
Permission Engine
Workflow Engine
Audit Engine
Organization Network Foundation where cross-organization authority is evaluated
```

## Base Path

```text
/api/v1/organizations/{organizationId}/governance-authority
```

## Governance Authority Dashboard

```text
GET /api/v1/organizations/{organizationId}/governance-authority/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activePolicies": 8,
    "activeMandates": 24,
    "activeDelegations": 4,
    "openConflicts": 2,
    "recentAuthorityDecisions": 11
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Governance Policies

```text
GET /api/v1/organizations/{organizationId}/governance-authority/policies
POST /api/v1/organizations/{organizationId}/governance-authority/policies
GET /api/v1/organizations/{organizationId}/governance-authority/policies/{policyKey}
PATCH /api/v1/organizations/{organizationId}/governance-authority/policies/{policyKey}
```

## Authority Mandates

```text
GET /api/v1/organizations/{organizationId}/governance-authority/mandates
POST /api/v1/organizations/{organizationId}/governance-authority/mandates
GET /api/v1/organizations/{organizationId}/governance-authority/mandates/{mandateId}
PATCH /api/v1/organizations/{organizationId}/governance-authority/mandates/{mandateId}
POST /api/v1/organizations/{organizationId}/governance-authority/mandates/{mandateId}/disable-placeholder
```

### Create Authority Mandate Request

```json
{
  "mandateKey": "finance-expense-approval-placeholder",
  "actorReference": {
    "type": "role_placeholder",
    "id": "finance-admin"
  },
  "authorityScope": "organization_placeholder",
  "domainActionType": "expense_approval_placeholder",
  "startDatePlaceholder": "optional",
  "endDatePlaceholder": "optional"
}
```

## Delegated Approvers

```text
GET /api/v1/organizations/{organizationId}/governance-authority/delegated-approvers
POST /api/v1/organizations/{organizationId}/governance-authority/delegated-approvers
GET /api/v1/organizations/{organizationId}/governance-authority/delegated-approvers/{delegationId}
PATCH /api/v1/organizations/{organizationId}/governance-authority/delegated-approvers/{delegationId}
POST /api/v1/organizations/{organizationId}/governance-authority/delegated-approvers/{delegationId}/revoke-placeholder
```

## Approval Thresholds

```text
GET /api/v1/organizations/{organizationId}/governance-authority/approval-thresholds
POST /api/v1/organizations/{organizationId}/governance-authority/approval-thresholds
GET /api/v1/organizations/{organizationId}/governance-authority/approval-thresholds/{thresholdId}
PATCH /api/v1/organizations/{organizationId}/governance-authority/approval-thresholds/{thresholdId}
```

## Domain Action Mappings

```text
GET /api/v1/organizations/{organizationId}/governance-authority/domain-action-mappings
POST /api/v1/organizations/{organizationId}/governance-authority/domain-action-mappings
GET /api/v1/organizations/{organizationId}/governance-authority/domain-action-mappings/{mappingId}
PATCH /api/v1/organizations/{organizationId}/governance-authority/domain-action-mappings/{mappingId}
```

## Temporary Delegations

```text
GET /api/v1/organizations/{organizationId}/governance-authority/temporary-delegations
POST /api/v1/organizations/{organizationId}/governance-authority/temporary-delegations
GET /api/v1/organizations/{organizationId}/governance-authority/temporary-delegations/{delegationId}
POST /api/v1/organizations/{organizationId}/governance-authority/temporary-delegations/{delegationId}/activate-placeholder
POST /api/v1/organizations/{organizationId}/governance-authority/temporary-delegations/{delegationId}/expire-placeholder
```

## Authority Conflict Warnings

```text
GET /api/v1/organizations/{organizationId}/governance-authority/conflict-warnings
GET /api/v1/organizations/{organizationId}/governance-authority/conflict-warnings/{warningId}
POST /api/v1/organizations/{organizationId}/governance-authority/conflict-warnings/{warningId}/mark-reviewed-placeholder
```

## Authority Decision Logs

```text
GET /api/v1/organizations/{organizationId}/governance-authority/decision-logs
POST /api/v1/organizations/{organizationId}/governance-authority/decision-logs/evaluate-placeholder
GET /api/v1/organizations/{organizationId}/governance-authority/decision-logs/{decisionLogId}
```

## Governance Audit History

```text
GET /api/v1/organizations/{organizationId}/governance-authority/audit-history
GET /api/v1/organizations/{organizationId}/governance-authority/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- governance authority feature availability
- required permission
- actor has permission to manage authority rules
- cross-organization authority is allowed only by explicit network relationship and sharing policy
- authority decision does not bypass workflow state transition rules
- delegated authority is not expired or revoked

Governance authority records must not cross organization boundaries without explicit organization network policy.

## Status Direction

Suggested policy statuses:

```text
draft
active
inactive
expired_placeholder
manual_review
```

Suggested mandate statuses:

```text
active
inactive
expired_placeholder
revoked_placeholder
manual_review
```

Suggested conflict warning statuses:

```text
open
reviewed_placeholder
resolved_placeholder
ignored_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- governance policy created or updated
- authority mandate created or updated
- delegation created or revoked placeholder
- threshold rule changed
- domain action mapping changed
- temporary delegation activated/expired placeholder
- authority conflict reviewed placeholder
- authority decision evaluated placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- governance policy not found
- mandate not found
- delegation not found
- threshold not found
- domain action mapping not found
- authority conflict warning not found
- authority denied
- permission denied
- cross-organization authority denied
- organization not found

## Final Rule

Governance authority APIs must be organization-scoped, permission-protected, workflow-aware, audit-safe, and explicit about all delegated decision rights.
