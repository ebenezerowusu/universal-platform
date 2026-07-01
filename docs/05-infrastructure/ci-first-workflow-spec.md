# CI First Workflow Spec

## Purpose

This document defines the first CI workflow target for Universal Platform implementation.

The first CI workflow should protect the backend scaffold without becoming overly complex.

## Principle

Start with simple checks that can run consistently.

Add service-backed integration checks after the backend and database setup mature.

## First Workflow Target

Recommended workflow name:

```text
Backend CI
```

Recommended file path later:

```text
.github/workflows/backend-ci.yml
```

## Trigger Direction

Run on:

- pull requests to main
- pushes to main

## Sprint 1 Checks

The first workflow should run:

- Rust formatting check
- Rust lint check where available
- Rust build check
- Rust tests

## Working Directory

The workflow should run from:

```text
backend/
```

unless implementation chooses a different backend structure and documents the reason.

## Environment Direction

Sprint 1 should avoid requiring live database or Redis services for basic checks.

Database-backed CI checks can be added when migrations and repositories are implemented.

## Future Checks

Add later:

- PostgreSQL service
- Redis service
- migration check
- integration tests
- container build
- Flutter checks
- documentation link checks

## Failure Expectations

If CI fails, the PR should not merge until the failure is understood.

If a check is not yet available, the PR should explain what was run locally instead.

## Final Rule

The first CI workflow should be small, fast, and reliable. Expand it as implementation grows.
