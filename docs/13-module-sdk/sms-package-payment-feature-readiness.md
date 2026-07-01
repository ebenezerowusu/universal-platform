# SMS Package Payment Feature Readiness

## Purpose

This document prepares the SMS Package Payment feature wave for implementation.

It ensures SMS package purchases use Payment Engine boundaries and auditable wallet crediting.

## Wave Summary

Build a foundation for organizations to purchase SMS packages, track payment status, and receive wallet credit after confirmation.

## Problem Being Solved

Organizations need a clear way to buy SMS units.

The platform needs a safe revenue path that protects wallet accuracy and keeps payment providers abstracted.

## Evidence

This wave is supported by:

- SMS wallet foundation
- package catalog direction
- revenue readiness roadmap
- need for wallet balance trust
- need for provider-agnostic payment flow

## Primary Users

- organization owner/admin
- communication manager
- finance/admin user where permitted
- platform support/admin user

## MVP Scope

Included:

- SMS package purchase request
- Payment Engine initiation foundation
- payment status tracking
- wallet credit after confirmed payment
- package purchase ledger
- reconciliation foundation
- system/admin manual review direction

Excluded:

- full invoicing system
- provider credential UI
- organization-managed payment provider setup
- subscription plan overhaul
- advanced finance reports
- unrelated payment flows

## Domain Ownership

Communication Domain owns SMS package purchase workflow.

Payment Engine owns payment initiation, provider interaction, and normalized payment status.

SMS Engine owns wallet crediting and wallet transactions.

Platform Core must not own package purchase business rules.

## Required Engines

- Payment Engine
- SMS Engine
- Permission Engine
- Audit Engine
- Feature Flag or License Engine
- Notification Engine where useful

## Suggested Permissions

```text
communication.sms_packages.view
communication.sms_packages.purchase
communication.sms_wallet.view
communication.sms_wallet.transactions.view
system.sms_wallet.adjust
system.sms_purchase.review
```

## Data Ownership

Organization-owned records should include:

- SMS package purchase request
- payment reference link
- wallet credit transaction
- reconciliation item where needed

System/admin records may include:

- manual review action
- manual wallet correction record

## API Direction

Planned API groups:

- package catalog
- package purchase request
- payment status
- wallet transaction history
- admin review for unresolved purchases

## Flutter Direction

Planned screens:

- package catalog
- purchase confirmation
- payment status
- wallet transaction history
- support/admin review placeholder where needed

## Audit Direction

Audit important actions:

- package purchase requested
- payment initiated
- payment status changed
- wallet credited
- wallet credit manually adjusted
- reconciliation resolved

## Revenue Direction

Near-term revenue direction:

```text
usage-based add-on
SMS wallet package sales
```

Future direction may include plan bundles or recurring SMS allocations.

## Risks

- wallet credited before payment confirmation
- payment success not reflected in wallet
- provider-specific payment behavior leaking into Communication Domain
- manual credits without audit
- duplicate payment callback causing double credit

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

SMS package payment must be traceable from request to payment to wallet credit. No silent wallet crediting.
