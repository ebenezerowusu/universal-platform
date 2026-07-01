# Production Smoke Test Checklist

## Purpose

This document defines the smoke test checklist for MVP rollout environments.

Smoke tests confirm that the most important platform paths work after setup, deployment, or rollout changes.

## Principle

Smoke tests should be quick, repeatable, and focused on core confidence.

They do not replace full regression testing.

## Backend Smoke Tests

Confirm:

- backend health endpoint responds
- standard response shape is returned
- application logs are available
- database connection is healthy
- migrations are applied

## Authentication Smoke Tests

Confirm:

- known test user can log in
- invalid login fails safely
- current user endpoint works
- session expiration behavior is safe where implemented

## Organization Smoke Tests

Confirm:

- organization list loads
- selected organization is correct
- organization details load
- user does not see unrelated organization data

## Module and Feature Smoke Tests

Confirm:

- Religious module is active for the organization
- unavailable module state works where applicable
- starter features are available as expected

## Religious Smoke Tests

Confirm:

- Religious dashboard loads
- member list loads
- member create works
- visitor list loads
- visitor create works
- congregation/service/group list or placeholder works
- attendance session list loads
- manual attendance marking works where implemented

## Flutter Smoke Tests

Confirm:

- app starts
- login screen loads
- organization selector loads
- selected organization state persists where expected
- Religious navigation appears when available
- loading, empty, and error states are usable

## Support Smoke Tests

Confirm:

- support channel is known
- issue reporting fields are clear
- support owner is assigned
- known limitations are available to support users

## Result Format

Use:

```text
passed
failed
blocked
not applicable
```

Each failed or blocked item should include:

- issue summary
- affected organization
- severity
- owner
- next action

## Final Rule

Do not onboard more organizations when smoke tests are failing on core workflows.
