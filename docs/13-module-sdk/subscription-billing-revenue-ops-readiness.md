# Subscription Billing Revenue Operations Readiness

## Purpose

This document prepares the Subscription Billing and Revenue Operations foundation for implementation.

It ensures billing records, payment references, plan subscriptions, entitlement changes, invoices, and revenue notes are organization-scoped, provider-agnostic, permission-protected, and auditable.

## Wave Summary

Build foundations for plan catalog metadata, subscription accounts, billing cycles, invoice metadata, Payment Engine references, entitlement change history, renewal placeholders, cancellation placeholders, and revenue operation notes.

## Problem Being Solved

The platform needs one consistent way to track commercial status and feature entitlements.

Without a shared billing foundation, module access, plan status, invoices, and payment state can drift across domains.

## Evidence

This wave is supported by:

- subscription/feature API contract
- Payment Engine direction
- SMS package payment readiness
- module registry foundation
- finance foundation
- support operations foundation
- reporting foundation
- future premium modules and plans

## Primary Users

- organization owner/admin
- billing/admin user
- platform support/admin user
- revenue operations user
- platform finance/admin user

## MVP Scope

Included:

- plan catalog metadata
- subscription account foundation
- subscription status foundation
- trial placeholder
- invoice metadata foundation
- billing cycle metadata foundation
- payment reference placeholder through Payment Engine
- entitlement change history
- renewal and cancellation placeholders
- revenue operation notes
- audit and permission direction

Excluded:

- direct payment provider calls inside billing domain
- tax calculation engine
- full accounting ledger
- automatic card charging
- revenue recognition engine
- external billing provider integration
- cross-organization billing visibility

## Domain Ownership

Billing Foundation owns subscription records, invoice metadata, billing cycles, entitlement change history, and revenue operation notes.

Payment Engine owns provider-specific payment behavior and payment status updates.

Module/Subscription Engine owns feature entitlement evaluation.

Finance Domain may consume invoice/payment summaries later but should not own subscription status.

Platform Core must not contain provider-specific billing rules.

## Required Engines

- Payment Engine
- Permission Engine
- Audit Engine
- Module Registry or Feature Entitlement Engine
- Notification Engine for billing reminders later
- Reporting Engine for revenue reports later
- Feature Flag or License Engine

## Suggested Permissions

```text
billing.view
billing.manage
billing.invoices.view
billing.invoices.manage
billing.payments.view
billing.entitlements.view
billing.entitlements.manage
billing.revenue_notes.view
billing.revenue_notes.manage
billing.admin.manage_plans
```

## Data Ownership

Records should include:

- plan catalog metadata
- subscription account
- billing cycle metadata
- invoice metadata
- payment reference placeholder
- entitlement change history
- renewal/cancellation placeholder
- revenue operation note

## API Direction

Planned API groups:

- billing dashboard
- plan catalog
- subscriptions
- invoices
- payment references
- entitlement history
- renewal/cancellation placeholders
- revenue operation notes

## UI Direction

Planned screens:

- billing dashboard
- plan list
- subscription detail
- invoice list
- invoice detail
- entitlement history
- renewal/cancellation placeholder
- revenue operations notes

## Audit Direction

Audit important actions:

- plan metadata changed
- subscription created or updated
- subscription status changed
- invoice metadata created or updated
- payment reference attached
- entitlement changed
- cancellation requested placeholder
- revenue note added

## Revenue Direction

This wave supports subscriptions, premium modules, paid add-ons, SMS packages, managed support tiers, marketplace monetization, and future revenue reporting.

## Risks

- entitlement state drifting from subscription status
- billing records visible to unauthorized users
- direct payment provider calls leaking into billing domain
- invoices treated as full accounting ledger too early
- cancellation or renewal changes not audited

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Billing should explain plan status and entitlement state while delegating payment execution to Payment Engine.
