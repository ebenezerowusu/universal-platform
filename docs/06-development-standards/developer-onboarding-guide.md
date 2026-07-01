# Developer Onboarding Guide

## Purpose

This guide helps new developers and coding agents understand how to start working on Universal Platform.

It should be read before implementation work begins.

## First Reading Order

Read these files first:

1. `README.md`
2. `CLAUDE.md`
3. `docs/00-vision/00-platform-vision.md`
4. `docs/01-platform-constitution/01-platform-constitution.md`
5. `docs/02-architecture/02-platform-architecture.md`
6. `docs/06-development-standards/backend-crate-boundaries.md`
7. `docs/06-development-standards/application-service-pattern.md`
8. `docs/06-development-standards/repository-pattern.md`
9. Relevant sprint plan
10. Relevant engine or domain document

## Project Principles

Remember:

- Platform Core is domain-agnostic.
- Domains own business meaning.
- Engines provide reusable capabilities.
- Infrastructure implements adapters.
- API handlers stay thin.
- Application services execute use cases.
- Repositories protect data access.

## Before Starting a Task

Write down:

- task classification
- docs read
- files expected to change
- data model impact
- API impact
- permission impact
- test plan

## During Implementation

Keep work small and reviewable.

Avoid adding unrelated improvements to the same change.

Do not create provider-specific code inside domains.

Do not add domain-specific terms to Platform Core.

## Before Finishing a Task

Confirm:

- code builds
- relevant tests pass
- docs are updated
- architecture boundaries are respected
- no secrets are committed
- implementation summary is prepared

## PR Summary Format

Use this structure:

```text
What changed:
Why it changed:
Files changed:
Tests run:
Architecture rules followed:
Known limitations:
Follow-up work:
```

## Final Rule

Understand the architecture before writing code. The platform is designed to expand across many domains, not just one MVP.
