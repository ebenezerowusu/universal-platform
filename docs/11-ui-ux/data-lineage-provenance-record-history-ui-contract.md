# Data Lineage Provenance Record History UI Contract

## Purpose

This document defines the MVP UI contract for Data Lineage, Provenance, and Record History foundation.

The screens should help permitted users review source systems, provenance records, record history placeholders, import/job/report/export lineage, transformation steps, derived relationships, lineage graph summaries, correction/merge lineage, and lineage audit history.

## Navigation Direction

Data lineage screens should appear when:

- user is authenticated
- organization is selected
- data lineage feature is available
- user has required permission
- feature availability allows lineage visibility

## Lineage Dashboard

Should show:

- source system count
- provenance record count
- lineage link count
- records with unknown source
- recent lineage events
- disputed lineage count where available

## Source System List

Should show:

- source key
- source type
- owner module/domain
- trust level placeholder
- status
- action to view detail

## Provenance Detail

Should show:

- entity safe label
- source system
- source record reference safe summary
- created through label
- process reference placeholder
- created date
- provenance quality status

## Record History List

Should show:

- entity safe label
- version placeholder
- changed fields safe summary
- changed by safe label
- change source
- reason placeholder
- timestamp

## Import Job Report Export Lineage View

Should show:

- source object safe label
- derived objects safe labels
- import/job/report/export references
- transformation summary
- warnings where available

## Transformation Step List

Should show:

- step type
- source field safe label
- target field safe label
- transformation safe summary
- validation/correction link where available
- timestamp

## Lineage Graph Summary

Should show:

- source nodes safe summary
- derived nodes safe summary
- relationship types
- warning count
- privacy masking indicators

This is a summary view, not a full graph database UI.

## Correction and Merge Lineage View

Should show:

- correction request safe label
- duplicate candidate safe label where available
- merge review safe label where available
- canonical marker safe label where available
- lineage status
- review action where permitted

## Lineage Audit History

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

Do not implement in Sprint 52:

- full graph database visualizer UI
- full historical record reconstruction UI
- destructive history rewrite UI
- cross-tenant lineage browser UI
- privacy bypass UI
- AI-inferred lineage as verified source UI

## Final Rule

Lineage UI should make data origin and derivation understandable without exposing sensitive fields or overstating historical completeness.
