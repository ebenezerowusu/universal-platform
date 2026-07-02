# Sprint 27 HR Workforce Roadmap

## Purpose

This document defines Sprint 27 for HR and Workforce foundation.

Sprint 27 should create a reusable workforce layer for staff, volunteers, departments, role assignments, status history, and basic HR records without building payroll or advanced performance management yet.

## Sprint Goal

Enable organizations to define departments, create workforce profiles, assign roles/positions, track worker status, and maintain auditable HR history.

## Why This Sprint

The platform supports organizations beyond commerce and religious workflows.

Every organization eventually needs people operations:

- staff records
- volunteers
- departments/teams
- role assignments
- status history
- HR documents or references
- attendance and leave hooks later

## Work Package 1: Workforce Profiles

Prepare organization-owned workforce profiles with:

- person reference optional
- display name
- contact references where allowed
- worker type
- status
- start date optional
- notes optional

## Work Package 2: Departments and Teams

Prepare organization-owned departments/teams with:

- name
- code optional
- parent department optional
- status

## Work Package 3: Positions and Assignments

Prepare position/role assignment foundation:

- workforce profile
- department/team
- position label
- assignment type
- start date
- end date optional
- status

## Work Package 4: Worker Status History

Track status changes:

```text
active
inactive
on_leave
suspended
ended
volunteer_placeholder
manual_review
```

Status changes must be auditable.

## Work Package 5: HR Document/Reference Placeholder

Prepare placeholder for HR documents or references.

Use Storage Engine later where file uploads are implemented.

## Work Package 6: Attendance and Leave Direction

Prepare integration direction for:

- attendance engine hooks
- leave/request placeholders
- approval workflow later

Do not implement full HR automation yet.

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- workforce dashboard
- department list
- workforce profile list
- workforce profile detail
- assignment history
- status history

## Out of Scope

Do not implement:

- payroll
- tax filing
- benefits administration
- complex performance management
- biometric attendance
- full HR document processing
- advanced leave approval automation

## Completion Standard

Sprint 27 is complete when organizations can define departments, maintain workforce profiles, assign roles/positions, track status history, and review basic HR records through permissioned, auditable flows.

## Final Rule

HR foundation should support staff and volunteers across many organization types without becoming payroll in the MVP.
