# Sprint 18 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 18 after SMS wallet and provider readiness foundations exist.

Sprint 18 should implement Communication Governance: recipient preferences, opt-out handling, message templates, recipient segmentation, and safe send rules.

## Prompt

```text
You are implementing Sprint 18 for Universal Platform.

Your task is to implement Communication Governance foundations.

Required reading before coding:
1. CLAUDE.md
2. docs/01-platform-constitution/01-platform-constitution.md
3. docs/14-roadmap/sprint-18-communication-governance-roadmap.md
4. docs/13-module-sdk/communication-governance-feature-readiness.md
5. docs/12-api/communication-governance-api-contract.md
6. docs/11-ui-ux/communication-governance-flutter-contract.md
7. docs/16-quality/sprint-18-communication-governance-review.md
8. docs/12-api/communication-sms-api-contract.md
9. docs/04-capability-engines/sms-engine.md
10. docs/04-capability-engines/audit-engine.md

Implement only:
- communication preference foundation
- recipient opt-out foundation
- message template foundation
- recipient segment foundation
- recipient preview filters
- safe send rule checks
- audit records for preference/template/segment changes
- permission and feature checks
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests for opt-out, template, segment, and safe-send behavior where practical

Do not implement:
- advanced campaign automation
- WhatsApp or email campaigns
- AI message generation
- provider credential screens
- direct provider calls from domain logic
- message sending that ignores recipient preferences
- public marketing workflows beyond MVP governance

Recommended branch:
feature/communication-governance-foundation

Recommended PR title:
feat: add communication governance foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- safe-send rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused governance foundation that protects recipients, improves communication quality, and prepares the platform for future communication workflows.

## Final Rule

No message should be sent by future communication features unless preferences, permissions, feature availability, wallet rules, and audit expectations are respected.
