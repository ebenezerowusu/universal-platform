# Module SDK Overview

## Purpose

The Module SDK defines how domains and modules plug into Universal Platform.

The SDK may start as internal conventions and later evolve into a formal developer platform for trusted third-party modules.

## Why This Matters

The long-term vision is not only to build modules internally, but to allow the platform to grow into an ecosystem.

A clear module contract allows new domains to be added without modifying the Platform Core.

## Module Manifest

Every module should eventually define a manifest.

Example:

```yaml
module:
  id: religious.members
  domain: religious
  name: Member Management
  version: 1.0.0
  description: Manage religious organization members.

permissions:
  - religious.members.view
  - religious.members.create
  - religious.members.update
  - religious.members.delete

navigation:
  - label: Members
    route: /religious/members
    permission: religious.members.view

events:
  publishes:
    - MemberCreated
    - MemberUpdated
  consumes:
    - PaymentCompleted

settings:
  - key: member_label
    type: string
    default: Member
```

## Module Responsibilities

A module may define:

- Permissions
- Navigation entries
- Settings
- API routes
- Database migrations
- Event publishers
- Event subscribers
- Background jobs
- Reports
- UI metadata
- Feature flags

## Module Lifecycle

Expected lifecycle:

1. Registered
2. Installed for organization
3. Configured
4. Activated
5. Used
6. Updated
7. Suspended/deactivated
8. Uninstalled where safe

## Installation Rules

Installing a module should:

- Verify subscription/plan access
- Register permissions
- Create default settings
- Create navigation entries
- Run safe migrations if needed
- Emit ModuleInstalled event
- Audit the action

## Configuration Rules

Module behavior should be configurable per organization where appropriate.

Examples:

- Religious Domain can configure terminology such as member, leader, branch.
- Attendance module can configure QR/GPS/manual rules.
- SMS module can configure templates and sender options within platform limits.
- Marketplace can configure delivery settings.

## Permission Rules

Modules must not rely only on frontend hiding.

Every protected API action must enforce backend permission checks.

Permissions should support scopes such as:

- own
- own_group
- branch
- subtree
- organization
- platform

## Event Rules

Modules should publish events instead of tightly coupling to other modules.

Example:

```text
MemberCreated
  -> Notification module may send welcome message
  -> Reporting module may update dashboard
  -> Workflow module may start onboarding workflow
```

The Members module should not know which modules are listening.

## Migration Rules

Module migrations must be controlled and reversible where practical.

A module must not modify another module's private tables without an approved ADR.

## UI Rules

The backend should expose module and permission metadata that helps clients build navigation dynamically.

Flutter apps should show/hide features based on:

- Installed modules
- User permissions
- Organization subscription plan
- Feature flags
- Platform type: mobile or desktop

## Future Module Marketplace

In the future, the platform may support a marketplace where approved modules can be published and installed.

Before that happens, internal modules should already follow strict SDK-style conventions.

## Final Principle

A module is a guest inside the platform.

It must respect the Platform Core, tenant boundaries, permissions, events, and configuration rules.
