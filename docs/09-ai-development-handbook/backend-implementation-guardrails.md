# Backend Implementation Guardrails

## Purpose

This document gives coding agents and developers a final guardrail checklist before implementing backend tasks.

It complements `CLAUDE.md`, the Platform Constitution, and the development standards.

## Before Coding

Confirm:

- Which layer the task belongs to
- Which docs apply
- Which tables are affected
- Which APIs are affected
- Which permissions are required
- Which module checks are required
- Which tests are required

## Layer Decision

Classify the task as one of:

- API
- Core
- Engine
- Domain
- Infrastructure
- Shared
- Documentation

If the classification is unclear, document the assumption before coding.

## Core Guardrails

Do not add domain-specific concepts to Platform Core.

Core may know:

- users
- organizations
- modules
- permissions
- subscriptions
- settings
- audit

Core must not know:

- members
- pastors
- products
- staff
- orders
- pledges
- counseling cases

## Engine Guardrails

Engines provide reusable capabilities.

Domains should call engines for:

- payment
- SMS
- storage
- notifications
- reporting
- workflow
- audit
- localization

## Domain Guardrails

Domains own business meaning.

Religious Domain may know members, visitors, services, groups, attendance, giving, care, and events.

It must not process payment providers, SMS providers, or storage providers directly.

## API Guardrails

API handlers should be thin.

Handlers should not contain business workflow logic.

Use application services for use cases.

## Database Guardrails

For organization-owned data:

- include organization context
- use scoped queries
- add appropriate indexes
- write migrations
- add tests

## Security Guardrails

Check:

- authentication
- organization access
- permission
- module availability
- subscription or feature access
- audit requirement
- sensitive data handling

## Testing Guardrails

At minimum, consider tests for:

- success path
- validation failure
- permission failure
- selected organization context
- duplicate handling where relevant
- audit event where relevant

## Final Rule

When in doubt, choose the architecture-safe path, update documentation, and avoid shortcuts that make the platform harder to extend.
