# Sprint 2 Issue Breakdown

## Purpose

This document breaks Sprint 2 into small implementation issues.

Sprint 2 should implement Identity and Organization foundations after the backend scaffold is working.

## Issue 1: Create Identity Migrations

### Goal

Create the first Identity database tables.

### Scope

- users
- user credential/session records as selected during implementation
- required indexes
- basic lifecycle fields

### Acceptance Criteria

- Identity migrations apply successfully.
- User records can be stored.
- Credential/session records can be stored according to the selected implementation pattern.
- No organization or domain-specific behavior is mixed into Identity tables.

## Issue 2: Implement User Repository

### Goal

Create data access for user records.

### Scope

- create user
- find user by id
- find user by login identifier where needed
- update user status where needed

### Acceptance Criteria

- Repository methods are focused and testable.
- Repository does not contain route logic.
- Repository does not contain domain behavior.

## Issue 3: Implement Login Foundation

### Goal

Create the first login use case.

### Scope

- validate login request
- check account state
- verify credential using Identity service
- return safe session response

### Acceptance Criteria

- Valid login flow works.
- Invalid login flow returns safe response.
- No sensitive credential values are logged or returned.

## Issue 4: Implement Current User Endpoint

### Goal

Allow clients to retrieve the current user summary.

### Scope

- authentication context extractor
- current user application service
- current user route

### Acceptance Criteria

- Route returns safe user summary.
- Route uses standard response format.
- Route does not expose internal credential/session fields.

## Issue 5: Create Organization Migrations

### Goal

Create the first Organization database tables.

### Scope

- organizations
- organization_users
- organization_settings foundation
- required indexes

### Acceptance Criteria

- Organization records can be stored.
- User membership can be stored.
- Settings foundation exists.

## Issue 6: Implement Organization Repository

### Goal

Create organization data access.

### Scope

- create organization
- add organization user
- list organizations for user
- find organization by id
- read basic settings where needed

### Acceptance Criteria

- Organization data access is centralized.
- Organization context is handled consistently.

## Issue 7: Implement Organization Use Cases

### Goal

Create first organization application services.

### Scope

- create organization
- list current user organizations
- get organization detail
- update organization foundation

### Acceptance Criteria

- Use cases follow application service pattern.
- User membership is created when organization owner creates organization.
- Responses follow API standards.

## Issue 8: Add Sprint 2 Tests

### Goal

Cover Identity and Organization foundation behavior.

### Scope

- login success
- login failure
- current user
- create organization
- list organizations
- organization detail by selected context

### Acceptance Criteria

- Tests cover main success and failure paths.
- Tests are documented in the implementation summary.

## Final Rule

Sprint 2 proves Identity and Organization. Do not add Permissions, Religious Domain, payment, SMS, or Flutter UI in this sprint.
