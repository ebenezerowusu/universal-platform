# Sprint 20 Income Foundation Review

## Purpose

This document defines review checks for Sprint 20.

Sprint 20 implements the organization income foundation.

## Review 1: Scope

Sprint 20 may include:

- income module registration foundation
- income category foundation
- commitment or pledge foundation
- income record foundation
- Payment Engine initiation foundation where payment is used
- status tracking foundation
- receipt record foundation
- income ledger foundation
- reconciliation between payment records and income records
- permission and feature checks
- audit records for important actions
- API and Flutter foundations

Sprint 20 should not include:

- direct payment provider calls inside Finance Domain
- hardcoded provider logic in domain workflows
- live provider credentials in repository
- advanced accounting
- payroll
- full expense management
- tax filing
- full invoicing
- unrelated Commerce/POS features

## Review 2: Domain Boundaries

Confirm:

- Finance Domain owns generic income workflows
- Religious Domain may use this foundation for giving labels and workflows
- Platform Core does not contain finance-specific business rules
- Payment Engine owns payment provider interaction

## Review 3: Payment Engine Boundaries

Confirm:

- payment initiation goes through Payment Engine
- provider details stay behind adapters
- Finance Domain does not call payment providers directly
- Flutter does not expose low-level provider details

## Review 4: Income Rules

Confirm:

- income categories are organization-owned
- income records are organization-owned
- commitments are organization-owned
- receipts are unique and traceable
- ledger entries are linked where implemented

## Review 5: Reconciliation

Confirm:

- payment confirmed but income not received can be reviewed
- received income without receipt can be reviewed
- missing ledger entry can be reviewed
- manual review actions are auditable

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/income-foundation-api-contract.md
```

## Review 7: Flutter Contract

Confirm screens align with:

```text
docs/11-ui-ux/income-foundation-flutter-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- category create/update
- commitment create/update
- income record creation
- payment initiation
- payment status update
- receipt issuing
- duplicate receipt prevention
- permission denied behavior
- reconciliation item behavior

## Final Rule

Sprint 20 is complete when organizations can track basic income through auditable, organization-scoped, provider-agnostic workflows.
