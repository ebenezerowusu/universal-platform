# Reporting Engine Specification

## Purpose

The Reporting Engine provides a shared foundation for dashboards, reports, exports, analytics summaries, and operational insights across all domains.

Reporting must be tenant-aware, permission-aware, and module-aware.

## Responsibilities

The Reporting Engine is responsible for:

- Report definitions
- Dashboard widgets
- Aggregated metrics
- Report filters
- Export coordination
- Report access checks
- Scheduled reports later
- Report caching strategy later
- Cross-domain reporting patterns

## Non-Responsibilities

The Reporting Engine does not own domain business data.

Domains own their data and expose reportable views, queries, or events.

The Reporting Engine provides standard reporting patterns and delivery.

## Example Report Areas

Religious Domain:

- Member growth
- Visitor conversion
- Attendance trends
- Giving summaries
- Branch comparison
- Group activity

Commerce Domain:

- Sales summary
- Product performance
- Order status
- Customer activity

POS Domain:

- Daily sales
- Stock movement
- Cashier performance
- Inventory valuation

Finance Domain:

- Income and expenses
- Budgets
- Fund balances
- Financial statements

HR Domain:

- Staff count
- Attendance
- Leave balances
- Payroll summaries later

## Report Access

Every report must enforce:

- Authentication
- Organization context
- Permission
- Scope
- Module access
- Subscription access where relevant

A report is not safe because it is read-only. Reports can expose sensitive data.

## Suggested Entities

Candidate entities:

- report_definitions
- dashboard_widgets
- organization_dashboards
- report_exports
- saved_report_filters
- scheduled_reports later

## Common Filters

Reports should support common filters where relevant:

- organization_id
- branch_id
- date_from
- date_to
- status
- module
- group_id
- user_id
- category

## Export Formats

Possible export formats:

- CSV
- Excel
- PDF later

Large exports should run as background jobs.

## Dashboard Widgets

Dashboard widgets should be permission-aware.

Example widgets:

- Total members
- Attendance this week
- SMS balance
- Recent payments
- Pending follow-ups
- Upcoming events

## Performance Direction

Start with simple queries and indexes.

As data grows, consider:

- Materialized views
- Precomputed summaries
- Read replicas
- Background aggregation jobs
- Analytics database later

Do not introduce heavy analytics infrastructure before it is needed.

## Events Published

- ReportRequested
- ReportGenerated
- ReportExportStarted
- ReportExportCompleted
- ReportExportFailed
- DashboardViewed optional

## Security Requirements

- Sensitive reports require explicit permissions.
- Exports must be audited where sensitive.
- Report queries must enforce tenant filters.
- Large exports must not expose data across tenants.

## Testing Requirements

Tests must cover:

- Report permission checks
- Tenant isolation
- Date filtering
- Export creation
- Sensitive report audit logging
- Module/subscription feature checks

## Final Rule

Reports must never bypass tenant isolation or permissions. Read-only data can still be sensitive.
