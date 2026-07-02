# Sprint 26 Delivery Logistics Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 26.

Sprint 26 prepares Delivery and Logistics foundation after Order Fulfillment and Pickup foundation.

## Prompt

```text
You are implementing Sprint 26 for Universal Platform.

Your task is to implement the Delivery and Logistics foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/commerce-domain.md
3. docs/12-api/order-fulfillment-api-contract.md
4. docs/05-infrastructure/order-fulfillment-reconciliation-playbook.md
5. docs/14-roadmap/sprint-26-delivery-logistics-roadmap.md
6. docs/13-module-sdk/delivery-logistics-readiness.md
7. docs/12-api/delivery-logistics-api-contract.md
8. docs/11-ui-ux/delivery-logistics-ui-contract.md
9. docs/05-infrastructure/delivery-logistics-reconciliation-playbook.md
10. docs/16-quality/sprint-26-delivery-logistics-review.md

Implement only:
- delivery feature registration foundation
- delivery zone foundation
- delivery fee rule foundation
- customer delivery address foundation
- delivery request foundation linked to order
- delivery assignment placeholder where practical
- delivery status foundation
- proof-of-delivery placeholder where practical
- customer notification trigger foundation where practical
- permission and feature checks
- audit records for important delivery actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- live rider tracking
- route optimization
- driver payout management
- ride-hailing workflows
- third-party logistics integrations
- marketplace commission settlement
- advanced return/refund logistics
- direct payment provider calls inside Logistics or Commerce Domain
- unrelated POS expansion

Recommended branch:
feature/delivery-logistics-foundation

Recommended PR title:
feat: add delivery logistics foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- delivery/order boundaries followed
- notification and audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused delivery foundation for delivery zones, fee rules, delivery requests, status tracking, assignment placeholders, and proof-of-delivery direction.

## Final Rule

Delivery must extend order fulfillment safely without becoming full ride-hailing, live dispatch, or driver payout management in this sprint.
