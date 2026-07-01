# Database Strategy

## Decision Summary

Universal Platform will use PostgreSQL as the primary database for the MVP and initial production versions.

PostgreSQL will initially run on the VPS together with the API, Redis, workers, and storage. Later, PostgreSQL can be moved to a dedicated server or managed database service without changing business logic.

## Why PostgreSQL

The platform has strong relational requirements:

- Organizations and tenants
- Users and identities
- Roles and permissions
- Module installations
- Subscription plans
- Branch hierarchies
- Members and households
- Attendance records
- Payments and financial records
- POS and inventory
- Marketplace orders
- HR records
- Audit logs
- Reports

PostgreSQL is a strong fit because it provides:

- Relational integrity
- Foreign keys
- Transactions
- Indexes
- JSONB when flexible fields are needed
- Strong reporting support
- Mature backup/restore tooling
- Reliable production behavior

## MVP Placement

For the MVP, PostgreSQL will be installed on the VPS.

```text
Hostinger VPS
  - Ubuntu 24 LTS
  - Nginx
  - Rust + Axum API
  - PostgreSQL
  - Redis
  - Workers
  - Local Storage
```

Example configuration:

```env
DATABASE_URL=postgres://app_user:password@localhost:5432/universal_platform
```

## Future Movement

When the system grows, PostgreSQL can move to a dedicated database server.

Only configuration should change:

```env
DATABASE_URL=postgres://app_user:password@db.internal:5432/universal_platform
```

The Rust application must not change.

## Multi-Tenancy

Every tenant-owned record must be associated with an organization/tenant identifier unless there is a clear platform-level reason not to.

Tenant isolation must be enforced at multiple levels:

- Application service authorization
- Repository query filters
- Permission checks
- Tests
- Audit logs

A user must never be able to access another organization's data through missing filters or weak authorization.

## Schema Strategy

The initial recommendation is a shared database with tenant-aware tables.

Example:

```text
organizations
users
organization_users
roles
permissions
members
attendance_sessions
attendance_records
payments
audit_logs
```

Most tenant-owned tables should include:

```text
organization_id
created_at
updated_at
deleted_at nullable
created_by nullable
updated_by nullable
```

## IDs

Use UUIDs or ULIDs for public-facing identifiers.

Internal numeric IDs may be considered only where appropriate, but exposed APIs should avoid predictable sequential IDs where security matters.

Recommended default:

- UUID/ULID primary keys for most platform records
- Stable external references where needed
- Provider transaction references stored separately

## JSONB Usage

PostgreSQL JSONB may be used for flexible data, but it must not become a dumping ground.

Good JSONB use cases:

- Custom field values
- Provider metadata
- Dynamic settings
- Event payloads
- Audit metadata

Avoid JSONB for core relational concepts that need strong constraints and reporting.

## Migrations

All database changes must use migrations.

Migration rules:

- No manual production schema changes.
- Migrations must be reversible where practical.
- Dangerous migrations must be reviewed.
- Data migrations must be separated from schema migrations when complex.
- Existing tenant data must be protected.

## Index Strategy

Indexes should be designed around real query patterns.

Common indexes:

- organization_id
- created_at
- status
- foreign keys
- date ranges for reports
- unique constraints scoped by organization where needed

Example:

```sql
unique(organization_id, email)
```

instead of globally unique email, unless global uniqueness is required.

## Soft Deletes

Soft deletes should be used for important business records where history matters.

Examples:

- Members
- Branches
- Roles
- Financial categories
- Attendance sessions

Hard deletes may be used for temporary records, tokens, expired sessions, and safe cleanup tables.

## Auditability

Critical data changes must be auditable.

Audit logs should capture:

- actor
- organization
- action
- entity type
- entity id
- before/after where appropriate
- timestamp
- IP/user agent where available

## Backups

PostgreSQL backups are mandatory from day one.

Minimum requirements:

- Daily backup
- Restore testing
- Retention policy
- Off-server backup as soon as possible

## Reporting

PostgreSQL should support operational reporting at the beginning.

As reporting grows, consider:

- Read replicas
- Materialized views
- Partitioning
- Data warehouse later

Do not introduce separate analytics infrastructure before it is needed.

## Future Database Options

Cosmos DB or other NoSQL databases may be introduced later for specific use cases such as:

- High-volume event logs
- Flexible activity feeds
- Notification history
- Certain catalog/search workloads

However, PostgreSQL remains the primary system of record unless an ADR decides otherwise.

## Database Principle

PostgreSQL is the source of truth for the platform's core relational data.

Flexible storage is allowed, but core business integrity must remain relational and reliable.
