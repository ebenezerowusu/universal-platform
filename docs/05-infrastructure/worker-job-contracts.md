# Worker Job Contracts

## Purpose

This document defines background worker job contract standards for Universal Platform.

Jobs should be observable, retry-safe, and consistent across engines and domains.

## Job Principle

A job is a durable request for background work.

Jobs should be used for work that is delayed, retryable, heavy, or external-provider dependent.

## Common Job Fields

A job record or payload should include:

- job_id
- job_type
- organization_id nullable
- actor_user_id nullable
- payload
- status
- attempts
- max_attempts
- scheduled_at
- started_at nullable
- completed_at nullable
- failed_at nullable
- error_code nullable
- error_message nullable
- trace_id nullable

## Job Statuses

Recommended statuses:

- queued
- processing
- completed
- failed
- retrying
- cancelled

## Job Types

Possible initial job types:

- sms.send_message
- notification.deliver
- report.generate_export
- import.process_file
- payment.verify_later
- storage.process_file
- workflow.send_reminder

## Payload Rules

Job payloads should contain only what is needed.

Prefer record IDs and safe metadata instead of large or sensitive copies of data.

Avoid storing raw secrets in job payloads.

## Idempotency

Jobs that can cause external or financial effects must support safe retry behavior.

Examples:

- SMS sending
- payment verification
- wallet crediting
- report export generation

## Retry Policy

Each job type should define:

- max attempts
- retry delay strategy
- retryable errors
- non-retryable errors

Do not retry permanently without limits.

## Tenant Context

Jobs related to organization data must preserve organization context.

Worker execution must not bypass organization boundaries.

## Observability

Workers should log:

- job type
- job id
- organization id where available
- status changes
- failure reason
- trace id where available

## Testing Requirements

Tests should cover:

- job creation
- job processing
- retry behavior
- non-retryable failure
- duplicate-safe behavior where relevant
- tenant context preservation

## Final Rule

Background jobs must be reliable, visible, retry-safe, and tenant-aware.
