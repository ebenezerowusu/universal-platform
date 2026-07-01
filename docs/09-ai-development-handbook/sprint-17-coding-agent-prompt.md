# Sprint 17 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 17 after the Communication and SMS Wallet foundation is in place.

Sprint 17 should prepare real SMS provider adapter readiness, delivery tracking, and reconciliation while keeping providers behind the SMS Engine.

## Prompt

```text
You are implementing Sprint 17 for Universal Platform.

Your task is to implement SMS provider adapter readiness and delivery tracking foundations.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/sms-engine.md
3. docs/04-capability-engines/provider-adapter-contracts.md
4. docs/14-roadmap/sprint-17-sms-provider-readiness-roadmap.md
5. docs/04-capability-engines/sms-provider-adapter-contract.md
6. docs/05-infrastructure/sms-delivery-reconciliation-playbook.md
7. docs/05-infrastructure/sms-provider-operations-guide.md
8. docs/12-api/sms-provider-admin-api-contract.md
9. docs/16-quality/sprint-17-provider-readiness-review.md
10. docs/12-api/communication-sms-api-contract.md

Implement only:
- SMS provider adapter interface refinements
- local/mock provider behavior improvements
- provider send result normalization
- delivery status foundation
- delivery callback/webhook receiving foundation where practical
- reconciliation job foundation
- provider health check foundation
- provider admin operation notes or endpoints where approved
- audit records for provider configuration and manual wallet adjustments
- tests for adapter normalization, delivery status, and reconciliation behavior where practical

Do not implement:
- provider calls directly inside Communication Domain
- provider details in Flutter screens
- organization-managed provider credentials
- live provider credentials in repository
- real payment collection
- WhatsApp integration
- email campaigns
- complex marketing automation

Recommended branch:
feature/sms-provider-readiness

Recommended PR title:
feat: add SMS provider readiness foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- adapter behavior added
- delivery tracking added
- reconciliation behavior added
- tests added
- checks run
- provider abstraction rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused provider-readiness foundation that allows real providers to be added safely behind SMS Engine without exposing provider details to domains or Flutter.

## Final Rule

Sprint 17 prepares real delivery reliability. Provider-specific behavior belongs only inside provider adapters and system-admin operations.
