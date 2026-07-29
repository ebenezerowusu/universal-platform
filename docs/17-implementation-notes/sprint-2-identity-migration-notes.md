# Sprint 2 Identity Migration Notes

## Purpose

This document records the database migration notes for the Sprint 2 Identity foundation.

## Sprint / PR

```text
Sprint: Sprint 2 Identity Foundation
Branch: feature/identity-foundation
PR: pending
Date: 2026-07-29
Author: implementation agent
```

## Migration Files

```text
migrations/0001_identity_users.sql
```

## Schema Purpose

The migration introduces `identity_users`, the global account table for Universal Platform authentication identities.

Domain-specific actors such as religious members, commerce customers, staff, partners, or operators must not be stored directly as Identity users. Those profiles should link to Identity users where login access is required.

## Entities Added Or Changed

| Entity/Table | Change Type | Purpose |
|---|---|---|
| `identity_users` | create | Store platform-level user identities and password hashes |

## Tenant Isolation

Confirm:

```text
[ ] Organization-owned records include organization_id
[ ] Queries must filter by organization_id where applicable
[ ] Cross-tenant access is explicitly prevented
[ ] Indexes support tenant-scoped lookups
```

Identity users are global accounts, not tenant-owned records. Organization membership and tenant-scoped access are intentionally deferred to the Organization/Tenant foundation.

## Audit Requirements

Future identity actions that should be audited:

```text
audit.identity.user_created
audit.identity.user_updated
audit.identity.user_deactivated
audit.identity.login_failed_placeholder
audit.identity.password_reset_requested_placeholder
audit.identity.password_changed_placeholder
```

Audit persistence is intentionally not implemented in Sprint 2.

## Permission Requirements

Initial planned permissions:

```text
identity.users.view
identity.users.manage
identity.profiles.view
identity.profiles.manage
```

Self-service authentication actions will require authentication/security controls rather than organization-level admin permissions.

## Indexes And Constraints

Added:

```text
PRIMARY KEY (id)
UNIQUE INDEX lower(email)
INDEX status
INDEX created_at
CHECK status values
CHECK email not blank
```

## Data Privacy

Confirm:

```text
[x] Password hashes are stored, not plain passwords
[x] Raw provider secrets are not stored
[x] Email is treated as personal data
[x] Identity users are separated from domain profiles
```

## Rollback Notes

Rollback should drop `identity_users` only if no dependent membership/session/profile tables have been created.

Do not drop this table in production without a reviewed data retention and tenant exit plan.

## Checks Run

```text
Not run in this environment.
Expected CI checks:
cargo fmt
cargo check
cargo test
migration dry run placeholder
```

## Known Limitations

- No real password hasher implementation yet.
- No PostgreSQL repository implementation yet.
- No authentication routes yet.
- No session/token persistence yet.
- No audit persistence yet.
- No organization membership table yet.

## Follow-Up Work

- Add Organization/Tenant foundation.
- Add identity repository implementation after database access is wired.
- Add password hashing implementation after security review.
- Add sessions and authentication flows in a later sprint.

## Final Rule

Identity is global and domain-agnostic. Tenant access must be introduced through organization membership, not domain-specific identity tables.
