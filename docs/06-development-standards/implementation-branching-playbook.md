# Implementation Branching Playbook

## Purpose

This document defines the branching playbook for early Universal Platform implementation.

It helps keep changes small, reviewable, and aligned with the sprint plan.

## Branch Principle

One branch should represent one focused implementation task or closely related task group.

Avoid large branches that mix architecture, backend, database, Flutter, and documentation changes without a clear reason.

## Branch Naming Pattern

Recommended patterns:

```text
chore/backend-scaffold
feature/identity-foundation
feature/organization-foundation
feature/permission-foundation
feature/religious-members
fix/health-response-shape
docs/api-contract-update
```

## First Branch

Use:

```text
chore/backend-scaffold
```

Purpose:

```text
Initialize backend scaffold and health endpoint.
```

## Branch Rules

A branch should define:

- task objective
- sprint or phase
- expected files/folders
- tests expected
- docs expected

## Commit Rules

Commits should be clear and purposeful.

Examples:

```text
chore: create backend workspace
chore: add standard response types
chore: add health endpoint
test: add health endpoint tests
docs: update local backend notes
```

## PR Rules

Each branch should become a PR when:

- scope is complete
- tests have been run where possible
- PR template is filled
- known limitations are documented

## Avoid

Avoid branches that:

- mix unrelated features
- include broad refactors without need
- introduce domain features during foundation sprints
- skip tests without explanation
- change docs and code inconsistently

## Final Rule

Branches are delivery units. Keep them small enough to review and safe enough to merge.
