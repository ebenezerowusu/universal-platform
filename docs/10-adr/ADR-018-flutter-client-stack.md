# ADR-018: Flutter Client Stack — Riverpod, go_router, http, Secure Storage

## Status

Accepted

## Date

2026-08-21

## Context

The Flutter documentation (docs/11-ui-ux/) deliberately mandates no
packages: it defines required capabilities (eight shared state areas, six
shell states, six client error states, backend-driven navigation) and left
the package choice to implementation. Sprint 6 required the decision.

## Decision

```text
1. State management: flutter_riverpod (Notifier providers). A single
   SessionController owns startup, login/logout, organization selection,
   and the module/feature summaries that drive navigation.

2. Navigation: go_router. Routing is derived state: a redirect maps the
   six shell states (starting, unauthenticated,
   authenticated_without_organization, authenticated_with_organization,
   session_expired, error) to their screens. Screens never navigate
   around authentication or organization selection.

3. HTTP: package:http wrapped by one shared ApiClient that owns base URL
   (--dart-define=API_BASE_URL), bearer-token attachment, standard
   envelope parsing, trace-id capture, and mapping of platform error
   codes to the six client error states from the API client contract.
   Feature code never performs raw HTTP.

4. Token storage: flutter_secure_storage behind a SessionStore port
   (in-memory implementation for tests). Screens never touch tokens.

5. Strings: centralized in an AppStrings class as the staging ground for
   the ARB-based localization wave (en/tw/fr/sw).

6. Structure follows repository-structure-target.md:
   lib/{app,core,shared,features,modules,l10n} — features hold
   cross-module UX, modules hold domain UI (religious first).
```

## Options Considered

### Bloc instead of Riverpod

Pros: explicit event/state pairs. Cons: heavier ceremony for the shell
state machine; Riverpod's provider overrides make widget tests trivial.

### dio instead of http

Pros: interceptors, cancellation. Cons: the platform contract needs only
envelope parsing and one auth header; a thin wrapper over http keeps the
dependency surface minimal. dio can replace http inside ApiClient later
without touching feature code.

## Consequences

Positive: minimal dependency surface; the shell contract, API client
contract, and design guardrails from the docs map one-to-one onto code;
tests swap SessionStore/SessionController via provider overrides.
Tradeoffs: no interceptor pipeline yet (retry/refresh handled manually
when the refresh flow lands); manual JSON mapping until codegen is
justified.

## Architecture Boundaries

- No business logic in the UI: yes
- Backend authoritative for permissions/modules: yes (hiding is UX only)
- Navigation backend-driven: yes (module summaries, no fixed menus)
- Configuration environment-driven: yes (API_BASE_URL dart-define)

## Implementation Impact

- universal-platform-app PR #1 (Sprint 6 foundation)
- docs/11-ui-ux package-choice gap resolved; folder-structure conflict
  resolved in favor of repository-structure-target.md

## Final Rule

The client renders platform state; it never owns platform rules.
