# Implementation Epics

## Purpose

This document breaks the first development work into epics.

It is intended to guide Claude Code, human developers, and future project planning.

## Epic 1: Backend Foundation

Goal:

Create the Rust backend foundation.

Scope:

- Workspace structure
- Axum app shell
- Configuration loader
- Logging/tracing
- Health endpoint
- Standard response/error format
- Basic tests

## Epic 2: Database Foundation

Goal:

Prepare PostgreSQL and migrations.

Scope:

- Database connection
- Migration tooling
- Initial migration structure
- Repository pattern
- Transaction helper pattern
- Database health check

## Epic 3: Identity Engine Foundation

Goal:

Support platform user identity.

Scope:

- Users table
- Credentials
- Password hashing
- Login
- Token/session foundation
- Auth middleware

## Epic 4: Organization Engine Foundation

Goal:

Support tenants and organization membership.

Scope:

- Organizations
- Organization users
- Organization context resolver
- Organization settings foundation
- Membership checks

## Epic 5: Permission Engine Foundation

Goal:

Support authorization.

Scope:

- Permissions
- Roles
- Role permissions
- User role assignment
- Permission check service
- Scope foundation

## Epic 6: Audit Engine Foundation

Goal:

Record important platform actions.

Scope:

- Audit log table
- Audit service
- Actor context
- Organization context
- Basic audit queries

## Epic 7: Module Registry and Subscription Foundation

Goal:

Control module and plan access.

Scope:

- Modules
- Organization modules
- Plans
- Plan features
- Organization subscriptions
- Feature access checks

## Epic 8: Engine Skeletons

Goal:

Prepare shared capability interfaces.

Scope:

- Payment Engine interface
- SMS Engine interface
- Storage Engine interface
- Notification Engine interface
- Reporting Engine interface
- Workflow Engine interface

Use mock or local implementations first.

## Epic 9: Religious Domain Foundation

Goal:

Begin first domain on top of platform core.

Scope:

- Religious settings
- Member foundation
- Visitor foundation
- Congregation/service/group foundation
- Attendance foundation

## Epic 10: Flutter Shell Foundation

Goal:

Create client shell after backend contracts stabilize.

Scope:

- Shared design system
- Auth screens
- Organization selector
- Module-aware navigation
- Mobile shell
- Desktop/admin shell

## Epic Rules

- Do not start a later epic if required foundations are missing.
- Keep each epic reviewable.
- Each epic must include tests where applicable.
- Architecture docs must be updated when behavior changes.

## Final Rule

Build the platform in foundation-first epics, not scattered feature fragments.
