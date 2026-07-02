# Authentication Session Device Security UI Contract

## Purpose

This document defines the MVP UI contract for Authentication Session, Device, and Security Control foundation.

The screens should help users and permitted admins review active sessions, devices, security events, MFA readiness placeholders, suspicious access reviews, and token/access key placeholders.

## Navigation Direction

Security screens should appear when:

- user is authenticated
- security feature is available
- user has required permission for organization/admin views
- personal security views are limited to the current user
- organization security views are organization-scoped

## Security Dashboard

Should show:

- active sessions
- known devices
- recent security events
- MFA readiness status placeholder
- suspicious access review count where permitted
- token/access key warning count where available

## Active Sessions

Should show:

- session safe label
- device/browser placeholder
- organization context placeholder where available
- created date
- last seen date
- status
- revoke action where permitted

## Device List

Should show:

- device label placeholder
- platform/browser placeholder
- last used date
- trust status placeholder
- status
- forget placeholder action where permitted

## MFA Readiness Placeholder

Should show:

- MFA enabled status placeholder
- organization MFA requirement placeholder
- recovery method placeholder
- enable/disable placeholder actions where permitted

## Security Event List

Should show:

- event type
- severity placeholder
- actor/user safe label
- device safe label where available
- timestamp
- status
- action to view detail

## Security Event Detail

Should show:

- event type
- safe summary
- user safe label
- device safe label where available
- organization context where available
- timestamp
- metadata safe summary

## Suspicious Access Review Placeholder

Should show:

- review reason
- risk signal placeholder
- user safe label
- session/device reference safe label
- status
- resolve placeholder action where permitted

## Token and Access Key Placeholder

Should show:

- key label
- scope placeholder
- created date
- last used placeholder
- status
- rotate/revoke placeholder action where permitted

Raw secrets must never be displayed after creation placeholder.

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 45:

- password reset provider UI
- external identity provider setup UI
- biometric authentication UI
- device fingerprinting SDK UI
- hidden user impersonation UI
- automatic lockout configuration UI

## Final Rule

Security UI should help users protect their own access while giving admins only permission-scoped visibility into organization security risks.
