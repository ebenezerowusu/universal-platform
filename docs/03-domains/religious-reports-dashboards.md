# Religious Reports and Dashboards Specification

## Purpose

This document defines Religious Domain reports and dashboards.

Religious reports must use the Reporting Engine and must respect organization boundaries, permissions, module status, and subscription rules.

## Dashboard Areas

Recommended dashboard areas:

- Organization overview
- Branch overview
- Member activity
- Visitor activity
- Attendance trends
- Giving summary
- Group activity
- Event activity
- Care follow-up summary
- SMS/communication summary

## Key Metrics

Possible metrics:

- Total members
- Active members
- New members
- Inactive members
- First-time visitors
- Visitor conversion rate
- Attendance this week
- Attendance trend
- Giving by category
- Open care follow-ups
- Upcoming events
- SMS balance

## Report Types

Possible reports:

- Member report
- Visitor report
- Attendance report
- Giving report
- Branch hierarchy report
- Group report
- Education program report
- Event report
- Communication report
- Care report

## Filters

Reports may support:

- date_from
- date_to
- branch_id
- congregation_id
- service_id
- group_id
- program_id
- status
- category
- leader_id

## Scope Rules

Report visibility depends on:

- user permissions
- leadership assignment
- branch scope
- hierarchy governance settings
- module availability
- subscription plan

## Sensitive Reports

Sensitive reports require stricter permissions.

Examples:

- Member giving summary
- Welfare support reports
- Counseling summary reports
- Confidential care activity
- Detailed member personal data exports

## Export Rules

Exports must use the Import/Export Engine.

Sensitive exports should be audited.

## Required Platform Engines

This module must use:

- Reporting Engine for report definitions and execution patterns
- Permission Engine for access control
- Organization Engine for tenant context
- Audit Engine for sensitive exports
- Subscription Engine for premium reports
- Module Registry for feature availability

## Events Published

- ReligiousReportViewed
- ReligiousReportExported
- ReligiousDashboardViewed

## Testing Requirements

Tests must cover:

- Basic report generation
- Date filtering
- Branch filtering
- Permission checks
- Scope checks
- Sensitive export audit
- Organization boundary checks

## Final Rule

Reports are not harmless just because they are read-only. Religious reports must be permission-aware and tenant-safe.
