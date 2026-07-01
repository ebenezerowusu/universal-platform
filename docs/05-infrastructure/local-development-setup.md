# Local Development Setup Guide

## Purpose

This document defines the local development setup direction for Universal Platform.

The goal is to make the project easy to run for developers and AI coding agents.

## Local Development Principle

Local setup should be repeatable, documented, and safe.

It should not require production credentials or live provider integrations.

## Expected Local Services

Local development may require:

- Rust backend
- PostgreSQL
- Redis
- Local file storage
- Mock provider adapters

## Environment Files

The repository may include safe example files such as:

```text
.env.example
.env.development.example
```

These files must not contain real secrets.

## Local Provider Strategy

Local development should use mock or local adapters for:

- payments
- SMS
- storage where practical
- notifications where practical

This allows the platform to be developed without live external accounts.

## Local Database Strategy

Local database setup should support:

- migrations
- repeatable seed data
- easy reset for development
- isolated sample data

## Local Storage Strategy

Local storage may use a folder-based adapter during MVP.

The Storage Engine must still be used, even if the adapter stores files locally.

## Developer Workflow

Recommended local workflow:

1. Prepare local environment file.
2. Start required services.
3. Run migrations.
4. Run seed data where needed.
5. Start backend.
6. Run tests.

## Troubleshooting Notes

Common issues to document during implementation:

- database connection errors
- migration errors
- port conflicts
- missing configuration
- Redis connection errors

## Final Rule

Local development should mirror the architecture without depending on production services.
