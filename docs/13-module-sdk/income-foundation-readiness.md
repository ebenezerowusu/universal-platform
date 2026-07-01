# Income Foundation Readiness

## Purpose

This document prepares the organization income foundation for implementation.

It keeps the first finance workflow reusable across domains while allowing Religious giving to be built on top of it.

## Wave Summary

Build foundations for income categories, commitments, income records, payment-linked income, receipts, and reconciliation.

## Problem Being Solved

Organizations need a safe way to record and review money received.

Religious organizations also need giving and pledge support, but the underlying finance foundation should remain reusable.

## Evidence

This wave is supported by:

- Finance Domain direction
- Religious giving and pledge requirements
- Payment Engine roadmap
- revenue and reporting needs
- need for auditable money records

## Primary Users

- organization owner/admin
- finance/admin user
- Religious finance team where applicable
- platform support/admin user

## MVP Scope

Included:

- income category foundation
- commitment or pledge foundation
- income record foundation
- payment-linked income foundation
- receipt foundation
- basic ledger foundation
- reconciliation foundation

Excluded:

- full accounting
- full expense management
- payroll
- tax filing
- full invoicing
- advanced finance reports

## Domain Ownership

Finance Domain owns generic income workflows.

Religious Domain may provide giving labels and workflows that use Finance Domain capabilities.

Payment Engine owns payment initiation and normalized payment status.

Platform Core must not own finance-specific business rules.

## Required Engines

- Payment Engine
- Permission Engine
- Audit Engine
- Configuration Engine
- Feature Flag or License Engine
- Reporting Engine later

## Suggested Permissions

```text
finance.income.view
finance.income.create
finance.income.manage
finance.categories.view
finance.categories.manage
finance.commitments.view
finance.commitments.manage
finance.receipts.view
finance.reconciliation.review
```

## Data Ownership

Organization-owned records should include:

- income category
- income commitment
- income record
- payment reference link
- receipt record
- ledger entry
- reconciliation item

## API Direction

Planned API groups:

- categories
- commitments
- income records
- payment initiation
- receipts
- reconciliation review

## Flutter Direction

Planned screens:

- income dashboard
- category list
- commitment list
- record income
- income history
- receipt details
- reconciliation review placeholder

## Audit Direction

Audit important actions:

- category created or updated
- commitment created or updated
- income record created or adjusted
- payment status changed
- receipt issued
- reconciliation resolved

## Revenue Direction

This wave supports organization value and future premium finance/reporting features.

## Risks

- overbuilding accounting too early
- mixing Religious-specific labels into Platform Core
- accepting payment without clear status tracking
- creating records without auditability
- weak reconciliation around payment-linked income

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

The income foundation should support giving use cases without becoming a hardcoded Religious-only module.
