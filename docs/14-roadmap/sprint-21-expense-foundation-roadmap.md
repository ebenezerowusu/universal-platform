# Sprint 21 Expense Foundation Roadmap

## Purpose

This document defines Sprint 21 for organization expense foundation.

Sprint 21 should create a simple, auditable foundation for tracking expenses, approvals, budgets, and reconciliation without turning the MVP into a full accounting system.

## Sprint Goal

Enable organizations to define expense categories, prepare budget lines, request expenses, approve or reject requests, record expenses, and reconcile expense records.

## Why This Sprint

Sprint 20 prepared the income foundation.

A usable finance module also needs controlled spending records so organizations can understand money coming in and basic money going out.

## Work Package 1: Expense Categories

Prepare organization-owned categories such as:

```text
administration
operations
welfare
events
utilities
transport
maintenance
project expense
custom category
```

Labels should be configurable by organization and domain.

## Work Package 2: Budget Lines

Prepare simple budget lines:

- category
- amount
- currency
- period
- status
- notes

Do not build advanced budgeting in this sprint.

## Work Package 3: Expense Requests

Prepare expense requests with:

- requester
- category
- amount
- currency
- purpose
- required date optional
- status
- attachment/reference optional

## Work Package 4: Approval Foundation

Use Workflow Engine direction where practical.

Suggested statuses:

```text
draft
submitted
approved
rejected
cancelled
paid
manual_review
```

## Work Package 5: Expense Records

Prepare records for approved or manual expenses.

Expense records should link to request, category, budget line, payment reference, receipt/reference, and ledger entry where available.

## Work Package 6: Reconciliation

Prepare reconciliation between:

- expense request
- approval decision
- expense record
- payment status where linked
- ledger entry
- manual correction

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- expense dashboard
- expense categories
- budget lines
- expense requests
- expense approval
- expense history

## Out of Scope

Do not implement:

- full accounting
- full procurement
- payroll
- tax filing
- vendor management beyond simple notes
- provider-specific payment screens
- Commerce/POS finance workflows

## Completion Standard

Sprint 21 is complete when organizations can define expense categories, create expense requests, approve or reject requests, record expenses, and reconcile basic expense records through auditable flows.

## Final Rule

Expense foundation must control spending without overbuilding accounting. Keep it simple, auditable, and reusable.
