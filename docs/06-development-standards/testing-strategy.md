# Testing Strategy

## Purpose

This document defines the testing strategy for Universal Platform.

Testing is required to protect the platform's architecture, tenant isolation, permissions, security, and business correctness.

## Testing Principle

A feature is not complete until it is tested at the correct levels.

The most important early tests are tenant isolation and permission tests.

## Test Types

### Unit Tests

Used for:

- Domain logic
- Core logic
- Engine logic
- Validation rules
- Pure functions

### Application Service Tests

Used for:

- Use cases
- Permission-aware actions
- Tenant-aware workflows
- Engine coordination

### Repository Tests

Used for:

- Database queries
- Tenant filtering
- Constraints
- Index-supported query patterns

### API Tests

Used for:

- Route behavior
- Response format
- Error format
- Authentication flow
- Authorization flow

### Integration Tests

Used for:

- Database connection
- Migration behavior
- Engine and repository interaction
- Provider adapter mocks

## Mandatory Early Test Areas

### Health and Runtime

- Health endpoint responds.
- Configuration loads correctly.
- Database connection works.

### Identity

- User can be created.
- Login succeeds with valid credentials.
- Login fails with invalid credentials.
- Inactive user cannot access protected routes.

### Organization / Tenant Isolation

Test pattern:

1. Create Organization A.
2. Create Organization B.
3. Create user in Organization A.
4. Create data in Organization B.
5. Attempt access as Organization A user.
6. Verify access is denied or data is not returned.

### Permissions

- User with permission succeeds.
- User without permission fails.
- User with wrong scope fails.
- Platform permission is not treated as organization permission.

### Modules and Subscriptions

- Disabled module blocks feature access.
- Missing module blocks feature access.
- Plan without feature blocks access.
- Plan limit prevents new usage.

### Audit

- Sensitive action creates audit log.
- Audit log includes actor.
- Audit log includes organization context.

## Provider Adapter Tests

External providers should be tested through mocks or sandbox adapters.

Do not make tests depend on real SMS or payment provider accounts.

## Test Data Rules

Test data should be explicit and isolated.

Avoid tests that depend on hidden global state.

Use clear organization names and user roles in tests.

## Continuous Integration Direction

CI should eventually run:

- Formatting checks
- Lint checks
- Unit tests
- Integration tests where practical
- Migration checks

## Final Rule

If a feature can leak data across tenants, bypass permissions, or break core architecture, it must have tests proving it does not.
