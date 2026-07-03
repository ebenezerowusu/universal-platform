# Migration Notes Template

## Purpose

Use this template whenever database schema migrations are created or changed.

Migration notes explain why the schema changed, how tenant isolation is preserved, and what safety checks were considered.

## Sprint / PR

```text
Sprint:
Branch:
PR:
Date:
Author:
```

## Migration Files

```text
<list migration files>
```

## Schema Purpose

Explain the feature or foundation that required these tables/columns/indexes.

## Entities Added Or Changed

| Entity/Table | Change Type | Purpose |
|---|---|---|
| `<table>` | create/update/drop placeholder | `<purpose>` |

## Tenant Isolation

Confirm:

```text
[ ] Organization-owned records include organization_id
[ ] Queries must filter by organization_id where applicable
[ ] Cross-tenant access is explicitly prevented
[ ] Indexes support tenant-scoped lookups
```

## Audit Requirements

Confirm:

```text
[ ] Sensitive changes create audit events
[ ] Created/updated actor fields are included where needed
[ ] Status changes are traceable
[ ] Destructive operations are avoided or reviewed
```

## Permission Requirements

List permissions required to create, read, update, or review this data.

```text
<permission list>
```

## Indexes And Constraints

Describe:

- primary keys
- foreign keys
- unique constraints
- tenant-scoped indexes
- status/date indexes

## Data Privacy

Confirm:

```text
[ ] Sensitive fields are minimized
[ ] Raw secrets are not stored
[ ] PII fields are identified
[ ] Export/privacy implications are understood
```

## Rollback Notes

Explain rollback behavior.

Do not claim rollback is safe unless verified.

## Checks Run

```text
cargo fmt
cargo check
cargo test
migration dry run placeholder
```

## Known Limitations

- 

## Follow-Up Work

- 

## Final Rule

No tenant-owned table should be introduced without tenant isolation, indexes, audit direction, and permission direction.
