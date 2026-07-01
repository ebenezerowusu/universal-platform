# Initial Core Schema Guide

## Purpose

This document defines the first database tables to design before any product domain tables.

The first schema must support Platform Core, tenant isolation, permissions, modules, subscriptions, and auditability.

## Schema Principle

Core tables come before domain tables.

Do not create Religious, POS, Commerce, Finance, or HR tables until the Platform Core foundation is ready.

## Initial Table Groups

### Identity

Candidate tables:

- users
- user_credentials
- user_sessions or refresh_tokens

Purpose:

- Global user identity
- Login credentials
- Session/token tracking if needed

### Organizations

Candidate tables:

- organizations
- organization_users
- organization_settings
- organization_invitations

Purpose:

- Tenant records
- User membership in organizations
- Organization configuration

### Permissions

Candidate tables:

- permissions
- roles
- role_permissions
- organization_user_roles

Purpose:

- Role-based access control
- Permission assignment
- Scoped authorization

### Modules

Candidate tables:

- modules
- module_versions
- organization_modules
- module_dependencies
- module_permissions

Purpose:

- Module registry
- Organization module installation
- Module availability

### Subscriptions

Candidate tables:

- subscription_plans
- subscription_plan_features
- subscription_plan_limits
- organization_subscriptions
- subscription_usage_counters

Purpose:

- SaaS plan control
- Feature access
- Usage limits

### Audit

Candidate table:

- audit_logs

Purpose:

- Trace sensitive and important actions

### Provider Registry

Candidate tables:

- provider_definitions
- provider_configs
- regional_provider_rules

Purpose:

- Store provider configuration metadata
- Support payment and SMS provider selection later

## Common Columns

Most tables should include:

```text
id
created_at
updated_at
```

Organization-owned tables should usually include:

```text
organization_id
created_by nullable
updated_by nullable
deleted_at nullable
```

## ID Strategy

Use UUID or ULID style identifiers for public-facing records.

Avoid exposing predictable sequential identifiers in public APIs where security matters.

## Tenant Isolation

Every organization-owned table must support tenant filtering.

Example:

```text
organization_users.organization_id
organization_modules.organization_id
organization_subscriptions.organization_id
audit_logs.organization_id nullable
```

## Initial Migration Order

Recommended migration order:

1. users
2. organizations
3. organization_users
4. permissions
5. roles
6. role_permissions
7. organization_user_roles
8. modules
9. organization_modules
10. subscription_plans
11. organization_subscriptions
12. audit_logs
13. provider registry tables

## Index Direction

Add indexes for:

- organization_id
- user_id
- role_id
- permission_id
- module_id
- status
- created_at
- foreign keys

Use scoped unique constraints where needed.

Example:

```text
unique(organization_id, role_name)
```

## Soft Delete Direction

Use soft deletes where records represent important business configuration.

Examples:

- organizations
- roles
- modules where applicable
- organization settings where applicable

## Final Rule

The first schema must make multi-tenancy, permissions, modules, subscriptions, and audit possible before any domain feature begins.
