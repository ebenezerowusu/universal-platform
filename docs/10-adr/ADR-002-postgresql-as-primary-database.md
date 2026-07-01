# ADR-002: Use PostgreSQL as the Primary Database

## Status

Accepted

## Context

Universal Platform has many relational business requirements, including organizations, users, roles, permissions, modules, subscriptions, branch hierarchies, members, households, attendance, payments, finance, inventory, marketplace orders, HR records, and audit logs.

A NoSQL-first database was considered, including Cosmos DB, but the system's core data model is strongly relational.

## Decision

Use PostgreSQL as the primary database for the MVP and initial production platform.

PostgreSQL will initially be installed on the VPS with the API and Redis.

The application must use `DATABASE_URL` and must not assume PostgreSQL always runs on localhost.

## Alternatives Considered

### Cosmos DB / NoSQL First

Rejected for the primary system of record because the platform has strong relational consistency and reporting needs.

NoSQL may be introduced later for specific high-volume or flexible workloads.

### MySQL

MySQL could work, but PostgreSQL provides stronger support for relational integrity, JSONB, advanced indexing, reporting, and long-term extensibility.

## Consequences

### Positive

- Strong relational integrity.
- Good fit for multi-tenant SaaS.
- Good reporting support.
- Mature backup and restore tools.
- JSONB support for flexible fields.
- Easier modeling of relationships and transactions.

### Negative / Cost

- Requires disciplined schema design.
- Requires indexing strategy as data grows.
- May need dedicated server or managed database later.

## Future Direction

PostgreSQL can later move from the VPS to a dedicated database server or managed service.

Only configuration should change:

```env
DATABASE_URL=postgres://user:password@db.internal:5432/universal_platform
```

Application business logic must not change.
