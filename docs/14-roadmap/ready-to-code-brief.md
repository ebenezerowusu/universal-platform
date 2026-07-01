# Ready-to-Code Brief

## Purpose

This document is the final short brief before Sprint 1 implementation begins.

It summarizes what is ready, what should happen first, and what must not be built yet.

## Current Status

Documentation Phase 0 is ready for implementation handoff.

The platform has documented:

- vision
- constitution
- architecture
- domains
- engines
- database strategy
- security strategy
- API standards
- Flutter direction
- roadmap
- sprint breakdowns
- coding-agent rules
- PR and issue templates

## First Implementation Target

Start with:

```text
Sprint 1: Backend Scaffold and Health Endpoint
```

Primary handoff document:

```text
docs/09-ai-development-handbook/first-implementation-task.md
```

## First PR Should Build

The first PR should build:

- backend workspace
- Axum API shell
- runtime configuration foundation
- standard response model
- standard error model
- health endpoint
- first tests

## First PR Must Not Build

The first PR must not build:

- Identity
- login
- organizations
- permissions
- audit
- modules
- subscriptions
- Religious Domain
- payment integration
- SMS integration
- Flutter UI

## First Branch Name

Recommended branch name:

```text
chore/backend-scaffold
```

## First PR Title

Recommended PR title:

```text
chore: initialize backend scaffold
```

## Required Review Focus

Reviewers should focus on:

- architecture boundaries
- clean backend structure
- thin API layer
- standard response and error model
- local run ability
- first tests
- no scope creep

## Handoff Rule

Give Claude Code the first implementation task document and require the PR template summary.

## Final Rule

The documentation phase is complete enough. Start coding with Sprint 1 and keep the first PR small.
