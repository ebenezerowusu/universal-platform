# Flutter State and Navigation Implementation Flow

## Purpose

This document defines the MVP implementation flow for Flutter state and navigation.

It complements the Flutter state management and navigation direction document.

## Principle

State and navigation should be driven by platform context.

The app should not hardcode menus that ignore session, organization, module, feature, or permission summaries.

## Initial State Areas

Sprint 6 should prepare state areas for:

- startup state
- current user summary
- session summary
- selected organization
- organization list
- language
- module summary
- feature summary
- navigation summary

## Startup State Flow

```text
App launches
  -> startup state loading
    -> check session foundation
      -> unauthenticated or authenticated
        -> load current user
          -> load organizations
            -> require selected organization
              -> load module/feature/navigation summaries where available
                -> show mobile or desktop shell
```

## Organization Selection Flow

```text
Authenticated user
  -> organization list loaded
    -> no organization selected
      -> show organization selector
        -> user selects organization
          -> update selected organization state
            -> refresh module/feature/navigation summaries
```

## Navigation Flow

Navigation should be built from:

- platform type
- selected organization
- module availability
- feature availability
- permission/navigation summary where available

## Mobile Navigation Direction

Mobile navigation should prioritize:

- home
- profile/session area
- quick workflows
- module shortcuts
- simple lists and detail pages

## Desktop/Admin Navigation Direction

Desktop/admin navigation should prioritize:

- dashboard
- side navigation
- management workflows
- tables
- filters
- reports

## Loading and Empty States

Every major route area should support:

- loading
- empty
- unavailable
- error
- retry where useful

## Unavailable Feature State

When a module or feature is unavailable, show a safe unavailable state instead of broken navigation.

Backend checks still remain the source of truth.

## Testing Direction

Tests should cover:

- app startup state
- unauthenticated routing
- authenticated without selected organization
- organization selection update
- mobile shell rendering
- desktop shell rendering
- unavailable module placeholder where practical

## Final Rule

Flutter navigation should be dynamic and platform-aware. Do not build fixed menus that will break when modules, plans, or permissions change.
