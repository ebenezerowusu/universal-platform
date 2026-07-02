# Subscription Billing Revenue Operations API Contract

## Purpose

This document defines the MVP API contract for Subscription Billing and Revenue Operations foundation.

The API should support billing dashboard summaries, plan catalog metadata, subscriptions, invoice metadata, payment references, entitlement history, renewal/cancellation placeholders, and revenue operation notes while preserving organization boundaries.

## Owner

```text
Billing Foundation
Payment Engine
Module/Subscription Engine
Permission Engine
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/billing
```

## Billing Dashboard

```text
GET /api/v1/organizations/{organizationId}/billing/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "subscriptionStatus": "active",
    "planKey": "growth",
    "openInvoices": 1,
    "nextRenewalDate": "2026-08-01",
    "paymentStatus": "pending_placeholder"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Plan Catalog

```text
GET /api/v1/organizations/{organizationId}/billing/plans
POST /api/v1/organizations/{organizationId}/billing/plans
GET /api/v1/organizations/{organizationId}/billing/plans/{planKey}
PATCH /api/v1/organizations/{organizationId}/billing/plans/{planKey}
```

Plan create/update should be platform-admin or billing-admin only where applicable.

## Subscriptions

```text
GET /api/v1/organizations/{organizationId}/billing/subscriptions
POST /api/v1/organizations/{organizationId}/billing/subscriptions
GET /api/v1/organizations/{organizationId}/billing/subscriptions/{subscriptionId}
PATCH /api/v1/organizations/{organizationId}/billing/subscriptions/{subscriptionId}
```

### Create Subscription Request

```json
{
  "planKey": "growth",
  "billingInterval": "monthly_placeholder",
  "trialEndsAt": "optional",
  "notes": "Optional note"
}
```

## Invoices

```text
GET /api/v1/organizations/{organizationId}/billing/invoices
POST /api/v1/organizations/{organizationId}/billing/invoices
GET /api/v1/organizations/{organizationId}/billing/invoices/{invoiceId}
PATCH /api/v1/organizations/{organizationId}/billing/invoices/{invoiceId}
```

### Invoice Metadata Direction

```json
{
  "subscriptionId": "...",
  "invoiceNumberPlaceholder": "INV-0001",
  "amount": 250.00,
  "currency": "GHS",
  "dueDate": "2026-08-01",
  "status": "open"
}
```

## Payment References

```text
GET /api/v1/organizations/{organizationId}/billing/payment-references
POST /api/v1/organizations/{organizationId}/billing/invoices/{invoiceId}/payment-references
GET /api/v1/organizations/{organizationId}/billing/payment-references/{paymentReferenceId}
```

Billing should store Payment Engine references only.

## Entitlement History

```text
GET /api/v1/organizations/{organizationId}/billing/entitlement-history
POST /api/v1/organizations/{organizationId}/billing/subscriptions/{subscriptionId}/entitlement-history
```

## Renewal and Cancellation Placeholders

```text
POST /api/v1/organizations/{organizationId}/billing/subscriptions/{subscriptionId}/renewal-placeholder
POST /api/v1/organizations/{organizationId}/billing/subscriptions/{subscriptionId}/cancel-placeholder
POST /api/v1/organizations/{organizationId}/billing/subscriptions/{subscriptionId}/restore-placeholder
```

## Revenue Operation Notes

```text
GET /api/v1/organizations/{organizationId}/billing/revenue-notes
POST /api/v1/organizations/{organizationId}/billing/revenue-notes
GET /api/v1/organizations/{organizationId}/billing/revenue-notes/{noteId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership or permitted platform billing role
- billing feature availability
- required permission
- organization ownership
- Payment Engine reference validity where payment references are attached

Billing records must not cross organization boundaries.

## Status Direction

Suggested subscription statuses:

```text
trialing_placeholder
active
past_due_placeholder
suspended_placeholder
cancel_requested_placeholder
cancelled
manual_review
```

Suggested invoice statuses:

```text
draft
open
payment_pending_placeholder
paid
void_placeholder
overdue_placeholder
manual_review
```

Suggested entitlement change types:

```text
plan_started
plan_upgraded
plan_downgraded
trial_started
trial_ended
module_enabled
module_disabled
subscription_suspended
subscription_restored
manual_review
```

## Audit Direction

Audit should be written for:

- plan metadata changed
- subscription created or updated
- subscription status changed
- invoice metadata changed
- payment reference attached
- entitlement changed
- renewal placeholder updated
- cancellation placeholder requested
- revenue note added

## Error Direction

Use standard response shape.

Expected error areas:

- plan not found
- subscription not found
- invoice not found
- payment reference not found
- entitlement history not found
- billing permission denied
- Payment Engine reference invalid
- organization not found

## Final Rule

Billing APIs must be organization-scoped, permission-protected, provider-agnostic, entitlement-aware, and auditable.
