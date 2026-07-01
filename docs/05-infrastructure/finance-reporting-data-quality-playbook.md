# Finance Reporting Data Quality Playbook

## Purpose

This document defines data-quality checks for finance reporting.

The goal is to prevent misleading finance summaries when source records are incomplete, unreconciled, cancelled, or outside the selected period.

## Principle

Reports should be useful and honest.

If data quality is weak, the report should show warnings instead of hiding uncertainty.

## Data Sources

Finance reports may use:

- income records
- expense records
- income categories
- expense categories
- commitments
- budget lines
- receipts
- ledger entries
- reconciliation items
- payment status records where linked

## Common Data-Quality Warnings

Warn when:

- income is pending payment
- income is received but receipt is missing where required
- expense request is approved but no expense record exists
- expense record exists without approval where policy requires approval
- ledger entry is missing
- reconciliation item is open
- cancelled record appears in a summary
- currency mismatch affects aggregation
- record date is missing or outside period

## Report Inclusion Rules

Reports should define which statuses are included.

Example direction:

```text
income totals include received income
income pending includes pending payment records
expense totals include recorded expenses
pending expenses include submitted or approved requests not yet recorded
cancelled records are excluded unless explicitly requested
```

## Currency Direction

If multiple currencies exist, reports should either:

- filter by one currency
- show separate totals by currency
- avoid converting currencies until exchange-rate policy exists

## Period Direction

Use organization timezone where available.

When timezone is missing, use platform default and show no misleading precision.

## Export Direction

Exports should include enough metadata to explain filters:

```text
organization
report_type
period
start_date
end_date
currency
created_at
created_by
```

## Support/Admin Review

Support/admin users should be able to identify:

- records blocking report confidence
- reconciliation items affecting totals
- missing receipts or ledger entries
- invalid date or currency cases

## Final Rule

Finance reporting should never hide unresolved data problems. Warnings protect trust.
