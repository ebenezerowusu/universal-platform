# Sprint 19 Coding Agent Prompt

## Purpose

This document is a copy-paste-ready prompt for Claude Code or another coding agent to begin Sprint 19 after SMS wallet, provider readiness, and communication governance foundations exist.

Sprint 19 should connect SMS package requests to Payment Engine readiness, sales reconciliation, wallet crediting, and finance/admin review.

## Prompt

```text
You are implementing Sprint 19 for Universal Platform.

Your task is to implement SMS package purchase and payment-readiness foundations.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/payment-engine.md
3. docs/04-capability-engines/sms-engine.md
4. docs/14-roadmap/sprint-19-sms-payment-readiness-roadmap.md
5. docs/13-module-sdk/sms-package-payment-feature-readiness.md
6. docs/12-api/sms-package-payment-api-contract.md
7. docs/11-ui-ux/sms-package-payment-flutter-contract.md
8. docs/05-infrastructure/sms-package-reconciliation-playbook.md
9. docs/16-quality/sprint-19-sms-payment-review.md
10. docs/12-api/communication-sms-api-contract.md

Implement only:
- SMS package purchase request foundation
- Payment Engine initiation foundation for SMS packages
- payment status tracking foundation
- wallet crediting after confirmed payment
- package purchase ledger foundation
- reconciliation between payment records and wallet credit records
- system/admin review for unresolved package purchases
- audit records for payment status and wallet credit actions
- permission and feature checks
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests for purchase request, payment status, wallet credit, and reconciliation behavior where practical

Do not implement:
- direct payment provider calls inside Communication Domain
- hardcoded Hubtel, Paystack, M-Pesa, or any provider logic in domain workflows
- live provider credentials in repository
- SMS wallet credit before confirmed payment unless manually approved and audited
- unrelated subscription plan overhaul
- advanced invoicing beyond MVP package purchase records
- WhatsApp or email campaigns

Recommended branch:
feature/sms-package-payment-readiness

Recommended PR title:
feat: add SMS package payment readiness

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Payment Engine boundaries followed
- wallet crediting rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused payment-readiness foundation that allows SMS package purchases to be tracked, paid, reconciled, and credited safely.

## Final Rule

SMS package revenue must flow through Payment Engine and auditable wallet crediting. Communication Domain must not own provider-specific payment behavior.
