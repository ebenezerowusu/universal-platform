# Sprint 47 Background Jobs Queues Scheduler Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 47.

Sprint 47 prepares the Background Jobs, Queues, Scheduler, and Async Operations foundation after developer integrations, webhook delivery, imports/exports, notifications, reporting, payment reconciliation, backup/restore, monitoring, and workflow foundations have introduced shared asynchronous execution needs.

## Prompt

```text
You are implementing Sprint 47 for Universal Platform.

Your task is to implement the Background Jobs, Queues, Scheduler, and Async Operations foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/05-infrastructure/05-deployment-strategy.md
3. docs/12-api/developer-api-webhooks-integrations-api-contract.md
4. docs/12-api/import-export-data-migration-api-contract.md
5. docs/12-api/monitoring-observability-incident-api-contract.md
6. docs/14-roadmap/sprint-47-background-jobs-queues-scheduler-roadmap.md
7. docs/13-module-sdk/background-jobs-queues-scheduler-readiness.md
8. docs/12-api/background-jobs-queues-scheduler-api-contract.md
9. docs/11-ui-ux/background-jobs-queues-scheduler-ui-contract.md
10. docs/05-infrastructure/background-jobs-queues-scheduler-guide.md
11. docs/16-quality/sprint-47-background-jobs-queues-scheduler-review.md

Implement only:
- async operations feature registration foundation
- job definition metadata foundation
- job run metadata foundation
- queue metadata foundation
- worker metadata placeholder where practical
- retry policy placeholder foundation
- dead-letter job placeholder foundation
- scheduled job metadata foundation
- recurring schedule placeholder where practical
- job dependency/correlation placeholder where practical
- job progress/status history foundation
- permission and feature checks
- audit records for sensitive job actions
- monitoring hooks/placeholders where practical
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- arbitrary script execution
- unrestricted admin job runner
- direct provider-specific queue logic inside domain modules
- full workflow engine replacement
- production autoscaling controller
- cross-organization job visibility
- destructive retry behavior without idempotency rules
- unrelated Commerce/POS expansion

Recommended branch:
feature/background-jobs-queues-scheduler-foundation

Recommended PR title:
feat: add background jobs queues scheduler foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Async/Monitoring/Audit boundaries followed
- retry/dead-letter/idempotency rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused async operations foundation for job definitions, job runs, queue metadata, worker placeholders, retry policies, dead-letter placeholders, scheduled jobs, recurring schedules, progress history, monitoring hooks, and audit-safe job operations.

## Final Rule

Async operations must be organization-aware where relevant, provider-agnostic, observable, auditable, retry-safe, and idempotency-conscious. Domains should enqueue approved job types through shared async boundaries rather than implementing isolated background execution.
