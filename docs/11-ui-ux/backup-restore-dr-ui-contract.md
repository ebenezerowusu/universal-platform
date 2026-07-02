# Backup Restore Disaster Recovery UI Contract

## Purpose

This document defines the MVP UI contract for Backup, Restore, and Disaster Recovery foundation.

The screens should help permitted users review backup policies, backup runs, verification status, restore requests, restore drills, recovery objectives, and readiness checklist placeholders.

## Navigation Direction

Recovery screens should appear when:

- user is authenticated
- organization is selected
- backup/restore feature is available
- user has required permission
- feature availability allows recovery visibility

## Backup Dashboard

Should show:

- latest backup status
- last verified backup placeholder
- failed backup count
- restore request count
- restore drill status
- readiness checklist progress placeholder

## Backup Policy List

Should show:

- policy key/title
- scope
- frequency placeholder
- retention period placeholder
- storage target placeholder
- status

## Backup Run List

Should show:

- policy
- started at
- finished at where available
- status
- verification status
- action to view detail

## Backup Run Detail

Should show:

- backup run metadata
- status
- verification placeholder
- notes
- related restore drills where available

## Restore Request List

Should show:

- request type
- scope
- requested by safe label
- status
- target recovery point placeholder
- action to view detail

## Restore Request Detail

Should show:

- request type
- requested scope
- reason
- approval placeholder status
- linked backup run where available
- audit-friendly timestamps

## Restore Drill List

Should show:

- drill date
- backup run used
- environment placeholder
- result
- issues found
- follow-up status

## Recovery Readiness Checklist

Should show:

- checklist item
- status
- owner placeholder
- last checked date
- notes placeholder

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 37:

- live production restore execution UI
- destructive restore UI
- external backup provider setup UI
- full business continuity suite UI
- database script runner UI

## Final Rule

Recovery UI should make backup safety visible without enabling unsafe restore operations in MVP.
