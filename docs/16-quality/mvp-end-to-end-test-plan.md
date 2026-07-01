# MVP End-to-End Test Plan

## Purpose

This document defines the MVP end-to-end test plan for Universal Platform.

The goal is to verify that the full MVP journey works across backend, Flutter, Core, Engines, and Religious Domain foundations.

## Principle

End-to-end tests should verify real user workflows, not only isolated technical units.

## Test Data Direction

Use safe test data only.

Do not use real member, visitor, payment, SMS, or private organization data.

## Flow 1: Backend Health

Validate:

- backend starts
- health endpoint returns standard success response
- logs show startup context

## Flow 2: Authentication

Validate:

- user can log in
- invalid login fails safely
- current user can be loaded
- session expiration is handled safely where implemented

## Flow 3: Organization Context

Validate:

- user can view organizations
- user can select organization in Flutter
- backend enforces organization membership
- selected organization drives subsequent requests

## Flow 4: Module and Feature Availability

Validate:

- Religious modules can be available for an organization
- unavailable module blocks Religious screens/actions safely
- feature availability is respected where implemented

## Flow 5: Member Management

Validate:

- member list loads
- empty state works
- member can be created
- created member appears in list
- member detail foundation loads where implemented
- validation errors display safely

## Flow 6: Visitor Management

Validate:

- visitor list loads
- visitor can be created
- created visitor appears in list
- visitor can be converted to member where implemented
- converted visitor state updates safely

## Flow 7: Structure Management

Validate:

- congregation can be created or listed
- service can be created or listed
- group can be created or listed
- member can be added to group where implemented

## Flow 8: Attendance

Validate:

- attendance session can be created
- attendance session list loads
- attendance records can be marked manually
- attendance summary foundation works where implemented

## Flow 9: Audit Visibility

Validate where available:

- important member changes produce audit records
- visitor conversion produces audit record
- attendance submission produces audit record

## Flow 10: Flutter UX States

Validate:

- loading states appear
- empty states appear
- error states appear
- unavailable states appear
- submit progress appears
- success feedback appears

## Flow 11: Access Control

Validate:

- unauthenticated user cannot access protected routes
- user cannot access another organization's records
- missing permission denies protected action where implemented

## Result Recording

Record each test result as:

```text
Passed
Failed
Blocked
Not implemented yet
```

Each failed or blocked item should include:

- issue summary
- affected area
- severity
- next action

## Final Rule

The MVP should not enter pilot until core end-to-end flows are either passing or clearly documented as known limitations.
