# Monitoring Observability Incident API Contract

## Purpose

This document defines the MVP API contract for Monitoring, Observability, and Incident Response foundation.

The API should support monitoring dashboard summaries, service health metadata, status snapshots, alert rules, alert events, incident records, incident timelines, runbook references, and post-incident review placeholders while preserving organization and platform boundaries.

## Owner

```text
Monitoring Foundation
Notification Engine where alert notifications are used
Audit Engine
Permission Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/operations/monitoring
```

## Monitoring Dashboard

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "serviceStatus": "degraded_placeholder",
    "openAlerts": 2,
    "openIncidents": 1,
    "lastCheckedAt": "..."
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Service Health Metadata

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/services
POST /api/v1/organizations/{organizationId}/operations/monitoring/services
GET /api/v1/organizations/{organizationId}/operations/monitoring/services/{serviceKey}
PATCH /api/v1/organizations/{organizationId}/operations/monitoring/services/{serviceKey}
```

## Service Status Snapshots

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/status-snapshots
POST /api/v1/organizations/{organizationId}/operations/monitoring/status-snapshots
GET /api/v1/organizations/{organizationId}/operations/monitoring/status-snapshots/{snapshotId}
```

## Alert Rules

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/alert-rules
POST /api/v1/organizations/{organizationId}/operations/monitoring/alert-rules
GET /api/v1/organizations/{organizationId}/operations/monitoring/alert-rules/{alertRuleId}
PATCH /api/v1/organizations/{organizationId}/operations/monitoring/alert-rules/{alertRuleId}
```

## Alert Events

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/alert-events
POST /api/v1/organizations/{organizationId}/operations/monitoring/alert-events
GET /api/v1/organizations/{organizationId}/operations/monitoring/alert-events/{alertEventId}
POST /api/v1/organizations/{organizationId}/operations/monitoring/alert-events/{alertEventId}/resolve-placeholder
```

## Incidents

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/incidents
POST /api/v1/organizations/{organizationId}/operations/monitoring/incidents
GET /api/v1/organizations/{organizationId}/operations/monitoring/incidents/{incidentId}
PATCH /api/v1/organizations/{organizationId}/operations/monitoring/incidents/{incidentId}
```

### Create Incident Request

```json
{
  "title": "API degraded",
  "severity": "high",
  "affectedServices": ["platform-api"],
  "summary": "Safe summary",
  "status": "open"
}
```

## Incident Timeline

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/incidents/{incidentId}/timeline
POST /api/v1/organizations/{organizationId}/operations/monitoring/incidents/{incidentId}/timeline
```

### Timeline Update Request

```json
{
  "updateType": "investigation_note",
  "message": "Investigating increased API errors",
  "statusAfterUpdate": "investigating_placeholder"
}
```

## Runbook Reference Placeholder

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/runbook-references
POST /api/v1/organizations/{organizationId}/operations/monitoring/runbook-references
PATCH /api/v1/organizations/{organizationId}/operations/monitoring/runbook-references/{runbookId}
```

## Post-Incident Review Placeholder

```text
GET /api/v1/organizations/{organizationId}/operations/monitoring/incidents/{incidentId}/review-placeholder
POST /api/v1/organizations/{organizationId}/operations/monitoring/incidents/{incidentId}/review-placeholder
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- monitoring feature availability
- required permission
- organization or platform visibility scope
- incident ownership/scope where applicable

Incidents and alerts must not expose another organization's operational details.

## Status Direction

Suggested service statuses:

```text
healthy
degraded_placeholder
unavailable
unknown
manual_review
```

Suggested alert event statuses:

```text
open
acknowledged_placeholder
resolved_placeholder
suppressed_placeholder
manual_review
```

Suggested incident statuses:

```text
open
investigating_placeholder
mitigated_placeholder
resolved_placeholder
closed_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- service check metadata changed
- alert rule changed
- alert resolved placeholder
- incident created
- incident severity changed
- incident status changed
- incident timeline update added
- post-incident review completed placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- service not found
- alert rule not found
- alert event not found
- incident not found
- runbook reference not found
- monitoring permission denied
- organization not found

## Final Rule

Monitoring APIs must be permission-protected, provider-agnostic, audit-safe, and clear about service visibility scope.
