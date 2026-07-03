# Customer Success Account Health Adoption Playbooks API Contract

## Purpose

This document defines the MVP API contract for Customer Success, Account Health, and Adoption Playbooks foundation.

The API should support customer success dashboards, account health summaries, adoption signals, risk signals, success playbooks, playbook tasks, interventions/actions, renewal/support/product links, success notes, and customer success audit history while preserving tenant boundaries.

## Owner

```text
Customer Success Foundation
Permission Engine
Audit Engine
Product Analytics Foundation
Support Operations Foundation
Subscription Billing Foundation
Feedback Intake Foundation
Knowledge Base Foundation
Product Communication Foundation
Notification and Workflow Foundations where tasks/alerts are consumed later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/customer-success
```

Platform-level customer success views may later use stricter platform-admin routes.

## Customer Success Dashboard

```text
GET /api/v1/organizations/{organizationId}/customer-success/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "healthyAccounts": 18,
    "atRiskAccounts": 4,
    "openPlaybookTasks": 21,
    "renewalWarnings": 3,
    "supportEscalationWarnings": 2
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Account Health Summaries

```text
GET /api/v1/organizations/{organizationId}/customer-success/account-health
POST /api/v1/organizations/{organizationId}/customer-success/account-health
GET /api/v1/organizations/{organizationId}/customer-success/account-health/{healthId}
PATCH /api/v1/organizations/{organizationId}/customer-success/account-health/{healthId}
POST /api/v1/organizations/{organizationId}/customer-success/account-health/{healthId}/mark-reviewed-placeholder
```

## Adoption Health Signals

```text
GET /api/v1/organizations/{organizationId}/customer-success/adoption-signals
POST /api/v1/organizations/{organizationId}/customer-success/account-health/{healthId}/adoption-signals
GET /api/v1/organizations/{organizationId}/customer-success/adoption-signals/{signalId}
POST /api/v1/organizations/{organizationId}/customer-success/adoption-signals/{signalId}/mark-reviewed-placeholder
```

## Risk Signals

```text
GET /api/v1/organizations/{organizationId}/customer-success/risk-signals
POST /api/v1/organizations/{organizationId}/customer-success/account-health/{healthId}/risk-signals
GET /api/v1/organizations/{organizationId}/customer-success/risk-signals/{signalId}
POST /api/v1/organizations/{organizationId}/customer-success/risk-signals/{signalId}/mark-reviewed-placeholder
```

## Success Playbooks

```text
GET /api/v1/organizations/{organizationId}/customer-success/playbooks
POST /api/v1/organizations/{organizationId}/customer-success/playbooks
GET /api/v1/organizations/{organizationId}/customer-success/playbooks/{playbookId}
PATCH /api/v1/organizations/{organizationId}/customer-success/playbooks/{playbookId}
POST /api/v1/organizations/{organizationId}/customer-success/playbooks/{playbookId}/activate-placeholder
POST /api/v1/organizations/{organizationId}/customer-success/playbooks/{playbookId}/retire-placeholder
```

## Playbook Tasks

```text
GET /api/v1/organizations/{organizationId}/customer-success/playbook-tasks
POST /api/v1/organizations/{organizationId}/customer-success/playbooks/{playbookId}/tasks
GET /api/v1/organizations/{organizationId}/customer-success/playbook-tasks/{taskId}
PATCH /api/v1/organizations/{organizationId}/customer-success/playbook-tasks/{taskId}
POST /api/v1/organizations/{organizationId}/customer-success/playbook-tasks/{taskId}/complete-placeholder
```

## Interventions and Actions

```text
GET /api/v1/organizations/{organizationId}/customer-success/interventions
POST /api/v1/organizations/{organizationId}/customer-success/interventions
GET /api/v1/organizations/{organizationId}/customer-success/interventions/{interventionId}
PATCH /api/v1/organizations/{organizationId}/customer-success/interventions/{interventionId}
POST /api/v1/organizations/{organizationId}/customer-success/interventions/{interventionId}/complete-placeholder
```

## Renewal Support Product Links

```text
GET /api/v1/organizations/{organizationId}/customer-success/related-links
POST /api/v1/organizations/{organizationId}/customer-success/related-links
PATCH /api/v1/organizations/{organizationId}/customer-success/related-links/{linkId}
```

## Success Notes

```text
GET /api/v1/organizations/{organizationId}/customer-success/success-notes
POST /api/v1/organizations/{organizationId}/customer-success/success-notes
GET /api/v1/organizations/{organizationId}/customer-success/success-notes/{noteId}
PATCH /api/v1/organizations/{organizationId}/customer-success/success-notes/{noteId}
```

## Customer Success Audit History

```text
GET /api/v1/organizations/{organizationId}/customer-success/audit-history
GET /api/v1/organizations/{organizationId}/customer-success/audit-history/{historyId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership where applicable
- customer success feature availability
- required permission
- health records belong to visible organization scope
- linked support/billing/product records are visible to actor
- account health signals do not trigger billing changes automatically
- health/risk signals are treated as reviewable indicators only

Customer success records must not expose unauthorized cross-tenant account details.

## Status Direction

Suggested health statuses:

```text
healthy_placeholder
watch_placeholder
at_risk_placeholder
unknown_placeholder
manual_review
```

Suggested playbook statuses:

```text
draft
active_placeholder
retired_placeholder
manual_review
```

Suggested task statuses:

```text
open
in_progress_placeholder
completed_placeholder
overdue_placeholder
cancelled_placeholder
manual_review
```

Suggested intervention statuses:

```text
planned_placeholder
in_progress_placeholder
completed_placeholder
cancelled_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- account health summary created or updated
- adoption signal reviewed placeholder
- risk signal reviewed placeholder
- success playbook created or updated
- playbook task created or completed placeholder
- intervention action created or completed placeholder
- renewal/support/product link changed
- success note added placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- account health summary not found
- adoption signal not found
- risk signal not found
- playbook not found
- playbook task not found
- intervention not found
- related link not found
- success note not found
- customer success permission denied
- organization not found

## Final Rule

Customer success APIs must be tenant-safe, permission-protected, support-linked, billing-aware but non-mutating, privacy-aware, and auditable.
