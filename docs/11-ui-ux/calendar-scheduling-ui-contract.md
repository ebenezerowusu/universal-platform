# Calendar Scheduling UI Contract

## Purpose

This document defines the MVP UI contract for Organization Calendar and Scheduling foundation.

The screens should help organization users view calendars, create scheduled items, link schedules to domain records, configure reminder placeholders, and review conflict warnings.

## Navigation Direction

Calendar screens should appear when:

- user is authenticated
- organization is selected
- calendar feature is available
- user has required permission
- feature availability allows the section

## Calendar Dashboard

Should show:

- upcoming scheduled items
- today's items
- cancelled items where relevant
- conflict warnings count where available
- quick action to create scheduled item

## Calendar List

Should show:

- calendar name
- type
- visibility
- timezone
- status
- edit action where permitted

## Scheduled Item List

Should show:

- title
- calendar
- date/time
- timezone
- status
- domain reference type where available
- action to view detail

## Scheduled Item Detail

Should show:

- title
- description where available
- start date/time
- end date/time
- timezone
- status
- linked domain reference where available
- reminder placeholder where available
- conflict warning where available

## Create/Edit Scheduled Item

Should support:

- calendar selection
- title
- description optional
- start date/time
- end date/time
- timezone
- visibility/status controls where permitted
- domain reference placeholder

## Reminder Placeholder

Should support:

- reminder type
- offset before start
- channel placeholder
- status

## Conflict Warning

Should show:

- warning type
- affected resource or participant safe label where available
- conflicting time range
- suggested action placeholder

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 29:

- external calendar sync screens
- Google Calendar or Outlook screens
- advanced recurrence editor
- public ticketing screens
- advanced resource marketplace screens
- payroll scheduling screens

## Final Rule

Calendar UI should make scheduling clear across domains without duplicating domain-specific workflows inside the calendar screens.
