# Pull Request

## What changed

Describe the change clearly.

## Why it changed

Explain the reason for the change.

## Type of change

Choose all that apply:

- Documentation
- Backend foundation
- Platform Core
- Capability Engine
- Domain Module
- Flutter Client
- Infrastructure
- Database migration
- Test
- Refactor

## Architecture classification

State where this work belongs:

```text
API / Core / Engine / Domain / Infrastructure / Flutter / Docs
```

## Documents reviewed

List the key documents used before implementation.

## Files changed

List important files or folders changed.

## Database impact

State whether this PR adds or changes migrations, seed data, indexes, constraints, or data models.

## API impact

State whether this PR adds, removes, or changes API contracts.

## Permission/module impact

State whether permissions, modules, features, or plan checks are affected.

## Tests

List tests added or updated.

## Checks run

List commands or checks run locally or in CI.

## Risks and limitations

State known risks, limitations, or follow-up work.

## Architecture checklist

- Platform Core remains domain-agnostic.
- API handlers stay thin.
- Application services own use cases.
- Repositories own data access.
- Engines provide shared capabilities.
- Domains do not call providers directly.
- Organization-owned data uses organization context.
- Sensitive changes are auditable where needed.
- Documentation is updated where behavior changed.
