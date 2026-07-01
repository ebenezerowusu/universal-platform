# Deployment Runbook

## Purpose

This document defines the deployment runbook direction for Universal Platform.

A runbook helps operators deploy consistently and recover from common issues.

## Deployment Principle

Deployments should be repeatable, observable, and reversible where practical.

## Pre-Deployment Checklist

Before deployment, verify:

- Code has been reviewed.
- Required checks have passed.
- Migration impact is understood.
- Environment configuration is ready.
- Backup status is known.
- Release notes or change summary exists.

## Deployment Steps

High-level deployment flow:

```text
Prepare release
Run checks
Backup critical data where needed
Apply migrations
Deploy application
Restart services
Run health checks
Verify logs
Monitor key flows
```

Exact commands will be added after implementation and infrastructure scripts exist.

## Migration Handling

Before running migrations:

- Review migration files.
- Confirm target environment.
- Confirm backup status.
- Understand rollback or recovery approach.

## Health Verification

After deployment, verify:

- API health endpoint
- Database connectivity
- Redis connectivity where applicable
- Worker process where applicable
- Login flow where applicable
- Important background jobs where applicable

## Post-Deployment Monitoring

Monitor:

- error rate
- request failures
- database connectivity
- job failures
- provider failures
- logs with trace IDs

## Rollback Direction

Rollback strategy depends on the type of change.

Application-only changes may be rolled back by redeploying the previous version.

Database changes may require a planned recovery approach.

## Incident Notes

If deployment fails, record:

- time of failure
- version or commit
- failing service
- error summary
- actions taken
- final status

## Final Rule

Deployment is not complete when files are copied. Deployment is complete when the system is healthy and verified.
