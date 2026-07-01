# Permission Engine Specification

## Purpose

The Permission Engine controls authorization across the platform.

It ensures users can only perform actions they are allowed to perform within the correct organization, module, domain, and scope.

## Responsibilities

The Permission Engine is responsible for:

- Role management
- Permission definitions
- Permission assignment
- Scope-based access control
- Tenant-aware authorization
- Module-aware permissions
- Permission checks
- Permission audit support
- Role templates
- System-level and organization-level permissions

## Core Concepts

### User

A person with an identity in the platform.

### Organization

A tenant/customer organization.

### Organization User

A user's membership or employment relationship with an organization.

### Role

A named group of permissions.

Examples:

- Platform Super Admin
- Organization Admin
- Branch Admin
- Finance Officer
- Group Leader
- Member
- Staff

### Permission

A specific action that can be allowed or denied.

Examples:

- religious.members.view
- religious.attendance.mark
- sms.campaigns.send
- payments.transactions.view
- platform.organizations.suspend

### Scope

The boundary within which a permission applies.

Recommended scopes:

- own
- own_group
- branch
- subtree
- organization
- platform

## Permission Format

Use a namespaced permission format:

```text
<domain-or-engine>.<resource>.<action>
```

Examples:

```text
religious.members.view
religious.members.create
religious.attendance.mark
sms.messages.send
payments.transactions.view
platform.modules.install
```

## Scope Examples

A group leader may have:

```text
religious.attendance.mark scope=own_group
```

A branch admin may have:

```text
religious.members.view scope=branch
```

A national administrator may have:

```text
religious.reports.view scope=subtree
```

A platform administrator may have:

```text
platform.organizations.manage scope=platform
```

## Platform vs Organization Permissions

Platform-level permissions are for SaaS owner/system administration.

Examples:

- platform.organizations.view
- platform.organizations.suspend
- platform.modules.manage
- platform.sms.providers.manage
- platform.payment.providers.manage

Organization-level permissions are scoped to tenant operations.

Examples:

- religious.members.view
- sms.messages.send
- payments.transactions.view

## Module-Aware Permissions

A permission should only be usable if:

1. The user has the permission.
2. The organization has the relevant module installed and active.
3. The organization's subscription allows the feature.
4. The user's scope permits access to the target resource.

## Authorization Flow

```text
Request received
  -> Authenticate user
  -> Resolve organization context
  -> Resolve installed modules and subscription status
  -> Check permission
  -> Check scope
  -> Allow or deny
```

## Tenant Isolation

Permission checks must always consider tenant boundaries.

A user with `religious.members.view` in Organization A must not access Organization B.

Permission alone is not enough. Tenant context must match.

## Suggested Entities

Candidate entities:

- permissions
- roles
- role_permissions
- organization_roles
- organization_user_roles
- permission_scopes
- role_templates
- user_permission_overrides optional

## Role Templates

The platform may provide templates that organizations can customize.

Examples:

- Organization Admin
- Branch Admin
- Finance Officer
- Attendance Leader
- Group Leader
- Communication Officer

Templates should not prevent custom roles.

## Deny Rules

Future versions may support explicit deny rules, but the MVP should start with allow-based permissions unless a strong need exists.

## Sensitive Permissions

Some permissions require extra audit logging.

Examples:

- Permission changes
- Payment viewing/export
- Provider settings
- Confidential counseling records
- Financial reports
- Member deletion
- Branch disconnect approval

## Backend Enforcement

Frontend hiding is not security.

Every protected backend action must call the Permission Engine.

## Events Published

- RoleCreated
- RoleUpdated
- RoleDeleted
- PermissionAssigned
- PermissionRevoked
- UserRoleAssigned
- UserRoleRemoved
- PermissionDenied optional

## Testing Requirements

Tests must cover:

- User with permission succeeds
- User without permission fails
- Correct permission but wrong organization fails
- Correct permission but wrong scope fails
- Disabled module blocks permission
- Subscription limitation blocks feature
- Platform permission cannot be used as organization permission
- Organization permission cannot be used as platform permission

## Final Rule

Every protected action requires permission, tenant context, and scope validation.
