# HR Workforce Data Governance Playbook

## Purpose

This document defines data governance guidance for HR and Workforce records.

The goal is to protect workforce data, keep status and assignment history explainable, and prevent accidental exposure of people records.

## Principle

Workforce data should be organization-scoped, permission-protected, and auditable.

## Data Sources

Workforce workflows may use:

- workforce profile
- department/team
- assignment
- status history
- reference placeholder
- attendance link reference later
- leave/request placeholder later
- audit records

## Access Direction

Access should be based on:

- organization membership
- HR/workforce feature availability
- permission level
- record ownership or department scope where implemented later

## Sensitive Data Direction

Avoid storing sensitive data until access rules and retention rules are clear.

When sensitive references are needed later, use Storage Engine and strict permissions.

## Status History Direction

Every important status change should capture:

```text
profile
old_status
new_status
effective_date
changed_by
reason optional
created_at
```

## Assignment History Direction

Assignments should preserve history.

Do not overwrite assignment history when a worker changes department or role.

Use end dates or status changes instead.

## Data Quality Warnings

Warn or flag when:

- active worker has no assignment where policy requires it
- assignment has no department
- profile has invalid status
- ended worker still has active assignment
- department is inactive but has active assignments

## Audit Direction

Audit should record:

- profile created or updated
- department created or updated
- assignment created, changed, or ended
- status changed
- reference added or removed

## Retention Direction

Retention should be organization-configurable later.

For MVP, do not hard-delete important workforce history without a clear policy.

## Final Rule

Workforce records must be explainable after the fact. Preserve history and protect access.
