# Observability and Monitoring Guide

## Purpose

This document defines the observability direction for Universal Platform.

The platform must be understandable in production. Developers and operators should be able to investigate errors, slow requests, failed jobs, provider issues, and tenant-specific problems.

## Observability Principle

If the system cannot be observed, it cannot be operated confidently.

Build basic visibility from the beginning.

## Minimum MVP Observability

The MVP should include:

- Structured logs
- Request IDs
- Health endpoint
- Database connectivity check
- Worker logs
- Job status tracking
- Provider interaction logs
- Error response trace IDs

## Logs

Logs should include:

- timestamp
- log level
- service name
- request_id or trace_id
- organization_id where available
- user_id where available
- action or route
- error code where relevant

Logs must not expose secrets.

## Health Checks

The platform should expose health endpoints.

Examples:

```text
GET /health
GET /health/ready later
```

Health checks may verify:

- API process running
- Database reachable
- Redis reachable
- Worker status later

## Metrics Direction

Future metrics may include:

- Request count
- Request latency
- Error rate
- Database query timing
- Queue depth
- Failed jobs
- SMS send success rate
- Payment success rate
- Active organizations
- Module usage

## Provider Monitoring

Provider integrations should track:

- Payment initialization failures
- Payment callback failures
- SMS send failures
- Storage upload failures
- Provider response time

Provider issues should be visible without exposing raw provider secrets.

## Audit vs Logs

Logs are for system operations and debugging.

Audit logs are for accountability and sensitive action history.

Do not use application logs as a replacement for Audit Engine.

## Error Tracking

The platform should eventually support error tracking.

Errors should include trace IDs so frontend/API responses can be connected to backend logs.

## Tenant-Aware Investigation

When safe, operational logs should include organization context to investigate organization-specific issues.

Do not leak one organization's sensitive data into logs visible to another organization.

## Testing Requirements

Tests should cover:

- Health endpoint
- Trace ID in response metadata
- Structured error response
- Critical failures produce logs

## Final Rule

Every production issue should be traceable through request IDs, logs, health checks, and audit records where appropriate.
