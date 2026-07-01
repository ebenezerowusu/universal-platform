# Module Registry Engine Specification

## Purpose

The Module Registry Engine manages module definitions, installation, activation, availability, permissions, navigation metadata, and module lifecycle across Universal Platform.

It is the foundation for the platform's modular ecosystem.

## Responsibilities

The Module Registry Engine is responsible for:

- Registering platform modules
- Tracking available modules
- Installing modules for organizations
- Activating/deactivating modules
- Managing module dependencies
- Exposing installed module metadata
- Supporting module permissions
- Supporting module navigation
- Supporting module settings defaults
- Enforcing module availability checks
- Auditing module lifecycle actions

## Non-Responsibilities

The Module Registry does not implement the business logic of modules.

It knows that a module exists, but it should not contain the internal logic for Religious Members, POS Inventory, Finance Budgets, or HR Leave.

## Core Concepts

### Domain

A high-level business area.

Examples:

- religious
- commerce
- finance
- hr
- logistics

### Module

An installable capability within a domain or platform area.

Examples:

- religious.members
- religious.attendance
- sms.wallet
- payments.transactions
- commerce.marketplace
- pos.inventory

### Module Installation

A record showing that an organization has installed a module.

### Module Activation

A module may be installed but inactive, suspended, expired, or unavailable due to subscription limits.

## Suggested Entities

Candidate entities:

- modules
- module_versions
- organization_modules
- module_dependencies
- module_permissions
- module_navigation_items
- module_settings_definitions
- organization_module_settings

## Module Fields

Candidate fields:

- id
- module_key
- domain_key
- name
- description
- version
- status
- category
- is_core
- is_paid
- created_at
- updated_at

## Organization Module Fields

Candidate fields:

- id
- organization_id
- module_id
- status
- installed_by
- installed_at
- activated_at nullable
- suspended_at nullable
- configuration JSONB nullable
- created_at
- updated_at

Possible statuses:

- installed
- active
- inactive
- suspended
- expired
- uninstalled

## Module Manifest

A module should eventually define a manifest.

Example:

```yaml
module:
  key: religious.members
  domain: religious
  name: Member Management
  version: 1.0.0
  isPaid: false

permissions:
  - religious.members.view
  - religious.members.create
  - religious.members.update

navigation:
  - label: Members
    route: /religious/members
    permission: religious.members.view

settings:
  - key: member_label
    type: string
    default: Member
```

## Module Availability Check

A module feature is available only when:

1. The module exists.
2. The module is installed for the organization.
3. The module is active.
4. The organization's subscription allows the module/feature.
5. The user has the required permission.

## Module Installation Flow

```text
Platform/Org admin requests installation
  -> Check permission
  -> Check subscription/plan eligibility
  -> Check dependencies
  -> Create organization module record
  -> Apply default settings
  -> Register permissions/navigation where needed
  -> Publish ModuleInstalled event
  -> Audit action
```

## Module Dependencies

Modules may depend on other modules or engines.

Examples:

- religious.giving depends on payment engine
- religious.communication depends on notification engine and SMS engine
- pos.marketplace_sync depends on commerce catalog

Dependencies must be explicit.

## Navigation Metadata

The registry should expose navigation metadata to clients based on:

- Installed modules
- Active modules
- User permissions
- Platform type: mobile/desktop
- Subscription feature access

Frontend apps should not hardcode all module navigation.

## Data Retention on Module Removal

If a module is disabled or uninstalled, existing business records should be retained according to platform retention policy unless an explicit administrative export/archive policy says otherwise.

Module removal should usually stop future access and actions, not remove historical records automatically.

## Events Published

- ModuleRegistered
- ModuleInstalled
- ModuleActivated
- ModuleDeactivated
- ModuleSuspended
- ModuleUninstalled
- ModuleSettingsUpdated

## Events Consumed

- OrganizationCreated
- SubscriptionChanged
- PlanFeatureChanged

## Security Requirements

- Only authorized users can install/deactivate modules.
- Platform modules require platform-level permissions.
- Organization module changes must be audited.
- Module removal must follow retention policy.

## Tenant Isolation

Organization module installation records must be scoped to the organization.

An organization must not see or control another organization's module state.

## Testing Requirements

Tests must cover:

- Module registration
- Module installation
- Module activation
- Module dependency failure
- Subscription blocks module
- Permission blocks installation
- Installed module appears in metadata
- Uninstalled module blocks access
- Tenant isolation

## Final Rule

The Module Registry controls what an organization can use. Domains control what their modules do.
