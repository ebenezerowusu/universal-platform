# Sprint 38 Monitoring Observability Incident Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 38.

Sprint 38 prepares the Monitoring, Observability, and Incident Response foundation after backup/restore, audit, reporting, workflow, notifications, commerce, finance, HR, religious, and operations modules have introduced critical service reliability needs.

## Prompt

```text
You are implementing Sprint 38 for Universal Platform.

Your task is to implement the Monitoring, Observability, and Incident Response foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/05-infrastructure/05-deployment-strategy.md
3. docs/05-infrastructure/incident-response-playbook.md
4. docs/04-capability-engines/notification-engine.md
5. docs/04-capability-engines/audit-engine.md
6. docs/14-roadmap/sprint-38-monitoring-observability-incident-roadmap.md
7. docs/13-module-sdk/monitoring-observability-incident-readiness.md
8. docs/12-api/monitoring-observability-incident-api-contract.md
9. docs/11-ui-ux/monitoring-observability-incident-ui-contract.md
10. docs/05-infrastructure/monitoring-observability-incident-operations-guide.md
11. docs/16-quality/sprint-38-monitoring-observability-incident-review.md

Implement only:
- monitoring feature registration foundation
- service health check metadata foundation
- service status snapshot foundation
- alert rule metadata foundation
- alert event foundation
- incident record foundation
- incident timeline/update foundation
- runbook link/reference placeholder where practical
- post-incident review placeholder where practical
- notification trigger placeholder where practical
- permission and feature checks
- audit records for incident actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full Prometheus/Grafana deployment
- external monitoring provider integration
- live paging/on-call vendor integration
- automatic remediation engine
- arbitrary script execution
- cross-organization incident visibility
- provider-specific monitoring calls inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/monitoring-observability-incident-foundation

Recommended PR title:
feat: add monitoring observability incident foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Monitoring/Notification/Audit boundaries followed
- incident safety rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused monitoring and incident foundation for health checks, status snapshots, alert metadata, incident records, timelines, runbook references, notification placeholders, and post-incident review placeholders.

## Final Rule

Monitoring and incident response must be organization-aware where relevant, permission-protected, auditable, and provider-agnostic. Service health should be visible before automated remediation is introduced.
