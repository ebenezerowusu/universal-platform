# Sprint 40 Subscription Billing Revenue Operations Review

## Purpose

This document defines review checks for Sprint 40.

Sprint 40 implements Subscription Billing and Revenue Operations foundation.

## Review 1: Scope

Sprint 40 may include:

- subscription billing feature registration foundation
- plan catalog metadata foundation
- subscription account foundation
- subscription status foundation
- trial period placeholder where practical
- invoice metadata foundation
- billing cycle metadata foundation
- payment intent/reference placeholder through Payment Engine
- entitlement change history foundation
- renewal and cancellation placeholder where practical
- revenue operation notes where practical
- permission and feature checks
- audit records for billing and entitlement actions
- API and UI foundations

Sprint 40 should not include:

- direct payment provider calls inside billing domain
- tax calculation engine
- full accounting ledger
- automatic card charging
- revenue recognition engine
- external billing provider integration
- cross-organization billing visibility
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Billing Foundation owns subscriptions, invoice metadata, billing cycles, entitlement history, and revenue notes
- Payment Engine owns payment execution and provider-specific behavior
- Module/Subscription Engine owns feature entitlement evaluation
- Permission Engine controls billing visibility and updates
- Audit Engine records sensitive billing actions
- Platform Core does not contain provider-specific billing rules

## Review 3: Organization Boundaries

Confirm:

- subscription records are organization-scoped
- invoice records are organization-scoped
- payment references cannot cross organization boundaries
- revenue notes are organization-scoped
- billing dashboards do not leak other organizations' information

## Review 4: Entitlement Rules

Confirm:

- entitlement changes are recorded when subscription state changes
- disabled/suspended subscription states do not silently leave premium entitlements active
- plan upgrade/downgrade actions create history
- trial start/end placeholders are tracked
- manual entitlement changes require permission and audit

## Review 5: Payment and Invoice Rules

Confirm:

- invoices do not call payment providers directly
- payment references are attached through Payment Engine references
- invoice status changes are auditable
- overdue/payment-pending placeholders are visible where practical
- invoice metadata is not treated as a full accounting ledger

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/subscription-billing-revenue-ops-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/subscription-billing-revenue-ops-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- plan catalog retrieval
- subscription creation/update
- invoice metadata creation/update
- payment reference attachment
- entitlement history creation
- cancellation placeholder request
- organization boundary enforcement
- billing permission denied behavior
- audit record creation

## Final Rule

Sprint 40 is complete when plans, subscriptions, invoices, payment references, entitlement history, renewal/cancellation placeholders, and revenue notes are available through permission-protected and auditable foundations.
