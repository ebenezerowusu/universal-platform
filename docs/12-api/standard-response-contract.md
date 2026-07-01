# Standard Response Contract

## Purpose

This document defines the standard API response contract for Universal Platform.

The contract should be used by all backend routes unless a documented exception exists.

## Success Response

All successful API responses should follow this shape:

```json
{
  "success": true,
  "data": {},
  "meta": {
    "traceId": "..."
  }
}
```

## Success Fields

### success

Type:

```text
boolean
```

For success responses, value must be:

```text
true
```

### data

Contains the response payload.

For list responses, `data` may contain items and pagination fields depending on the route contract.

### meta

Contains response metadata.

Common fields:

```text
traceId
requestId later if needed
```

## Error Response

All failed API responses should follow this shape:

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Safe error message",
    "details": {}
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Error Fields

### success

For failed responses, value must be:

```text
false
```

### error.code

A stable error code.

Example:

```text
VALIDATION_ERROR
```

### error.message

A safe user-readable message.

Do not expose internal implementation details.

### error.details

Optional structured details.

Use this for validation fields or safe context.

### meta

Metadata should help support and tracing without exposing sensitive details.

## List Response Direction

List responses should use a consistent pagination shape.

Recommended data shape:

```json
{
  "items": [],
  "pagination": {
    "page": 1,
    "perPage": 20,
    "total": 0,
    "totalPages": 0
  }
}
```

## Sprint 1 Requirement

Sprint 1 should implement the response foundation even if only the health endpoint uses it.

## Final Rule

Clients should not need to guess the response shape. Keep API responses consistent from the first route.
