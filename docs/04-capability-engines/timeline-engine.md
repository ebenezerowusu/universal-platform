# Timeline Engine Specification

## Purpose

The Timeline Engine provides a shared way to record chronological activity history for important records across domains.

It helps users understand what happened to a record over time without each domain building separate timeline logic.

## Responsibilities

The Timeline Engine is responsible for:

- Recording timeline entries
- Linking entries to records
- Showing chronological activity
- Linking entries to actors
- Supporting filters
- Supporting access rules
- Working with the Audit Engine where useful

## Difference from Audit Engine

The Audit Engine is for accountability and system traceability.

The Timeline Engine is for operational history shown to authorized users.

Some actions may appear in both, but they have different purposes.

## Example Timelines

Member timeline:

```text
Registered
Joined group
Marked present
Made payment
Became worker
Assigned as leader
```

Product timeline:

```text
Created
Stock added
Price changed
Sold
Restocked
```

Order timeline:

```text
Created
Payment completed
Packed
Dispatched
Delivered
```

Staff timeline:

```text
Hired
Department changed
Leave approved
Review added
```

## Suggested Entities

Candidate entities:

- timeline_entries
- timeline_entry_links

Candidate fields:

- id
- organization_id
- entity_type
- entity_id
- actor_user_id nullable
- event_type
- title
- description nullable
- metadata JSONB nullable
- visibility
- occurred_at
- created_at

## Timeline Creation Flow

```text
Domain action occurs
  -> Domain publishes event or calls timeline service
  -> Timeline entry is created
  -> Entry is linked to record
  -> Entry appears in authorized timeline views
```

## Events Consumed

Possible consumed events:

- ReligiousMemberCreated
- PaymentCompleted
- AttendanceMarked
- OrderStatusChanged
- StaffLeaveApproved

## Security Requirements

- Timeline views must respect permissions.
- Tenant context must be enforced.
- Timeline metadata must not expose secrets.
- Sensitive entries require stricter access rules.

## Tenant Isolation

Timeline entries must include organization context for organization-owned records.

Users from one organization must not view another organization's timeline entries.

## Testing Requirements

Tests must cover:

- Timeline entry creation
- Record timeline retrieval
- Ordering by time
- Access filtering
- Tenant isolation
- Event-created timeline entry

## Final Rule

Timelines explain operational history. Audits prove accountability. Use both correctly.
