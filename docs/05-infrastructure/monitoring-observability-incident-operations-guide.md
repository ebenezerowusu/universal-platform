# Monitoring Observability Incident Operations Guide

## Purpose

This document defines operating guidance for service health checks, status snapshots, alert events, incident records, incident timelines, runbook references, and post-incident reviews.

The goal is to make reliability visible, response traceable, and follow-up accountable.

## Principle

A service issue should produce a clear path from signal to alert to incident to resolution to learning.

## Data Sources

Monitoring operations may use:

- service health metadata
- service status snapshot
- alert rule metadata
- alert event
- incident record
- incident timeline update
- runbook reference placeholder
- post-incident review placeholder
- audit records

## Service Health Direction

Service health metadata should capture:

```text
service_key
service_name
category
owner_placeholder
check_endpoint_placeholder
status
```

## Status Snapshot Direction

Status snapshots should capture:

```text
service_key
status
checked_at
latency_placeholder
message_placeholder
source_placeholder
```

## Alert Direction

Alert events should capture:

```text
alert_rule_id
service_key
severity
status
triggered_at
resolved_at optional
message
```

## Incident Direction

Incident records should capture:

```text
incident_key
title
severity
status
affected_services
started_at
resolved_at optional
owner_placeholder
```

## Timeline Direction

Incident timeline updates should capture:

```text
incident_id
update_type
message
actor
created_at
status_after_update optional
```

## Runbook Direction

Runbook references should be safe and provider-neutral.

They may reference internal docs, repository docs, or future knowledge-base records.

## Notification Direction

Notifications may be triggered for:

- alert opened
- alert resolved placeholder
- incident created
- incident severity changed
- incident status changed
- post-incident review assigned placeholder

Use Notification Engine where practical.

## Audit Direction

Audit should record:

- alert rule changed
- incident created
- incident severity changed
- incident status changed
- incident timeline update added
- post-incident review completed placeholder
- runbook reference changed

## Data Quality Warnings

Warn or flag when:

- service has no recent status snapshot
- alert has no owner placeholder
- incident has no timeline update
- severe incident has no post-incident review placeholder
- runbook reference is missing for critical service

## Final Rule

Monitoring operations must create traceable response records and should not hide reliability issues inside logs only.
