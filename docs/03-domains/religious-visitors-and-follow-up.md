# Religious Visitors and Follow-Up Specification

## Purpose

This document defines visitor management and follow-up behavior inside the Religious Domain.

Visitor management helps organizations record first-time visitors, assign follow-up responsibility, and convert interested visitors into members where appropriate.

## Scope

Visitor management may include:

- Visitor registration
- Visitor check-in
- Invited-by tracking
- Visit history
- Follow-up assignment
- Follow-up notes
- Follow-up reminders
- Conversion to member
- Visitor reports

## Visitor Statuses

Possible statuses:

- new
- contacted
- interested
- converted
- inactive

Status labels may be configurable later.

## Visitor Capture Sources

Visitors may be captured through:

- Admin registration
- Mobile registration
- Event registration
- Service check-in
- Public form later
- Import later

## Follow-Up Flow

```text
Visitor registered
  -> Follow-up task created
  -> Responsible leader assigned
  -> Reminder notification scheduled
  -> Follow-up notes recorded
  -> Visitor status updated
  -> Visitor converted to member where appropriate
```

## Suggested Entities

Candidate entities:

- religious_visitors
- religious_visitor_visits
- religious_visitor_follow_ups
- religious_visitor_status_history
- religious_visitor_conversion_records

## Required Platform Engines

This module must use:

- Organization Engine for tenant context
- Permission Engine for access control
- Workflow Engine for follow-up process
- Notification Engine for reminders
- SMS Engine through Notification Engine where needed
- Audit Engine for important changes
- Reporting Engine for visitor reports
- Timeline Engine for visitor history

## Permission Examples

- religious.visitors.view
- religious.visitors.create
- religious.visitors.update
- religious.visitors.assign_follow_up
- religious.visitors.convert
- religious.visitors.reports.view

## Events Published

- ReligiousVisitorRegistered
- ReligiousVisitorFollowUpAssigned
- ReligiousVisitorContacted
- ReligiousVisitorConverted
- ReligiousVisitorStatusChanged

## Reports

Visitor reports may include:

- New visitors by date
- Visitors by branch
- Visitors by service
- Follow-up completion
- Conversion rate
- Invited-by member report

## Security Requirements

- Visitor records are organization-owned.
- Follow-up notes must respect permissions.
- Conversion to member must be audited.
- Reports must respect organization and scope.

## Testing Requirements

Tests must cover:

- Register visitor
- Assign follow-up
- Update follow-up status
- Convert visitor to member
- Permission checks
- Organization boundary checks
- Follow-up reminder event

## Final Rule

Visitors belong to the Religious Domain. Follow-up should use Workflow and Notification capabilities instead of hardcoded process logic.
