# Runtime Configuration Catalog

## Purpose

This document defines the first runtime configuration catalog for Universal Platform.

Runtime configuration should describe how the application connects to infrastructure and how it behaves in each environment.

## Principle

Runtime configuration is for deployment and infrastructure behavior.

Organization-specific business behavior should be handled through the Configuration Engine.

## Application Configuration

Examples of application-level configuration areas:

- environment name
- application name
- application port
- public base URL
- log level

## Database Configuration

Examples of database configuration areas:

- database connection reference
- connection pool sizing
- database health check settings

## Cache and Queue Configuration

Examples of cache and queue configuration areas:

- Redis connection reference
- queue provider
- queue naming prefix
- worker concurrency

## Session Configuration

Examples of session configuration areas:

- session lifetime
- token lifetime
- issuer name
- server-managed signing configuration reference

Sensitive values must come from secure runtime configuration and must not be committed to the repository.

## Storage Configuration

Examples of storage configuration areas:

- storage provider
- local storage path for development
- public file base URL where applicable

## Observability Configuration

Examples of observability configuration areas:

- trace enablement
- log format
- request logging enablement

## Client Configuration

Examples of client-facing configuration areas:

- allowed client origins
- mobile app scheme
- desktop app scheme

## Provider Configuration

Provider credentials and private provider settings should be handled through secure provider configuration and provider adapters.

Local development should prefer mock providers where practical.

## Example Files

Safe example files may exist, but they must contain placeholders only.

## Final Rule

Use runtime configuration for infrastructure and deployment behavior. Use Configuration Engine for organization, module, and feature behavior.
