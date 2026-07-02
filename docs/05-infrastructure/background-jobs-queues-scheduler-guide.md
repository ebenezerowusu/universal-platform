# Background Jobs Queues Scheduler Guide

## Purpose

This document defines operating guidance for job definitions, job runs, queues, worker placeholders, retry policies, dead-letter placeholders, scheduled jobs, recurring schedules, and job history.

The goal is to make async execution visible, reliable, and safe.

## Principle

Long-running, retryable, scheduled, or external-delivery work should run through shared async boundaries.

Async operations must be observable and idempotency-conscious.

## Data Sources

Async operations may use:

- job definition metadata
- job run metadata
- queue metadata
- worker metadata placeholder
- retry policy placeholder
- dead-letter job placeholder
- scheduled job metadata
- recurring schedule placeholder
- job dependency/correlation placeholder
- job progress/status history
- audit records

## Job Definition Direction

Job definitions should capture:

```text
job_key
title
module_domain_owner
queue_key
idempotency_required_placeholder
timeout_placeholder
status
```

## Job Run Direction

Job runs should capture:

```text
job_key
organization_context optional
source_reference optional
status
started_at optional
finished_at optional
attempt_count
correlation_id_placeholder
idempotency_key_placeholder
```

## Queue Direction

Queue metadata should capture:

```text
queue_key
queue_name
priority_placeholder
concurrency_placeholder
status
```

## Worker Direction

Worker placeholders should capture:

```text
worker_key
queue_key
status
last_heartbeat_placeholder
current_job_placeholder
```

## Retry Direction

Retry policies should define:

- max attempts
- backoff strategy placeholder
- retryable error type placeholder
- idempotency required flag
- manual review rules where needed

Do not retry destructive jobs unless idempotency and safe side-effect rules are defined.

## Dead-Letter Direction

Dead-letter placeholders should capture:

- original job run
- failure summary
- final attempt count
- review status
- retry eligibility placeholder
- reviewed by optional

## Scheduler Direction

Scheduled jobs should capture:

```text
schedule_key
job_key
next_run_placeholder
recurrence_placeholder
timezone_placeholder
status
```

Use organization timezone where relevant.

## Monitoring Direction

Monitoring hooks may surface:

- failed job spikes
- stale workers
- dead-letter count
- schedule missed run placeholder
- queue backlog placeholder

## Audit Direction

Audit should record:

- job definition changed
- schedule created or updated
- job cancelled placeholder
- retry requested placeholder
- dead-letter reviewed placeholder
- queue metadata changed
- worker marked reviewed placeholder

## Data Quality Warnings

Warn or flag when:

- job has no idempotency key but retry is enabled
- schedule timezone is missing
- worker heartbeat is stale
- queue backlog is high placeholder
- dead-letter job has not been reviewed
- job source reference is missing
- retry requested for unsafe job type

## Final Rule

Async operations must make background work repeatable, inspectable, and safe before advanced automation is introduced.
