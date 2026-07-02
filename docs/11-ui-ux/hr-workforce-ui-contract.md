# HR Workforce UI Contract

## Purpose

This document defines the MVP UI contract for HR and Workforce foundation.

The screens should help organization users manage departments, workforce profiles, assignments, status history, and basic workforce summaries.

## Navigation Direction

Workforce screens should appear when:

- user is authenticated
- organization is selected
- HR/workforce feature is available
- user has required permission
- feature availability allows the section

## Workforce Dashboard

Should show:

- active workers
- staff count
- volunteer count
- department count
- status summary
- quick action to add profile where permitted

## Department List

Should show:

- department name
- code or safe label
- parent department where available
- status
- edit action where permitted

## Workforce Profile List

Should show:

- display name
- worker type
- current department where available
- current position where available
- status
- view detail action

## Workforce Profile Detail

Should show:

- display name
- worker type
- status
- start date
- assignment summary
- status history link
- references placeholder where available

## Assignment History

Should show:

- department
- position label
- assignment type
- start date
- end date where available
- status

## Status History

Should show:

- status
- effective date
- changed by where available
- reason or safe note

## Reference Placeholder

Should show:

- reference label
- type
- created date
- safe notes

Do not expose sensitive references unless the user has the correct permission.

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 27:

- payroll screens
- tax screens
- benefits screens
- biometric attendance screens
- advanced performance screens
- full document processing screens

## Final Rule

Workforce UI should be simple, permission-aware, and safe for organization people records.
