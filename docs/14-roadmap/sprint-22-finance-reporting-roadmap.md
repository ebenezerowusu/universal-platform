# Sprint 22 Finance Reporting Roadmap

## Purpose

This document defines Sprint 22 for finance reporting and cash-position visibility.

Sprint 22 should summarize income and expense foundations into useful organization reporting without overbuilding a full accounting suite.

## Sprint Goal

Enable organizations to view basic finance summaries, period-based income and expenses, category breakdowns, budget comparison, and net position.

## Why This Sprint

Sprint 20 prepared income tracking.

Sprint 21 prepared expense tracking.

Organizations now need visibility over:

- money received
- money spent
- net position
- category trends
- budget usage
- incomplete or unreconciled records

## Work Package 1: Finance Dashboard Summary

Prepare dashboard summary cards:

```text
total income
total expenses
net position
pending income
pending expenses
unreconciled items
```

## Work Package 2: Period Filters

Prepare filters for:

- today
- this week
- this month
- this quarter
- this year
- custom date range

Use organization timezone rules where available.

## Work Package 3: Income Summary

Summarize by:

- category
- period
- payment status where linked
- source where available

## Work Package 4: Expense Summary

Summarize by:

- category
- period
- approval status
- budget line where available

## Work Package 5: Budget Comparison

Where budget lines exist, compare:

- budgeted amount
- approved expenses
- recorded expenses
- remaining amount

## Work Package 6: Data Quality Warnings

Show warnings for:

- income without receipt where required
- expense without approval where policy requires approval
- missing ledger entry
- unreconciled payment-linked record
- cancelled records appearing in totals

## Work Package 7: Export Foundation

Prepare export request foundation where practical.

Exports must be permission-protected and auditable where implemented.

## Out of Scope

Do not implement:

- full accounting statements
- tax reporting
- payroll reports
- complex forecasting
- external accounting integrations
- unrestricted exports

## Completion Standard

Sprint 22 is complete when organizations can view basic finance summaries and understand income, expenses, net position, and data-quality warnings.

## Final Rule

Reporting must summarize traceable records. Do not hide weak data quality behind attractive charts.
