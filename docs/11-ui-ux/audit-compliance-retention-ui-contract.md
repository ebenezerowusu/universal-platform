# Audit Compliance Retention UI Contract

## Purpose

This document defines the MVP UI contract for Audit, Compliance, and Data Retention foundation.

The screens should help permitted organization users review audit events, filter sensitive actions, review access placeholders, manage retention policy metadata, and track safe archive/delete request placeholders.

## Navigation Direction

Governance screens should appear when:

- user is authenticated
- organization is selected
- governance feature is available
- user has required permission
- feature availability allows governance

## Audit Dashboard

Should show:

- recent audit events
- sensitive action count where permitted
- retention requests count
- access review placeholder count
- compliance checklist progress placeholder

## Audit Event List

Should show:

- date/time
- actor safe label
- action
- target type
- category
- severity placeholder
- action to view detail

## Audit Event Detail

Should show:

- actor safe label
- action
- target type
- target safe label where available
- category
- timestamp
- safe metadata

Sensitive metadata should require sensitive audit permission.

## Access Review Placeholder

Should show:

- review title
- scope
- status
- users/roles summary placeholder
- complete placeholder action where permitted

## Retention Policy List

Should show:

- policy title/key
- module/domain
- target record type
- retention period placeholder
- archive behavior placeholder
- delete behavior placeholder
- status

## Archive/Delete Request Placeholder

Should show:

- request type
- target type
- target safe label
- reason
- status
- requested by safe label
- resolve placeholder action where permitted

## Compliance Checklist Placeholder

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

Do not implement in Sprint 36:

- legal advice UI
- destructive delete UI
- full GRC suite UI
- government filing UI
- automated regulatory classification UI

## Final Rule

Governance UI should make audit and retention actions explainable without enabling unsafe deletion or pretending to provide legal compliance advice.
