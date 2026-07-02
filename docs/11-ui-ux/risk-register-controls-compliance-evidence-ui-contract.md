# Risk Register Controls Compliance Evidence UI Contract

## Purpose

This document defines the MVP UI contract for Risk Register, Internal Controls, and Compliance Evidence foundation.

The screens should help permitted users review risks, assessments, control objectives, control checks, evidence links, exceptions, remediation actions, checklist mappings, review cycles, and risk/control audit history.

## Navigation Direction

Risk and controls screens should appear when:

- user is authenticated
- organization is selected
- risk and controls feature is available
- user has required permission
- feature availability allows risk/control visibility

## Risk and Controls Dashboard

Should show:

- open risk count
- high risk count
- active control count
- open exception count
- overdue remediation count
- recent risk/control audit events

## Risk Register List

Should show:

- risk key/title
- risk category
- owner safe label
- severity placeholder
- likelihood placeholder
- status
- action to view detail

## Risk Assessment Detail

Should show:

- risk safe label
- inherent risk placeholder
- residual risk placeholder
- impact summary
- likelihood summary
- review date placeholder
- assessment history placeholder

## Control Objective List

Should show:

- control key/title
- control category
- mapped domain/module
- owner safe label
- review frequency placeholder
- status

## Control Check List

Should show:

- control safe label
- check type
- due date placeholder
- evidence required indicator
- status
- complete/fail placeholder actions where permitted

## Evidence Link View

Should show:

- evidence type
- linked source safe label
- related risk/control safe label
- linked by safe label
- linked date
- permission/masking indicators

## Exception Review List

Should show:

- control safe label
- exception summary
- severity placeholder
- review status
- remediation link placeholder
- mark reviewed action where permitted

## Remediation Action List

Should show:

- action summary
- owner safe label
- due date placeholder
- linked risk/control/exception safe label
- status
- completion evidence placeholder

## Compliance Checklist Mapping

Should show:

- checklist key/title
- mapped controls count
- evidence coverage placeholder
- open exceptions count
- status
- operational-readiness disclaimer

## Risk Control Audit History

Should show:

- event type
- actor safe label
- target safe label
- timestamp
- safe metadata summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 56:

- legal certification UI
- regulatory advice UI
- unrestricted evidence browser UI
- cross-tenant risk view UI
- replacement monitoring dashboard UI
- arbitrary control scripting UI

## Final Rule

Risk and controls UI should make operational readiness visible and reviewable without presenting checklist status as legal certification.
