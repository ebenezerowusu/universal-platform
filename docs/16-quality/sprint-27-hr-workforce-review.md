# Sprint 27 HR Workforce Review

## Purpose

This document defines review checks for Sprint 27.

Sprint 27 implements HR and Workforce foundation.

## Review 1: Scope

Sprint 27 may include:

- HR module registration foundation
- workforce person/profile foundation
- department/team foundation
- position/role assignment foundation
- worker status history foundation
- staff/volunteer type foundation
- HR document/reference placeholder where practical
- attendance integration hook direction where practical
- leave/request placeholder where practical
- permission and feature checks
- audit records for important HR actions
- API and UI foundations

Sprint 27 should not include:

- payroll
- tax filing
- benefits administration
- complex performance management
- biometric attendance
- direct payment provider calls inside HR Domain
- sensitive document processing beyond placeholders
- unrelated Commerce/POS expansion

## Review 2: Domain Boundaries

Confirm:

- HR Domain owns workforce workflows
- Permission Engine owns access decisions
- Audit Engine records important changes
- Platform Core does not contain HR-specific business rules

## Review 3: Organization Boundaries

Confirm:

- every workforce record is organization-owned
- departments are organization-owned
- assignments cannot cross organization boundaries
- dashboard aggregates use only organization records

## Review 4: Privacy and Access

Confirm:

- workforce views require permission
- sensitive references are hidden unless permitted
- customer/member/public views do not expose workforce records
- support/admin access is controlled and auditable

## Review 5: History Rules

Confirm:

- status changes are recorded as history
- assignments preserve history
- ended workers do not silently lose prior assignments
- inactive departments do not silently remove historical assignments

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/workforce-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/hr-workforce-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- department create/update
- workforce profile create/update
- assignment create/end
- status history creation
- organization boundary enforcement
- permission denied behavior
- inactive department behavior
- dashboard summary counts

## Final Rule

Sprint 27 is complete when organizations can manage workforce profiles, departments, assignments, and status history through permission-protected and auditable workflows.
