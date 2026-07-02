# Sprint 28 Staff Attendance and Leave Review

## Purpose

This document defines review checks for Sprint 28.

Sprint 28 implements Staff Attendance and Leave foundation.

## Review 1: Scope

Sprint 28 may include:

- staff attendance feature registration foundation
- work schedule/session foundation where practical
- check-in/check-out record foundation
- manual attendance adjustment foundation
- leave request placeholder foundation
- leave approval placeholder where practical
- attendance summary foundation
- permission and feature checks
- audit records for important attendance/leave actions
- API and UI foundations

Sprint 28 should not include:

- payroll
- overtime payment calculation
- biometric attendance
- facial recognition
- complex shift planning
- full leave entitlement engine
- tax or benefits features
- direct payment provider calls inside HR Domain
- unrelated Commerce/POS expansion

## Review 2: Domain Boundaries

Confirm:

- HR Domain owns attendance and leave workflows
- Workflow Engine supports leave approval placeholders where used
- Audit Engine records important changes
- Platform Core does not contain HR attendance business rules

## Review 3: Organization Boundaries

Confirm:

- sessions are organization-owned
- attendance records are organization-owned
- leave requests are organization-owned
- records cannot cross organization boundaries
- dashboard aggregates use only organization records

## Review 4: Attendance Rules

Confirm:

- duplicate check-in is prevented where required
- check-out without check-in is handled safely
- closed sessions cannot be edited without permission
- manual adjustments require reason and audit

## Review 5: Leave Rules

Confirm:

- leave requests are linked to workforce profiles
- approval/rejection is permission-protected
- overlapping attendance/leave is flagged where practical
- leave status history is auditable

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/staff-attendance-leave-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/staff-attendance-leave-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- attendance session create/update
- check-in behavior
- duplicate check-in prevention
- check-out behavior
- manual adjustment creation
- leave request creation
- leave approve/reject behavior
- permission denied behavior
- summary counts

## Final Rule

Sprint 28 is complete when organizations can record attendance, review staff presence, submit leave requests, and process approval placeholders through permission-protected and auditable workflows.
