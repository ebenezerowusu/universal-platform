# Sprint 18 Communication Governance Review

## Purpose

This document defines review checks for Sprint 18.

Sprint 18 implements Communication Governance foundations.

## Review 1: Scope

Sprint 18 may include:

- communication preference foundation
- recipient opt-out foundation
- message template foundation
- recipient segment foundation
- recipient preview filters
- safe-send rule checks
- audit records for important changes
- permission and feature checks
- API and Flutter foundations

Sprint 18 should not include:

- advanced campaign automation
- WhatsApp or email campaigns
- AI message generation
- provider credential screens
- direct provider calls from domain logic
- sending that ignores preferences

## Review 2: Preference Rules

Confirm:

- recipient preferences are organization-scoped
- opt-out status is respected
- preference updates are auditable
- preference checks are part of send review

## Review 3: Template Rules

Confirm:

- templates are organization-owned
- templates have status
- template updates are auditable
- template body validation exists where practical

## Review 4: Segment Rules

Confirm:

- segments are organization-owned
- segment rules do not cross organization boundaries
- recipient preview applies preferences when requested
- private recipient details are not exposed unnecessarily

## Review 5: Safe Send Checks

Confirm send review checks:

- permission
- module availability
- feature availability
- wallet balance where required
- recipient eligibility
- preferences and opt-outs
- message content validity

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/communication-governance-api-contract.md
```

## Review 7: Flutter Contract

Confirm screens align with:

```text
docs/11-ui-ux/communication-governance-flutter-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- preference update
- opt-out exclusion
- template create/update
- segment create/update
- recipient preview
- send review cannot send when no eligible recipients
- permission denied behavior

## Final Rule

Sprint 18 is complete when communication can respect preferences, use templates, preview recipient segments, and apply safe-send checks before delivery.
