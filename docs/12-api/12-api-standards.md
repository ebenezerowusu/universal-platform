# API Standards

## Purpose

This document defines API standards for Universal Platform.

All clients, including Flutter mobile, Flutter desktop, future web clients, and third-party integrations, must consume the same backend APIs.

## API Principles

- API first
- Secure by default
- Versioned where necessary
- Consistent response format
- Tenant-aware
- Permission-aware
- Documented with OpenAPI
- No business logic in handlers

## URL Structure

Recommended base structure:

```text
/api/v1/...
```

Examples:

```text
/api/v1/auth/login
/api/v1/organizations
/api/v1/organizations/{organizationId}/members
/api/v1/organizations/{organizationId}/attendance/sessions
/api/v1/organizations/{organizationId}/sms/messages
/api/v1/platform/modules
```

## Platform vs Organization APIs

Platform admin APIs should be clearly separated from organization APIs.

Examples:

```text
/api/v1/platform/organizations
/api/v1/platform/modules
/api/v1/platform/sms/providers
/api/v1/platform/payment/providers
```

Organization APIs should include organization context.

```text
/api/v1/organizations/{organizationId}/...
```

## Handler Responsibilities

API handlers should only:

1. Authenticate user.
2. Resolve tenant context.
3. Validate input.
4. Call application service.
5. Return standardized response.

Handlers must not contain business logic.

## Response Format

Successful response:

```json
{
  "success": true,
  "data": {},
  "meta": {
    "traceId": "..."
  }
}
```

Error response:

```json
{
  "success": false,
  "error": {
    "code": "PERMISSION_DENIED",
    "message": "You do not have permission to perform this action.",
    "details": {}
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Error Codes

Use stable error codes.

Examples:

- VALIDATION_ERROR
- AUTHENTICATION_REQUIRED
- INVALID_CREDENTIALS
- PERMISSION_DENIED
- TENANT_ACCESS_DENIED
- RESOURCE_NOT_FOUND
- MODULE_NOT_INSTALLED
- FEATURE_NOT_AVAILABLE
- SUBSCRIPTION_LIMIT_REACHED
- PROVIDER_ERROR
- RATE_LIMITED
- INTERNAL_ERROR

## Pagination

List endpoints should support pagination.

Recommended query params:

```text
?page=1&perPage=25
```

Response meta:

```json
{
  "page": 1,
  "perPage": 25,
  "total": 200,
  "totalPages": 8
}
```

Cursor pagination may be introduced for high-volume endpoints.

## Filtering and Sorting

Use clear query parameters.

Examples:

```text
?status=active
?dateFrom=2026-01-01&dateTo=2026-01-31
?sort=createdAt:desc
```

Filters must be validated server-side.

## Authentication

Protected APIs require authentication.

The authentication method may be JWT/session-based depending on final implementation.

No protected endpoint should rely on frontend-only checks.

## Authorization

Every protected action must enforce permissions through the Permission Engine.

Authorization must consider:

- User
- Organization
- Permission
- Scope
- Module status
- Subscription status
- Resource ownership

## Idempotency

High-risk operations should support idempotency.

Examples:

- Payment initialization
- Webhook processing
- SMS sending
- Import jobs
- Bulk operations

## Webhooks

Webhook endpoints must:

- Verify provider signatures where available
- Be idempotent
- Avoid exposing sensitive errors
- Log provider event IDs
- Reject invalid payloads

Example:

```text
/api/v1/webhooks/payments/{provider}
/api/v1/webhooks/sms/{provider}
```

## OpenAPI

The backend should produce OpenAPI documentation.

OpenAPI docs should include:

- Routes
- Request schemas
- Response schemas
- Error responses
- Auth requirements
- Tags by domain/engine

## Versioning

Start with `/api/v1`.

Breaking changes require a new version or compatibility layer.

Do not break existing clients without migration planning.

## Rate Limiting

Apply rate limits to sensitive endpoints:

- Login
- Password reset
- SMS sending
- Payment initialization
- Public registration
- Webhooks where appropriate

## Traceability

Every API response should include or support a trace/request ID.

Logs should include this ID so production issues can be investigated.

## Final Rule

APIs are contracts. Keep them consistent, secure, documented, and stable.
