# Documentation Index

## Purpose

This index helps developers, architects, reviewers, and coding agents navigate the Universal Platform documentation.

The documentation is intentionally broad because the platform is designed to support many organizations, countries, modules, and domains.

## Start Here

Read these first:

1. `README.md`
2. `CLAUDE.md`
3. `docs/00-vision/00-platform-vision.md`
4. `docs/01-platform-constitution/01-platform-constitution.md`
5. `docs/02-architecture/02-platform-architecture.md`
6. `docs/14-roadmap/implementation-execution-tracker.md`

## Vision and Constitution

Key files:

- `docs/00-vision/00-platform-vision.md`
- `docs/00-vision/platform-glossary.md`
- `docs/01-platform-constitution/01-platform-constitution.md`

Purpose:

- explain what the platform is
- define the permanent architectural rules
- establish shared terminology

## Architecture

Key files:

- `docs/02-architecture/02-platform-architecture.md`
- `docs/02-architecture/event-driven-architecture.md`
- `docs/02-architecture/internal-event-bus-contract.md`

Purpose:

- explain platform layers
- define event-driven direction
- describe Core, Engines, Domains, and Infrastructure

## Capability Engines

Folder:

```text
docs/04-capability-engines/
```

Important engines:

- Identity
- Organization
- Permission
- Audit
- Module Registry
- Subscription
- Payment
- SMS
- Storage
- Notification
- Reporting
- Workflow
- Localization
- Import/Export
- Timeline
- Configuration
- Feature Flag and License

Purpose:

- define reusable platform capabilities
- prevent duplicated provider or domain-specific logic

## Domains

Folder:

```text
docs/03-domains/
```

Important domains:

- Religious Domain
- Commerce Domain
- POS Domain
- Finance Domain
- HR Domain

Purpose:

- describe business modules that sit on top of Platform Core and Engines

## Backend Development Standards

Folder:

```text
docs/06-development-standards/
```

Key files:

- `backend-crate-boundaries.md`
- `rust-backend-structure.md`
- `application-service-pattern.md`
- `repository-pattern.md`
- `rust-error-handling-standards.md`
- `transaction-and-consistency-strategy.md`
- `repository-structure-target.md`

Purpose:

- guide Rust backend implementation
- keep boundaries clear
- make code review consistent

## Database

Folder:

```text
docs/07-database/
```

Key files:

- `07-database-strategy.md`
- `core-data-model.md`
- `religious-domain-data-model.md`
- `database-relationship-map.md`
- `database-naming-conventions.md`
- `migration-standards.md`
- `seed-implementation-plan.md`

Purpose:

- define relational strategy
- guide migrations
- keep data ownership clear

## Security

Folder:

```text
docs/08-security/
```

Key files:

- `08-security-standards.md`
- `auth-session-standards.md`
- `credential-lifecycle-standards.md`
- `password-and-token-standards.md`
- `data-classification-and-privacy.md`
- `mvp-security-checklist.md`

Purpose:

- define authentication, authorization, data protection, privacy, and operational safety rules

## API

Folder:

```text
docs/12-api/
```

Key files:

- `12-api-standards.md`
- `initial-api-route-map.md`
- `api-ownership-map.md`
- `api-contract-checklist.md`
- `api-test-plan.md`
- `api-verification-plan.md`
- `openapi-mvp-outline.md`

Purpose:

- define API contracts, route ownership, response standards, and client integration rules

## Flutter UI/UX

Folder:

```text
docs/11-ui-ux/
```

Key files:

- `11-flutter-ui-ux-standards.md`
- `flutter-state-management-and-navigation.md`
- `flutter-client-build-sequence.md`
- `flutter-mobile-app-spec.md`
- `flutter-desktop-admin-spec.md`
- `design-system-component-catalog.md`

Purpose:

- guide mobile and desktop/admin Flutter implementation

## Roadmap and Execution

Folder:

```text
docs/14-roadmap/
```

Key files:

- `implementation-execution-tracker.md`
- `mvp-scope-lock.md`
- `phase-0-documentation-signoff.md`
- `sprint-1-issue-breakdown.md`
- `sprint-2-issue-breakdown.md`
- `sprint-3-issue-breakdown.md`
- `sprint-5-issue-breakdown.md`
- `sprint-6-flutter-foundation-plan.md`

Purpose:

- guide implementation order
- prevent scope creep
- define handoff from documentation to coding

## AI Development Handbook

Folder:

```text
docs/09-ai-development-handbook/
```

Key files:

- `09-ai-coding-agent-rules.md`
- `backend-implementation-guardrails.md`
- `coding-agent-review-rubric.md`
- `first-implementation-task.md`

Purpose:

- instruct Claude Code and other coding agents how to work safely in this repo

## ADRs

Folder:

```text
docs/10-adr/
```

Purpose:

- record accepted architecture decisions
- explain tradeoffs and constraints

## Operations and Infrastructure

Folder:

```text
docs/05-infrastructure/
```

Key files:

- `05-vps-architecture-notes.md`
- `docker-compose-mvp-plan.md`
- `deployment-runbook.md`
- `production-readiness-checklist.md`
- `support-operations-guide.md`
- `runtime-configuration-catalog.md`

Purpose:

- define deployment, runtime, worker, observability, backup, and support operations

## Final Rule

When starting implementation, begin with `docs/09-ai-development-handbook/first-implementation-task.md` and do not skip the architecture reading order.
