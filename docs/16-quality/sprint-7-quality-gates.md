# Sprint 7 Quality Gates

## Purpose

This document defines quality gates for Sprint 7 implementation.

Sprint 7 should connect the Flutter client foundation to the Religious MVP backend foundation.

## Gate 1: Scope Control

Sprint 7 may include:

- Religious navigation entry points
- Religious dashboard placeholder or summary screen
- member list screen
- member detail placeholder or foundation
- create member form foundation
- visitor list screen
- create visitor form foundation
- visitor conversion action foundation where backend is ready
- congregation/service/group list screens or placeholders
- attendance session list screen
- attendance marking foundation where backend is ready
- API client methods for Religious MVP contracts
- loading, empty, error, and unavailable states
- tests for key screens and API parsing where practical

Sprint 7 must not include:

- full offline sync
- QR attendance
- GPS attendance
- live SMS sending UI
- live payment/giving UI
- full family/household UI
- full care/counseling/welfare UI
- advanced reports
- POS or Commerce UI

## Gate 2: Shell Integration Check

Confirm:

- Religious screens are reached through the shared app shell
- navigation uses selected organization state
- unavailable module state is handled safely
- screens do not bypass the existing app shell

## Gate 3: API Client Check

Confirm:

- Religious API methods use the shared API client
- standard success parsing is used
- standard error parsing is used
- raw HTTP calls are not placed inside screens
- backend validation errors are displayed safely

## Gate 4: State Check

Confirm:

- loading states exist
- empty states exist
- error states exist
- unavailable states exist
- forms handle submit progress where practical

## Gate 5: Design System Check

Confirm:

- screens use reusable design system components
- one-off styling is minimized
- mobile layouts are touch-friendly
- desktop/admin layouts remain compatible where applicable

## Gate 6: Backend Contract Alignment

Confirm screens align with:

- `docs/12-api/religious-settings-api-contract.md`
- `docs/12-api/religious-member-visitor-api-contract.md`
- `docs/12-api/religious-structure-attendance-api-contract.md`
- `docs/12-api/frontend-backend-integration-checklist.md`

## Gate 7: MVP Boundary Check

Confirm:

- no advanced Religious feature was added
- no live SMS or payment UI was added
- no QR/GPS attendance was added
- no POS or Commerce UI was added

## Gate 8: Test Check

Minimum tests should cover where practical:

- Religious navigation visibility
- member list state rendering
- create member form foundation
- visitor list state rendering
- create visitor form foundation
- unavailable module state
- API success parsing
- API error parsing

## Gate 9: Documentation Check

If implementation differs from documented screen, API, or integration flow, update the relevant docs in the same PR.

## Final Rule

Sprint 7 is complete only when Religious MVP workflows are usable through the Flutter foundation without bypassing platform client architecture.
