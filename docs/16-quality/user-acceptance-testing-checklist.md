# User Acceptance Testing Checklist

## Purpose

This document defines the MVP user acceptance testing checklist for Universal Platform.

It helps confirm that the MVP is understandable and usable by a first pilot organization.

## UAT Principle

UAT should validate whether real users can complete important workflows, not whether the architecture is elegant.

## Participant Types

Recommended pilot testers:

- organization owner/admin
- branch or group leader where applicable
- attendance marker
- member/visitor manager
- support/admin observer

## UAT Area 1: Login and Organization Selection

Confirm users can:

- log in
- understand login errors
- view available organizations
- select the right organization
- recover safely from session expiration where implemented

## UAT Area 2: Navigation

Confirm users can:

- find the Religious area
- understand unavailable module messages
- move between dashboard, members, visitors, groups, and attendance
- return to main navigation easily

## UAT Area 3: Member Workflow

Confirm users can:

- view member list
- search or filter where available
- create a member
- understand validation errors
- view created member where available

## UAT Area 4: Visitor Workflow

Confirm users can:

- view visitor list
- create a visitor
- understand visitor status
- convert a visitor to member where implemented
- understand confirmation messages

## UAT Area 5: Structure Workflow

Confirm users can understand:

- congregation
- service
- group
- group membership where implemented

Confirm users can complete available create/list actions.

## UAT Area 6: Attendance Workflow

Confirm users can:

- view attendance sessions
- create attendance session where available
- mark attendance manually where available
- understand present/absent/late/excused statuses

## UAT Area 7: Error and Empty States

Confirm users understand:

- empty lists
- loading states
- validation errors
- unavailable feature messages
- permission denied messages

## UAT Area 8: Basic Reports

Confirm users can understand available summaries such as:

- member count
- visitor count
- group count
- attendance summary

## Feedback Recording

For each feedback item, capture:

```text
screen/workflow
user role
issue summary
severity
suggested improvement
must fix before pilot: yes/no
```

## Severity Levels

Use:

```text
critical - blocks pilot
high - serious workflow issue
medium - usable but confusing
low - polish or future improvement
```

## Acceptance Standard

The MVP is acceptable for controlled pilot when:

- critical issues are resolved
- high issues are either resolved or explicitly accepted
- users can complete core flows with minimal guidance
- known limitations are documented

## Final Rule

UAT should protect the pilot experience. Do not treat user confusion as a minor issue if it blocks real workflow completion.
