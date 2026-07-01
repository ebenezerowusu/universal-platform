# Database Migration Standards

## Purpose

This document defines how database migrations should be handled in Universal Platform.

Database migrations must be safe, reviewable, reversible where practical, and respectful of tenant data.

## Migration Principle

No manual production schema changes.

All database schema changes must be represented as migrations in the repository.

## Migration Naming

Migration names should clearly describe the change.

Examples:

```text
create_users_table
create_organizations_table
add_status_to_organization_modules
create_audit_logs_table
```

## Migration Order

Core migrations should come before engine and domain migrations.

Recommended early order:

1. Identity
2. Organizations
3. Permissions
4. Modules
5. Subscriptions
6. Audit
7. Provider registry
8. Engine tables
9. Domain tables

## Safe Migration Rules

Before writing a migration, consider:

- Does it affect existing data?
- Is a default value needed?
- Is backfill required?
- Will it lock a large table?
- Does it need to be split into multiple steps?
- Is rollback possible?

## Data Migrations

Complex data changes should be handled carefully.

Where possible, separate schema changes from data backfills.

## Rollback Direction

Migrations should support rollback where practical.

If rollback is not practical, document why and provide a recovery plan.

## Tenant-Aware Migrations

When adding organization-owned tables, include organization scope.

Most organization-owned tables should include:

```text
organization_id
created_at
updated_at
```

## Constraints

Use database constraints for important integrity rules.

Examples:

- foreign keys
- not-null constraints
- unique constraints
- scoped uniqueness by organization

## Indexes

Add indexes for expected access patterns.

Do not add random indexes without query need.

## Review Checklist

Migration review should verify:

- Naming is clear
- Tenant scope is correct
- Foreign keys are appropriate
- Indexes are appropriate
- Existing data is protected
- Rollback/recovery is considered

## Final Rule

Migrations are production changes. Treat them with the same care as application code.
