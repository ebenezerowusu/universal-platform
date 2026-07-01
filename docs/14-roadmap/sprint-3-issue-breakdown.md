# Sprint 3 Issue Breakdown

## Purpose

This document breaks Sprint 3 into implementation issues.

Sprint 3 should implement Permission and Audit foundations after Identity and Organization are working.

## Issue 1: Create Permission Migrations

### Goal

Create the database tables required for the Permission Engine foundation.

### Scope

- permissions
- roles
- role_permissions
- organization_user_roles
- required indexes

### Acceptance Criteria

- Migrations apply successfully.
- Permission and role data can be stored.
- Role assignment can be connected to organization users.

## Issue 2: Seed Initial Permission Keys

### Goal

Seed initial permission keys from the permission catalog.

### Scope

- platform permission keys
- organization permission keys
- module permission keys
- Religious MVP permission keys where planned

### Acceptance Criteria

- Seed can run repeatedly.
- Required permission keys exist.
- Duplicate permissions are not created.

## Issue 3: Implement Permission Repository

### Goal

Create data access for permission and role records.

### Scope

- find permission by key
- create or sync permissions
- assign permission to role
- assign role to organization user
- list roles for organization

### Acceptance Criteria

- Repository methods are focused.
- Organization-specific role access is scoped.
- Repository does not contain route behavior.

## Issue 4: Implement Permission Check Service

### Goal

Create the service that answers whether a user can perform an action.

### Scope

- check user organization membership
- check assigned roles
- check role permissions
- return allow or deny result
- prepare future scope checks

### Acceptance Criteria

- Permission check can be called from application services.
- Allow and deny results are deterministic.
- Tests cover both outcomes.

## Issue 5: Apply Permission Checks to Organization Routes

### Goal

Connect permission checks to selected organization use cases.

### Scope

- update organization
- invite/list users foundation where implemented
- role management foundation where implemented

### Acceptance Criteria

- Use cases ask Permission Engine before sensitive action.
- Standard error response is returned when action is not allowed.

## Issue 6: Create Audit Migration

### Goal

Create audit log table.

### Scope

- audit_logs
- common audit fields
- indexes for organization and created date

### Acceptance Criteria

- Audit records can be stored.
- Audit query foundation can be added later.

## Issue 7: Implement Audit Service

### Goal

Create a reusable service for writing audit records.

### Scope

- record action
- actor context
- organization context
- entity type/id
- metadata

### Acceptance Criteria

- Application services can write audit records.
- Sensitive values are not required in audit metadata.
- Tests cover audit write behavior.

## Issue 8: Add Sprint 3 Tests

### Goal

Cover Permission and Audit foundation behavior.

### Scope

- permission allow
- permission deny
- role assignment
- seeded permission exists
- audit record write
- protected use case audit where applicable

### Acceptance Criteria

- Tests pass.
- Tests show authorization and audit foundations are usable.

## Final Rule

Sprint 3 proves authorization and audit. Do not add domain features until these foundations are working.
