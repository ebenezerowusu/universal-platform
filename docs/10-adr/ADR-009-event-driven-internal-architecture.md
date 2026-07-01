# ADR-009: Use Event-Driven Internal Architecture

## Status

Accepted

## Context

Universal Platform will contain many engines and domains.

Direct dependencies between all modules would create tight coupling and make future expansion difficult.

Examples of cross-cutting reactions:

- Payment completion may trigger notification, reporting, and domain updates.
- Member creation may trigger workflow, timeline, and reporting updates.
- Module installation may trigger permission and navigation updates.

## Decision

Use an internal event-driven architecture for important platform and domain actions.

Events should describe facts that already happened.

Examples:

- PaymentCompleted
- MemberCreated
- ModuleInstalled
- SmsSent
- ReportExportCompleted

## Consequences

### Positive

- Reduced module coupling
- Easier background processing
- Better extensibility
- Easier future service extraction
- Clear business activity history

### Cost

- Requires event naming discipline
- Requires idempotency thinking
- Requires event consumer testing
- Adds complexity compared to direct calls

## Implementation Direction

Start simple.

The MVP may use in-process events and Redis-backed jobs where needed.

Keep event publishers and consumers behind interfaces so the implementation can evolve.

## Final Rule

Use events to communicate important facts across modules without creating direct dependencies.
