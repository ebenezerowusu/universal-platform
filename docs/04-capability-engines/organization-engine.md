# Organization Engine Specification

## Purpose

The Organization Engine manages tenants, organization profiles, organization users, organization settings, and tenant context across Universal Platform.

It is one of the most important engines because every domain and module depends on organization boundaries.

## Engine Position

```text
Platform Core
  -> Organization Engine
    -> Domains and Modules use organization context
```

## Responsibilities

The Organization Engine is responsible for:

- Organization creation
- Organization profile management
- Organization status management
- Organization user membership
- Tenant context resolution
- Organization settings
- Organization branding
- Country, currency, timezone, and language configuration
- Organization hierarchy foundation where generic
- Organization-level feature configuration
- Organization audit support

## Non-Responsibilities

The Organization Engine does not implement domain-specific branch logic.

Examples:

- Religious branch hierarchy belongs to the Religious Domain, although it uses organization relationships.
- Shop/store hierarchy belongs to Commerce/POS domains.
- School campuses would belong to an Education domain in the future.

The engine provides generic tenant foundation, not domain-specific business meaning.

## Core Concepts

### Organization

An organization is a tenant/customer entity using the platform.

An organization may represent:

- Religious organization
- Retail business
- NGO
- Foundation
- Future organization type

### Organization User

An organization user links a platform user to an organization.

A single user may belong to multiple organizations.

### Organization Context

The active tenant context for a request.

Every tenant-owned action must resolve and validate organization context.

### Organization Settings

Configuration that controls organization-specific behavior.

Examples:

- country
- currency
- timezone
- default language
- supported languages
- branding
- date format
- terminology defaults

## Organization Statuses

Possible statuses:

- pending
- active
- suspended
- disabled
- archived

Suspended organizations should have restricted access according to platform policy.

## Suggested Entities

Candidate entities:

- organizations
- organization_users
- organization_settings
- organization_profiles
- organization_status_history
- organization_invitations
- organization_domains optional
- organization_branding optional

## Organization Fields

Core organization fields may include:

- id
- name
- legal_name nullable
- slug
- type
- country
- currency
- timezone
- default_language
- status
- created_at
- updated_at
- deleted_at nullable

## Organization User Fields

Candidate fields:

- id
- organization_id
- user_id
- status
- joined_at
- invited_by nullable
- created_at
- updated_at

Possible statuses:

- invited
- active
- suspended
- removed

## Tenant Context Resolution

API requests should resolve organization context from the route, token, or explicit organization selector.

Preferred organization API pattern:

```text
/api/v1/organizations/{organizationId}/...
```

The system must verify that the authenticated user has access to the requested organization.

## Tenant Isolation Rule

Organization context must be enforced before accessing tenant-owned data.

A user cannot access Organization B simply by changing `organizationId` in a URL.

## Organization Settings

Settings should be extensible and typed where practical.

Examples:

```json
{
  "country": "GH",
  "currency": "GHS",
  "timezone": "Africa/Accra",
  "defaultLanguage": "en",
  "supportedLanguages": ["en", "tw", "fr"],
  "dateFormat": "DD/MM/YYYY"
}
```

Do not hardcode Ghana-specific behavior in the engine.

## Branding

Organization branding may include:

- logo
- primary color
- secondary color
- contact email
- contact phone
- public profile visibility

Branding should not affect platform security or authorization logic.

## Organization Invitations

Organizations should be able to invite users.

Invitation flow:

```text
Admin invites user
  -> Invitation created
  -> Notification sent
  -> User accepts
  -> Organization user activated
  -> Role assigned
  -> Audit log recorded
```

## Events Published

- OrganizationCreated
- OrganizationUpdated
- OrganizationSuspended
- OrganizationReactivated
- OrganizationUserInvited
- OrganizationUserJoined
- OrganizationUserRemoved
- OrganizationSettingsUpdated

## Events Consumed

Possible consumed events:

- SubscriptionCreated
- SubscriptionExpired
- ModuleInstalled
- PaymentCompleted

## Security Requirements

- Organization access must be verified on every tenant-owned request.
- Organization slug changes must be controlled.
- Suspended organizations must be restricted.
- Organization settings changes must be audited.
- Invites must use secure expiring tokens.

## Testing Requirements

Tests must cover:

- User can access own organization
- User cannot access another organization
- Suspended organization restrictions
- Organization settings update audit
- Invitation acceptance
- Multi-organization user context switching

## Final Rule

Every tenant-owned feature depends on Organization Engine correctness. Tenant isolation failure is a critical platform failure.
