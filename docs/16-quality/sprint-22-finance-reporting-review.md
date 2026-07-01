# Sprint 22 Finance Reporting Review

## Purpose

This document defines review checks for Sprint 22.

Sprint 22 implements finance reporting and cash-position visibility.

## Review 1: Scope

Sprint 22 may include:

- finance dashboard summary foundation
- period filter foundation
- income summary foundation
- expense summary foundation
- net position summary foundation
- category breakdown foundation
- budget comparison foundation where practical
- export request foundation where practical
- data-quality warnings for incomplete records
- permission and feature checks
- API and Flutter foundations

Sprint 22 should not include:

- full accounting statements
- tax reporting
- payroll reports
- external accounting integrations
- complex forecasting
- unrestricted exports without permission checks
- reports that bypass organization boundaries
- unrelated Commerce/POS analytics

## Review 2: Organization Boundaries

Confirm:

- every report is scoped to one organization
- aggregates do not include another organization's records
- export requests preserve organization scope
- system/admin views are permission-protected

## Review 3: Source Record Rules

Confirm:

- income totals use appropriate income statuses
- expense totals use appropriate expense statuses
- cancelled records are excluded unless explicitly requested
- pending records are not mixed into received/recorded totals without labeling

## Review 4: Data Quality

Confirm warnings exist for:

- unreconciled income
- unreconciled expenses
- missing receipts where required
- missing ledger entries where implemented
- currency mismatch
- invalid date or period issues

## Review 5: Permission and Feature Checks

Confirm:

- report view permission is checked
- export permission is checked where exports exist
- Finance module availability is checked
- feature availability is checked

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/finance-reporting-api-contract.md
```

## Review 7: Flutter Contract

Confirm screens align with:

```text
docs/11-ui-ux/finance-reporting-flutter-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- dashboard summary totals
- income summary by category
- expense summary by category
- net position calculation
- organization boundary aggregation
- cancelled records excluded
- export permission denied
- data-quality warning generation

## Final Rule

Sprint 22 is complete when organizations can view basic finance summaries that are scoped, permission-protected, and honest about data quality.
