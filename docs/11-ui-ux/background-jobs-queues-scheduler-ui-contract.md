# Background Jobs Queues Scheduler UI Contract

## Purpose

This document defines the MVP UI contract for Background Jobs, Queues, Scheduler, and Async Operations foundation.

The screens should help permitted users review async job activity, queues, worker placeholders, scheduled jobs, retry status, dead-letter jobs, and job history.

## Navigation Direction

Async operations screens should appear when:

- user is authenticated
- organization is selected
- async operations feature is available
- user has required permission
- feature availability allows async operations visibility

## Async Operations Dashboard

Should show:

- queued jobs
- running jobs
- failed jobs
- retrying jobs
- dead-letter jobs
- scheduled jobs
- recent job warnings

## Job Run List

Should show:

- job title/key
- source reference safe label
- status
- attempt count
- queued date
- started date where available
- finished date where available
- action to view detail

## Job Run Detail

Should show:

- job metadata
- organization context safe label where available
- source reference safe label
- status
- attempt count
- correlation placeholder
- idempotency placeholder
- progress/status history
- retry/cancel placeholder actions where permitted

## Queue List

Should show:

- queue key
- queue name
- priority placeholder
- concurrency placeholder
- queued count placeholder
- status

## Worker Placeholder List

Should show:

- worker key
- queue key
- status
- last heartbeat placeholder
- current job safe label where available
- review action where permitted

## Scheduled Jobs

Should show:

- schedule key
- job key
- next run placeholder
- recurrence placeholder
- timezone placeholder
- status
- disable placeholder action where permitted

## Retry and Dead-Letter Review

Should show:

- failed job safe label
- failure reason safe summary
- attempt count
- retry policy summary
- dead-letter status
- review/retry placeholder action where permitted

## Job History

Should show:

- status change
- actor/system safe label
- reason safe summary
- timestamp
- metadata safe summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 47:

- arbitrary job execution UI
- unrestricted admin job runner UI
- production autoscaling UI
- full workflow builder UI
- provider-specific queue management UI
- cross-tenant job browser UI

## Final Rule

Async UI should make background execution visible and recoverable without giving users unsafe job execution power.
