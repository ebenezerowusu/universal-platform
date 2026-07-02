# Sprint 33 Reporting Analytics Roadmap

## Purpose

This document defines Sprint 33 for Reporting and Analytics foundation.

Sprint 33 should create a reusable reporting layer for dashboards, metrics, report definitions, filters, export placeholders, and data-quality warnings across all platform domains.

## Sprint Goal

Enable organizations to view safe dashboards, run permission-protected reports, apply filters, review metric cards, request exports, and see data-quality warnings without building a heavy BI platform too early.

## Why This Sprint

Many platform modules now generate operational data:

- finance income and expense records
- commerce products, sales, orders, and delivery
- HR workforce and attendance
- religious members, services, events, and giving
- documents and media
- notifications and search
- calendar schedules

Without a shared reporting foundation, each module will build isolated dashboards and inconsistent report behavior.

## Work Package 1: Report Definitions

Prepare report definition metadata with:

- report key
- title
- domain/module
- description optional
- required permission
- available filters
- status

## Work Package 2: Dashboard Cards and Widgets

Prepare dashboard card/widget metadata with:

- metric key
- title
- domain/module
- display type
- required permission
- refresh behavior placeholder
- status

## Work Package 3: Metric Definitions

Prepare metric definitions such as:

```text
total_income
total_expense
active_workers
orders_pending
attendance_present
notifications_unread
documents_uploaded
calendar_items_upcoming
custom
```

## Work Package 4: Report Filters

Prepare reusable filters:

- date range
- status
- department/team
- category
- module/domain
- branch/congregation where relevant

## Work Package 5: Result Snapshot Placeholder

Prepare result snapshot placeholder for expensive reports.

Do not build a full data warehouse yet.

## Work Package 6: Export Request Placeholder

Prepare report export request direction using Import/Export Engine where practical.

Exports must be permission-protected and auditable where required.

## Work Package 7: Data-Quality Warnings

Prepare warnings for:

- missing required source data
- stale source data
- inconsistent totals
- records excluded by permission
- report partially unavailable

## Work Package 8: Flutter Foundation

Prepare screens or placeholders for:

- reporting dashboard
- metric cards
- report list
- report filters
- report result view
- export request placeholder
- data-quality warnings

## Out of Scope

Do not implement:

- full BI/OLAP warehouse
- advanced charting engine
- external analytics provider integration
- AI-generated insights
- unrestricted cross-module reporting
- sensitive exports without permissions

## Completion Standard

Sprint 33 is complete when organizations can see safe reporting dashboards, run basic reports, apply filters, request export placeholders, and view data-quality warnings through permissioned APIs and UI foundations.

## Final Rule

Reporting must be shared infrastructure that respects source-module permissions and data-quality limitations.
