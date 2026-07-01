# Flutter API Client Contract

## Purpose

This document defines the MVP Flutter API client contract.

The Flutter client should communicate with the backend through a consistent API layer instead of scattered direct HTTP calls.

## Principle

Feature screens should not manually parse raw HTTP responses.

The API client foundation should handle standard response and error parsing centrally.

## Backend Response Contract

The Flutter API client should follow:

```text
docs/12-api/standard-response-contract.md
```

## Client Responsibilities

The API client foundation should handle:

- base URL configuration
- request headers
- standard success response parsing
- standard error response parsing
- trace id extraction where available
- session token attachment where applicable
- selected organization context where applicable
- safe error mapping for UI

## Client Must Not Own

The API client should not own:

- business rules
- backend authorization decisions
- feature availability decisions
- provider logic
- database logic

## Standard Success Handling

The API client should parse responses shaped like:

```json
{
  "success": true,
  "data": {},
  "meta": {
    "traceId": "..."
  }
}
```

## Standard Error Handling

The API client should parse errors shaped like:

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

## UI Error Mapping

The client should map backend errors into UI-friendly states such as:

```text
validationError
notAuthenticated
notAllowed
notFound
serverUnavailable
unknownError
```

## Session Handling Direction

Session handling should be centralized.

Feature screens should not manually attach tokens or handle session expiration directly.

## Organization Context Direction

Organization context should be attached through shared client state or request context.

Feature screens should not hardcode organization ids independently.

## Testing Direction

Tests should cover:

- success response parsing
- error response parsing
- missing or malformed response behavior
- session expired mapping where practical

## Final Rule

All Flutter feature code should go through the API client foundation. Avoid raw HTTP logic inside screens and widgets.
