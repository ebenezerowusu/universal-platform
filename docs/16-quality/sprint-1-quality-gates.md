# Sprint 1 Quality Gates

## Purpose

This document defines the quality gates for Sprint 1 implementation.

Sprint 1 must create a stable backend foundation before product features are introduced.

## Gate 1: Scope Control

The Sprint 1 PR must only include:

- backend workspace scaffold
- API shell
- runtime configuration foundation
- standard response model
- standard error model
- health endpoint
- first tests

It must not include Identity, Organization, Permission, Audit, Religious Domain, providers, or Flutter UI.

## Gate 2: Build Check

The backend should build successfully using the selected Rust workspace setup.

The PR summary should state the exact command used.

## Gate 3: Test Check

The first tests should run successfully.

Minimum expected tests:

- health endpoint response
- standard success response shape
- configuration behavior where practical

## Gate 4: Architecture Boundary Check

Reviewers should confirm:

- API layer is thin
- shared response and error types are reusable
- infrastructure concerns are not placed inside domain folders
- no domain-specific terms are added to Core

## Gate 5: Runtime Configuration Check

Reviewers should confirm:

- configuration is loaded consistently
- missing required runtime values fail clearly where applicable
- no real secrets are committed
- local examples use placeholders only

## Gate 6: Health Contract Check

The health endpoint should follow:

```text
docs/12-api/health-api-contract.md
```

## Gate 7: Documentation Check

The PR should update documentation only if implementation differs from the current documented direction.

If no docs update is needed, the PR summary should say so.

## Final Rule

Sprint 1 is successful only if the foundation is clean, tested, and intentionally small.
