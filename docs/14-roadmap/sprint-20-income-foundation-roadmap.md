# Sprint 20 Income Foundation Roadmap

## Purpose

This document defines Sprint 20 for organization income foundation.

Sprint 20 should create a simple, auditable foundation for recording income, initiating payments where available, issuing basic receipts, and reconciling records.

## Sprint Goal

Enable organizations to track income categories, commitments, received amounts, payment status, receipts, and reconciliation in a way that can support Religious giving first and future finance domains later.

## Why This Sprint

The platform already has direction for:

- Payment Engine
- SMS package purchases
- Finance Domain
- Religious giving and pledges

Sprint 20 converts those foundations into the first reusable finance workflow.

## Work Package 1: Income Categories

Prepare organization-owned categories such as:

```text
general income
donation
offering
tithe
pledge
special project
membership contribution
custom category
```

The platform should allow labels to be configured by organization and domain.

## Work Package 2: Commitments or Pledges

Prepare simple commitment records:

- person or contact reference optional
- category
- amount
- currency
- due date optional
- status

## Work Package 3: Income Records

Prepare income records for:

- manual received amount
- payment-linked amount
- commitment-linked amount
- anonymous amount where allowed

## Work Package 4: Payment Engine Link

Where online payment is used, initiate through Payment Engine.

No domain should call payment providers directly.

## Work Package 5: Receipts

Prepare receipt records with:

- receipt number
- amount
- currency
- category
- payer or anonymous indicator
- payment reference where available

## Work Package 6: Reconciliation

Prepare reconciliation between:

- income record
- payment status
- receipt record
- ledger entry
- manual correction

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- income dashboard
- categories
- commitments/pledges
- record income
- payment status
- receipts

## Out of Scope

Do not implement:

- advanced accounting
- full expense management
- payroll
- tax filing
- full invoicing
- provider-specific payment screens
- Commerce/POS finance workflows

## Completion Standard

Sprint 20 is complete when organizations can define income categories, record income, track basic payment status, issue basic receipts, and reconcile income records through auditable flows.

## Final Rule

Finance foundations must be reusable. Religious giving can use this foundation, but the foundation must not be hardcoded to one organization type.
