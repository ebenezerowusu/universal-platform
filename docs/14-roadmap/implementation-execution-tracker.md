# Implementation Execution Tracker

## Purpose

This document provides a high-level tracker for moving Universal Platform from documentation into implementation.

It is not a project management replacement. It is a documentation-based execution map.

## Phase 0: Architecture Documentation

Status:

```text
in progress / mostly complete
```

Focus:

- vision
- constitution
- architecture
- domains
- engines
- API standards
- database standards
- security standards
- roadmap
- ADRs

## Phase 1: Backend Foundation

Primary docs:

- `docs/14-roadmap/18-first-sprint-plan.md`
- `docs/14-roadmap/sprint-1-issue-breakdown.md`
- `docs/06-development-standards/backend-scaffold-acceptance-criteria.md`

Expected result:

- Rust backend workspace
- Axum API shell
- configuration
- standard response/error format
- health endpoint
- database foundation
- first tests

## Phase 2: Identity and Organization

Primary docs:

- `docs/14-roadmap/20-second-sprint-plan.md`
- `docs/14-roadmap/sprint-2-issue-breakdown.md`
- `docs/06-development-standards/identity-organization-acceptance-criteria.md`

Expected result:

- Identity foundation
- login foundation
- current user endpoint
- organization foundation
- organization membership

## Phase 3: Permission and Audit

Primary docs:

- `docs/14-roadmap/21-third-sprint-plan.md`
- `docs/14-roadmap/sprint-3-issue-breakdown.md`
- `docs/06-development-standards/permission-audit-acceptance-criteria.md`

Expected result:

- Permission Engine foundation
- role foundation
- permission checks
- Audit Engine foundation

## Phase 4: Modules, Plans, and Engines

Primary docs:

- `docs/14-roadmap/22-fourth-sprint-plan.md`
- `docs/14-roadmap/sprint-4-module-registry-tasks.md`
- `docs/14-roadmap/sprint-4-plan-and-engine-tasks.md`

Expected result:

- Module Registry foundation
- plan and feature foundation
- shared engine interfaces
- local/mock engine implementations

## Phase 5: Religious MVP Foundation

Primary docs:

- `docs/14-roadmap/23-fifth-sprint-plan.md`
- `docs/14-roadmap/sprint-5-issue-breakdown.md`
- `docs/06-development-standards/religious-mvp-acceptance-criteria.md`

Expected result:

- Religious modules
- Religious settings
- members
- visitors
- congregations
- services
- groups
- attendance foundation
- basic reports

## Phase 6: Flutter Foundation

Primary docs:

- `docs/14-roadmap/sprint-6-flutter-foundation-plan.md`
- `docs/11-ui-ux/flutter-client-build-sequence.md`
- `docs/11-ui-ux/flutter-state-management-and-navigation.md`

Expected result:

- Flutter app shell
- shared design system
- API client foundation
- state foundation
- mobile shell
- desktop/admin shell

## Execution Rule

Implementation should proceed in phase order unless a documented decision explains why the order changed.

## Final Rule

This tracker should be updated when implementation begins, when phases complete, and when roadmap assumptions change.
