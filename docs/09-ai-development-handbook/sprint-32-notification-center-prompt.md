# Sprint 32 Notification Center Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 32.

Sprint 32 prepares the Notification Center and User Preferences foundation after scheduling, search, commerce, HR, delivery, finance, and religious workflows have introduced many notification triggers.

## Prompt

```text
You are implementing Sprint 32 for Universal Platform.

Your task is to implement the Notification Center and User Preferences foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/notification-engine.md
3. docs/04-capability-engines/sms-engine.md
4. docs/04-capability-engines/permission-engine.md
5. docs/04-capability-engines/audit-engine.md
6. docs/14-roadmap/sprint-32-notification-center-roadmap.md
7. docs/13-module-sdk/notification-center-readiness.md
8. docs/12-api/notification-center-api-contract.md
9. docs/11-ui-ux/notification-center-ui-contract.md
10. docs/05-infrastructure/notification-delivery-operations-guide.md
11. docs/16-quality/sprint-32-notification-center-review.md

Implement only:
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
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full marketing campaign engine
- bulk SMS sales expansion
- push notification provider integration
- email provider integration
- WhatsApp provider integration
- advanced notification personalization
- unrestricted broadcast messaging
- provider-specific delivery calls inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/notification-center-foundation

Recommended PR title:
feat: add notification center foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Notification/SMS/Permission boundaries followed
- preference and delivery rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused notification center foundation that records safe in-app notifications, stores preferences, tracks delivery attempts, and links notifications to domain triggers.

## Final Rule

Notifications must be user-scoped or organization-scoped, preference-aware, auditable where required, and provider-agnostic. Domain modules should request notifications through Notification Engine instead of calling providers directly.
