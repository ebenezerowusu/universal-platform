# Reporting Data Quality Operations Guide

## Purpose

This document defines operating guidance for reporting data quality, export placeholders, report definitions, and dashboard metrics.

The goal is to make reports useful while being honest about missing, stale, restricted, or partial data.

## Principle

A report should show what it knows, clearly explain what it does not know, and never bypass source-module permissions.

## Data Sources

Reporting workflows may use:

- report definition
- metric definition
- dashboard card metadata
- source domain records
- filter metadata
- report result snapshot placeholder
- export request placeholder
- data-quality warning
- audit records

## Data-Quality Direction

Reports should warn when:

- source module is unavailable
- source data is stale
- required source records are missing
- totals do not reconcile
- user permissions hide part of the data
- filters exclude expected records

## Partial Data Direction

When report data is partial, response and UI should indicate:

```text
partial_data=true
reason=permission_limited | source_unavailable | stale_source | missing_records
```

Do not silently show incomplete totals as final truth.

## Export Direction

Exports should check:

- report permission
- export permission
- source-module access
- data sensitivity
- organization ownership

Export requests should be auditable where policy requires it.

## Snapshot Direction

Snapshots may be used for expensive reports.

Snapshots should capture:

```text
report_key
filters
created_by
created_at
status
data_quality_status
```

## Metric Direction

Metrics should define:

- source module
- calculation ownership
- required permission
- refresh behavior placeholder
- data-quality behavior

## Audit Direction

Audit may record:

- sensitive report viewed
- export requested
- report definition changed
- dashboard configuration changed
- data-quality warning resolved

## Data Quality Warning Types

Suggested warning types:

```text
missing_source_data
stale_source_data
permission_limited
reconciliation_mismatch
source_module_unavailable
partial_result
manual_review
```

## Final Rule

Reporting must be transparent about weak data and must never use reports or exports to bypass source-module access rules.
