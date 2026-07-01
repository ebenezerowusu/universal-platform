# Background Jobs and Queues Guide

## Purpose

This document defines how Universal Platform should handle background jobs and queues.

Long-running or retryable work should not block API requests unnecessarily.

## Job Principle

API requests should perform immediate validation and create clear records. Background workers should handle delayed, heavy, or retryable processing.

## Initial Queue Direction

For the MVP, Redis may be used for queue support.

The queue implementation should be hidden behind an interface so it can be replaced later if needed.

## Common Job Types

Examples:

- Send SMS messages
- Send notifications
- Generate report exports
- Process imports
- Verify payment follow-ups
- Update reporting summaries
- Run scheduled reminders
- Process file tasks

## Job Structure

A job should include:

- job_id
- job_type
- organization_id nullable
- payload
- status
- attempts
- max_attempts
- scheduled_at
- started_at nullable
- completed_at nullable
- failed_at nullable
- error_message nullable

## Job Statuses

Possible statuses:

- queued
- processing
- completed
- failed
- retrying
- cancelled

## Retry Rules

Retryable jobs should have a clear retry policy.

Examples:

- SMS provider temporarily unavailable
- Notification delivery timeout
- Report generation transient issue

Do not retry jobs indefinitely without limits.

## Idempotency

Jobs should be safe to retry where practical.

Important examples:

- Payment callbacks
- SMS sending
- Wallet crediting
- Report exports

Use idempotency keys or unique references for critical jobs.

## Worker Responsibilities

Workers should:

- Pull jobs from queue
- Execute job logic
- Update job status
- Record failures
- Publish events where needed
- Respect tenant context

## Observability

Job processing should be visible.

Track:

- queued jobs
- failed jobs
- retry count
- processing time
- job type
- organization context

## Security

Workers must respect the same platform boundaries as API requests.

They must not bypass tenant rules, permission-sensitive workflows, or audit requirements.

## Testing Requirements

Tests must cover:

- Job is queued
- Worker processes job
- Failed job records error
- Retry policy works
- Duplicate job does not create duplicate critical effect
- Tenant context is preserved

## Final Rule

Background jobs improve reliability and user experience, but they must remain observable, retry-safe, and tenant-aware.
