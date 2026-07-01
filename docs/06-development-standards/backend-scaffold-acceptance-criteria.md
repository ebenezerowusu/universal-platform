# Backend Scaffold Acceptance Criteria

## Purpose

This document defines the acceptance criteria for the first backend scaffold.

The backend scaffold is complete only when it creates a stable foundation for future Core, Engine, and Domain work.

## Required Structure

The backend should include a clear workspace structure.

Expected areas:

```text
api
shared
core
engines
domains
infrastructure
migrations
tests
```

Exact Rust crate naming may evolve, but responsibilities must remain clear.

## Runtime Acceptance Criteria

The backend scaffold must:

- start locally
- load configuration
- initialize logging
- expose health route
- return standard response shape
- return standard error shape

## Configuration Acceptance Criteria

Configuration must:

- read from environment or local config source
- fail clearly when required values are missing
- avoid committing real secrets
- support local development setup

## API Acceptance Criteria

The API layer must:

- keep handlers thin
- register routes centrally
- support versioned routes
- use shared response types
- use shared error types

## Database Acceptance Criteria

Database foundation must:

- create connection pool
- support migrations
- support health check
- keep database implementation inside infrastructure boundary

## Testing Acceptance Criteria

Initial tests must cover:

- health endpoint
- response shape
- configuration behavior
- database foundation where practical

## Documentation Acceptance Criteria

Implementation summary must state:

- files created
- architecture layers used
- tests added
- tests run
- known limitations

## Out of Scope

The scaffold must not include:

- Religious Domain features
- payment provider integration
- SMS provider integration
- full identity flow
- Flutter UI

## Final Rule

A backend scaffold is successful when future features can build on it without restructuring the foundation immediately.
