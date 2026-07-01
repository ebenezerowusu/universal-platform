# Notification Engine Specification

## Purpose

The Notification Engine provides one platform-wide way to create, deliver, track, and manage notifications across domains and modules.

It coordinates in-app, push, email-style, and SMS-triggered notifications through appropriate channels and engines.

## Responsibilities

The Notification Engine is responsible for:

- Creating notifications
- Managing notification channels
- Delivering in-app notifications
- Coordinating push notifications later
- Coordinating email notifications later
- Coordinating SMS through the SMS Engine
- Notification templates
- Notification preferences
- Delivery status tracking
- Notification events
- Notification audit support where needed

## Non-Responsibilities

The Notification Engine does not decide domain-specific business triggers.

Examples:

- Religious Domain decides when a visitor follow-up reminder is needed.
- Commerce Domain decides when an order update is needed.
- HR Domain decides when leave approval notifications are needed.

The Notification Engine decides how notifications are created and delivered.

## Channels

Possible channels:

- in_app
- push
- sms
- email
- whatsapp future

SMS delivery must go through the SMS Engine.

## Notification Flow

```text
Domain publishes event or requests notification
  -> Notification Engine evaluates template and preferences
  -> Notification record created
  -> Delivery channel selected
  -> Channel adapter/engine used
  -> Delivery status updated
  -> NotificationDelivered or NotificationFailed event published
```

## Notification Preferences

Users/members/customers should be able to control preferences where appropriate.

Examples:

- allow_sms
- allow_push
- allow_email
- birthday_messages
- event_reminders
- marketing_messages
- giving_reminders

Important system/security notifications may have different rules from optional campaigns.

## Suggested Entities

Candidate entities:

- notifications
- notification_templates
- notification_preferences
- notification_deliveries
- notification_events

## Notification Statuses

Possible statuses:

- created
- queued
- sent
- delivered
- read
- failed
- dismissed

## Templates

Templates should support:

- Localization
- Variables
- Channel-specific content
- Organization customization where allowed
- Approval requirements where needed

Example:

```text
Hello {{name}}, you have a reminder for {{event_name}} on {{date}}.
```

## Events Published

- NotificationCreated
- NotificationQueued
- NotificationDelivered
- NotificationRead
- NotificationFailed
- NotificationPreferenceUpdated

## Events Consumed

Possible consumed events:

- MemberCreated
- VisitorRegistered
- PaymentCompleted
- SmsDelivered
- SubscriptionExpired
- WorkflowTaskAssigned

## Security Requirements

- Users must only see their own notifications unless authorized.
- Organization notification history must be tenant-scoped.
- Sensitive notification content must be handled carefully.
- Bulk notifications must require permission.

## Tenant Isolation

Every organization-owned notification must include organization context.

Users from one organization must not see notifications from another organization unless they legitimately belong to both and have selected that context.

## Testing Requirements

Tests must cover:

- Notification creation
- Template rendering
- Preference checks
- In-app delivery
- SMS channel delegates to SMS Engine
- Tenant isolation
- Read/dismiss status update
- Permission checks for bulk notifications

## Final Rule

Domains decide why to notify. The Notification Engine owns notification creation, preferences, delivery coordination, and tracking.
