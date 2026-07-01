# Audit Event Catalog

## Purpose

This document defines the initial audit event catalog for Universal Platform.

Audit events help the platform answer who did what, when, where, and to which record.

## Audit Principle

Audit should focus on important, sensitive, or security-relevant actions.

Do not audit every harmless read operation unless the data is sensitive enough to require it.

## Common Audit Fields

Audit records should capture:

- action
- actor_user_id nullable
- actor_type
- organization_id nullable
- entity_type
- entity_id
- severity
- trace_id
- metadata
- created_at

## Severity Levels

Suggested levels:

- info
- warning
- critical

## Platform Audit Events

```text
platform.organization.created
platform.organization.updated
platform.organization.suspended
platform.organization.reactivated
platform.module.registered
platform.provider.configured
platform.subscription_plan.created
platform.subscription_plan.updated
```

## Identity Audit Events

```text
identity.user.created
identity.user.updated
identity.user.disabled
identity.login.succeeded
identity.login.failed
identity.session.revoked
```

## Organization Audit Events

```text
organization.user.invited
organization.user.joined
organization.user.removed
organization.role.created
organization.role.updated
organization.role.deleted
organization.permission.assigned
organization.settings.updated
```

## Module Audit Events

```text
module.installed
module.activated
module.deactivated
module.suspended
module.settings.updated
```

## Subscription Audit Events

```text
subscription.created
subscription.plan_changed
subscription.cancelled
subscription.reactivated
subscription.limit_reached
```

## Payment Audit Events

```text
payment.transaction.created
payment.transaction.verified
payment.transaction.failed
payment.refund.requested
payment.provider_callback.received
```

## SMS Audit Events

```text
sms.wallet.credited
sms.wallet.debited
sms.campaign.created
sms.message.sent
sms.provider.failed
sms.package.created
sms.package.updated
```

## Storage Audit Events

```text
storage.file.uploaded
storage.file.deleted
storage.file.access_changed
storage.temporary_link.created
```

## Religious Audit Events

```text
religious.member.created
religious.member.updated
religious.member.status_changed
religious.member.exported
religious.visitor.converted
religious.attendance.edited
religious.branch.connected
religious.branch.disconnected
religious.giving.recorded
religious.giving.exported
religious.care.confidential_viewed
religious.counseling.case_updated
religious.welfare.request_approved
```

## Audit Metadata Rules

Metadata should be useful but safe.

Do not store:

- raw secrets
- raw credential values
- full provider secrets
- full confidential notes where unnecessary

## Testing Requirements

Tests should cover:

- Audit record is created for sensitive action
- Audit record includes actor
- Audit record includes organization where applicable
- Audit record does not expose restricted values

## Final Rule

If a future investigation would need the action history, audit it.
