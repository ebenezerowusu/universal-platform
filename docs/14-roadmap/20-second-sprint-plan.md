# Second Sprint Plan

## Purpose

This document defines the recommended second sprint after the backend foundation sprint.

The second sprint should begin Platform Core capabilities, not product features.

## Sprint Goal

Implement the first usable Platform Core foundations: Identity and Organization.

## Prerequisites

Before this sprint begins:

- Backend workspace exists.
- API shell runs.
- Health endpoint works.
- Configuration loads.
- Database connection exists.
- Migration structure exists.
- Standard response/error format exists.

## Sprint Scope

### 1. Identity Tables

Create migrations for:

- users
- user credential/session records as selected during implementation

Expected result:

- User identity data can be stored.

### 2. Identity Use Cases

Implement:

- create user foundation
- login foundation
- current user lookup
- authenticated route guard

Expected result:

- Backend can identify an authenticated user.

### 3. Organization Tables

Create migrations for:

- organizations
- organization_users
- organization_settings foundation

Expected result:

- Organization records and user membership can be stored.

### 4. Organization Use Cases

Implement:

- create organization
- list current user's organizations
- get organization
- organization membership check

Expected result:

- User can belong to an organization.
- Organization context can be verified.

### 5. API Contracts

Implement initial APIs from:

- auth-organization API contracts
- initial route map

Expected result:

- Routes follow shared response and error standards.

### 6. Tests

Create tests for:

- login success/failure
- current user endpoint
- organization creation
- organization membership
- organization boundary failure

## Out of Scope

Do not implement:

- full role/permission engine
- Religious Domain features
- Flutter UI
- payment provider integration
- SMS provider integration

## Completion Standard

The sprint is complete when:

- Identity foundation works.
- Organization foundation works.
- Authenticated routes can identify current user.
- Organization routes verify membership.
- Tenant boundary tests exist.

## Final Rule

Sprint 2 proves identity and organization context. Permission Engine comes next.
