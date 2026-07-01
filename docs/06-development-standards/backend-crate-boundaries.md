# Backend Crate Boundaries

## Purpose

This document defines recommended crate boundaries for the Rust backend.

The goal is to keep the modular monolith clean, testable, and ready for future extraction if needed.

## Boundary Principle

A crate should have a clear reason to exist.

Avoid a large unstructured backend where every module can import everything else freely.

## Recommended Workspace Shape

```text
backend/
  crates/
    api/
    shared/
    core/
    infrastructure/
    engines/
    domains/
```

## api Crate

Responsible for:

- Axum application setup
- Route registration
- Middleware wiring
- Request extraction
- Response conversion

Must not contain:

- Domain business rules
- SQL queries
- Provider-specific integrations
- Permission logic implementation

## shared Crate

Responsible for:

- Shared error types
- API response types
- Common ID types
- Common time helpers
- Validation helpers
- Trace metadata types

Must not contain:

- Domain business rules
- Provider-specific logic
- Database access logic

## core Crate

Responsible for:

- Identity foundation
- Organization foundation
- Permission foundation
- Module Registry foundation
- Subscription foundation
- Shared platform application services

Must remain domain-agnostic.

It must not contain concepts such as member, pastor, product, order, staff, or pledge.

## engines Crate

Responsible for shared capability engines such as:

- Audit
- Payment
- SMS
- Notification
- Storage
- Reporting
- Workflow
- Import/Export
- Localization

Engines provide reusable capabilities to Core and Domains.

## domains Crate

Responsible for business domains such as:

- Religious
- Commerce
- POS
- Finance
- HR

Domains own business language and workflows.

Domains must use Core and Engines through clear interfaces.

## infrastructure Crate

Responsible for:

- Database implementations
- Redis implementations
- Provider adapters
- Storage adapters
- External clients
- Configuration loading

Infrastructure implements interfaces. It must not own business decisions.

## Dependency Direction

Preferred direction:

```text
api -> core/domains/engines -> shared
infrastructure -> interfaces/contracts
```

Business logic should not depend directly on concrete provider clients.

## Review Rule

When adding a new file, ask:

- Is this API boundary code?
- Is this shared utility code?
- Is this Core platform logic?
- Is this reusable engine logic?
- Is this domain business logic?
- Is this infrastructure implementation?

## Final Rule

Clear crate boundaries protect the architecture more than folder names alone.
