# OpenAPI Contract Standards

## Purpose

This document defines how Universal Platform should document APIs using OpenAPI.

OpenAPI contracts help backend, Flutter clients, third-party integrations, and AI coding agents stay aligned.

## Principle

APIs are contracts.

Every stable API should be documented and kept consistent with implementation.

## OpenAPI Scope

OpenAPI should document:

- Routes
- Request bodies
- Response bodies
- Error responses
- Authentication requirements
- Permission notes where useful
- Query parameters
- Pagination
- Tags by engine/domain

## API Tags

Use clear tags such as:

- Auth
- Organizations
- Permissions
- Modules
- Subscriptions
- Audit
- Payments
- SMS
- Storage
- Religious
- POS
- Finance
- HR

## Schema Naming

Use consistent schema names.

Examples:

```text
LoginRequest
LoginResponse
OrganizationResponse
ApiErrorResponse
PaginatedMemberResponse
```

## Error Documentation

Endpoints should document expected errors.

Examples:

- AUTHENTICATION_REQUIRED
- PERMISSION_DENIED
- TENANT_ACCESS_DENIED
- VALIDATION_ERROR
- RESOURCE_NOT_FOUND
- MODULE_NOT_INSTALLED
- FEATURE_NOT_AVAILABLE

## Versioning

Start with:

```text
/api/v1
```

Breaking API changes require deliberate versioning or compatibility handling.

## Client Generation

The OpenAPI document may later be used to generate API clients for Flutter or other consumers.

Do not rely on generated clients until API standards stabilize.

## Documentation Accuracy

OpenAPI documentation must match real behavior.

A documented endpoint that behaves differently from implementation creates risk.

## Testing Direction

Future tests should verify that documented routes and response shapes remain consistent with implementation.

## Final Rule

If an API is part of the platform contract, it should be documented through OpenAPI.
