# ADR-016: Architecture Documentation Closed; Start Implementation

## Status

Accepted

## Context

The Universal Platform documentation track has expanded across core architecture, domains, capability engines, infrastructure, security, database strategy, quality, operations, governance, compliance, revenue, support, partner ecosystem, implementation delivery, solution blueprints, analytics, experimentation, and customer success.

The platform now has enough architecture guidance to begin implementation without continuing speculative documentation.

Continuing to add more documentation before code creates delivery risk:

- implementation gets delayed
- architecture remains untested
- future modules become theoretical
- developers lack executable feedback
- product progress slows

ADR-015 already stated that Phase 0 documentation was complete and implementation should start. The later sprints have further clarified broad future foundations. ADR-016 now closes the architecture expansion track.

## Decision

Architecture documentation is considered complete enough for implementation.

The default next action is now:

```text
Start implementation.
```

The first implementation branch should be:

```text
chore/backend-scaffold
```

The first implementation PR should be:

```text
chore: initialize backend scaffold
```

## Scope of First Implementation

The first implementation should include:

- Rust workspace scaffold
- Axum API crate
- platform-core crate
- platform-config crate
- platform-infra crate
- health endpoint
- readiness endpoint placeholder
- environment configuration loader
- local Docker Compose with PostgreSQL and Redis
- .env.example
- backend CI workflow
- implementation log

## Future Documentation Rule

New documentation is allowed only when:

- implementation exposes a real missing contract
- code needs a new ADR to resolve a concrete decision
- a user explicitly asks for a new roadmap/domain plan
- a blocker cannot be solved without documenting a new boundary

Documentation should not continue by default.

## Consequences

Positive:

- engineering begins
- architecture becomes testable
- CI and code structure become real
- future sprint planning can be implementation-driven
- MVP progress accelerates

Tradeoffs:

- some future features remain high-level
- domain docs may need refinement during implementation
- exact schemas will evolve as code is built

## Non-Goals

This ADR does not:

- remove any existing roadmap
- cancel future module work
- force all domains into MVP
- change the chosen stack
- authorize shortcuts around tenant isolation, audit, permissions, security, or configuration

## Final Rule

Build now.

The platform should move from architecture planning to executable foundation.
