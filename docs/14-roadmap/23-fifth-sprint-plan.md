# Fifth Sprint Plan

## Purpose

This document defines the recommended fifth sprint after Module Registry, Subscription foundation, and engine skeletons.

The fifth sprint should begin the Religious Domain foundation.

## Sprint Goal

Implement the first usable Religious Domain foundation on top of Platform Core and shared engines.

## Prerequisites

Before this sprint begins:

- Identity foundation works.
- Organization foundation works.
- Permission foundation works.
- Audit foundation works.
- Module Registry foundation works.
- Subscription or feature availability check exists.
- Storage, Notification, Reporting, Payment, and SMS engine skeletons exist.

## Sprint Scope

### 1. Religious Module Registration

Seed/register:

- religious.members
- religious.visitors
- religious.structure
- religious.attendance
- religious.reports

Expected result:

- Organization can install and activate Religious MVP modules.

### 2. Religious Settings Foundation

Implement Religious settings table and service.

Expected result:

- Organization can store basic Religious Domain settings.

### 3. Member Foundation

Implement:

- member migration
- member repository
- create member use case
- list member use case
- member detail use case
- member update use case
- status history foundation

### 4. Structure Foundation

Implement:

- congregations
- services
- groups
- group membership foundation

### 5. API Routes

Implement MVP routes from Religious MVP API contracts.

Expected result:

- Routes follow standard API response and error format.
- Routes use organization context and permission checks.

### 6. Tests

Create tests for:

- create member
- list members
- create congregation
- create service
- create group
- module inactive behavior
- permission allow/deny behavior
- scoped query behavior

## Out of Scope

Do not implement yet:

- payment provider integration
- SMS provider integration
- advanced attendance methods
- full giving module
- family/household module
- care/counseling module
- Flutter UI

## Completion Standard

The sprint is complete when:

- Religious MVP modules are registered.
- Religious settings exist.
- Members can be created and listed.
- Congregations, services, and groups can be created.
- APIs are protected by Core checks.
- Tests cover main success and denial flows.

## Final Rule

Sprint 5 begins the first domain, but it must prove the platform foundation rather than bypass it.
