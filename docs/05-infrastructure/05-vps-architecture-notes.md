# VPS Architecture Notes

## Purpose

This document records how the MVP infrastructure should be organized at a high level.

The first version of Universal Platform may run on a single VPS, but the application must be written so that database, cache, workers, and storage can later move to separate infrastructure through configuration changes.

## Initial MVP Services

The initial VPS may host:

- Reverse proxy
- Rust API
- PostgreSQL
- Redis
- Background workers
- Local upload storage

## Initial Architecture

```text
Internet
  -> Reverse Proxy
    -> Rust API
      -> PostgreSQL
      -> Redis
      -> Workers
      -> Local Storage
```

## Configuration Principle

Infrastructure locations must come from environment configuration.

The application should not assume that PostgreSQL, Redis, or storage are local forever.

Example configuration names:

```text
DATABASE_URL
REDIS_URL
STORAGE_PROVIDER
STORAGE_LOCAL_PATH
QUEUE_PROVIDER
```

## Growth Path

The infrastructure should scale in stages:

1. Single VPS for MVP.
2. Move PostgreSQL to a dedicated database server.
3. Move Redis to a dedicated cache/queue service.
4. Move workers to separate worker nodes.
5. Move uploads to object storage.
6. Add multiple API nodes behind a load balancer.

## Backup Principle

Database and uploaded files must be backed up from the beginning.

Backups should be restorable and should not live only beside the running application.

## Monitoring Principle

The platform should monitor:

- API health
- Database availability
- Cache availability
- Worker status
- Disk usage
- Backup success
- Payment provider health
- SMS provider health

## Final Rule

Start on one VPS, but write the application as if every infrastructure dependency will eventually move.
