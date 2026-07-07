# Master Documentation Index

## Purpose

This document is the top-level index for the Universal Platform documentation set.

It points coding agents, architects, and implementation teams to the correct documents before making design or implementation decisions.

## Current Documentation Status

```text
architecture documentation track complete
production operations coverage added
implementation should start now
```

The default next action is implementation, not more speculative documentation.

## Required Reading Order For Coding Agents

1. `CLAUDE.md`
2. `docs/00-vision/00-platform-vision.md`
3. `docs/01-platform-constitution/01-platform-constitution.md`
4. `docs/02-architecture/02-platform-architecture.md`
5. `docs/06-development-standards/06-engineering-standards.md`
6. `docs/07-database/07-database-strategy.md`
7. `docs/08-security/08-security-standards.md`
8. `docs/05-infrastructure/production-operations-readiness-addendum.md`
9. `docs/09-ai-development-handbook/09-ai-coding-agent-rules.md`
10. `docs/10-adr/ADR-015-phase-0-documentation-complete-start-implementation.md`
11. `docs/10-adr/ADR-016-architecture-documentation-closed-start-implementation.md`
12. `docs/17-implementation-notes/implementation-cutover-guide.md`
13. The API/domain/engine document relevant to the task being implemented

## Vision And Constitution

| Area | Document |
|---|---|
| Platform vision | `docs/00-vision/00-platform-vision.md` |
| Platform constitution | `docs/01-platform-constitution/01-platform-constitution.md` |
| Architecture overview | `docs/02-architecture/02-platform-architecture.md` |
| Domain overview | `docs/03-domains/03-domain-overview.md` |
| Capability engine overview | `docs/04-capability-engines/04-engine-overview.md` |

## Development, Infrastructure, Security, And Database

| Area | Document |
|---|---|
| Deployment strategy | `docs/05-infrastructure/05-deployment-strategy.md` |
| Production operations readiness | `docs/05-infrastructure/production-operations-readiness-addendum.md` |
| Engineering standards | `docs/06-development-standards/06-engineering-standards.md` |
| Database strategy | `docs/07-database/07-database-strategy.md` |
| Security standards | `docs/08-security/08-security-standards.md` |
| AI coding agent rules | `docs/09-ai-development-handbook/09-ai-coding-agent-rules.md` |
| Architecture decisions | `docs/10-adr/` |

## API Contracts

All API contracts live under:

```text
docs/12-api/
```

Use `docs/18-traceability/api-route-catalogue.md` for a consolidated route catalogue.

## UI/UX Contracts

All UI/UX contracts live under:

```text
docs/11-ui-ux/
```

Each UI contract describes screens, states, permission boundaries, and exclusions.

## Module SDK And Readiness Documents

All module readiness and SDK-related documents live under:

```text
docs/13-module-sdk/
```

Use them before adding new module-facing behavior.

## Roadmaps

All sprint roadmaps live under:

```text
docs/14-roadmap/
```

Use `docs/14-roadmap/sprint-0-to-68-roadmap-summary.md` for the consolidated Sprint 0 to Sprint 68 view.

## Quality Reviews

All quality and review gates live under:

```text
docs/16-quality/
```

Use `docs/16-quality/final-architecture-completion-review.md` to confirm architecture completion status.

## Implementation Notes

All implementation handoff and execution notes live under:

```text
docs/17-implementation-notes/
```

Start with:

```text
docs/17-implementation-notes/implementation-cutover-guide.md
```

## Traceability Catalogues

All consolidated traceability documents live under:

```text
docs/18-traceability/
```

Primary traceability documents:

- `feature-to-engine-traceability-matrix.md`
- `permission-catalogue.md`
- `api-route-catalogue.md`
- `entity-data-model-catalogue.md`
- `event-catalogue.md`
- `audit-event-catalogue.md`
- `module-dependency-map.md`

## Architecture Closure Documents

| Purpose | Document |
|---|---|
| Final roadmap closure | `docs/14-roadmap/architecture-completion-wave-sprints-69-72-roadmap.md` |
| Final coding-agent prompt | `docs/09-ai-development-handbook/final-architecture-completion-coding-agent-prompt.md` |
| Final ADR | `docs/10-adr/ADR-016-architecture-documentation-closed-start-implementation.md` |
| Implementation transition | `docs/17-implementation-notes/implementation-transition-plan.md` |
| Final review | `docs/16-quality/final-architecture-completion-review.md` |

## Final Rule

Use this index to find documents quickly, then build.

Build the platform once. Expand it forever.
