# ADR-013: Centralize Configuration and Feature Availability

## Status

Accepted

## Context

Universal Platform will support many organizations, countries, domains, modules, providers, subscriptions, and feature rollout strategies.

If configuration and feature checks are scattered across domains, API handlers, and Flutter clients, the platform will become difficult to maintain and risky to extend.

## Decision

Use centralized Configuration Engine and Feature Flag/License Engine concepts for configurable behavior and feature availability checks.

Module Registry remains responsible for module installation and activation.

Subscription Engine remains responsible for plan, billing status, features, and limits.

Permission Engine remains responsible for user authorization.

Feature availability should be evaluated by combining the required checks instead of scattering the logic across the codebase.

## Consequences

### Positive

- Less hardcoding
- Easier country and organization customization
- Safer feature rollout
- Cleaner module access checks
- Better subscription enforcement
- Easier client UI adaptation

### Cost

- More upfront structure
- Requires clear setting definitions
- Requires consistent feature keys
- Requires discipline to avoid bypassing central checks

## Implementation Direction

A feature is available only when required conditions pass.

Typical conditions:

```text
module installed
module active
subscription allows feature
feature flag enabled
organization setting allows behavior
user has permission
usage limit not exceeded
```

Flutter clients may receive safe summaries for UI presentation, but backend checks remain authoritative.

## Final Rule

Configuration and feature availability must be centralized. Do not scatter feature checks across handlers, domains, or UI code.
