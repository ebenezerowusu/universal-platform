# Policy Library Attestations Training UI Contract

## Purpose

This document defines the MVP UI contract for Policy Library, Attestations, and Training Acknowledgement foundation.

The screens should help permitted users review policies, versions, publication reviews, attestation requests, acknowledgement status, training assignments, audience targeting, evidence links, and policy audit history.

## Navigation Direction

Policy library screens should appear when:

- user is authenticated
- organization is selected
- policy library feature is available
- user has required permission
- feature availability allows policy visibility

## Policy Library Dashboard

Should show:

- active policy count
- pending attestation count
- overdue acknowledgement count
- active training assignment count
- policies due for review
- recent policy audit events

## Policy List

Should show:

- policy key/title
- category
- owner module/domain
- current version placeholder
- document reference safe label
- status
- action to view detail

## Policy Version Detail

Should show:

- version label
- document reference safe label
- effective date placeholder
- review date placeholder
- replaced version placeholder
- publication status

## Publication Review Screen

Should show:

- policy version safe label
- reviewer safe label
- review status
- approval/publish placeholder actions where permitted
- safe review notes

## Attestation Request List

Should show:

- policy version safe label
- target audience safe summary
- due date placeholder
- acknowledgement count
- overdue count
- status

## Acknowledgement Status List

Should show:

- recipient safe label
- policy version safe label
- acknowledgement status
- acknowledged date placeholder
- source channel placeholder
- overdue indicator

## Training Assignment List

Should show:

- training safe label
- related policy safe label
- assigned audience safe summary
- due date placeholder
- completion count placeholder
- status

## Audience Targeting Screen

Should show:

- target type
- target safe label
- estimated recipient count placeholder
- role/workforce/branch indicator
- status

## Evidence Links

Should show:

- evidence type
- linked risk/control safe label where available
- linked policy safe label
- linked by safe label
- linked date
- permission/masking indicators

## Policy Audit History

Should show:

- event type
- actor safe label
- target safe label
- timestamp
- safe metadata summary

## User Acknowledgement Experience Placeholder

Should support a simple user-facing acknowledgement flow:

- view policy safe summary
- open document reference where permitted
- acknowledge placeholder action
- show acknowledgement status

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 57:

- full learning management system UI
- legal policy authoring UI
- legal certification UI
- cross-tenant policy browser UI
- document permission bypass UI
- arbitrary training rules UI

## Final Rule

Policy UI should make policy publication, assignment, acknowledgement, and evidence visible without replacing document permissions or formal training systems.
