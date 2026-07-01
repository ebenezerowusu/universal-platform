# Third Sprint Plan

## Purpose

This document defines the recommended third sprint after Identity and Organization foundations.

The third sprint should implement Permission and Audit foundations.

## Sprint Goal

Create the first authorization and audit capabilities required before product domain work begins.

## Prerequisites

Before this sprint begins:

- Backend foundation exists.
- Identity foundation works.
- Organization foundation works.
- Authenticated routes can identify the current user.
- Organization routes verify membership.

## Sprint Scope

### 1. Permission Tables

Create migrations for:

- permissions
- roles
- role permissions
- organization user roles

Expected result:

- Permission and role data can be stored.

### 2. Permission Seed Data

Seed the first permission keys from the initial permission catalog.

Expected result:

- Required platform and organization permission keys exist.

### 3. Permission Check Service

Implement the first permission check service.

Expected result:

- Application services can ask whether a user has a required permission.

### 4. Route Authorization Integration

Apply permission checks to selected organization routes.

Expected result:

- User without permission receives standard error response.
- User with permission can continue.

### 5. Audit Table

Create migration for audit logs.

Expected result:

- Important actions can be recorded.

### 6. Audit Service

Implement basic audit write service.

Expected result:

- Application services can record important actions with actor and organization context.

### 7. Tests

Create tests for:

- permission allow
- permission deny
- role assignment
- audit write
- protected route behavior

## Out of Scope

Do not implement:

- Religious Domain features
- full module registry
- full subscription checks
- payment provider integration
- SMS provider integration
- Flutter UI

## Completion Standard

The sprint is complete when:

- Permissions can be seeded.
- Roles can be assigned.
- Protected routes enforce permission checks.
- Audit records can be written.
- Tests cover allow and deny flows.

## Final Rule

Sprint 3 proves authorization and audit. Domain features should wait until these are reliable.
