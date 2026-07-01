# Indexing and Performance Standards

## Purpose

This document defines early database performance and indexing standards for Universal Platform.

The platform must remain fast as organizations, users, modules, payments, attendance records, and reports grow.

## Performance Principle

Design queries around tenant isolation and expected access patterns.

Do not wait until tables are large before thinking about indexes.

## Common Query Pattern

Most organization-owned queries should filter by organization first.

Example pattern:

```text
organization_id + status
organization_id + created_at
organization_id + date range
organization_id + foreign key
```

## Common Indexes

Consider indexes for:

- organization_id
- status
- created_at
- updated_at
- deleted_at
- user_id
- role_id
- module_id
- foreign keys
- report date fields

## Composite Indexes

Use composite indexes for frequent query combinations.

Examples:

```text
organization_id, status
organization_id, created_at
organization_id, user_id
organization_id, branch_id
organization_id, date
```

## Unique Indexes

Use scoped uniqueness where appropriate.

Examples:

```text
organization_id + role_name
organization_id + member_code
organization_id + product_sku
```

Global uniqueness should be used only when the platform requires it.

## Soft Delete Indexing

If a table uses soft deletes, common queries may need to filter active records.

Example pattern:

```text
organization_id + deleted_at
```

## Reporting Performance

Reports can become expensive.

Start with proper indexes and simple queries.

Later options:

- materialized views
- summary tables
- background aggregation
- read replicas
- analytics store

Do not introduce heavy reporting infrastructure before it is needed.

## Pagination

Large list endpoints must use pagination.

Avoid returning unbounded lists.

## Query Review

Review queries for:

- Missing organization filter
- Missing index on common filters
- Unbounded result sets
- N+1 query patterns
- Expensive joins without need
- Report queries on large tables

## Testing Direction

As implementation grows, add tests or benchmarks for critical query paths.

Important areas:

- authentication lookups
- organization membership checks
- permission checks
- attendance reports
- payment reports
- SMS history
- imports/exports

## Final Rule

Every major query should be tenant-aware, bounded, and supported by appropriate indexes.
