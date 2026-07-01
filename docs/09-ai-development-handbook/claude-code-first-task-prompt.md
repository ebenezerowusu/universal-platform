# Claude Code First Task Prompt

## Purpose

This document provides the first implementation prompt for Claude Code.

Use this prompt when the project is ready to move from documentation into backend scaffolding.

## Prompt

You are working on Universal Platform.

Before coding, read these files in order:

1. `CLAUDE.md`
2. `docs/00-vision/00-platform-vision.md`
3. `docs/01-platform-constitution/01-platform-constitution.md`
4. `docs/02-architecture/02-platform-architecture.md`
5. `docs/06-development-standards/rust-backend-structure.md`
6. `docs/06-development-standards/testing-strategy.md`
7. `docs/06-development-standards/definition-of-done.md`
8. `docs/14-roadmap/15-development-kickoff-plan.md`
9. `docs/14-roadmap/18-first-sprint-plan.md`

Your task is to create the first backend foundation only.

Scope:

- Create Rust backend workspace structure.
- Create Axum API shell.
- Add configuration foundation.
- Add health endpoint.
- Add standard success/error response models.
- Add database connection foundation.
- Add migration structure.
- Add basic tests for the foundation.

Do not implement product features.

Do not implement Religious Domain features.

Do not implement payment or SMS provider integrations.

Do not build Flutter UI.

Architecture rules:

- Keep Platform Core domain-agnostic.
- Keep API handlers thin.
- Keep infrastructure behind interfaces where practical.
- Use environment configuration.
- Follow standard response/error format.
- Add tests.

Before making changes, summarize:

- Classification of the task
- Files you plan to create
- Architecture rules that apply
- Testing plan

After making changes, summarize:

- Files changed
- Tests run
- Architecture rules followed
- Risks and follow-ups

## Expected Outcome

A running backend foundation that future Identity, Organization, Permission, Audit, Module Registry, and Subscription work can build on.

## Final Rule

Implement foundation only. No product/domain shortcuts.
