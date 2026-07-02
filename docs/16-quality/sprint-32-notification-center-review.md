# Sprint 32 Notification Center Review

## Purpose

This document defines review checks for Sprint 32.

Sprint 32 implements Notification Center and User Preferences foundation.

## Review 1: Scope

Sprint 32 may include:

- notification center feature registration foundation
- in-app notification record foundation
- notification type/category foundation
- notification template metadata foundation
- user preference foundation
- organization preference foundation where practical
- delivery log foundation
- read/unread and archive foundation
- domain trigger/reference foundation
- permission and feature checks
- audit records for important notification preference/actions where required
- API and UI foundations

Sprint 32 should not include:

- full marketing campaign engine
- bulk SMS sales expansion
- push notification provider integration
- email provider integration
- WhatsApp provider integration
- advanced notification personalization
- unrestricted broadcast messaging
- provider-specific delivery calls inside domain modules
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Notification Center owns notification records and preferences
- Notification Engine owns orchestration
- SMS Engine owns SMS delivery where used
- domain modules do not call delivery providers directly
- Platform Core does not contain domain-specific notification rules

## Review 3: Organization and Recipient Boundaries

Confirm:

- notifications are organization-scoped
- recipient-specific notifications are only visible to the recipient or permitted admin
- domain trigger references cannot cross organization boundaries
- disabled modules do not create active notifications

## Review 4: Preference Rules

Confirm:

- user preferences are checked before delivery placeholders
- organization defaults are applied where user preferences are missing
- skipped-by-preference delivery logs are recorded where practical
- preference changes are permission-protected

## Review 5: Safe Content Rules

Confirm:

- notification titles and summaries are safe
- restricted domain details are not exposed through notification text
- inaccessible source records show a generic unavailable state
- broadcast-like requests are controlled

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/notification-center-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/notification-center-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- notification creation
- inbox filtering
- mark read/unread
- archive action
- user preference update
- skipped by preference behavior
- delivery log creation
- domain reference organization boundary
- permission denied behavior

## Final Rule

Sprint 32 is complete when users can receive and manage safe in-app notifications, configure preferences, and domains can request notifications through engine boundaries without calling providers directly.
