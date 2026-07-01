# Module Dependency Rules

## Purpose

This document defines how modules may depend on one another inside Universal Platform.

Clear dependency rules prevent hidden coupling and make modules easier to install, disable, test, and evolve.

## Dependency Principle

A module must explicitly declare its dependencies.

No module should silently require another module without the Module Registry knowing.

## Dependency Types

### Engine Dependency

A module requires a shared capability engine.

Examples:

```text
religious.giving -> payments
religious.communication -> notifications
religious.communication -> sms
religious.events -> storage
```

### Domain Module Dependency

A module requires another module in the same or another domain.

Examples:

```text
religious.attendance -> religious.members
religious.groups -> religious.members
religious.giving -> religious.members optional
```

### Platform Dependency

A module requires Platform Core capabilities.

Examples:

```text
identity
organizations
permissions
modules
subscriptions
audit
```

## Required vs Optional Dependencies

A dependency may be:

- required
- optional

Required dependencies must be installed or active before the module can work.

Optional dependencies enhance behavior but should not block basic module use.

## Example

```yaml
module:
  key: religious.attendance

dependencies:
  required:
    engines:
      - permissions
      - audit
      - reporting
    modules:
      - religious.members
  optional:
    engines:
      - notifications
```

## Installation Rule

When installing a module, the Module Registry should check:

- module exists
- organization is eligible
- required dependencies are available
- subscription allows the module or feature
- current user has permission to install

## Deactivation Rule

Before deactivating a module, check whether other active modules depend on it.

If dependent modules exist, the system should warn or block according to policy.

## Runtime Rule

Runtime access should check:

- module installed
- module active
- required dependencies available
- feature allowed by subscription
- user permission

## Final Rule

Dependencies must be visible to the platform. Hidden module dependencies are architecture debt.
