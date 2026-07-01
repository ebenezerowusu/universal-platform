# Sprint 16 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 16 after Sprint 15 selects the Communication and SMS Wallet feature wave.

Sprint 16 should prepare and implement the foundation for communication and SMS wallet behavior without provider lock-in.

## Prompt

```text
You are implementing Sprint 16 for Universal Platform.

Your task is to implement the Communication and SMS Wallet foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/04-capability-engines/sms-engine.md
4. docs/04-capability-engines/notification-engine.md
5. docs/04-capability-engines/provider-adapter-contracts.md
6. docs/14-roadmap/sprint-16-communication-sms-wave-roadmap.md
7. docs/13-module-sdk/communication-sms-feature-wave-readiness.md
8. docs/12-api/communication-sms-api-contract.md
9. docs/11-ui-ux/communication-sms-flutter-screens-contract.md
10. docs/16-quality/sprint-16-sms-wave-review.md

Implement only:
- Communication module registration foundation
- SMS wallet balance foundation
- SMS package/catalog foundation
- SMS transaction ledger foundation
- message draft/send request foundation
- message recipient group foundation
- provider-agnostic SMS Engine usage
- local/mock SMS provider for MVP testing
- audit records for important changes
- permission and feature checks
- API routes matching the contract
- Flutter placeholders or foundations where practical
- tests for wallet, package, send request, and permission behavior where practical

Do not implement:
- direct provider-specific calls inside domain logic
- provider credential management screens
- bulk marketing automation
- advanced campaign analytics
- WhatsApp integration
- email campaigns
- real payment collection unless Payment Engine foundation is approved
- public provider switching by organization users

Recommended branch:
feature/communication-sms-wallet-foundation

Recommended PR title:
feat: add communication SMS wallet foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- provider abstraction rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused foundation that enables organizations to understand SMS balance, prepare message sending, and use the SMS Engine without coupling the product to one provider.

## Final Rule

Communication and SMS Wallet must be platform-engine-driven. Provider details stay behind adapters, not in domain workflows or Flutter screens.
