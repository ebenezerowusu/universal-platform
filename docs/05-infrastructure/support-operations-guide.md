# Support Operations Guide

## Purpose

This document defines support operations direction for Universal Platform.

Support operations help platform admins investigate user issues, organization issues, failed jobs, provider problems, and configuration mistakes without breaking tenant boundaries.

## Principle

Support tools should help diagnose problems safely.

Support access must be limited, auditable, and aligned with platform roles.

## Support Areas

Support operations may cover:

- organization lookup
- user lookup summary
- module status review
- subscription status review
- payment transaction status review
- SMS wallet and message status review
- job status review
- provider health review
- audit review
- logs and trace lookup

## Support Context

Support actions should capture:

- support user
- organization involved
- reason or ticket reference where available
- action taken
- time of action

## Common Support Flows

### Login or Access Issue

Check:

- user summary
- account status
- organization membership
- role assignment
- module status where relevant

### Missing Feature Issue

Check:

- module installed
- module active
- plan feature availability
- feature flag status
- user role and permission summary

### Payment Issue

Check:

- transaction record
- provider reference
- status history
- callback history where available
- related domain record

### SMS Issue

Check:

- wallet balance
- campaign/message record
- provider status
- delivery status
- failure reason where available

### Background Job Issue

Check:

- job status
- attempts
- error code
- trace id
- related organization

## Safety Rules

Support operations should not:

- expose secret configuration values
- bypass normal organization boundaries casually
- edit production records without audit
- expose confidential notes unnecessarily
- make silent changes without traceability

## Final Rule

Support tooling must be helpful, safe, and auditable. Production support must never become a hidden backdoor around platform rules.
