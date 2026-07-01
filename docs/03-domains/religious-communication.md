# Religious Communication Specification

## Purpose

This document defines communication behavior inside the Religious Domain.

Religious communication includes announcements, reminders, targeted messages, and member engagement communication. It must use shared Notification and SMS capabilities.

## Scope

Communication may include:

- Announcements
- Targeted messages
- Scheduled messages
- Birthday messages
- Event reminders
- Attendance reminders
- Visitor follow-up reminders
- Group messages
- Branch hierarchy messages

## Channels

Supported channels may include:

- in-app
- push later
- SMS
- email later

SMS delivery must go through the SMS Engine.

## Audience Targeting

Messages may target:

- Entire organization
- Branch
- Branch subtree
- Congregation
- Service attendees
- Group
- Visitors
- Members
- Leaders
- Households
- Custom filters

## Message Flow

```text
Authorized user creates message
  -> Audience resolved
  -> Notification Engine creates deliveries
  -> SMS Engine used where SMS is selected
  -> Delivery status tracked
  -> Communication report updated
```

## Suggested Entities

Candidate entities:

- religious_announcements
- religious_message_audiences
- religious_message_templates
- religious_message_campaigns

Shared delivery records may live in Notification/SMS engines.

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Notification Engine for message creation and delivery coordination
- SMS Engine for SMS delivery
- Localization Engine for templates
- Reporting Engine for campaign reports
- Audit Engine for bulk or sensitive sends

## Permission Examples

- religious.communication.view
- religious.communication.send
- religious.communication.schedule
- religious.communication.manage_templates
- religious.communication.view_reports

## Events Published

- ReligiousAnnouncementCreated
- ReligiousMessageScheduled
- ReligiousMessageAudienceResolved

## Events Consumed

- SmsDelivered
- SmsFailed
- NotificationDelivered
- NotificationFailed

## Consent and Preferences

Communication should respect member preferences where applicable.

Examples:

- allow_sms
- allow_push
- allow_email
- birthday_messages
- event_reminders
- announcement_messages

Important organization messages may have different rules from optional campaign messages.

## Reports

Reports may include:

- Messages sent
- SMS usage by campaign
- Delivery status
- Failed messages
- Audience count
- Cost summary where available

## Security Requirements

- Bulk sending requires explicit permission.
- Sensitive audience filters require care.
- Message content should not expose confidential records.
- SMS wallet and subscription checks may apply.

## Testing Requirements

Tests must cover:

- Create announcement
- Resolve audience
- Send message through Notification Engine
- SMS path delegates to SMS Engine
- Permission checks
- Organization boundary checks
- Preference handling

## Final Rule

Religious Communication owns message purpose and audience. Notification and SMS engines own delivery coordination and tracking.
