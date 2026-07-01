# Sprint 6 Quality Gates

## Purpose

This document defines quality gates for Sprint 6 implementation.

Sprint 6 should establish the Flutter client foundation without rushing into complete feature screens.

## Gate 1: Scope Control

Sprint 6 may include:

- Flutter project foundation
- shared app shell
- startup flow
- API client foundation
- standard response/error parsing foundation
- shared app state foundation
- selected organization state foundation
- mobile shell placeholder
- desktop/admin shell placeholder
- shared design system foundation
- login screen placeholder or foundation
- organization selector placeholder or foundation
- tests for startup, shell, and shared components where practical

Sprint 6 must not include:

- full Religious feature screens
- payment screens
- SMS screens
- POS or Commerce UI
- full offline sync
- app store packaging
- production push notifications
- advanced animations before core flows work

## Gate 2: App Shell Check

Confirm:

- startup state is represented
- unauthenticated state is represented
- authenticated state is represented
- organization selection state is represented
- mobile shell exists
- desktop/admin shell foundation exists where practical

## Gate 3: API Client Check

Confirm:

- standard success response parsing exists
- standard error response parsing exists
- safe UI error mapping exists
- session handling is centralized
- feature screens do not perform raw HTTP calls directly

## Gate 4: State and Navigation Check

Confirm:

- current user summary state exists or is planned in the foundation
- selected organization state exists
- module/feature/navigation summary state is prepared
- navigation is platform-aware
- fixed hardcoded menus are avoided where module-aware navigation is required

## Gate 5: Design System Check

Confirm:

- reusable components exist for common UI patterns
- components avoid business logic
- loading, empty, and error states exist
- mobile and desktop/admin layouts can share design language

## Gate 6: Backend Contract Alignment

Confirm Flutter client aligns with:

- `docs/12-api/standard-response-contract.md`
- `docs/12-api/identity-api-contract.md`
- `docs/12-api/organization-api-contract.md`

Feature-specific API usage may remain placeholder if backend APIs are not fully ready.

## Gate 7: Test Check

Minimum tests should cover where practical:

- app starts
- login or unauthenticated screen renders
- organization selector renders
- selected organization state updates
- mobile shell renders
- desktop/admin shell renders
- API success response parses
- API error response parses
- shared component renders

## Gate 8: Documentation Check

If implementation differs from documented Flutter shell, API client, or state/navigation direction, update the relevant docs in the same PR.

## Final Rule

Sprint 6 is complete only when the Flutter foundation is stable enough for feature screens to be added without rewriting startup, API, state, navigation, or design system foundations.
