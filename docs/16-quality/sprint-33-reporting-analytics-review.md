# Sprint 33 Reporting Analytics Review

## Purpose

This document defines review checks for Sprint 33.

Sprint 33 implements Reporting and Analytics foundation.

## Review 1: Scope

Sprint 33 may include:

- reporting feature registration foundation
- report definition metadata foundation
- dashboard card/widget metadata foundation
- metric definition foundation
- report filter foundation
- report result snapshot placeholder where practical
- export request placeholder where practical
- scheduled report placeholder where practical
- data-quality warning foundation
- permission and feature checks
- audit records for sensitive reports/exports where required
- API and UI foundations

Sprint 33 should not include:

- full BI/OLAP warehouse
- advanced charting engine
- external analytics provider integration
- AI-generated insights
- unrestricted cross-module reporting
- sensitive report exports without permissions
- provider-specific reporting calls inside domain modules
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Reporting Foundation owns report definitions and orchestration
- domain modules own source data and metric calculations
- Permission Engine controls access decisions
- Import/Export Engine owns export execution where used
- Platform Core does not contain domain-specific report calculations

## Review 3: Organization Boundaries

Confirm:

- report definitions are organization-aware or platform-defined safely
- report executions are organization-scoped
- metrics use only organization-owned source records
- export requests cannot cross organization boundaries

## Review 4: Permission Rules

Confirm:

- reports check required permission
- exports require explicit export permission
- disabled modules do not contribute active report data
- partial results are marked clearly where permissions hide data

## Review 5: Data-Quality Rules

Confirm warnings exist where practical for:

- missing source data
- stale source data
- permission-limited data
- reconciliation mismatch
- source module unavailable
- partial result

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/reporting-analytics-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/reporting-analytics-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- dashboard summary scope
- metric card retrieval
- report definition retrieval
- report execution filters
- permission denied behavior
- export request permission
- partial data warning behavior
- source module unavailable behavior
- organization boundary enforcement

## Final Rule

Sprint 33 is complete when organizations can view safe dashboards, run permission-protected reports, request export placeholders, and understand data-quality warnings without bypassing source-module permissions.
