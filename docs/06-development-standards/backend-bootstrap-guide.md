# Backend Bootstrap Guide

## Purpose

This document explains the expected backend bootstrap direction for the first implementation work.

It is not a complete implementation manual. It gives the first developer or coding agent a clear build order.

## Bootstrap Goal

Create a backend that can:

- start locally
- load runtime configuration
- expose a health route
- return standard responses
- return standard errors
- prepare for database connection
- support first tests

## Recommended Build Order

### Step 1: Create Backend Folder

Create:

```text
backend/
```

The backend should remain separate from Flutter apps and documentation.

### Step 2: Create Rust Workspace

Create a Rust workspace under backend.

Follow:

```text
docs/06-development-standards/repository-structure-target.md
```

### Step 3: Create API Shell

Create the Axum app shell.

The API shell should own:

- app startup
- route registration
- middleware wiring
- response conversion

It should not own business workflows.

### Step 4: Create Shared Response Types

Create common response structures for:

- success
- error
- metadata

These should match API standards.

### Step 5: Create Configuration Foundation

Create a config model for runtime values.

The first config should support local development and clear missing-value messages.

### Step 6: Create Health Route

Create a route that proves the app is running.

Recommended route:

```text
GET /health
```

or:

```text
GET /api/v1/health
```

Choose one and document the decision.

### Step 7: Add First Tests

Add tests for:

- health response
- standard success response shape
- standard error shape where practical
- configuration loading where practical

### Step 8: Prepare Database Foundation

Prepare database connection structure, but do not overbuild data models in the first PR.

## First PR Should Avoid

Do not add:

- Identity
- organizations
- permissions
- domains
- providers
- Flutter UI

## Final Rule

Bootstrap the backend in the smallest clean step possible. The goal is a reliable foundation, not feature volume.
