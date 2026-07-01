# Sprint 19 SMS Package Payment Readiness Roadmap

## Purpose

This document defines Sprint 19 for SMS package payment readiness.

Sprint 19 should connect SMS package purchase requests to Payment Engine, wallet crediting, reconciliation, and support/admin review.

## Sprint Goal

Enable organizations to request and complete SMS package purchases in a way that is provider-agnostic, auditable, and safe for wallet balances.

## Why This Sprint

Sprint 16 prepared SMS wallet and package catalog.

Sprint 17 prepared provider readiness.

Sprint 18 prepared communication governance.

Sprint 19 makes the SMS wallet monetization path operational:

- package purchase request
- Payment Engine initiation
- payment status tracking
- wallet credit after payment confirmation
- package purchase ledger
- reconciliation and manual review

## Work Package 1: Package Purchase Request

Prepare organization-level purchase request records for selected SMS packages.

The request should include:

```text
organization
package
amount
currency
status
requested_by
created_at
```

## Work Package 2: Payment Engine Initiation

Package purchases should initiate payment through Payment Engine.

Communication Domain should not call payment providers directly.

## Work Package 3: Payment Status Tracking

Track normalized statuses such as:

```text
pending
processing
paid
failed
cancelled
expired
manual_review
```

## Work Package 4: Wallet Credit Rules

Wallet units should be credited only when:

- payment is confirmed by Payment Engine
- manual admin credit is approved and audited
- a test/mock flow explicitly marks payment as successful in non-production

## Work Package 5: Reconciliation

Prepare reconciliation between:

- package purchase request
- payment record
- payment provider result
- wallet credit transaction
- SMS wallet balance

## Work Package 6: Flutter Foundation

Prepare screens or placeholders for:

- package catalog
- purchase confirmation
- payment status
- wallet transaction history

## Work Package 7: Admin Review

Prepare system/admin review for:

- paid but not credited
- credited without payment
- failed payment with wallet impact
- stuck pending payments
- manual credit requests

## Out of Scope

Do not implement:

- direct provider payment logic in Communication Domain
- live provider credentials
- full invoicing system
- subscription plan overhaul
- unrelated finance features
- advanced billing dashboards

## Completion Standard

Sprint 19 is complete when SMS package purchases can be initiated, tracked, reconciled, and credited safely through Payment Engine boundaries.

## Final Rule

SMS package purchases should protect both revenue and wallet accuracy. Credit wallet units only through confirmed or audited flows.
