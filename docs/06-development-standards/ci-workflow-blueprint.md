# CI Workflow Blueprint

## Purpose

This document defines the first CI workflow blueprint for Universal Platform.

The goal is to protect the backend foundation before implementation grows.

## Principle

CI should start simple and become stronger over time.

Do not wait for a large codebase before adding quality checks.

## First Backend Checks

Initial checks should include:

- Rust formatting check
- Rust lint check
- Rust tests
- Build check

## Future Backend Checks

Later checks may include:

- migration validation
- dependency audit
- security scanning
- container build
- integration tests with PostgreSQL and Redis

## Flutter Checks Later

When Flutter implementation begins, add:

- Flutter format check
- Flutter analyze
- unit tests
- widget tests where useful

## Documentation Checks

Documentation changes should still be reviewed for architecture consistency.

Optional future checks:

- Markdown lint
- broken link check
- docs index check

## Workflow Triggers

Recommended triggers:

- pull request
- push to main

## CI Result Expectations

A PR should not be merged if required checks fail.

If a check is not yet implemented, the PR summary should state what was not run.

## AI Agent Rule

When Claude Code opens or prepares a PR, it must report:

- commands/checks it ran
- checks it could not run
- failures it encountered
- assumptions made

## Final Rule

CI is a safety net. Keep it meaningful and improve it as the codebase grows.
