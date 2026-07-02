# Data Privacy Consent Governance UI Contract

## Purpose

This document defines the MVP UI contract for Data Privacy, Consent, and Sensitive Data Governance foundation.

The screens should help permitted users review classifications, sensitive field catalog placeholders, consent purposes, consent records, withdrawal placeholders, export reviews, subject requests, and privacy audit history.

## Navigation Direction

Privacy governance screens should appear when:

- user is authenticated
- organization is selected
- privacy governance feature is available
- user has required permission
- feature availability allows privacy governance visibility

## Privacy Governance Dashboard

Should show:

- classification count
- sensitive field count
- active consent record count
- pending export review count
- open subject request count
- recent privacy audit events

## Data Classification List

Should show:

- classification key
- title
- sensitivity level placeholder
- description safe summary
- status
- action to view detail

## Sensitive Field Catalog

Should show:

- domain/module
- entity type
- field key
- classification
- masking policy placeholder
- export review requirement placeholder
- status

## Consent Purpose List

Should show:

- purpose key
- title
- domain/module owner
- retention linkage placeholder
- status
- action to view detail

## Consent Record List

Should show:

- subject safe label
- purpose
- consent status
- source channel placeholder
- captured date
- withdrawal status where available

## Consent Withdrawal Review Placeholder

Should show:

- consent record safe label
- withdrawal date
- reason safe summary where available
- impact review placeholder
- status
- review action where permitted

## Export Review Placeholder

Should show:

- export source safe label
- requested by safe label
- sensitive field count placeholder
- redaction recommendation placeholder
- status
- approve/reject placeholder actions where permitted

## Subject Request Placeholder

Should show:

- request type
- subject safe label
- status
- requested date
- due date placeholder
- assigned reviewer safe label where available
- completion placeholder action where permitted

## Privacy Audit History

Should show:

- event type
- actor safe label
- target safe label
- purpose safe summary where available
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

Do not implement in Sprint 50:

- legal advice UI
- automatic regulatory decision UI
- destructive deletion automation UI
- unrestricted sensitive record browser UI
- AI classification UI
- public data broker integration UI

## Final Rule

Privacy UI should make sensitive data handling visible and reviewable without exposing sensitive records unnecessarily or making legal decisions for users.
