# Reporting Analytics UI Contract

## Purpose

This document defines the MVP UI contract for Reporting and Analytics foundation.

The screens should help organization users view dashboards, metric cards, reports, filters, export placeholders, and data-quality warnings.

## Navigation Direction

Reporting screens should appear when:

- user is authenticated
- organization is selected
- reporting feature is available
- user has required permission
- feature availability allows reporting

## Reporting Dashboard

Should show:

- key metric cards
- report shortcuts
- data-quality warning count
- export request status where available
- date range filter

## Metric Cards

Should show:

- metric title
- value
- module/domain label
- comparison placeholder where available
- data-quality indicator

## Report List

Should show:

- report title
- module/domain
- description
- required permission indicator where useful
- status
- action to run report

## Report Filters

Should support:

- date range
- status
- module/domain
- category
- department/team where available
- branch/congregation where available

## Report Result View

Should show:

- report title
- applied filters
- result table/card placeholder
- partial-data warning where applicable
- export request action where permitted

## Export Request Placeholder

Should show:

- export format placeholder
- report key
- requested date
- status
- download placeholder where available later

## Data-Quality Warnings

Should show:

- warning title
- module/domain
- severity
- affected report or metric
- suggested action placeholder

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- running report progress
- export request progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 33:

- advanced BI designer UI
- complex chart builder
- AI insight UI
- external analytics provider UI
- unrestricted sensitive export UI

## Final Rule

Reporting UI should make insights easy to read while clearly warning when data is partial, unavailable, or restricted by permissions.
