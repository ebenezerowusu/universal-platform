# Sprint 1 PR Checklist

## Purpose

This document defines the checklist for the first implementation pull request.

The first PR should create the backend foundation only. It should prove project discipline before feature development begins.

## PR Name

Recommended title:

```text
chore: initialize backend scaffold
```

## Required Reading

Before opening the PR, read:

- `CLAUDE.md`
- `docs/09-ai-development-handbook/first-implementation-task.md`
- `docs/14-roadmap/sprint-1-issue-breakdown.md`
- `docs/06-development-standards/backend-scaffold-acceptance-criteria.md`
- `docs/06-development-standards/repository-structure-target.md`

## Scope Checklist

The PR should include:

- [ ] backend workspace folder
- [ ] Rust workspace or crate structure
- [ ] Axum application shell
- [ ] configuration foundation
- [ ] standard response model
- [ ] standard error model
- [ ] health endpoint
- [ ] first tests
- [ ] local setup notes if needed

## Out of Scope Checklist

The PR should not include:

- [ ] Identity implementation
- [ ] login flow
- [ ] organization implementation
- [ ] permission engine implementation
- [ ] audit implementation
- [ ] Religious Domain implementation
- [ ] payment provider integration
- [ ] SMS provider integration
- [ ] Flutter UI

## Review Checklist

Reviewers should confirm:

- [ ] API handlers remain thin
- [ ] no domain feature was introduced
- [ ] no provider-specific logic was added
- [ ] standard response format exists
- [ ] standard error format exists
- [ ] health endpoint works
- [ ] configuration values are not hardcoded unsafely
- [ ] tests exist and are documented

## Completion Criteria

The first PR can be considered complete when:

- project builds
- tests pass
- health route returns expected response
- code structure matches documented architecture direction
- PR summary uses the repository PR template

## Final Rule

The first PR is a foundation PR. Keep it small, clean, and reviewable.
