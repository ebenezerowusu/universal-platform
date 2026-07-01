# Sprint 16 SMS Wave Review

## Purpose

This document defines review checks for Sprint 16.

Sprint 16 implements the Communication and SMS Wallet foundation.

## Review 1: Scope

Sprint 16 may include:

- Communication module registration foundation
- SMS wallet balance foundation
- SMS package catalog foundation
- SMS transaction ledger foundation
- message draft and send request foundation
- recipient preview foundation
- message history foundation
- local/mock SMS provider foundation
- permission and feature checks
- audit records for important actions
- API and Flutter foundations

Sprint 16 should not include:

- WhatsApp integration
- email campaigns
- advanced campaign analytics
- provider setup screens for organization users
- complex automation rules
- real payment collection unless approved

## Review 2: Engine Boundaries

Confirm:

- SMS sending goes through SMS Engine
- provider details stay behind adapters
- Communication Domain does not call providers directly
- Flutter does not know provider details

## Review 3: Wallet Rules

Confirm:

- wallet is organization-owned
- balance is readable
- transactions are recorded
- send requests check available units where implemented
- manual adjustments are auditable where supported

## Review 4: Permission and Feature Checks

Confirm protected actions check permissions for:

- wallet view
- package request
- message create
- message send
- history view

Confirm feature availability controls Communication/SMS access.

## Review 5: API Contract

Confirm implementation aligns with:

```text
docs/12-api/communication-sms-api-contract.md
```

## Review 6: Flutter Contract

Confirm screens align with:

```text
docs/11-ui-ux/communication-sms-flutter-screens-contract.md
```

## Review 7: Revenue Direction

Confirm:

- SMS package revenue direction is preserved
- real payment collection is not added unless approved
- future Payment Engine integration remains possible

## Review 8: Test Coverage

Tests should cover where practical:

- wallet summary
- package catalog
- package request
- message draft creation
- send request
- insufficient units behavior
- permission denied behavior
- mock provider behavior

## Final Rule

Sprint 16 is complete when Communication and SMS Wallet are usable as a provider-agnostic foundation and ready for future real provider/payment integration.
