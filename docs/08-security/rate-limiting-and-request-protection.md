# Rate Limiting and Request Protection Standards

## Purpose

This document defines how Universal Platform should protect important endpoints from excessive or unsafe request patterns.

The goal is to protect users, organizations, providers, and platform resources.

## Principle

Rate limits should protect high-risk and high-cost operations without making normal usage difficult.

## Endpoints That Need Limits

Recommended early candidates:

- Login
- Password reset
- Public registration
- SMS sending
- Payment initialization
- File upload
- Import upload
- Report export
- Search endpoints
- Webhook endpoints where useful

## Limit Dimensions

Rate limits may be applied by:

- IP address
- user_id
- organization_id
- endpoint
- provider action
- API key future

## Response Code

When a request is limited, return a standard error:

```text
RATE_LIMITED
```

The response should be safe and should not expose internal rate limit configuration details.

## Organization-Level Limits

Some limits should apply at organization level.

Examples:

- SMS sends per minute
- Report exports per hour
- Import jobs per day
- File upload size or frequency

## Provider Protection

Provider-facing operations should be protected so one organization cannot overload shared provider capacity.

Examples:

- SMS provider calls
- Payment initialization
- Storage uploads

## Subscription and Rate Limits

Rate limits are not the same as subscription limits.

Subscription limits control product access and plan usage.

Rate limits protect system stability and cost.

Both may apply to the same operation.

## Logging and Observability

Rate-limited requests should be visible in logs or metrics.

Useful fields:

- endpoint
- organization_id where available
- user_id where available
- limit type
- trace_id

## Testing Requirements

Tests should cover:

- Limit is enforced
- Normal requests pass
- Limited requests return RATE_LIMITED
- Organization-level limits work where implemented
- Provider-facing limits work where implemented

## Final Rule

Rate limiting is a platform protection layer, not a replacement for authentication, permissions, or subscription checks.
