# Deployment Notes Template

## Purpose

Use this template when production, staging, or infrastructure deployment work begins.

Deployment notes should document what changed, where it was deployed, how it was verified, and how rollback or recovery should be handled.

## Deployment Context

```text
Environment:
Branch:
Commit:
Date:
Deployed by:
Related PR:
```

## Deployment Scope

Describe what was deployed.

```text
API
Worker
Database migration
Redis configuration
Nginx configuration
SSL/TLS
Flutter build
Other
```

## Infrastructure Target

```text
Provider: Hostinger VPS placeholder
OS: Ubuntu 24 LTS placeholder
Runtime: Docker Compose / system services placeholder
Database: PostgreSQL placeholder
Cache/Queue: Redis placeholder
```

## Pre-Deployment Checklist

```text
[ ] CI checks passed
[ ] Environment variables reviewed
[ ] Database backup created where applicable
[ ] Migration plan reviewed where applicable
[ ] Rollback/recovery plan reviewed
[ ] Secrets are not committed to repository
[ ] Health check route available
[ ] Maintenance window communicated where applicable
```

## Environment Variables Changed

```env
# List only variable names or safe values.
# Do not write secrets.
```

## Commands Run

```bash
# commands
```

## Verification

Confirm:

```text
[ ] API starts
[ ] /health passes
[ ] /ready passes or known placeholder status documented
[ ] Database connection works
[ ] Redis connection works
[ ] Logs checked
[ ] Basic smoke test completed
```

## Migration Notes

List migrations run:

```text
<list migrations or none>
```

## Rollback Or Recovery Notes

Explain the reviewed rollback or recovery approach.

Do not claim rollback is safe unless tested.

## Issues Observed

- 

## Post-Deployment Actions

- 

## Follow-Up Work

- 

## Final Rule

Deployment notes must never include passwords, tokens, private keys, provider secrets, or raw production credentials.
