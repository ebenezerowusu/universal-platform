# MVP Launch Readiness Checklist

## Purpose

This document defines the MVP launch readiness checklist for Universal Platform.

The MVP should not launch simply because features exist. It should launch when the foundation is usable, secure, supportable, and recoverable.

## Platform Foundation

Confirm:

- backend starts reliably
- health endpoint works
- database connection works
- configuration loading works
- standard API responses are used
- standard API errors are used
- logs include useful operation context

## Core Readiness

Confirm:

- Identity foundation works
- Organization foundation works
- Permission foundation works
- Audit foundation works
- Module Registry foundation works
- plan and feature checks exist where needed

## Engine Readiness

Confirm:

- Payment Engine interface exists
- SMS Engine interface exists
- Storage Engine interface exists
- Notification Engine interface exists
- Reporting Engine interface exists
- Workflow Engine interface exists
- live providers are clearly marked as implemented or deferred

## Religious MVP Readiness

Confirm:

- Religious modules can be enabled for an organization
- Religious settings exist
- members can be managed
- visitors can be managed
- congregations can be managed
- services can be managed
- groups can be managed
- manual attendance works
- basic reports exist

## Flutter Readiness

Confirm:

- app shell works
- login screen works
- organization selection works
- mobile shell works
- desktop/admin shell works where included
- API client handles standard responses
- loading and empty states exist

## Data Readiness

Confirm:

- migrations are reviewed
- seed data is repeatable
- scoped query tests exist for organization-owned records
- backup plan exists
- restore direction is documented

## Security Readiness

Confirm:

- no real secrets are committed
- protected actions use backend checks
- sensitive records are not exposed in general lists
- important changes are auditable
- file access is controlled

## Operational Readiness

Confirm:

- deployment runbook exists
- production readiness checklist is reviewed
- support operations guide exists
- logs and trace fields are available
- failed jobs can be investigated

## Documentation Readiness

Confirm:

- API contracts are documented
- database changes are documented
- sprint docs are updated
- ADRs exist for major decisions
- developer onboarding docs exist

## Final Rule

MVP launch means the platform is usable, supportable, and safe enough for early organizations, not just that screens have been built.
