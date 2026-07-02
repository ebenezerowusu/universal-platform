# Subscription Billing Revenue Operations Guide

## Purpose

This document defines operating guidance for plan catalog metadata, subscriptions, invoices, payment references, entitlement changes, renewals, cancellations, and revenue operation notes.

The goal is to keep commercial status explainable, auditable, and aligned with feature entitlements.

## Principle

Billing should explain why an organization has access to a plan, module, or feature.

Payment Engine should explain payment execution and provider status.

## Data Sources

Billing operations may use:

- plan catalog metadata
- subscription account
- billing cycle metadata
- invoice metadata
- payment reference placeholder
- entitlement change history
- renewal/cancellation placeholder
- revenue operation note
- audit records

## Plan Catalog Direction

Plan catalog metadata should capture:

```text
plan_key
title
description optional
billing_interval_placeholder
currency
amount_placeholder
status
```

## Subscription Direction

Subscriptions should capture:

```text
organization_id
plan_key
status
started_at
current_period_start
current_period_end
trial_ends_at optional
cancelled_at optional
```

## Invoice Direction

Invoice metadata should capture:

```text
invoice_number_placeholder
organization_id
subscription_id
amount
currency
status
due_date
paid_at optional
```

Invoice metadata is not a full accounting ledger in MVP.

## Payment Reference Direction

Payment references should capture:

```text
payment_reference_id
payment_engine_reference
invoice_id
status_placeholder
attached_at
```

Do not store provider-specific payment behavior inside billing domain logic.

## Entitlement Direction

Entitlement changes should be recorded when billing changes affect access.

Track:

- plan started
- plan upgraded
- plan downgraded
- module enabled
- module disabled
- subscription suspended
- subscription restored
- trial started or ended

## Renewal and Cancellation Direction

Renewal/cancellation placeholders should capture:

- requested by
- requested date
- effective date placeholder
- reason optional
- status
- grace period placeholder

## Audit Direction

Audit should record:

- plan metadata changed
- subscription created or updated
- subscription status changed
- invoice metadata changed
- payment reference attached
- entitlement changed
- cancellation requested placeholder
- revenue note added

## Data Quality Warnings

Warn or flag when:

- active subscription has no current billing period
- invoice is overdue placeholder
- payment reference does not match invoice organization
- entitlement state does not match subscription status
- cancellation requested but entitlement still active after effective date placeholder

## Final Rule

Billing operations must keep subscription state, invoice state, payment references, and feature entitlements aligned without directly calling payment providers.
