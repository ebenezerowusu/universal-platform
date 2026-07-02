# Sprint 59 Product Announcements Release Notes Change Communication Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 59.

Sprint 59 prepares the Product Announcements, Release Notes, and Change Communication foundation after knowledge base, notification center, feature flags, configuration, support operations, policy library, monitoring, and organization lifecycle foundations have introduced controlled change communication needs.

## Prompt

```text
You are implementing Sprint 59 for Universal Platform.

Your task is to implement the Product Announcements, Release Notes, and Change Communication foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/notification-center-api-contract.md
3. docs/12-api/configuration-feature-flags-rules-api-contract.md
4. docs/12-api/knowledge-base-help-center-self-service-api-contract.md
5. docs/12-api/platform-support-operations-api-contract.md
6. docs/14-roadmap/sprint-59-product-announcements-release-notes-change-communication-roadmap.md
7. docs/13-module-sdk/product-announcements-release-notes-change-communication-readiness.md
8. docs/12-api/product-announcements-release-notes-change-communication-api-contract.md
9. docs/11-ui-ux/product-announcements-release-notes-change-communication-ui-contract.md
10. docs/05-infrastructure/product-announcements-release-notes-change-communication-guide.md
11. docs/16-quality/sprint-59-product-announcements-release-notes-change-communication-review.md

Implement only:
- product communication feature registration foundation
- announcement metadata foundation
- release note metadata foundation
- change notice metadata foundation
- maintenance notice placeholder foundation
- audience targeting placeholder where practical
- acknowledgement/read receipt placeholder where practical
- feature flag/configuration linkage placeholder where practical
- knowledge base/support link placeholder where practical
- communication delivery/status placeholder where practical
- permission and feature checks
- audit records for announcement and release communication actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- marketing campaign automation
- public newsletter platform
- unrestricted global announcement publishing
- bypass of Notification Center preferences
- bypass of organization visibility rules
- automatic legal/product liability claims
- arbitrary script widgets in announcements
- unrelated Commerce/POS expansion

Recommended branch:
feature/product-announcements-release-notes-change-communication-foundation

Recommended PR title:
feat: add product announcements release notes change communication foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Announcement/Notification/Feature Flag/Support boundaries followed
- audience targeting and acknowledgement rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused change communication foundation for announcements, release notes, change notices, maintenance notices, audience targeting, read receipts, feature/config links, knowledge base/support links, delivery status, and audit-safe product communication operations.

## Final Rule

Product change communication must be targeted, permission-aware, auditable, and connected to Notification Center, Knowledge Base, Feature Flags, Configuration, and Support Operations. It should not become a marketing automation system.
