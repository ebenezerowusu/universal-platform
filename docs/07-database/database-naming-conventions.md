# Database Naming Conventions

## Purpose

This document defines database naming conventions for Universal Platform.

Consistent naming helps developers, reporting tools, migrations, and coding agents understand the data model.

## Principle

Names should be clear, consistent, and aligned with ownership boundaries.

## Table Names

Use plural snake_case table names.

Examples:

```text
users
organizations
organization_users
religious_members
payment_transactions
sms_messages
```

## Core Table Names

Core table names should remain domain-agnostic.

Examples:

```text
users
organizations
permissions
roles
modules
subscriptions
audit_logs
```

## Domain Table Names

Domain tables should include the domain prefix where useful.

Examples:

```text
religious_members
religious_visitors
religious_groups
pos_products
commerce_orders
hr_staff_profiles
```

## Column Names

Use snake_case column names.

Common columns:

```text
id
organization_id
created_at
updated_at
deleted_at
created_by
updated_by
status
metadata
```

## Foreign Key Names

Foreign key columns should use the referenced resource name with `_id`.

Examples:

```text
user_id
organization_id
member_id
group_id
module_id
```

## Status Columns

Use `status` where a record has a lifecycle.

Keep status values documented per entity.

## Metadata Columns

Use `metadata` for optional non-core details.

Do not use metadata for important relational fields that must be filtered, joined, validated, or reported often.

## Index Names

Index names should be readable and predictable.

Suggested pattern:

```text
idx_<table>_<column_or_purpose>
```

Examples:

```text
idx_religious_members_organization_id
idx_religious_members_org_status
idx_audit_logs_org_created_at
```

## Unique Constraint Names

Suggested pattern:

```text
uniq_<table>_<column_or_purpose>
```

Examples:

```text
uniq_organizations_slug
uniq_roles_org_name
```

## Migration Names

Migration names should describe the change.

Examples:

```text
create_users_table
create_organizations_table
create_religious_members_table
add_status_to_modules
```

## Final Rule

Database names should make ownership, purpose, and query intent obvious.
