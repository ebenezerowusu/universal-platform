# Flutter Backend Integration Flow

## Purpose

This document defines the MVP flow for integrating Flutter screens with backend APIs.

It helps keep UI implementation consistent as modules are added.

## Principle

Screens should depend on client services, not raw backend details.

The API client, state layer, and feature-specific client methods should separate UI from HTTP mechanics.

## Flow

```text
Screen
  -> View Model / Controller / State Notifier
    -> Feature Client Method
      -> Shared API Client
        -> Backend API
          -> Standard Response Parser
            -> UI State
```

## Screen Responsibility

Screens should own:

- layout
- user input
- simple UI validation
- loading display
- error display
- empty display
- action triggers

Screens should not own:

- raw HTTP calls
- response envelope parsing
- token management
- organization context wiring beyond using shared state
- backend authorization assumptions

## Feature Client Responsibility

Feature-specific client methods should own:

- endpoint path construction
- request DTO mapping
- response DTO mapping
- feature-specific API method names

## Shared API Client Responsibility

The shared API client should own:

- base URL
- headers
- session token attachment
- standard response parsing
- standard error parsing
- trace metadata capture

## State Layer Responsibility

The state layer should own:

- loading state
- loaded state
- empty state
- error state
- unavailable state
- selected organization usage
- refresh behavior

## Backend Contract Rule

Feature clients must follow documented backend contracts.

If backend and Flutter disagree, update the API contract or implementation instead of hiding the mismatch in screen code.

## Retry Direction

Screens may provide retry actions for:

- network failure
- server unavailable
- temporary loading failure

Do not retry unsafe write actions automatically unless idempotency is defined.

## Final Rule

Flutter integration should be predictable: screen to state, state to feature client, feature client to shared API client, shared API client to backend.
