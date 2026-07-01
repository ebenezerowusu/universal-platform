# First Sprint Plan

## Purpose

This document defines the recommended first sprint for implementation.

The first sprint should create a running backend foundation, not a full product feature.

## Sprint Goal

Create the minimum backend foundation required for future platform work.

## Sprint Scope

### 1. Backend Workspace

Create the initial Rust backend workspace and crate structure.

Expected result:

- Backend workspace exists.
- Basic crates are created.
- Project can build.

### 2. API Shell

Create the initial Axum API shell.

Expected result:

- Application entrypoint exists.
- Router setup exists.
- API version structure exists.

### 3. Configuration Foundation

Create runtime configuration loading for local development and future environments.

Expected result:

- App can read required runtime settings.
- Configuration errors are clear.

### 4. Health Endpoint

Create the first health endpoint.

Expected result:

- Health endpoint responds successfully.
- Response follows platform response format.

### 5. Response and Error Foundation

Create shared response and error structures.

Expected result:

- Success responses are consistent.
- Error responses are consistent.
- Trace metadata foundation exists.

### 6. Database Foundation

Create the first database integration foundation.

Expected result:

- Database connection is available.
- Migration structure is ready.
- Database health can be checked.

### 7. Initial Tests

Create tests for the first foundation pieces.

Expected result:

- Health endpoint is tested.
- Configuration behavior is tested.
- Response format is tested.

## Out of Sprint Scope

Do not implement:

- Religious member management
- Flutter UI
- payment provider integration
- SMS provider integration
- full RBAC
- marketplace or POS

## Completion Standard

The sprint is complete when:

- Backend starts locally.
- Health endpoint works.
- Configuration loads correctly.
- Database foundation exists.
- Migration structure exists.
- Shared response and error format exists.
- Basic tests pass.

## Final Rule

Sprint 1 proves the backend foundation. Product features come after the foundation is stable.
