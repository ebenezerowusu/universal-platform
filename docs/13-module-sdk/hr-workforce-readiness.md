# HR Workforce Readiness

## Purpose

This document prepares the HR and Workforce foundation for implementation.

It ensures workforce records are privacy-aware, organization-scoped, permission-protected, and auditable.

## Wave Summary

Build foundations for workforce profiles, departments, teams, position assignments, status history, HR references, attendance hooks, and leave/request placeholders.

## Problem Being Solved

Organizations need a controlled way to manage workers, staff, volunteers, departments, and assignments.

Without an HR foundation, organization roles and people operations become scattered across domain-specific modules.

## Evidence

This wave is supported by:

- HR Domain direction
- Permission Engine direction
- Audit Engine direction
- Workflow Engine direction
- Religious volunteers and workers needs
- future staff attendance and leave needs

## Primary Users

- organization owner/admin
- HR/admin user
- department leader where permitted
- staff member or volunteer where self-service is later enabled
- platform support/admin user

## MVP Scope

Included:

- workforce profile foundation
- staff/volunteer type foundation
- department/team foundation
- position/role assignment foundation
- worker status history foundation
- HR reference placeholder
- attendance integration direction
- leave/request placeholder

Excluded:

- payroll
- tax filing
- benefits administration
- biometric attendance
- complex performance management
- full HR document processing
- advanced leave workflow

## Domain Ownership

HR Domain owns workforce workflows.

Permission Engine owns access decisions.

Audit Engine records sensitive HR changes.

Workflow Engine may support later approvals.

Platform Core must not own HR-specific business rules.

## Required Engines

- Permission Engine
- Audit Engine
- Workflow Engine later
- Storage Engine where HR files are used later
- Attendance Engine where staff attendance is enabled later
- Feature Flag or License Engine

## Suggested Permissions

```text
hr.workforce.view
hr.workforce.create
hr.workforce.manage
hr.departments.view
hr.departments.manage
hr.assignments.view
hr.assignments.manage
hr.status_history.view
hr.status_history.manage
hr.documents.view_placeholder
hr.leave.view_placeholder
hr.leave.manage_placeholder
```

## Data Ownership

Organization-owned records should include:

- workforce profile
- department/team
- position assignment
- status history
- HR reference placeholder
- leave/request placeholder
- attendance link reference where used later

## API Direction

Planned API groups:

- departments
- workforce profiles
- assignments
- status history
- HR references
- leave/request placeholder
- dashboard summary

## Flutter Direction

Planned screens:

- HR dashboard
- department list
- workforce list
- workforce profile detail
- assignment history
- status history
- HR reference placeholder

## Audit Direction

Audit important actions:

- workforce profile created or updated
- department created or updated
- assignment created or ended
- worker status changed
- HR reference added or removed
- leave placeholder action created or updated

## Revenue Direction

This wave supports future premium HR, attendance, leave, payroll, and workforce reporting modules.

## Risks

- exposing sensitive HR data without proper permissions
- mixing staff records with general member/customer records incorrectly
- creating status changes without audit
- overbuilding payroll too early
- hardcoding religious worker concepts into HR foundation

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

HR foundation should support staff and volunteers across organization types while protecting privacy and avoiding payroll complexity in MVP.
