# ADR-XXX: <Decision Title>

## Status

Proposed | Accepted | Superseded | Deprecated

## Date

YYYY-MM-DD

## Context

Describe the real implementation situation that forced this decision.

Include:

- the feature or sprint being implemented
- the technical conflict or uncertainty
- the options considered
- the constraints that matter
- affected modules, engines, or domains

## Decision

State the decision clearly.

```text
<the decision>
```

## Options Considered

### Option 1: <Name>

Pros:

- 

Cons:

- 

### Option 2: <Name>

Pros:

- 

Cons:

- 

## Consequences

Positive:

- 

Tradeoffs:

- 

Risks:

- 

## Architecture Boundaries

Confirm:

- Platform Core remains domain-agnostic
- Tenant isolation is preserved
- Permission checks are not bypassed
- Audit requirements are preserved
- Provider logic stays behind adapters
- Configuration remains environment/tenant driven where applicable

## Implementation Impact

Affected files/modules:

```text
<list paths>
```

Affected migrations:

```text
<list migration files or none>
```

Affected API routes:

```text
<list routes or none>
```

## Follow-Up Work

- 

## Final Rule

This ADR exists because implementation revealed a real decision. Do not create ADRs for speculative architecture.
