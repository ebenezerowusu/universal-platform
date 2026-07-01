# SMS Provider Operations Guide

## Purpose

This document defines operational guidance for managing SMS provider behavior from the platform side.

The goal is to keep provider complexity away from organization users while giving system/admin teams enough visibility to support delivery.

## Principle

Organizations should manage wallet, packages, messages, and history.

System/admin teams should manage provider health, provider configuration, and delivery troubleshooting.

## System/Admin Responsibilities

System/admin users may need to review:

- provider health
- provider send errors
- delivery status issues
- wallet reconciliation issues
- stuck messages
- manual correction requests
- provider configuration changes

## Organization User Responsibilities

Organization users should only see:

- SMS wallet balance
- packages
- message compose
- recipient preview
- message history
- safe delivery status

They should not see provider credentials or low-level provider settings.

## Provider Health Review

Provider health may be:

```text
healthy
degraded
unavailable
unknown
```

When degraded or unavailable, support should avoid promising delivery until the status is understood.

## Delivery Troubleshooting

For a message issue, review:

- organization
- message id
- recipient count
- estimated units
- wallet reservation
- provider reference
- platform status
- provider status
- latest callback or status lookup

## Wallet Troubleshooting

For wallet issues, review:

- opening balance
- reservation
- deductions
- released units
- manual adjustments
- transaction ledger

Do not edit balances without an auditable correction record.

## Retry Direction

Retry should only be allowed when:

- the previous send state is safe to retry
- duplicate delivery risk is understood
- wallet impact is clear
- retry action is auditable

## Configuration Direction

Provider configuration should be managed through secure system/admin operations.

Do not expose provider credentials to organization users.

Do not commit live credentials to the repository.

## Final Rule

Provider operations should protect the organization experience. Keep provider complexity in system/admin tools, not tenant-facing workflows.
