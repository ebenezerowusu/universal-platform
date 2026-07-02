# Calendar Scheduling Operations Guide

## Purpose

This document defines operating guidance for organization calendars and scheduled items.

The goal is to keep scheduling reusable, timezone-aware, permission-protected, and explainable across domains.

## Principle

Calendar records should be shared scheduling infrastructure.

Domain modules should link to scheduled items instead of creating isolated calendar logic.

## Data Sources

Calendar workflows may use:

- calendar
- scheduled item
- domain reference
- recurrence placeholder
- reminder placeholder
- participant placeholder
- resource placeholder
- conflict warning placeholder
- audit records

## Timezone Direction

Every schedule should use:

- explicit scheduled item timezone, or
- organization timezone, or
- platform fallback timezone only when no organization timezone exists

Do not rely only on the user's device timezone.

## Domain Reference Direction

Domain references should capture:

```text
domain_type
domain_record_id
organization_id
linked_by
linked_at
```

Domain references must not cross organization boundaries.

## Conflict Direction

Conflict checks may warn for:

- invalid start/end time
- same resource at same time
- same participant at same time
- cancelled item still linked to active domain workflow

MVP may warn without blocking unless policy later requires blocking.

## Reminder Direction

Reminder triggers should check:

- schedule status
- reminder status
- domain reference status where available
- notification eligibility

Do not send reminders for cancelled scheduled items.

## Recurrence Direction

Sprint 29 should not implement a full recurrence engine.

Use placeholders until recurrence rules and exception handling are explicitly planned.

## Audit Direction

Audit should record:

- calendar created or updated
- scheduled item created or updated
- scheduled item cancelled
- reminder configured
- domain reference linked or changed

## Data Quality Warnings

Warn or flag when:

- schedule has invalid time range
- schedule timezone is missing
- active domain reference points to cancelled schedule
- reminder exists for cancelled item
- linked domain record cannot be found

## Final Rule

Scheduling should be accurate, timezone-aware, organization-scoped, and honest about conflicts or weak data.
