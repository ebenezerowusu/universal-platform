# Authentication Session Device Security Guide

## Purpose

This document defines operating guidance for user sessions, device metadata, session revocation, MFA placeholders, security events, suspicious access review placeholders, and token/access key lifecycle placeholders.

The goal is to make user access safer, more visible, and auditable.

## Principle

Access does not end at role assignment.

Sessions, devices, security events, and revocation paths must also be visible and controlled.

## Data Sources

Security operations may use:

- user session metadata
- device metadata
- session revocation record
- MFA readiness placeholder
- security event
- suspicious access review placeholder
- token/access key lifecycle placeholder
- risk signal placeholder
- audit records

## Session Direction

Session records should capture:

```text
user_id
organization_context_placeholder
session_status
created_at
last_seen_at
revoked_at optional
device_id optional
```

## Device Direction

Device records should capture:

```text
user_id
device_label_placeholder
platform_browser_placeholder
last_used_at
trust_status_placeholder
status
```

Avoid storing excessive device fingerprinting details in MVP.

## Revocation Direction

Revocation actions should capture:

```text
session_id
revoked_by
revocation_type
reason optional
created_at
```

Admin revocation placeholders should require stronger permission and audit.

## MFA Direction

MFA placeholders may capture:

- organization requirement placeholder
- user enabled status placeholder
- challenge required placeholder
- recovery method placeholder

Do not implement biometric or external provider integration in Sprint 45.

## Security Event Direction

Security events should capture:

```text
user_id
event_type
severity_placeholder
session_id optional
device_id optional
organization_context_placeholder
safe_summary
created_at
```

## Suspicious Access Direction

Suspicious access review placeholders may include:

- unusual country/timezone placeholder
- repeated login failure placeholder
- new privileged session placeholder
- impossible travel placeholder
- manual review

Avoid automatic account lockout unless a future sprint defines clear review rules.

## Token and Access Key Direction

Token/access key placeholders should capture:

- key label
- scope placeholder
- created by
- rotated at placeholder
- revoked at placeholder
- last used placeholder

Raw secrets should not be displayed after creation placeholder.

## Notification Direction

Notifications may be triggered for:

- new device detected placeholder
- session revoked
- suspicious access created
- MFA setting changed placeholder
- token/access key created or revoked placeholder

Use Notification Engine where practical.

## Audit Direction

Audit should record:

- session revoked
- all sessions revoked
- admin session revoke placeholder
- device trust changed placeholder
- MFA setting changed placeholder
- suspicious access reviewed placeholder
- token/access key created placeholder
- token/access key revoked placeholder

## Data Quality Warnings

Warn or flag when:

- user has many active sessions
- privileged user has no recent security review
- device is untrusted but actively used
- suspicious access review is unresolved
- token/access key is stale placeholder
- admin revoked a session without reason where required

## Final Rule

Session and device security operations must reduce risk without bypassing Identity, Permission, Notification, or Audit boundaries.
