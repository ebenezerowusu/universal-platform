# Sprint 28 Staff Attendance and Leave Roadmap

## Purpose

This document defines Sprint 28 for Staff Attendance and Leave foundation.

Sprint 28 should create a reusable workforce attendance and leave layer after the HR workforce foundation, without building payroll, biometrics, or full shift planning yet.

## Sprint Goal

Enable organizations to define simple attendance sessions, record staff check-in/check-out, review attendance summaries, create leave requests, and process basic leave approvals through auditable workflows.

## Why This Sprint

Sprint 27 prepared workforce profiles, departments, assignments, and status history.

The next HR layer is operational workforce presence:

- attendance sessions
- check-in/check-out
- manual corrections
- leave requests
- approval placeholders
- attendance summaries

## Work Package 1: Attendance Sessions

Prepare organization-owned attendance sessions with:

- name
- date
- start time optional
- end time optional
- department/team optional
- status

## Work Package 2: Check-In and Check-Out

Prepare records linked to workforce profiles:

- profile
- session
- check-in time
- check-out time optional
- source
- status
- notes optional

## Work Package 3: Manual Adjustment Foundation

Prepare manual correction records for:

- missed check-in
- missed check-out
- wrong time
- excused absence

Manual adjustments must be auditable.

## Work Package 4: Leave Request Placeholder

Prepare simple leave requests:

- profile
- leave type
- start date
- end date
- reason optional
- status

## Work Package 5: Leave Approval Placeholder

Use Workflow Engine direction where practical.

Suggested statuses:

```text
submitted
approved
rejected
cancelled
manual_review
```

## Work Package 6: Attendance Summary

Prepare summary direction for:

- present
- absent placeholder
- late placeholder
- on leave
- missing check-out
- manual adjustment count

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- attendance dashboard
- session list
- check-in/check-out view
- manual adjustment review
- leave request list
- leave request detail

## Out of Scope

Do not implement:

- payroll
- overtime payment calculation
- biometric attendance
- facial recognition
- complex shift planning
- full leave entitlement engine
- tax or benefits features

## Completion Standard

Sprint 28 is complete when organizations can record staff attendance, review attendance summaries, submit leave requests, and process basic leave approval placeholders through permissioned, auditable flows.

## Final Rule

Attendance and leave foundation must support workforce operations without becoming payroll or biometric surveillance in the MVP.
