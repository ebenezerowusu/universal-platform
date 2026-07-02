# Backup Restore Disaster Recovery API Contract

## Purpose

This document defines the MVP API contract for Backup, Restore, and Disaster Recovery foundation.

The API should support backup policies, backup runs, verification placeholders, restore request placeholders, restore drills, recovery objectives, and readiness checklist placeholders while preserving organization boundaries and platform safety.

## Owner

```text
Backup Restore Foundation
Storage Engine where backup artifacts are referenced
Permission Engine
Audit Engine
Workflow Engine where restore approvals are used later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/recovery
```

## Backup Policies

```text
GET /api/v1/organizations/{organizationId}/recovery/backup-policies
POST /api/v1/organizations/{organizationId}/recovery/backup-policies
GET /api/v1/organizations/{organizationId}/recovery/backup-policies/{policyId}
PATCH /api/v1/organizations/{organizationId}/recovery/backup-policies/{policyId}
```

### Create Backup Policy Request

```json
{
  "policyKey": "daily_database_backup",
  "scope": "organization_data_placeholder",
  "frequencyPlaceholder": "daily",
  "retentionPeriodPlaceholder": "30_days",
  "storageTargetPlaceholder": "local_or_external_placeholder",
  "status": "active"
}
```

## Backup Runs

```text
GET /api/v1/organizations/{organizationId}/recovery/backup-runs
POST /api/v1/organizations/{organizationId}/recovery/backup-runs
GET /api/v1/organizations/{organizationId}/recovery/backup-runs/{backupRunId}
PATCH /api/v1/organizations/{organizationId}/recovery/backup-runs/{backupRunId}
```

### Create Backup Run Metadata Request

```json
{
  "policyId": "...",
  "startedAt": "2026-07-02T02:00:00Z",
  "status": "running_placeholder",
  "notes": "Optional note"
}
```

## Verification Placeholder

```text
POST /api/v1/organizations/{organizationId}/recovery/backup-runs/{backupRunId}/verification
GET /api/v1/organizations/{organizationId}/recovery/backup-runs/{backupRunId}/verification
```

### Request Direction

```json
{
  "verificationStatus": "passed_placeholder",
  "checksumPlaceholder": "optional",
  "notes": "Optional note"
}
```

## Restore Requests

```text
GET /api/v1/organizations/{organizationId}/recovery/restore-requests
POST /api/v1/organizations/{organizationId}/recovery/restore-requests
GET /api/v1/organizations/{organizationId}/recovery/restore-requests/{restoreRequestId}
POST /api/v1/organizations/{organizationId}/recovery/restore-requests/{restoreRequestId}/approve-placeholder
POST /api/v1/organizations/{organizationId}/recovery/restore-requests/{restoreRequestId}/reject-placeholder
```

### Create Restore Request

```json
{
  "requestType": "restore_drill_placeholder",
  "scope": "organization_data_placeholder",
  "backupRunId": "optional",
  "targetRecoveryPointPlaceholder": "2026-07-02T02:00:00Z",
  "reason": "Optional reason"
}
```

## Restore Drills

```text
GET /api/v1/organizations/{organizationId}/recovery/restore-drills
POST /api/v1/organizations/{organizationId}/recovery/restore-drills
GET /api/v1/organizations/{organizationId}/recovery/restore-drills/{drillId}
PATCH /api/v1/organizations/{organizationId}/recovery/restore-drills/{drillId}
```

## Recovery Objectives

```text
GET /api/v1/organizations/{organizationId}/recovery/objectives
POST /api/v1/organizations/{organizationId}/recovery/objectives
PATCH /api/v1/organizations/{organizationId}/recovery/objectives/{objectiveId}
```

## Readiness Checklist

```text
GET /api/v1/organizations/{organizationId}/recovery/readiness-checklist
PATCH /api/v1/organizations/{organizationId}/recovery/readiness-checklist/{itemId}
```

## Incident Recovery Notes

```text
GET /api/v1/organizations/{organizationId}/recovery/incident-notes
POST /api/v1/organizations/{organizationId}/recovery/incident-notes
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- backup/restore feature availability
- required permission
- organization ownership
- restore approval placeholder permission where used

Restore requests must not cross organization boundaries.

## Status Direction

Suggested backup run statuses:

```text
scheduled_placeholder
running_placeholder
completed_placeholder
failed
verification_pending
verified_placeholder
manual_review
```

Suggested restore request statuses:

```text
submitted
reviewing_placeholder
approved_placeholder
rejected_placeholder
drill_only_placeholder
completed_placeholder
cancelled
manual_review
```

Suggested drill statuses:

```text
planned
in_progress_placeholder
passed_placeholder
failed
needs_follow_up
manual_review
```

## Audit Direction

Audit should be written for:

- backup policy created or updated
- backup run metadata recorded
- verification status updated
- restore request created
- restore request approved/rejected placeholder
- restore drill completed
- recovery objective changed
- incident recovery note added

## Error Direction

Use standard response shape.

Expected error areas:

- backup policy not found
- backup run not found
- restore request not found
- restore drill not found
- verification not available
- restore approval denied
- organization not found

## Final Rule

Backup/restore APIs must be organization-scoped, permission-protected, non-destructive by default, and auditable.
