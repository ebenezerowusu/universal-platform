# Import Export Data Migration UI Contract

## Purpose

This document defines the MVP UI contract for Import, Export, and Data Migration foundation.

The screens should help organization users select templates, submit import jobs, preview validation results, request exports, and review job history.

## Navigation Direction

Import/export screens should appear when:

- user is authenticated
- organization is selected
- import/export feature is available
- user has required permission
- feature availability allows data movement

## Import Export Dashboard

Should show:

- recent import jobs
- recent export requests
- validation failure count
- manual review count
- quick actions where permitted

## Import Template List

Should show:

- template title
- module/domain
- expected file type placeholder
- required permission indicator where useful
- status

## Import Upload Placeholder

Should support:

- template selection
- source document/file placeholder
- notes optional
- submit job action

The UI should use platform APIs and must not access storage providers directly.

## Validation Preview

Should show:

- total rows
- valid rows
- warning rows
- error rows
- sample validation messages
- continue/execute placeholder where permitted

## Import Job Detail

Should show:

- job status
- template
- submitted by safe label
- submitted date
- validation summary
- execution status placeholder
- rollback note placeholder

## Export Request List

Should show:

- export key/title
- requested format
- requested by safe label
- status
- requested date
- action to view detail

## Export Detail

Should show:

- export key/title
- filters summary
- status
- output document placeholder where available
- data-quality warnings where available

## Data-Quality Warnings

Should show:

- severity
- row number where applicable
- field where applicable
- message
- suggested action placeholder

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- validation progress
- export request progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 35:

- direct database import UI
- destructive migration UI
- full ETL builder UI
- external warehouse integration UI
- AI data cleaning UI

## Final Rule

Import/export UI should make bulk data movement controlled, previewed, and auditable before execution.
