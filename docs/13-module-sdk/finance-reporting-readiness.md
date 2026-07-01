# Finance Reporting Readiness

## Purpose

This document prepares the finance reporting foundation for implementation.

It ensures reports are based on auditable income and expense records and remain reusable across organization types.

## Wave Summary

Build foundations for dashboard summaries, period filters, income summary, expense summary, net position, budget comparison, data-quality warnings, and export request foundation.

## Problem Being Solved

Organizations need simple visibility into money received, money spent, and basic net position.

Without reporting, income and expense records are useful only as raw history and not as decision support.

## Evidence

This wave is supported by:

- income foundation direction
- expense foundation direction
- finance domain roadmap
- support/admin need for data-quality visibility
- organization need for simple financial summaries

## Primary Users

- organization owner/admin
- finance/admin user
- Religious finance team where applicable
- platform support/admin user

## MVP Scope

Included:

- finance dashboard summary
- period filters
- income summary
- expense summary
- net position summary
- category breakdowns
- budget comparison where practical
- data-quality warnings
- export request foundation where practical

Excluded:

- full accounting statements
- tax reporting
- payroll reporting
- external accounting integrations
- complex forecasting
- advanced audit analytics

## Domain Ownership

Finance Domain owns finance reporting workflows.

Reporting Engine may support reusable report generation patterns where practical.

Export behavior should use Import/Export Engine where available.

Platform Core must not contain finance-specific reporting logic.

## Required Engines

- Permission Engine
- Audit Engine
- Reporting Engine where practical
- Import/Export Engine where practical
- Configuration Engine
- Feature Flag or License Engine

## Suggested Permissions

```text
finance.reports.view
finance.reports.export
finance.dashboard.view
finance.data_quality.view
```

## Data Ownership

Reports must derive from organization-owned records:

- income records
- expense records
- categories
- commitments
- budget lines
- receipts
- ledger entries
- reconciliation items

## API Direction

Planned API groups:

- dashboard summary
- income summary
- expense summary
- net position
- budget comparison
- data-quality warnings
- export request

## Flutter Direction

Planned screens:

- finance dashboard
- income summary
- expense summary
- budget comparison
- data-quality warnings
- export request placeholder

## Audit Direction

Audit important actions:

- report export requested
- report export downloaded where implemented
- sensitive summary viewed where policy requires it

## Revenue Direction

Basic finance dashboard may be included in the Finance module.

Advanced reports and exports may become premium later.

## Risks

- reports including cancelled or draft records incorrectly
- missing organization scope in aggregates
- exporting data without permission checks
- presenting unreliable data without warnings
- overbuilding accounting reports too early

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Finance reporting should be honest about data quality and must never cross organization boundaries.
