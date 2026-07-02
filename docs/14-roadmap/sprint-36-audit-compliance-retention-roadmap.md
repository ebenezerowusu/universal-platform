# Sprint 36 Audit Compliance Retention Roadmap

## Purpose

This document defines Sprint 36 for Audit, Compliance, and Data Retention foundation.

Sprint 36 should create a reusable accountability layer for audit events, sensitive-action categories, access review placeholders, retention policies, safe archive/delete requests, and compliance flags across all platform domains.

## Sprint Goal

Enable organizations to review important actions, search audit events, classify sensitive operations, define retention metadata, request safe archive/delete actions, and track compliance checklist placeholders through permissioned APIs and UI foundations.

## Why This Sprint

The platform now supports sensitive operational areas:

- import/export jobs
- workflow approvals
- report exports
- document access
- finance records
- HR/workforce records
- communication preferences
- order and delivery operations
- religious member and attendance records

Without a shared audit and retention foundation, important actions may be hard to explain, review, or safely retain.

## Work Package 1: Audit Event Metadata

Prepare audit event records with:

- organization
- actor
- action
- target type
- target id
- category
- severity placeholder
- created at
- metadata placeholder

## Work Package 2: Sensitive Action Categories

Prepare action categories such as:

```text
authentication
permission_change
finance_action
hr_action
document_access
import_export
workflow_decision
report_export
retention_action
system_admin
custom
```

## Work Package 3: Audit Search and Filters

Prepare filters for:

- date range
- actor
- action
- target type
- category
- severity placeholder
- module/domain

## Work Package 4: Access Review Placeholder

Prepare access review direction for:

- users with admin roles
- users with export permissions
- users with finance permissions
- users with HR/workforce permissions
- inactive users with active permissions

## Work Package 5: Retention Policy Metadata

Prepare retention policy metadata with:

- policy key
- domain/module
- target record type
- retention period placeholder
- archive behavior placeholder
- delete behavior placeholder
- status

## Work Package 6: Archive/Delete Request Placeholder

Prepare safe request records for:

- archive request
- deletion review request
- retention exception request
- manual review

Do not implement automatic destructive deletion.

## Work Package 7: Compliance Checklist Placeholder

Prepare checklist placeholder for organization readiness:

- audit enabled
- exports reviewed
- retention policy defined
- admin access reviewed
- sensitive actions monitored

## Work Package 8: Flutter Foundation

Prepare screens or placeholders for:

- audit dashboard
- audit event list
- audit event detail
- access review placeholder
- retention policy list
- archive/delete request placeholder
- compliance checklist

## Out of Scope

Do not implement:

- legal advice engine
- automatic destructive deletion
- full GRC/compliance suite
- government filing workflows
- automated regulatory classification
- cross-organization audit visibility

## Completion Standard

Sprint 36 is complete when organizations can view audit trails, filter sensitive actions, review access placeholders, define retention metadata, and request safe archive/delete placeholders through permission-protected and auditable workflows.

## Final Rule

Audit and retention must make platform actions explainable without enabling unsafe data destruction in MVP.
