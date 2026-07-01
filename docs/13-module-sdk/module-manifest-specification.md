# Module Manifest Specification

## Purpose

The module manifest defines how modules describe themselves to Universal Platform.

It allows the platform to understand module metadata, permissions, navigation, settings, dependencies, events, and lifecycle behavior.

## Manifest Principle

A module should declare what it needs and what it provides.

The Platform Core should not need to understand the module's internal business logic.

## Example Manifest

```yaml
module:
  key: religious.members
  domain: religious
  name: Member Management
  version: 1.0.0
  description: Manage religious organization members.
  type: domain_module
  isPaid: false

permissions:
  - key: religious.members.view
    description: View members
  - key: religious.members.create
    description: Create members
  - key: religious.members.update
    description: Update members

navigation:
  - labelKey: religious.members.title
    route: /religious/members
    permission: religious.members.view
    platforms:
      - desktop
      - mobile

settings:
  - key: member_label
    type: string
    default: Member

events:
  publishes:
    - ReligiousMemberCreated
    - ReligiousMemberUpdated
  consumes:
    - PaymentCompleted

dependencies:
  engines:
    - permission
    - audit
    - storage
  modules: []
```

## Required Sections

A production module manifest should eventually include:

- module metadata
- permissions
- navigation
- settings
- events
- dependencies
- supported platforms
- version information

## Module Metadata

Required metadata:

- key
- domain
- name
- version
- description
- type
- status

## Permissions

Permissions must be explicit.

Permission format:

```text
<domain-or-engine>.<resource>.<action>
```

Examples:

```text
religious.members.view
pos.products.create
finance.reports.export
```

## Navigation

Navigation entries should be permission-aware and platform-aware.

The frontend should use module metadata to build menus where practical.

## Settings

Settings should define configurable behavior.

Setting fields may include:

- key
- type
- default
- required
- description
- allowed values

## Events

Modules should declare important events they publish and consume.

This helps other modules and AI agents understand integration points.

## Dependencies

Modules may depend on engines or other modules.

Dependencies must be explicit to avoid hidden coupling.

## Versioning

Module versions should follow a consistent versioning strategy.

Breaking module behavior requires careful migration and documentation.

## Final Rule

A module without a clear manifest is not ready to become part of the platform ecosystem.
