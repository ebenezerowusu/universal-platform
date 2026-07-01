# CI/CD Standards

## Purpose

This document defines continuous integration and deployment standards for Universal Platform.

CI/CD should protect code quality, architecture, tests, and safe deployment.

## CI Principle

Every pull request should prove that the project still builds, formats, and passes required checks.

## Early CI Checks

Initial backend checks should include:

- Rust format check
- Rust lint check
- Unit tests
- Integration tests where practical
- Migration validation where practical

Initial Flutter checks later should include:

- Format check
- Static analysis
- Unit/widget tests where practical

## Documentation Checks

Documentation PRs may not need full application checks at the beginning, but they should still be reviewed for architecture consistency.

## Migration Checks

Database migrations should be validated before merge when practical.

Important checks:

- Migration applies successfully
- Migration order is correct
- No obvious tenant-scope violation

## Security-Oriented Checks

Future checks may include:

- Secret scanning
- Dependency audit
- Static analysis
- Container scanning

## Deployment Direction

Deployment should be predictable and repeatable.

Recommended high-level flow:

```text
Build
Test
Package
Deploy to staging
Validate
Deploy to production
```

For the MVP, deployment may remain manual or semi-automated until the process is stable.

## Release Gates

Before production deployment:

- Tests pass
- Migration impact reviewed
- Environment configuration reviewed
- Backup status understood
- Rollback/recovery path considered

## AI Agent Rule

AI-generated code must not bypass CI expectations.

If an AI agent cannot run a check, it must state that clearly in the PR summary.

## Final Rule

CI/CD exists to protect the platform. Do not treat failing checks as optional noise.
