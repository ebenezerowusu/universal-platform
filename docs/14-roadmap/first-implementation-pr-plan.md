# First Implementation PR Plan

## Purpose

This document defines the recommended first implementation pull request.

The first PR should be intentionally small and should prove the project can move from documentation into working backend foundation.

## PR Goal

Create the initial backend scaffold and health endpoint.

## PR Scope

The first PR should include:

- backend workspace structure
- initial API shell
- configuration foundation
- standard response/error foundation
- health endpoint
- first tests
- local development notes if needed

## PR Out of Scope

Do not include:

- Identity implementation
- Organization implementation
- Permission Engine
- Religious Domain
- Payment provider integration
- SMS provider integration
- Flutter UI

## Required Reading Before PR

The implementer must read:

- `CLAUDE.md`
- `docs/01-platform-constitution/01-platform-constitution.md`
- `docs/06-development-standards/backend-crate-boundaries.md`
- `docs/06-development-standards/application-service-pattern.md`
- `docs/06-development-standards/rust-error-handling-standards.md`
- `docs/14-roadmap/18-first-sprint-plan.md`
- `docs/14-roadmap/sprint-1-issue-breakdown.md`

## PR Description Must Include

- What changed
- Why it changed
- Files created
- Tests added
- Tests run
- Architecture rules followed
- Known limitations

## Review Checklist

Reviewers should verify:

- No domain feature was added.
- API handlers are thin.
- Response and error format follows docs.
- Configuration does not include real secrets.
- Tests exist for health and response behavior.
- The structure supports future Core and Engine work.

## Merge Criteria

The PR may be merged when:

- It builds.
- Tests pass.
- Architecture boundaries are respected.
- Review confirms no scope creep.

## Final Rule

The first PR should prove discipline, not feature volume.
