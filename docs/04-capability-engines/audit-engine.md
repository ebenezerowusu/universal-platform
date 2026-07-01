# Audit Engine Specification

## Purpose

The Audit Engine records important and sensitive actions across Universal Platform.

It provides accountability, traceability, security support, compliance readiness, and operational investigation capability.

## Responsibilities

The Audit Engine is responsible for:

- Recording auditable actions
- Capturing actor identity
- Capturing organization context
- Capturing entity changes
- Capturing metadata such as IP and user agent
- Supporting sensitive-data access tracking
- Supporting audit reports
- Providing a standard audit interface for all domains and engines

## Non-Responsibilities

The Audit Engine does not decide domain business rules.

It records what happened. It does not determine whether a payment, attendance, or member transfer is valid.

## Audit Event Structure

Every audit event should answer:

- Who did it?
- Which organization did it affect?
- What action happened?
- Which entity was affected?
- When did it happen?
- From where did it happen?
- What changed?

## Suggested Entity

Candidate table:

```text
audit_logs
```

Candidate fields:

- id
- organization_id nullable for platform-level actions
- actor_user_id nullable for system actions
- actor_type
- action
- entity_type
- entity_id
- severity
- before_values JSONB nullable
- after_values JSONB nullable
- metadata JSONB nullable
- ip_address nullable
- user_agent nullable
- trace_id nullable
- created_at

## Actor Types

Possible actor types:

- user
- system
- provider_webhook
- background_job
- api_key future

## Severity Levels

Possible severity levels:

- info
- warning
- critical

Examples:

- Viewing a report may be info.
- Failed provider verification may be warning.
- Permission escalation may be critical.

## Actions to Audit

### Platform-Level Actions

- Organization created
- Organization suspended/reactivated
- Module installed/uninstalled
- Provider configuration changed
- Subscription plan changed
- SMS package created/updated
- Platform admin role changed

### Security Actions

- Role created/updated/deleted
- Permission assigned/revoked
- User added to organization
- User removed from organization
- Sensitive setting changed
- Failed access attempt where useful

### Payment Actions

- Payment initialized
- Payment completed
- Payment failed
- Refund processed
- Webhook verification failed
- Payment status manually adjusted

### SMS Actions

- SMS wallet credited/debited
- Bulk SMS sent
- SMS provider changed
- Sender ID changed
- SMS package purchased

### Religious Domain Actions

- Member created/updated/deleted
- Member transferred
- Branch connected/disconnected
- Attendance edited after submission
- Confidential prayer/counseling accessed
- Welfare request updated
- Financial/giving report exported

## Sensitive Data Handling

Audit logs must be useful but should not expose unnecessary sensitive data.

Do not store raw passwords, provider secrets, full payment credentials, or private confidential notes in audit logs.

When necessary, store references or redacted values.

## Immutability

Audit logs should be append-only where practical.

Users should not edit audit records.

If audit cleanup is needed for retention policies, it must be controlled by platform-level jobs and documented.

## Tenant Isolation

Organization-level audit logs must be scoped to the organization.

Platform administrators may view broader audit logs if authorized.

Organization users should not view another organization's audit logs.

## Query and Reporting

Audit logs should support filtering by:

- organization
- actor
- action
- entity type
- entity id
- severity
- date range

## Events Published

The Audit Engine may publish:

- AuditLogRecorded
- SensitiveActionRecorded
- CriticalAuditEventRecorded

## Testing Requirements

Tests must cover:

- Audit log created for sensitive action
- Audit log includes organization context
- Audit log includes actor context
- Tenant isolation on audit queries
- Redaction of sensitive values
- System actor audit

## Final Rule

If a future investigation would need to know who did something, audit it.
