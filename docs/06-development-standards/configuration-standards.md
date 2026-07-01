# Configuration Standards

## Purpose

This document defines how Universal Platform handles configuration.

Configuration must make the application portable across local development, staging, production, and future distributed infrastructure.

## Configuration Principle

Application code must not hardcode infrastructure locations, provider choices, country rules, or environment-specific secrets.

Use configuration.

## Configuration Sources

Expected sources:

- Environment variables
- Local development example files
- Organization settings stored in database
- Platform settings stored in database
- Provider configuration stored securely

## Environment Configuration

Environment variables should control infrastructure and runtime settings.

Examples:

```text
APP_ENV
APP_PORT
APP_BASE_URL
DATABASE_URL
REDIS_URL
STORAGE_PROVIDER
QUEUE_PROVIDER
LOG_LEVEL
JWT_SECRET
```

Production secret values must not be committed to the repository.

## Application Configuration

Application configuration includes runtime behavior such as:

- server port
- log level
- database connection
- cache connection
- queue connection
- storage provider
- token settings

## Platform Configuration

Platform configuration includes SaaS-level settings such as:

- enabled regions
- available providers
- subscription plans
- module catalog
- global feature flags

## Organization Configuration

Organization configuration includes tenant-specific settings such as:

- default language
- timezone
- currency
- terminology
- branding
- enabled modules
- attendance settings
- communication preferences

## Provider Configuration

Provider configuration should be managed by platform administrators.

Domains should not know provider credentials.

Provider choices should be resolved through engines such as Payment Engine and SMS Engine.

## Feature Flags

Feature flags may be used for:

- staged rollouts
- beta features
- premium features
- region-specific availability
- internal testing

Feature flags must not replace proper permission checks.

## Validation

Configuration should be validated at application startup where possible.

Missing required configuration should fail fast in non-test environments.

## Example Files

The repository may include safe example files:

```text
.env.example
.env.development.example
```

These files must not include real secrets.

## Testing Requirements

Tests should cover:

- Required configuration validation
- Default values
- Invalid configuration handling
- Environment-specific behavior

## Final Rule

Configuration controls deployment and variability. Business logic should not be hardcoded for one environment, provider, country, or organization.
