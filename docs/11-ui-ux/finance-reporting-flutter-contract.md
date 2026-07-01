# Finance Reporting Flutter Contract

## Purpose

This document defines the MVP Flutter screen contract for finance reporting and cash-position visibility.

The screens should help organization users understand income, expenses, net position, budgets, and data-quality warnings without exposing full accounting complexity.

## Navigation Direction

Finance reporting screens should appear under Finance when:

- user is authenticated
- organization is selected
- Finance module is available
- user has required permission
- feature availability allows reports

## Finance Dashboard

Should show summary cards:

- total income
- total expenses
- net position
- pending income
- pending expenses
- unreconciled items

## Period Filter

Screens should support period filtering where available:

- today
- this week
- this month
- this quarter
- this year
- custom range

## Income Summary Screen

Should show:

- category name
- amount
- currency
- percentage or visual share where practical
- drill-in placeholder where useful

## Expense Summary Screen

Should show:

- category name
- amount
- currency
- approval status summary where useful
- drill-in placeholder where useful

## Net Position Screen

Should show:

- income
- expenses
- net amount
- selected period
- currency

## Budget Comparison Screen

Should show:

- budget line
- budgeted amount
- recorded expenses
- remaining amount
- warning when over budget where practical

## Data Quality Warnings Screen

Should show:

- warning type
- count
- severity
- short explanation
- action link where practical

Examples:

- unreconciled income
- pending expense approval
- receipt missing
- ledger entry missing

## Export Request Placeholder

Where permitted, show export action or placeholder.

Export actions should clearly indicate that export may be prepared asynchronously in future implementations.

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- filter loading
- success feedback for export request where applicable

## MVP Exclusions

Do not implement in Sprint 22:

- full accounting statements
- tax reports
- payroll reports
- external accounting integration screens
- advanced forecasting screens

## Final Rule

Flutter reports should be useful, simple, and honest about data quality. Do not present unreliable finance data as complete.
