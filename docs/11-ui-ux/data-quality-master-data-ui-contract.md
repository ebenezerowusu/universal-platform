# Data Quality Master Data UI Contract

## Purpose

This document defines the MVP UI contract for Data Quality, Validation, Duplicate Detection, and Master Data Governance foundation.

The screens should help permitted users review quality rules, validation results, duplicate candidates, merge review placeholders, master data registry placeholders, canonical markers, correction requests, and data quality audit history.

## Navigation Direction

Data quality screens should appear when:

- user is authenticated
- organization is selected
- data quality feature is available
- user has required permission
- feature availability allows data quality visibility

## Data Quality Dashboard

Should show:

- active quality rule count
- open validation issue count
- duplicate candidate count
- pending merge review count
- open correction request count
- recent data quality events

## Quality Rule List

Should show:

- rule key
- module/domain owner
- entity type
- rule type
- severity placeholder
- status
- action to view detail

## Validation Result List

Should show:

- rule key
- entity safe label
- status
- message safe summary
- detected date
- reviewed status

## Duplicate Candidate List

Should show:

- entity type
- primary record safe label
- candidate record safe label
- confidence placeholder
- detected by rule
- status
- action to review

## Merge Review Placeholder

Should show:

- duplicate candidate safe label
- field comparison safe summary
- privacy-sensitive field warnings
- merge plan placeholder
- approval/rejection actions where permitted

Merge approval should not imply automatic destructive deletion in MVP.

## Master Data Registry Placeholder

Should show:

- registry type
- source domain/module
- linked entity safe labels
- canonical marker status
- warning count where available

## Canonical Marker List

Should show:

- entity type
- canonical record safe label
- linked records count
- marked by safe label
- marked date
- status

## Correction Request List

Should show:

- request type
- entity safe label
- issue safe summary
- requester safe label
- status
- created date
- completion placeholder action where permitted

## Data Quality Audit History

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

Do not implement in Sprint 51:

- automatic destructive merge UI
- automatic duplicate deletion UI
- AI-only duplicate decision UI
- cross-tenant duplicate browser UI
- direct database editing UI
- unrestricted bulk correction UI

## Final Rule

Data quality UI should make validation, duplicate review, and correction workflows safe and visible without silently changing tenant data.
