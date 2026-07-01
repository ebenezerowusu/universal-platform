# Deployment Strategy

## Purpose

This document defines the deployment direction for Universal Platform.

The platform should start simple, but it must be designed so infrastructure can scale without rewriting application logic.

## MVP Deployment

The MVP will run on a single VPS.

Recommended initial server:

- Ubuntu 24 LTS
- Docker Compose
- Nginx
- Rust + Axum API
- PostgreSQL
- Redis
- Background workers
- Local file storage
- Let's Encrypt SSL

## Initial Architecture

```text
Internet
  -> Nginx
    -> Rust + Axum API
      -> PostgreSQL
      -> Redis
      -> Workers
      -> Local Storage
```

PostgreSQL, Redis, workers, and the API may all run on the same VPS at the beginning.

This keeps the MVP simple, affordable, and easier to manage.

## Configuration-Driven Infrastructure

The application must not hardcode infrastructure locations.

Use environment configuration:

```env
APP_ENV=production
APP_URL=https://api.example.com
DATABASE_URL=postgres://user:password@localhost:5432/universal_platform
REDIS_URL=redis://localhost:6379
STORAGE_PROVIDER=local
STORAGE_LOCAL_PATH=/var/app/uploads
QUEUE_PROVIDER=redis
SMS_PROVIDER_RESOLUTION=country
PAYMENT_PROVIDER_RESOLUTION=country
```

When infrastructure moves, only environment values should change.

## Scaling Stages

### Stage 1: Single VPS

Everything runs on one VPS.

Best for:

- MVP
- Early customers
- Low operational cost
- Fast development

### Stage 2: Dedicated PostgreSQL Server

Move PostgreSQL away from the API server.

```env
DATABASE_URL=postgres://user:password@db.internal:5432/universal_platform
```

Application code must not change.

### Stage 3: Dedicated Redis Server

Move Redis to its own server or managed service.

```env
REDIS_URL=redis://redis.internal:6379
```

Application code must not change.

### Stage 4: Separate Worker Nodes

Move background jobs to separate worker processes/servers.

Workers should consume queues and run independently from the API server.

### Stage 5: Object Storage

Move uploads from local storage to object storage.

Possible providers:

- S3-compatible storage
- Cloudflare R2
- Azure Blob Storage
- Future storage provider

```env
STORAGE_PROVIDER=s3
STORAGE_BUCKET=universal-platform-prod
```

Application code must use the Storage Engine, not local file paths directly.

### Stage 6: Multiple API Nodes

Run multiple API servers behind a load balancer.

Requirements before this stage:

- Stateless API design
- Sessions stored in Redis or token-based auth
- Shared object storage
- Centralized logs
- Proper health checks

## Docker Compose Direction

The MVP should use Docker Compose to run services consistently.

Expected services:

- api
- postgres
- redis
- worker
- nginx or reverse proxy

Production Docker files and Compose files must be kept inside the `infrastructure/` folder.

## Backups

PostgreSQL backups are mandatory from day one.

Minimum backup strategy:

- Daily database backup
- Retention policy
- Backup verification
- Off-server backup storage as soon as possible
- Restore documentation

Uploaded files must also be backed up until object storage is introduced.

## Observability

The platform should include basic observability from the beginning.

Minimum:

- Application logs
- Error logs
- Request IDs
- Health check endpoint
- Database connection monitoring
- Worker failure logs
- Payment and SMS error logs

Future:

- Metrics
- Tracing
- Dashboards
- Alerting
- Job monitoring

## Security Requirements

- Use HTTPS everywhere.
- Do not expose PostgreSQL publicly.
- Do not expose Redis publicly.
- Use firewall rules.
- Store secrets in environment variables or secret management tools.
- Rotate provider credentials when needed.
- Use least-privilege database users.

## Deployment Principle

Start with one VPS, but design as if every service will eventually move to its own infrastructure.

The application must not care whether PostgreSQL is on localhost, a dedicated server, or a managed database service.
