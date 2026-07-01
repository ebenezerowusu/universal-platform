# Rust Error Handling Standards

## Purpose

This document defines Rust error handling standards for Universal Platform.

Errors should be useful internally, safe externally, and consistent across API responses.

## Principle

Internal errors and client-facing errors are not the same thing.

The backend should preserve useful internal context while returning safe, standard error responses to clients.

## Error Layers

Recommended error layers:

- domain/core error
- application service error
- infrastructure error
- API error response

## Client-Facing Error Codes

Client-facing errors should use the API error catalog.

Examples:

- VALIDATION_ERROR
- AUTHENTICATION_REQUIRED
- TENANT_ACCESS_DENIED
- PERMISSION_DENIED
- RESOURCE_NOT_FOUND
- MODULE_NOT_INSTALLED
- FEATURE_NOT_AVAILABLE
- INTERNAL_ERROR

## Infrastructure Errors

Infrastructure errors should be mapped safely.

Examples:

- database connection failure -> INTERNAL_ERROR or DATABASE_ERROR
- Redis failure -> QUEUE_ERROR where relevant
- provider failure -> PROVIDER_ERROR or PROVIDER_UNAVAILABLE

Do not expose raw database or provider messages directly to clients.

## Validation Errors

Validation errors should include safe details where useful.

Example:

```json
{
  "code": "VALIDATION_ERROR",
  "message": "The request is invalid.",
  "details": {
    "field": "email"
  }
}
```

## Trace IDs

Every error response should include trace metadata.

Trace IDs allow support and developers to connect client reports to backend logs.

## Logging

Log internal errors with useful context.

Avoid logging:

- raw credentials
- secrets
- full confidential notes
- unnecessary personal data

## Testing Requirements

Tests should cover:

- validation error mapping
- permission error mapping
- not found error mapping
- infrastructure error mapping
- trace ID in error response
- no unsafe internal details in client response

## Final Rule

Errors should help developers debug without leaking sensitive internal details to users.
