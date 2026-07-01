# Local Docker Compose Service Contract

## Purpose

This document defines the expected local Docker Compose service contract for Universal Platform development.

It does not require Docker Compose to be fully implemented in Sprint 1, but it defines the target local service shape.

## Principle

Local development should not depend on production infrastructure.

Developers should be able to run required backing services locally.

## Expected Local Services

The MVP local environment should support:

- PostgreSQL
- Redis
- local file storage folder
- backend API

## PostgreSQL Service

Purpose:

- local relational database
- migration testing
- repository testing

Expected configuration areas:

- database name
- database user
- database password placeholder
- exposed local port
- persistent local volume

Never commit real production database credentials.

## Redis Service

Purpose:

- cache foundation
- queue foundation
- background job foundation later

Expected configuration areas:

- exposed local port
- optional local volume where useful

## Local Storage

Purpose:

- development file storage
- storage engine testing later

Expected path:

```text
.local/storage/
```

This folder should not contain production files.

## Backend API Service Later

The backend may later be included in Docker Compose.

For early Sprint 1, running backend directly through Cargo is acceptable.

## Health Checks

Local services should support basic health checks where practical.

Examples:

- PostgreSQL ready
- Redis ready
- backend health endpoint ready

## Final Rule

Local Docker Compose should make development easier without hiding how the backend works.
