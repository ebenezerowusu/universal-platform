# Platform Core Implementation Sequence

## Purpose

This document defines the recommended implementation order for Platform Core after the first backend foundation sprint.

It should guide the next development tasks after the backend can run, load configuration, connect to the database, and return standard responses.

## Sequence Principle

Build the minimum platform capabilities in dependency order.

Do not jump into domain features before Core dependencies are ready.

## Step 1: Identity Foundation

Implement:

- users
- user credentials/session records
- login use case
- current user endpoint
- authentication middleware

Definition of done:

- User can authenticate.
- Protected route can identify current user.
- Tests cover success and failure cases.

## Step 2: Organization Foundation

Implement:

- organizations
- organization users
- organization list for current user
- organization context resolver
- membership checks

Definition of done:

- User can belong to organization.
- Organization-scoped route verifies membership.
- Tenant boundary tests exist.

## Step 3: Permission Foundation

Implement:

- permissions
- roles
- role permissions
- organization user roles
- permission check service
- basic scope fields

Definition of done:

- Protected route checks permission.
- User without permission is denied.
- User with permission is allowed.

## Step 4: Audit Foundation

Implement:

- audit logs
- audit service
- actor context
- organization context
- audit query endpoint later

Definition of done:

- Important action can write audit record.
- Audit record includes actor and organization context where available.

## Step 5: Module Registry Foundation

Implement:

- modules
- organization modules
- module installation status
- module availability check
- module metadata endpoint

Definition of done:

- Organization can have module installed/active.
- Route can require active module.

## Step 6: Subscription Foundation

Implement:

- plans
- plan features
- plan limits
- organization subscription
- feature check service

Definition of done:

- Route or use case can check feature availability.
- Basic limit check pattern exists.

## Step 7: Engine Interface Skeletons

Implement interfaces for:

- Payment
- SMS
- Storage
- Notification
- Reporting
- Workflow

Definition of done:

- Domains can depend on interfaces.
- Mock/local implementations exist where useful.
- No external provider integration is required yet.

## Step 8: Core Seed Data

Implement safe seed data for:

- core modules
- engine modules
- permissions
- default subscription plans
- default role templates later

Definition of done:

- Seed can run repeatedly without duplicates.
- Required module and permission records exist.

## Step 9: Religious Domain Enablement

Only after Core checks are working, begin Religious Domain foundation.

Start with:

- religious settings
- member foundation
- visitor foundation
- congregation/service/group foundation
- attendance foundation

## Final Rule

Core capabilities must be usable before domain features depend on them.
