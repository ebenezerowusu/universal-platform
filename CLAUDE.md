# Claude Code Instructions

You are working on Universal Platform.

This is not a simple application. It is a multi-tenant, modular, extensible SaaS platform.

## Prime Directive

Read the documentation before writing code.

Do not implement features that violate the Platform Constitution.

## Required Reading Order

Before implementation, read:

1. `docs/00-vision/00-platform-vision.md`
2. `docs/01-platform-constitution/01-platform-constitution.md`
3. `docs/02-architecture/02-platform-architecture.md`
4. `docs/06-development-standards/06-engineering-standards.md`
5. `docs/07-database/07-database-strategy.md`
6. `docs/08-security/08-security-standards.md`
7. `docs/09-ai-development-handbook/09-ai-coding-agent-rules.md`
8. Relevant domain/engine documents for the task
9. Relevant ADRs in `docs/10-adr/`

## Architecture Rules

- Platform Core must remain domain-agnostic.
- Business logic belongs in domains/modules or application services, not API handlers.
- Domains must not call third-party providers directly.
- Payment, SMS, storage, notification, workflow, audit, and localization must go through capability engines.
- Every organization-owned record must be tenant-aware.
- Every protected action must enforce permissions.
- Every sensitive action must be auditable.
- Configuration must drive provider, country, language, currency, and feature behavior.

## Forbidden Implementations

Do not:

- Put religious-specific logic in Platform Core.
- Hardcode Ghana, Arkessel, Hubtel, church, pastor, member, or similar concepts into Platform Core.
- Call payment or SMS providers from domain code.
- Skip tenant filters in database queries.
- Trust frontend permissions.
- Put business logic in Flutter UI.
- Create database tables without considering organization ownership and indexes.
- Start domain features before Platform Core foundations are ready.

## First Backend Implementation Direction

When development begins, implement in this order:

1. Rust workspace structure
2. Configuration loader
3. Health check endpoint
4. Logging/tracing foundation
5. PostgreSQL connection
6. Migration setup
7. Error handling standard
8. API response standard
9. Identity foundation
10. Organization/tenant foundation
11. Permission Engine foundation
12. Audit Engine foundation
13. Module Registry foundation

Do not start with Religious Domain features first.

## Planning Format Before Coding

Before coding a task, explain:

```text
1. Classification: Core / Engine / Domain / Infrastructure / UI
2. Files to create/change
3. Data model impact
4. Permission impact
5. Tenant isolation approach
6. Events if any
7. Tests to add
8. Documentation update needed
```

## Completion Summary Format

After a task, summarize:

```text
What changed
Files changed
Architecture rules followed
Tests run
Risks/follow-ups
```

## Final Reminder

Build the platform once. Expand it forever.
