# Feature Availability Decision Flow

## Purpose

This document defines the MVP feature availability decision flow for Universal Platform.

Feature availability answers:

```text
Is this feature usable by this organization and user right now?
```

## Principle

Feature availability is not the same as permission.

A user may have permission, but the feature may still be unavailable because the module is inactive, the plan does not allow it, or a limit has been reached.

## Decision Layers

The full decision flow may include:

```text
Authenticated User
  -> Organization Context
    -> Organization Membership
      -> Module Availability
        -> Subscription / Plan Feature
          -> Feature Flag / License Rule
            -> Usage Limit
              -> Permission Check
                -> Application Service Action
```

## MVP Sprint 4 Scope

Sprint 4 should establish foundation checks for:

- module availability
- plan feature availability
- usage limit foundation
- engine interface availability

Full advanced licensing can be deferred.

## Module Availability

Module Registry answers:

```text
Is this module installed and active for this organization?
```

## Plan Feature Availability

Subscription Engine answers:

```text
Does this organization's plan allow this feature?
```

## Usage Limit

Subscription Engine may later answer:

```text
Has this organization reached a configured limit?
```

Examples:

- maximum members
- maximum users
- maximum branches
- storage limit
- SMS usage package

## Permission Check

Permission Engine answers:

```text
Can this user perform this action?
```

Permission should not replace module or plan checks.

## Application Service Rule

Application services should ask the correct engine instead of manually reading module, plan, or permission data.

## Denied Availability Behavior

When a feature is unavailable:

- return standard error shape
- use safe error messages
- include upgrade or activation hints later where appropriate
- do not expose internal plan or rule complexity unnecessarily

## Final Rule

Feature availability must be centralized and composable. Do not scatter module, plan, limit, and permission checks across unrelated code paths.
