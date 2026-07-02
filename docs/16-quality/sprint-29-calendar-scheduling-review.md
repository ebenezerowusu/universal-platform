# Sprint 29 Calendar Scheduling Review

## Purpose

This document defines review checks for Sprint 29.

Sprint 29 implements Organization Calendar and Scheduling foundation.

## Review 1: Scope

Sprint 29 may include:

- calendar feature registration foundation
- organization calendar foundation
- scheduled item/event foundation
- recurring schedule placeholder where practical
- participant/resource placeholder foundation
- reminder/notification trigger foundation where practical
- conflict detection placeholder where practical
- domain link/reference foundation for events, attendance, HR, commerce, and religious workflows
- permission and feature checks
- audit records for important schedule actions
- API and UI foundations

Sprint 29 should not include:

- full external calendar sync
- Google Calendar or Outlook integration
- complex recurrence engine beyond placeholder rules
- resource booking marketplace
- public ticketing
- advanced appointment automation
- payroll scheduling
- unrelated Commerce/POS expansion

## Review 2: Domain Boundaries

Confirm:

- scheduling foundation owns generic calendar records
- domain modules own their business records
- domain modules link to scheduled items instead of duplicating calendar logic
- Platform Core does not contain domain-specific scheduling rules

## Review 3: Organization and Timezone Rules

Confirm:

- calendars are organization-owned
- scheduled items are organization-owned
- domain references cannot cross organization boundaries
- timezone is explicit or resolved from organization settings
- invalid start/end time is rejected

## Review 4: Reminder Rules

Confirm:

- reminders are linked to scheduled items
- reminders do not trigger for cancelled items
- reminder actions use Notification Engine where practical
- reminder changes are auditable

## Review 5: Conflict Rules

Confirm conflict warnings exist where practical for:

- invalid time range
- overlapping participant placeholder
- overlapping resource placeholder
- cancelled schedule linked to active domain workflow

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/calendar-scheduling-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/calendar-scheduling-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- calendar create/update
- scheduled item create/update
- invalid time range
- organization boundary enforcement
- domain reference linking
- reminder placeholder creation
- cancelled item reminder behavior
- conflict warning generation
- permission denied behavior

## Final Rule

Sprint 29 is complete when organizations can create calendars and scheduled items, link them to domain workflows, configure reminder placeholders, and receive basic conflict warnings through permission-protected and auditable workflows.
