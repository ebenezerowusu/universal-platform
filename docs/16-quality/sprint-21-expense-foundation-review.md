# Sprint 21 Expense Foundation Review

## Purpose

This document defines review checks for Sprint 21.

Sprint 21 implements the organization expense foundation.

## Review 1: Scope

Sprint 21 may include:

- expense category foundation
- budget line foundation
- expense request foundation
- approval workflow foundation where practical
- expense record foundation
- attachment/reference foundation where practical
- payment status placeholder where applicable
- expense ledger foundation
- reconciliation between request, approval, payment, and ledger records
- permission and feature checks
- audit records for important actions
- API and Flutter foundations

Sprint 21 should not include:

- full accounting
- payroll
- tax filing
- full procurement
- vendor management beyond simple payee notes
- direct payment provider calls inside Finance Domain
- live provider credentials in repository
- unrelated Commerce/POS features

## Review 2: Domain Boundaries

Confirm:

- Finance Domain owns generic expense workflows
- Workflow Engine supports approval where used
- Storage Engine supports attachments where used
- Platform Core does not contain finance-specific business rules

## Review 3: Approval Rules

Confirm:

- expense requests are organization-owned
- approval actions are permission-protected
- approval decisions are auditable
- rejected/cancelled requests cannot be silently converted to expense records

## Review 4: Expense Rules

Confirm:

- expense categories are organization-owned
- budget lines are organization-owned
- expense records are organization-owned
- ledger entries are linked where implemented
- attachments or references are access-controlled where used

## Review 5: Reconciliation

Confirm:

- approved but not recorded items can be reviewed
- recorded without approval items can be reviewed where policy requires approval
- missing ledger entry can be reviewed
- manual review actions are auditable

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/expense-foundation-api-contract.md
```

## Review 7: Flutter Contract

Confirm screens align with:

```text
docs/11-ui-ux/expense-foundation-flutter-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- category create/update
- budget line create/update
- expense request creation
- approve/reject behavior
- expense record creation
- permission denied behavior
- invalid approval action
- reconciliation item behavior

## Final Rule

Sprint 21 is complete when organizations can track basic expenses through auditable, organization-scoped, permission-protected workflows.
