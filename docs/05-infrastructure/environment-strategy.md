# Environment Strategy

## Purpose

This document defines environment strategy for Universal Platform.

The platform should support local development, staging, and production with clear configuration boundaries.

## Environments

Recommended environments:

- local
- development
- staging
- production

## Local Environment

Used by developers and AI coding agents.

Goals:

- Easy setup
- Docker Compose support
- Safe sample data
- No real provider credentials required
- Mock or sandbox adapters

## Development Environment

Used for internal integration work.

Goals:

- Shared testing environment
- Early API testing
- Internal QA
- Non-production data

## Staging Environment

Used to validate release candidates before production.

Goals:

- Production-like configuration
- Migration validation
- Provider sandbox checks
- Release testing

## Production Environment

Used by real organizations and users.

Goals:

- Stable operations
- Strong configuration control
- Backups
- Monitoring
- Restricted secret access
- Tested deployment process

## Configuration Separation

Each environment must have separate configuration values.

Examples:

- database URL
- Redis URL
- storage provider
- provider credentials
- log level
- public base URLs

## Data Separation

Production data must not be used casually in non-production environments.

If production-like data is needed for testing, it should be sanitized first.

## Provider Strategy

Local and staging should use mock or sandbox provider adapters where possible.

Production should use approved live provider configurations.

## Deployment Promotion

Recommended flow:

```text
local -> development -> staging -> production
```

Do not skip staging for important backend or database changes once production users exist.

## Final Rule

Each environment must be isolated, configurable, and safe for its purpose.
