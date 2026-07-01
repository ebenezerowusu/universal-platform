# Religious Events Specification

## Purpose

This document defines event management inside the Religious Domain.

Religious events may include conferences, programs, retreats, seminars, classes, outreach activities, fundraisers, and special gatherings.

## Scope

Events may include:

- Event creation
- Event categories
- Free or paid registration
- RSVP
- Attendee list
- Event reminders
- Event attendance
- Volunteers
- Event reports
- Recurring events later

## Event Types

Possible event types:

- service_event
- conference
- seminar
- retreat
- outreach
- fundraiser
- training
- class
- custom

## Paid Events

Paid event registration must use the Payment Engine.

The Religious Domain may define event price and registration rules. Payment Engine processes and verifies payment.

## Event Flow

```text
Event created
  -> Event published
  -> Members/visitors register
  -> Payment processed if required
  -> Reminders sent
  -> Attendance marked
  -> Event report generated
```

## Suggested Entities

Candidate entities:

- religious_events
- religious_event_categories
- religious_event_registrations
- religious_event_attendance
- religious_event_volunteers
- religious_event_reminders

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Payment Engine for paid registrations
- Notification Engine for reminders
- SMS Engine through Notification Engine where applicable
- Reporting Engine for event reports
- Audit Engine for sensitive changes
- Storage Engine for event images/documents

## Permission Examples

- religious.events.view
- religious.events.create
- religious.events.update
- religious.events.publish
- religious.events.manage_registrations
- religious.events.view_reports

## Events Published

- ReligiousEventCreated
- ReligiousEventPublished
- ReligiousEventRegistrationCreated
- ReligiousEventPaid
- ReligiousEventAttendanceMarked

## Events Consumed

- PaymentCompleted
- PaymentFailed
- NotificationDelivered

## Reports

Reports may include:

- Event registrations
- Attendance
- Payment summary
- Volunteer list
- Reminder delivery
- Branch participation

## Security Requirements

- Event records are organization-owned.
- Paid event payment status must come from Payment Engine.
- Registration lists may contain personal data and require permission.
- Reports must respect organization and scope.

## Testing Requirements

Tests must cover:

- Create event
- Publish event
- Register attendee
- Paid registration flow
- Mark event attendance
- Permission checks
- Organization boundary checks

## Final Rule

Religious Events owns event meaning and registration rules. Payment, notification, storage, and reporting capabilities must be used through shared engines.
