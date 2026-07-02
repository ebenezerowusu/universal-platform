# Staff Attendance and Leave Governance Playbook

## Purpose

This document defines governance guidance for staff attendance and leave records.

The goal is to keep attendance and leave records explainable, fair, permission-protected, and separate from payroll until payroll is explicitly planned.

## Principle

Attendance and leave records should be organization-scoped, auditable, and linked to workforce profiles.

## Data Sources

Attendance and leave workflows may use:

- workforce profile
- department/team
- attendance session
- attendance record
- manual adjustment
- leave request
- approval placeholder
- audit records

## Access Direction

Access should be based on:

- organization membership
- HR/workforce feature availability
- permission level
- department scope where implemented later

## Manual Adjustment Direction

Manual adjustments should capture:

```text
record
adjustment_type
old_value
new_value
reason
created_by
created_at
reviewed_by optional
```

## Leave Request Direction

Leave requests should preserve:

```text
profile
leave_type
start_date
end_date
status
submitted_by
approved_by optional
reason optional
```

## Attendance Summary Direction

Summaries should distinguish:

- checked in
- checked out
- missing checkout
- on leave
- manually adjusted
- excused placeholder

Do not mix these states without labels.

## Data Quality Warnings

Warn or flag when:

- profile has duplicate check-in for one session
- check-out exists without check-in
- session is closed but records are still editable
- leave request overlaps attendance record
- manual adjustment has no reason

## Audit Direction

Audit should record:

- session created or updated
- check-in recorded
- check-out recorded
- adjustment created or reviewed
- leave request created
- leave request approved or rejected

## Payroll Boundary

Attendance records may support future payroll, but Sprint 28 must not calculate payroll or pay rules.

## Final Rule

Attendance and leave records must remain explainable after the fact and must not silently affect payroll or compensation in MVP.
