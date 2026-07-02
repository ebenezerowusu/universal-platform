# Sprint 46 Developer API Webhooks Integrations Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 46.

Sprint 46 prepares the Developer API Access, Webhooks, and Integration Operations foundation after authentication session/device security, user access lifecycle, payment operations, notifications, audit, and monitoring foundations have introduced controlled external access needs.

## Prompt

```text
You are implementing Sprint 46 for Universal Platform.

Your task is to implement the Developer API Access, Webhooks, and Integration Operations foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/08-security/08-security-standards.md
3. docs/12-api/auth-session-device-security-api-contract.md
4. docs/12-api/user-membership-access-lifecycle-api-contract.md
5. docs/04-capability-engines/audit-engine.md
6. docs/04-capability-engines/notification-engine.md
7. docs/14-roadmap/sprint-46-developer-api-webhooks-integrations-roadmap.md
8. docs/13-module-sdk/developer-api-webhooks-integrations-readiness.md
9. docs/12-api/developer-api-webhooks-integrations-api-contract.md
10. docs/11-ui-ux/developer-api-webhooks-integrations-ui-contract.md
11. docs/05-infrastructure/developer-api-webhooks-integrations-guide.md
12. docs/16-quality/sprint-46-developer-api-webhooks-integrations-review.md

Implement only:
- developer/integration feature registration foundation
- API client/application metadata foundation
- scoped access key placeholder foundation
- API scope registry placeholder where practical
- webhook subscription foundation
- webhook event catalog foundation
- webhook delivery attempt foundation
- webhook retry/dead-letter placeholder where practical
- inbound integration request placeholder where practical
- rate limit/quota metadata placeholder where practical
- integration audit/event history foundation
- permission and feature checks
- audit records for sensitive developer access actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- unrestricted API key generation
- raw secret display after creation
- external OAuth provider implementation
- public app marketplace
- arbitrary script execution
- provider-specific integration SDK calls inside domain modules
- cross-organization webhook visibility
- unrelated Commerce/POS expansion

Recommended branch:
feature/developer-api-webhooks-integrations-foundation

Recommended PR title:
feat: add developer API webhooks integrations foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Identity/Permission/Audit/Notification boundaries followed
- API key/webhook safety rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused developer integration foundation for API clients, scoped access keys, scope registry placeholders, webhook subscriptions, event catalogs, delivery attempts, retry/dead-letter placeholders, inbound integration placeholders, rate limit metadata, and integration audit history.

## Final Rule

Developer access must be organization-scoped, permission-protected, auditable, rate-limit-aware, and secret-safe. Domain modules should emit events through shared integration boundaries instead of creating isolated webhook behavior.
