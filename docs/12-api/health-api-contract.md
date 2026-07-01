# Health API Contract

## Purpose

This document defines the first API contract for the backend health endpoint.

The health endpoint is the first route implemented in Sprint 1.

## Route

Preferred route:

```text
GET /api/v1/health
```

Alternative route:

```text
GET /health
```

The implementation should choose one and keep it consistent.

## Owner

```text
API / Backend Foundation
```

## Authentication

The health route does not require authentication.

## Request

No request body.

## Success Response

Expected response shape:

```json
{
  "success": true,
  "data": {
    "status": "ok"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Optional Future Fields

Future health response may include:

```json
{
  "status": "ok",
  "version": "...",
  "environment": "local",
  "services": {
    "database": "ok",
    "redis": "ok"
  }
}
```

Do not add expensive health checks in the first version.

## Error Response

If the app is running, the health endpoint should return success.

Detailed infrastructure health checks can be added later under a readiness endpoint if needed.

## Test Requirements

Sprint 1 should test:

- health route returns success
- response uses standard success shape
- status field is present

## Final Rule

The health endpoint should be simple, stable, and useful for local development and deployment checks.
