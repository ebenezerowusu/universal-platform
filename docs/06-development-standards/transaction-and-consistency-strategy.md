# Transaction and Consistency Strategy

## Purpose

This document defines transaction and consistency rules for Universal Platform.

The platform will coordinate identity, organizations, permissions, modules, payments, SMS, reporting, and multiple domains. Consistency rules must be clear before complex workflows are implemented.

## Principle

Use strong consistency inside a single business transaction where correctness requires it.

Use events and jobs for follow-up work that can happen after the main transaction succeeds.

## Transaction Boundary

Application services should usually define transaction boundaries.

Examples:

- create organization and organization owner membership
- create member and member status history
- mark attendance records in bulk
- record giving and link verified payment reference
- install module and default module settings

## Keep Transactions Focused

Transactions should be as short as practical.

Avoid holding database transactions while performing slow external operations.

Examples of work that should usually happen outside the main transaction:

- SMS delivery
- email delivery
- large report generation
- file processing
- external provider callbacks

## Events After Commit

When a use case publishes an event, publish it after the data change succeeds.

For critical workflows, consider a future outbox pattern.

## Idempotency

Idempotency is important for repeated requests, jobs, and callbacks.

Common areas:

- payment callbacks
- SMS jobs
- imports
- exports
- wallet crediting
- event consumers

## Consistency by Area

### Identity and Organization

Creating an organization owner should keep user, organization, and membership records consistent.

### Permissions

Role and permission assignment should be transactionally consistent where practical.

### Payments

Payment provider status should be verified through Payment Engine before domain records treat it as final.

### SMS Wallet

Wallet credit and debit operations should be consistent and traceable.

### Attendance

Bulk attendance marking should avoid duplicate records for the same session and member.

## Testing Requirements

Tests should cover:

- transaction rollback on failure
- no partial records after failed use case
- duplicate-safe callback or job behavior where relevant
- event not published when main action fails

## Final Rule

Use transactions for correctness. Use events and jobs for decoupled follow-up work.
