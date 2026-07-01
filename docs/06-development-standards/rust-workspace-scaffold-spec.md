# Rust Workspace Scaffold Spec

## Purpose

This document defines the first Rust workspace scaffold target for Universal Platform.

It is a practical specification for Sprint 1 implementation.

## Workspace Location

The backend should live under:

```text
backend/
```

## Target Structure

```text
backend/
  Cargo.toml
  crates/
    api/
      Cargo.toml
      src/
        main.rs
        routes/
        state.rs
    shared/
      Cargo.toml
      src/
        lib.rs
        response.rs
        error.rs
    core/
      Cargo.toml
      src/
        lib.rs
    engines/
      Cargo.toml
      src/
        lib.rs
    domains/
      Cargo.toml
      src/
        lib.rs
    infrastructure/
      Cargo.toml
      src/
        lib.rs
        config.rs
  migrations/
  tests/
  README.md
```

## Crate Responsibilities

### api

Owns:

- Axum startup
- route registration
- HTTP-specific extraction
- response conversion

### shared

Owns:

- standard API response model
- standard API error model
- shared result types
- shared identifiers where needed

### core

Placeholder for future Platform Core foundations.

In Sprint 1, this crate may remain mostly empty.

### engines

Placeholder for future capability engines.

In Sprint 1, this crate may remain mostly empty.

### domains

Placeholder for future domain modules.

In Sprint 1, this crate must not implement Religious or other domain features.

### infrastructure

Owns:

- runtime configuration foundation
- future database setup
- future Redis setup
- future tracing/logging setup

## Sprint 1 Required Code

Sprint 1 should include:

- application startup
- configuration loading foundation
- standard response type
- standard error type
- health route
- first tests

## Sprint 1 Optional Code

Sprint 1 may include:

- simple app state struct
- basic tracing setup
- example local configuration documentation

## Sprint 1 Forbidden Code

Sprint 1 must not include:

- login
- user persistence
- organization persistence
- permission checks
- domain models
- provider integrations
- Flutter code

## Naming Direction

Use clear names that match documentation.

Avoid temporary names that conflict with architecture terms.

## Final Rule

The Rust workspace should make future architecture easy. Do not overbuild features, but do create clean boundaries from the beginning.
