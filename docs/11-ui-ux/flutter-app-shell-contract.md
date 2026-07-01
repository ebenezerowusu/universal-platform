# Flutter App Shell Contract

## Purpose

This document defines the MVP Flutter app shell contract.

The app shell provides the stable structure for mobile and desktop/admin experiences.

## Principle

Build the shell before feature-heavy screens.

The app shell should handle startup, authentication state, organization selection, navigation shell, and common layout areas.

## Shell States

The app shell should support these high-level states:

```text
starting
unauthenticated
authenticated_without_organization
authenticated_with_organization
session_expired
error
```

## Startup Flow

Recommended flow:

```text
App Start
  -> load local runtime/client config
    -> check saved session foundation
      -> load current user where available
        -> load organizations where available
          -> require organization selection if multiple/none selected
            -> enter mobile or desktop shell
```

## Mobile Shell

The mobile shell should prioritize:

- simple home
- quick actions
- module-aware navigation
- member/user-friendly workflows
- responsive screens

## Desktop/Admin Shell

The desktop/admin shell should prioritize:

- dashboard layout
- sidebar or rail navigation
- admin workflows
- tables and filters
- management screens

## Shared Shell Responsibilities

The shell should own:

- active session state
- selected organization state
- high-level route grouping
- module navigation summary
- loading and empty state display
- session expired handling

## Shell Must Not Own

The shell should not own:

- backend permission decisions
- business domain rules
- provider logic
- database logic
- complex feature-specific workflows

## Navigation Source Direction

Navigation should eventually be driven by:

- authenticated user summary
- selected organization
- module availability
- feature availability
- permissions returned or summarized by backend
- platform type mobile or desktop

## MVP Placeholder Direction

It is acceptable for Sprint 6 to use placeholders for feature areas while the shell is being established.

## Final Rule

The Flutter shell should make future modules easy to add without rewriting app startup, session state, organization selection, or navigation structure.
