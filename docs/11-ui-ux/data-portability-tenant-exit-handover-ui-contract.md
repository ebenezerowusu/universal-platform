# Data Portability Tenant Exit Handover UI Contract

## Purpose

This document defines the MVP UI contract for Data Portability, Tenant Exit, and Organization Data Handover foundation.

The screens should help permitted users review export package requests, export manifests, handover checklists, exit readiness reviews, ownership confirmations, privacy/lineage linkage, archive/retention linkage, operational links, and portability audit history.

## Navigation Direction

Data portability screens should appear when:

- user is authenticated
- organization is selected
- data portability feature is available
- user has required permission
- feature availability allows portability visibility

## Portability Dashboard

Should show:

- open export package count
- pending handover task count
- exit readiness status
- pending privacy review count
- retention warning count
- recent portability audit events

## Export Package Request List

Should show:

- package purpose
- scope summary safe label
- requested by safe label
- requested date
- status
- action to view detail

## Export Package Detail

Should show:

- package request metadata
- requested modules
- privacy/export review status
- lineage manifest status
- archive/retention linkage status
- package status
- approval/rejection placeholder actions where permitted

## Export Scope Manifest

Should show:

- included modules/domains
- included entity types
- excluded sensitive categories placeholder
- file/reference summary placeholder
- generated through export job placeholder
- warnings where available

## Handover Checklist

Should show checklist items for:

- owner confirmation
- billing status review
- privacy review
- export package review
- support/offboarding review
- archive/retention review

## Exit Readiness Review

Should show:

- readiness status
- billing warning placeholder
- subscription warning placeholder
- support case warning placeholder
- retention hold warning placeholder
- pending export warning placeholder
- mark reviewed action where permitted

## Ownership Confirmation Placeholder

Should show:

- owner/admin safe label
- package safe label
- confirmation status
- confirmed date placeholder
- safe notes

## Privacy and Lineage Linkage

Should show:

- privacy review safe label
- lineage manifest safe label
- data quality warning summary
- consent-sensitive category count placeholder
- link status

## Operational Links

Should show:

- billing status safe summary
- support case safe summary
- offboarding status placeholder
- archive/retention status placeholder
- blocking warnings

## Portability Audit History

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

Do not implement in Sprint 53:

- destructive tenant deletion UI
- forced account closure UI
- full database dump download UI
- legal advice UI
- cross-tenant package browser UI
- privacy review bypass UI

## Final Rule

Data portability UI should make handover transparent and controlled without exposing raw tenant data or destructive exit actions.
