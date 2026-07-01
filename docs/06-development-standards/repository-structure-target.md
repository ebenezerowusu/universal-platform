# Repository Structure Target

## Purpose

This document defines the target repository structure for Universal Platform implementation.

It gives developers and coding agents a shared direction before creating folders, crates, modules, and app structure.

## Principle

The repository structure should reflect the architecture.

Platform Core, Engines, Domains, Infrastructure, Flutter clients, and documentation must remain easy to locate and reason about.

## Target Root Structure

```text
universal-platform/
  backend/
  apps/
  docs/
  scripts/
  .github/
  README.md
  CLAUDE.md
```

## Backend Structure

```text
backend/
  Cargo.toml
  crates/
    api/
    shared/
    core/
    engines/
    domains/
    infrastructure/
  migrations/
  tests/
  README.md
```

## Backend Crate Direction

### api

Owns:

- Axum app setup
- route registration
- middleware wiring
- request extraction
- response conversion

Must not own:

- business rules
- direct provider calls
- raw database logic

### shared

Owns:

- common response types
- common error types
- shared identifiers
- shared utilities
- cross-cutting primitives

Must stay generic.

### core

Owns:

- Identity foundation
- Organization foundation
- Permission foundation
- Module Registry foundation
- Subscription foundation
- Audit foundation

Must remain domain-agnostic.

### engines

Owns reusable capability engines:

- payment
- sms
- storage
- notification
- reporting
- workflow
- localization
- configuration

Engines expose capability interfaces and application services.

### domains

Owns business domains:

- religious
- commerce later
- pos later
- finance later
- hr later

Domains own business meaning and call Core/Engines through defined boundaries.

### infrastructure

Owns implementations for:

- database access
- Redis
- provider adapters
- storage adapters
- configuration loading
- logging and tracing setup

Infrastructure should implement interfaces defined by Core, Engines, or Domains.

## Flutter Structure

```text
apps/
  flutter/
    pubspec.yaml
    lib/
      app/
      core/
      shared/
      features/
      modules/
      l10n/
    test/
    README.md
```

## Flutter Folder Direction

### app

Owns:

- app bootstrap
- root widget
- route setup
- shell setup

### core

Owns:

- API client foundation
- session state
- organization state
- configuration

### shared

Owns:

- design system
- reusable widgets
- common models
- helpers

### features

Owns cross-module user experiences such as login, organization selector, and dashboard shells.

### modules

Owns module-specific UI:

- religious
- commerce later
- pos later
- finance later
- hr later

## Docs Structure

Documentation remains under:

```text
docs/
```

Docs should continue to follow the existing numbered folder structure.

## Scripts Structure

```text
scripts/
  dev/
  db/
  deploy/
```

Scripts should be simple, documented, and safe for local development.

## GitHub Structure

```text
.github/
  ISSUE_TEMPLATE/
  pull_request_template.md
  workflows/
```

GitHub workflows should start simple and grow with implementation.

## Final Rule

Folder structure must support the architecture. Do not create shortcuts that make Core, Engines, Domains, Infrastructure, or Flutter responsibilities unclear.
