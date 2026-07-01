# Expense Foundation API Contract

## Purpose

This document defines the MVP API contract for organization expense foundation.

The API should support expense categories, budget lines, expense requests, approvals, expense records, and reconciliation while preserving organization boundaries.

## Owner

```text
Finance Domain
Workflow Engine where approval is used
Storage Engine where attachments are used
```

## Base Path

```text
/api/v1/organizations/{organizationId}/finance
```

## Expense Categories

```text
GET /api/v1/organizations/{organizationId}/finance/expense-categories
POST /api/v1/organizations/{organizationId}/finance/expense-categories
GET /api/v1/organizations/{organizationId}/finance/expense-categories/{categoryId}
PATCH /api/v1/organizations/{organizationId}/finance/expense-categories/{categoryId}
```

### Create Category Request

```json
{
  "name": "Operations",
  "code": "operations",
  "status": "active"
}
```

## Budget Lines

```text
GET /api/v1/organizations/{organizationId}/finance/budget-lines
POST /api/v1/organizations/{organizationId}/finance/budget-lines
GET /api/v1/organizations/{organizationId}/finance/budget-lines/{budgetLineId}
PATCH /api/v1/organizations/{organizationId}/finance/budget-lines/{budgetLineId}
```

### Create Budget Line Request

```json
{
  "categoryId": "...",
  "amount": 1000,
  "currency": "GHS",
  "period": "2026-Q3",
  "notes": "Operations budget"
}
```

## Expense Requests

```text
GET /api/v1/organizations/{organizationId}/finance/expense-requests
POST /api/v1/organizations/{organizationId}/finance/expense-requests
GET /api/v1/organizations/{organizationId}/finance/expense-requests/{expenseRequestId}
PATCH /api/v1/organizations/{organizationId}/finance/expense-requests/{expenseRequestId}
```

### Create Expense Request

```json
{
  "categoryId": "...",
  "budgetLineId": "optional",
  "amount": 250,
  "currency": "GHS",
  "purpose": "Event logistics",
  "requiredDate": "2026-07-20",
  "notes": "Optional note"
}
```

## Approval Actions

```text
POST /api/v1/organizations/{organizationId}/finance/expense-requests/{expenseRequestId}/approve
POST /api/v1/organizations/{organizationId}/finance/expense-requests/{expenseRequestId}/reject
```

### Approve Request

```json
{
  "notes": "Approved for event logistics"
}
```

### Reject Request

```json
{
  "reason": "Amount requires revision"
}
```

## Expense Records

```text
GET /api/v1/organizations/{organizationId}/finance/expense-records
POST /api/v1/organizations/{organizationId}/finance/expense-records
GET /api/v1/organizations/{organizationId}/finance/expense-records/{expenseRecordId}
```

### Create Expense Record

```json
{
  "expenseRequestId": "optional",
  "categoryId": "...",
  "amount": 250,
  "currency": "GHS",
  "spentAt": "2026-07-20T10:00:00Z",
  "payeeLabel": "Optional safe label",
  "notes": "Optional note"
}
```

## Reconciliation Review

```text
GET /api/v1/system/finance/expense-reconciliation-items
POST /api/v1/system/finance/expense-reconciliation-items/{itemId}/resolve
```

## Required Checks

Organization routes should check:

- authenticated user
- organization membership
- Finance module availability
- required permission
- feature availability
- organization ownership

System/admin routes should check:

- authenticated platform admin or support role
- required system permission
- audit logging

## Status Direction

Suggested expense request statuses:

```text
draft
submitted
approved
rejected
cancelled
paid
manual_review
```

Suggested expense record statuses:

```text
recorded
pending_review
reconciled
cancelled
```

## Audit Direction

Audit should be written for:

- category created or updated
- budget line created or updated
- expense request created or updated
- expense approved or rejected
- expense record created or adjusted
- reconciliation resolved

## Error Direction

Use standard response shape.

Expected error areas:

- category not found
- budget line not found
- expense request not found
- expense record not found
- permission denied
- invalid approval action
- organization not found

## Final Rule

Expense APIs must preserve organization ownership and approval controls. Expense workflows should be auditable from request to approval to record.
