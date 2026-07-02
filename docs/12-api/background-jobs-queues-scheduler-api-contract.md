# Background Jobs Queues Scheduler API Contract

## Purpose

This document defines the MVP API contract for Background Jobs, Queues, Scheduler, and Async Operations foundation.

The API should support async dashboard summaries, job definitions, job runs, queue metadata, worker placeholders, scheduled jobs, retry policies, dead-letter placeholders, and job history while preserving organization boundaries.

## Owner

```text
Async Operations Foundation
Permission Engine
Audit Engine
Monitoring/Incident Foundation where job health is surfaced
```

## Base Path

```text
/api/v1/organizations/{organizationId}/async-operations
```

Platform-admin global operational routes may be introduced later with stricter controls.

## Async Dashboard

```text
GET /api/v1/organizations/{organizationId}/async-operations/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "queuedJobs": 12,
    "runningJobs": 3,
    "failedJobs": 2,
    "deadLetterJobs": 1,
    "scheduledJobs": 5
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Job Definitions

```text
GET /api/v1/organizations/{organizationId}/async-operations/job-definitions
POST /api/v1/organizations/{organizationId}/async-operations/job-definitions
GET /api/v1/organizations/{organizationId}/async-operations/job-definitions/{jobKey}
PATCH /api/v1/organizations/{organizationId}/async-operations/job-definitions/{jobKey}
```

Job definition create/update should be platform-admin or async-admin only where applicable.

## Job Runs

```text
GET /api/v1/organizations/{organizationId}/async-operations/job-runs
POST /api/v1/organizations/{organizationId}/async-operations/job-runs
GET /api/v1/organizations/{organizationId}/async-operations/job-runs/{jobRunId}
POST /api/v1/organizations/{organizationId}/async-operations/job-runs/{jobRunId}/cancel-placeholder
POST /api/v1/organizations/{organizationId}/async-operations/job-runs/{jobRunId}/retry-placeholder
```

### Create Job Run Request

```json
{
  "jobKey": "report.export_placeholder",
  "sourceReference": {
    "domain": "reporting",
    "type": "export_request",
    "id": "..."
  },
  "correlationIdPlaceholder": "optional",
  "idempotencyKeyPlaceholder": "optional"
}
```

## Queues

```text
GET /api/v1/organizations/{organizationId}/async-operations/queues
GET /api/v1/organizations/{organizationId}/async-operations/queues/{queueKey}
PATCH /api/v1/organizations/{organizationId}/async-operations/queues/{queueKey}
```

## Worker Placeholder

```text
GET /api/v1/organizations/{organizationId}/async-operations/workers
GET /api/v1/organizations/{organizationId}/async-operations/workers/{workerKey}
POST /api/v1/organizations/{organizationId}/async-operations/workers/{workerKey}/mark-reviewed-placeholder
```

## Scheduled Jobs

```text
GET /api/v1/organizations/{organizationId}/async-operations/schedules
POST /api/v1/organizations/{organizationId}/async-operations/schedules
GET /api/v1/organizations/{organizationId}/async-operations/schedules/{scheduleId}
PATCH /api/v1/organizations/{organizationId}/async-operations/schedules/{scheduleId}
POST /api/v1/organizations/{organizationId}/async-operations/schedules/{scheduleId}/disable-placeholder
```

## Retry Policies

```text
GET /api/v1/organizations/{organizationId}/async-operations/retry-policies
POST /api/v1/organizations/{organizationId}/async-operations/retry-policies
PATCH /api/v1/organizations/{organizationId}/async-operations/retry-policies/{policyId}
```

## Dead-Letter Jobs

```text
GET /api/v1/organizations/{organizationId}/async-operations/dead-letter-jobs
GET /api/v1/organizations/{organizationId}/async-operations/dead-letter-jobs/{deadLetterId}
POST /api/v1/organizations/{organizationId}/async-operations/dead-letter-jobs/{deadLetterId}/mark-reviewed
POST /api/v1/organizations/{organizationId}/async-operations/dead-letter-jobs/{deadLetterId}/retry-placeholder
```

## Job History

```text
GET /api/v1/organizations/{organizationId}/async-operations/job-runs/{jobRunId}/history
POST /api/v1/organizations/{organizationId}/async-operations/job-runs/{jobRunId}/history
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- async operations feature availability
- required permission
- job run belongs to organization where organization context exists
- source reference belongs to organization where applicable
- retry is allowed only for idempotent or approved job types

Job visibility must not cross organization boundaries.

## Status Direction

Suggested job run statuses:

```text
queued
running
retrying_placeholder
completed
failed
cancelled_placeholder
dead_letter_placeholder
manual_review
```

Suggested schedule statuses:

```text
active
paused_placeholder
disabled_placeholder
failed
manual_review
```

Suggested worker statuses:

```text
online_placeholder
offline_placeholder
busy_placeholder
stale_heartbeat_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- job definition changed
- schedule created or updated
- job cancelled placeholder
- retry requested placeholder
- dead-letter reviewed placeholder
- queue metadata changed
- worker marked reviewed placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- job definition not found
- job run not found
- queue not found
- worker not found
- schedule not found
- retry policy not found
- dead-letter job not found
- retry not allowed for non-idempotent job
- async permission denied
- organization not found

## Final Rule

Async operations APIs must be organization-scoped where relevant, permission-protected, provider-agnostic, observable, retry-safe, and auditable.
