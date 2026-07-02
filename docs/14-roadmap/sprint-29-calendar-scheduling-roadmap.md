# Sprint 29 Calendar Scheduling Roadmap

## Purpose

This document defines Sprint 29 for Organization Calendar and Scheduling foundation.

Sprint 29 should create a reusable scheduling layer for events, workforce attendance, leave, religious services, commerce operations, fulfillment, delivery windows, and future appointments.

## Sprint Goal

Enable organizations to create calendars, schedule items, link schedules to domain records, trigger reminders, and detect basic conflicts while keeping scheduling reusable across modules.

## Why This Sprint

Many platform modules need time-based planning:

- Religious services and events
- Staff attendance sessions
- Leave requests
- Commerce order pickup windows
- Delivery windows
- Training or education programs
- Future appointments and bookings

Without a shared scheduling layer, every domain will create separate calendar logic.

## Work Package 1: Organization Calendars

Prepare organization-owned calendars with:

- name
- type
- visibility
- timezone
- status

## Work Package 2: Scheduled Items

Prepare scheduled items with:

- calendar
- title
- description optional
- start date/time
- end date/time
- timezone
- status
- domain reference optional

## Work Package 3: Domain References

Prepare domain references such as:

```text
religious_event
attendance_session
leave_request
commerce_pickup
commerce_delivery
training_session
custom
```

A scheduled item should link to a domain record without owning that domain's business logic.

## Work Package 4: Recurrence Placeholder

Prepare simple recurrence placeholder fields without implementing a complex recurrence engine.

Examples:

```text
none
daily_placeholder
weekly_placeholder
monthly_placeholder
custom_placeholder
```

## Work Package 5: Participants and Resources Placeholder

Prepare placeholders for:

- participant profiles
- teams/departments
- rooms/venues
- equipment

Do not build advanced resource booking yet.

## Work Package 6: Reminder Direction

Prepare reminder trigger direction using Notification Engine where practical.

Reminder examples:

- before start
- schedule changed
- schedule cancelled

## Work Package 7: Conflict Detection Placeholder

Prepare simple conflict checks:

- same resource same time
- same profile overlapping schedule
- invalid start/end time

## Work Package 8: Flutter Foundation

Prepare screens or placeholders for:

- calendar dashboard
- calendar list
- schedule list
- schedule detail
- create/edit scheduled item
- reminder placeholder
- conflict warning

## Out of Scope

Do not implement:

- external calendar sync
- Google Calendar or Outlook integration
- advanced recurrence engine
- public ticketing
- advanced resource marketplace
- payroll scheduling

## Completion Standard

Sprint 29 is complete when organizations can create calendar records, scheduled items, domain links, reminder placeholders, and basic conflict warnings through permissioned, auditable flows.

## Final Rule

Scheduling must be shared infrastructure for domains, not a domain-specific calendar hidden inside one module.
