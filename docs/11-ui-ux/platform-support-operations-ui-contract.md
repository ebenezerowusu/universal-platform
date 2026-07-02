# Platform Support Operations UI Contract

## Purpose

This document defines the MVP UI contract for Platform Support and Organization Operations foundation.

The screens should help permitted users open support cases, review case timelines, manage onboarding status, review organization health, and track operational checklist placeholders.

## Navigation Direction

Support screens should appear when:

- user is authenticated
- organization is selected
- support feature is available
- user has required permission
- feature availability allows support operations

## Support Dashboard

Should show:

- open support cases
- escalated cases
- onboarding status
- organization health status placeholder
- recent case updates
- checklist progress placeholder

## Support Case List

Should show:

- case title
- category
- priority
- status
- opened by safe label
- last updated date
- action to view detail

## Support Case Detail

Should show:

- title
- safe summary
- category
- priority
- status
- source reference safe label where allowed
- timeline updates
- escalation status placeholder

## Case Timeline

Should show:

- update type
- safe message
- actor safe label
- visibility
- timestamp

Internal notes should be hidden unless the user has internal-note permission.

## Onboarding Tracker

Should show:

- onboarding phase
- status
- owner placeholder
- blocker note where available
- started date
- completed date where available

## Organization Health Review

Should show:

- health status
- review summary
- unresolved support count
- readiness warning placeholder
- action items placeholder
- last reviewed date

## Operational Checklist

Should show:

- checklist item
- status
- owner placeholder
- notes placeholder
- update action where permitted

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 39:

- direct tenant data browsing UI
- hidden support access UI
- remote account control UI
- external helpdesk setup UI
- live chat provider UI
- AI support agent UI

## Final Rule

Support UI should help users resolve operational issues without exposing private tenant data or bypassing source-module access.
