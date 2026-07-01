# MVP Security Checklist

## Purpose

This document defines the security checklist for the Universal Platform MVP.

The MVP must be safe enough for early organizations before production use.

## Authentication

Confirm:

- login flow exists
- current user route exists
- session handling is server-controlled
- unsafe credential values are not returned through APIs
- unsafe credential values are not written to logs

## Organization Context

Confirm:

- organization-owned routes require selected organization context
- organization membership is checked where needed
- organization-owned queries use organization context
- organization boundary tests exist for organization-owned features

## Authorization

Confirm:

- Permission Engine foundation exists
- role assignment foundation exists
- protected actions call permission checks
- backend enforces access decisions
- Flutter UI does not replace backend checks

## Audit

Confirm:

- Audit Engine foundation exists
- important changes can be recorded
- actor context can be recorded
- organization context can be recorded
- audit records avoid unnecessary sensitive values

## API Safety

Confirm:

- standard error response exists
- raw internal errors are not exposed to clients
- validation errors are safe and useful
- trace metadata exists where practical

## Data Protection

Confirm:

- sensitive data is not returned in general list responses
- confidential records require explicit handling
- file metadata is organization-scoped
- private file access uses backend checks

## Runtime Configuration

Confirm:

- real secrets are not committed
- example configuration uses placeholders only
- provider credentials are hidden behind secure configuration direction
- local mock providers are preferred before live providers

## Provider Safety

Confirm:

- domains do not call providers directly
- provider adapters are used through engines
- provider errors are mapped to safe platform errors
- webhook/callback routes are documented before live integration

## Logging and Observability

Confirm:

- logs include useful trace context
- logs avoid unnecessary sensitive data
- background jobs include status and trace information where practical

## Final Rule

The MVP should not launch until authentication, organization context, authorization, audit, API safety, and runtime configuration are reviewed.
