# Staff Attendance and Leave UI Contract

## Purpose

This document defines the MVP UI contract for staff attendance and leave foundation.

The screens should help organization users create attendance sessions, record check-in/check-out, review adjustments, submit leave requests, and process approval placeholders.

## Navigation Direction

Attendance and leave screens should appear when:

- user is authenticated
- organization is selected
- HR/workforce feature is available
- user has required permission
- feature availability allows the section

## Attendance Dashboard

Should show:

- open sessions
- present count
- missing checkout count
- on leave count
- manual adjustment count

## Session List

Should show:

- session name
- date
- start/end time where available
- department where available
- status
- action to view detail

## Check-In / Check-Out Screen

Should support:

- workforce profile selection where permitted
- session selection
- check-in action
- check-out action
- current record status

## Attendance Records Screen

Should show:

- profile safe label
- session
- check-in time
- check-out time
- status
- adjustment indicator

## Manual Adjustment Review

Should support:

- adjustment type
- corrected value
- reason
- reviewer action where permitted

## Leave Request List

Should show:

- profile safe label
- leave type
- date range
- status
- action to view detail

## Leave Request Detail

Should show:

- requester safe label
- leave type
- start date
- end date
- reason where allowed
- status
- approve/reject actions where permitted

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 28:

- payroll screens
- overtime payment screens
- biometric screens
- facial recognition screens
- complex shift planning screens
- full leave entitlement screens

## Final Rule

Attendance and leave UI should make staff presence clear while staying permission-aware and avoiding payroll or biometric complexity.
