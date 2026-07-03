# Customer Success Account Health Adoption Playbooks UI Contract

## Purpose

This document defines the MVP UI contract for Customer Success, Account Health, and Adoption Playbooks foundation.

The screens should help permitted users review account health, adoption signals, risk signals, playbooks, tasks, interventions, renewal/support/product links, success notes, and customer success audit history.

## Navigation Direction

Customer success screens should appear when:

- user is authenticated
- organization is selected where required
- customer success feature is available
- user has required permission
- account health visibility rules allow access

## Customer Success Dashboard

Should show:

- healthy account count
- at-risk account count
- open playbook task count
- renewal warning count
- support escalation warning count
- recent customer success audit events

## Account Health List

Should show:

- organization safe label
- health status placeholder
- adoption status placeholder
- support status placeholder
- billing/renewal status placeholder
- owner safe label
- last reviewed date placeholder

## Account Health Detail

Should show:

- organization safe summary
- overall health status
- adoption signal summary
- risk signal summary
- support case summary
- billing/renewal safe summary
- active playbooks
- intervention/action summary

## Adoption Signal Summary

Should show:

- onboarding completion placeholder
- active module usage placeholder
- key workflow completion placeholder
- feature adoption trend placeholder
- knowledge base engagement placeholder
- product communication acknowledgement placeholder

## Risk Signal Summary

Should show:

- low adoption indicator
- high support volume indicator
- payment/renewal warning placeholder
- negative feedback trend placeholder
- inactive user warning placeholder
- failed onboarding milestone warning placeholder

Risk signals are indicators, not automatic churn decisions.

## Success Playbook List

Should show:

- playbook title
- playbook type
- owner safe label
- active task count placeholder
- status
- action to view detail

## Playbook Task Board

Should show:

- task title
- related account safe label
- task owner safe label
- due date placeholder
- status
- related risk/adoption signal placeholder
- complete placeholder action where permitted

## Intervention and Action List

Should show:

- intervention type
- account safe label
- action owner safe label
- status
- due date placeholder
- related support/billing/product link where available

## Renewal Support Linkage View

Should show:

- subscription plan safe label
- renewal readiness placeholder
- linked support cases
- linked feedback items
- linked knowledge base articles
- linked product analytics summary
- linked announcements/release notes

## Success Notes

Should show:

- note safe summary
- author safe label
- related account safe label
- timestamp
- permission/masking indicators

## Customer Success Audit History

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
- no health data yet

## MVP Exclusions

Do not implement in Sprint 63:

- automatic churn prediction UI
- intrusive account surveillance UI
- cross-tenant private account browser UI
- billing mutation UI
- replacement support desk UI
- AI-only decision UI

## Final Rule

Customer success UI should make account health and adoption support actionable while making it clear that health signals require human review.
