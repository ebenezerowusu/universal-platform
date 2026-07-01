# Multi-Tenancy Data Rules

## Purpose

This document defines database rules for multi-tenancy in Universal Platform.

Tenant isolation is one of the most important security and architecture requirements in the system.

## Core Rule

Every organization-owned record must include an organization reference unless there is a documented reason not to.

Typical field:

```text
organization_id
```

## Platform-Level Tables

Some tables are platform-level and may not belong to one organization.

Examples:

- global users
- global permissions
- module definitions
- subscription plan definitions
- country/provider configuration

Even when a table is platform-level, access must still be permission-controlled.

## Organization-Owned Tables

Most business tables are organization-owned.

Examples:

- organization users
- roles created by organization
- member records
- attendance sessions
- attendance records
- SMS wallets
- payment transactions
- file metadata
- workflow instances
- audit logs

## Required Common Fields

Most organization-owned tables should include:

```text
id
organization_id
created_at
updated_at
deleted_at nullable
created_by nullable
updated_by nullable
```

Not every table requires every field, but deviations should be intentional.

## Query Rule

Repository queries for organization-owned records must include organization scope.

Correct pattern:

```text
find member where id = member_id and organization_id = current_organization_id
```

Incorrect pattern:

```text
find member where id = member_id
```

## Unique Constraints

Many unique constraints should be scoped by organization.

Examples:

```text
unique(organization_id, email)
unique(organization_id, phone)
unique(organization_id, code)
```

Use global uniqueness only when the platform requires it.

## Foreign Keys

Use foreign keys for relational integrity where practical.

Important relationships should be enforced by the database, not only application logic.

## Indexes

Common indexes:

```text
organization_id
organization_id + status
organization_id + created_at
organization_id + deleted_at
foreign key columns
date fields used in reports
```

Indexes should support tenant-filtered queries.

## Soft Deletes

Important business records should usually use soft deletes.

Examples:

- members
- roles
- groups
- branches
- attendance sessions
- financial categories

Temporary records may use hard deletes where safe.

## Audit and History

Sensitive changes should be tracked through the Audit Engine.

Some entities may also need history tables.

Examples:

- member status history
- organization status history
- role/permission assignment history
- payment status events

## JSONB Rules

JSONB is allowed for flexible metadata, settings, and custom fields.

Good uses:

- provider metadata
- custom field values
- settings
- event payloads
- audit metadata

Avoid JSONB for core relational structures that require constraints and reporting.

## Data Access Tests

Every major repository/service must include tests proving tenant isolation.

Mandatory test pattern:

1. Create Organization A and Organization B.
2. Create data for both organizations.
3. Authenticate or operate as user from Organization A.
4. Attempt to access Organization B data.
5. Verify access is denied or data is not returned.

## Final Rule

A missing organization filter is a critical bug.
