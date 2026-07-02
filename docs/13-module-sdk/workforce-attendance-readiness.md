# Workforce Attendance Readiness

## Purpose

This document prepares the workforce attendance and leave foundation for implementation.

It ensures attendance and leave records are linked to workforce profiles, organization-scoped, permission-protected, and auditable.

## Wave Summary

Build foundations for attendance sessions, check-in/check-out records, manual adjustments, leave requests, approval placeholders, and attendance summaries.

## Problem Being Solved

Organizations need a controlled way to know who was present, who was away, and which attendance records require correction.

## MVP Scope

Included:

- attendance session foundation
- check-in/check-out record foundation
- manual adjustment foundation
- leave request placeholder
- leave approval placeholder
- attendance summary foundation

Excluded:

- payroll
- overtime payment calculation
- biometric attendance
- facial recognition
- complex shift planning
- full leave entitlement engine
- tax or benefits features

## Domain Ownership

HR Domain owns workforce attendance and leave workflows.

Workflow Engine may support leave approval placeholders.

Audit Engine records important changes.

Platform Core must not own HR attendance business rules.

## Required Engines

- Permission Engine
- Audit Engine
- Workflow Engine where leave approvals are used
- Notification Engine later
- Reporting Engine later
- Feature Flag or License Engine

## Suggested Permissions

```text
hr.attendance.view
hr.attendance.manage
hr.attendance.check_in
hr.attendance.adjust
hr.leave.view
hr.leave.request
hr.leave.approve
hr.leave.manage
hr.attendance.summary.view
```

## Data Ownership

Organization-owned records should include:

- attendance session
- attendance record
- manual adjustment
- leave request
- leave approval placeholder
- attendance summary snapshot where implemented

## API Direction

Planned API groups:

- attendance sessions
- check-in/check-out
- manual adjustments
- leave requests
- leave approval placeholder
- attendance summaries

## UI Direction

Planned screens:

- attendance dashboard
- session list
- check-in/check-out screen
- adjustment review
- leave request list
- leave request detail
- approval queue placeholder

## Audit Direction

Audit important actions:

- attendance session created or updated
- check-in recorded
- check-out recorded
- manual adjustment created or approved
- leave request created
- leave request approved or rejected

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Workforce attendance should improve organization visibility without introducing payroll or biometric complexity in MVP.
