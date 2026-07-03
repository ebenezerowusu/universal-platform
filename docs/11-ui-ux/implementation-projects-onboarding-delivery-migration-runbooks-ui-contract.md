# Implementation Projects Onboarding Delivery Migration Runbooks UI Contract

## Purpose

This document defines the MVP UI contract for Implementation Projects, Onboarding Delivery, and Migration Runbooks foundation.

The screens should help permitted users review implementation projects, onboarding milestones, migration runbooks, setup tasks, go-live readiness, implementation issues/risks, delivery links, implementation notes, and implementation audit history.

## Navigation Direction

Implementation delivery screens should appear when:

- user is authenticated
- organization is selected where required
- implementation delivery feature is available
- user has required permission
- implementation visibility rules allow access

## Implementation Dashboard

Should show:

- active project count
- at-risk project count
- open setup task count
- open migration runbook count
- go-live ready project count
- recent implementation audit events

## Implementation Project List

Should show:

- organization safe label
- project type
- implementation phase
- owner safe label
- target go-live placeholder
- status
- action to view detail

## Implementation Project Detail

Should show:

- organization safe summary
- implementation phase
- onboarding milestone summary
- migration runbook summary
- setup task summary
- go-live readiness summary
- issue/risk summary
- delivery links

## Onboarding Milestone Checklist

Should show:

- milestone title
- checklist item count placeholder
- owner safe label
- due date placeholder
- status
- blocked indicator
- complete placeholder action where permitted

## Migration Runbook Detail

Should show:

- source system safe summary
- migration scope summary
- migration owner safe label
- validation checklist summary
- linked import/export tasks
- data quality review status
- rollback/contingency placeholder
- status

## Setup Task Board

Should show setup tasks for:

- roles/permissions
- module settings
- payment/SMS/provider settings
- localization/timezone/currency
- notification preferences
- branch/network setup

Each task should show owner, due date placeholder, status, blocker indicator, and completion action where permitted.

## Go-Live Readiness Review

Should show:

- stakeholder sign-off placeholder
- data quality acceptance placeholder
- training/help readiness placeholder
- support readiness placeholder
- rollback readiness placeholder
- final readiness status
- mark ready/not ready placeholder actions where permitted

## Implementation Issue and Risk List

Should show:

- issue/risk safe summary
- type
- severity placeholder
- owner safe label
- due date placeholder
- status
- resolution placeholder action where permitted

## Delivery Linkage View

Should show links to:

- customer success playbooks
- support cases
- knowledge base articles
- product analytics onboarding funnels
- import/export migration jobs
- data quality reviews
- policy/training acknowledgements

## Implementation Notes

Should show:

- note safe summary
- author safe label
- related project safe label
- timestamp
- permission/masking indicators

## Implementation Audit History

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
- restricted by privacy
- no implementation data yet

## MVP Exclusions

Do not implement in Sprint 64:

- full project management suite UI
- automatic go-live approval UI
- destructive migration execution UI
- data quality bypass UI
- cross-tenant private implementation browser UI
- billing mutation UI
- AI-only implementation decision UI

## Final Rule

Implementation delivery UI should make onboarding progress, migration readiness, setup status, blockers, and go-live review clear without bypassing source engine controls.
