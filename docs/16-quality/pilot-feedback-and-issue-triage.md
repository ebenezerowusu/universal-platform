# Pilot Feedback and Issue Triage

## Purpose

This document defines how pilot feedback and issues should be captured, classified, and acted on.

The goal is to learn from the pilot without losing control of scope.

## Principle

Not every pilot feedback item becomes immediate work.

Feedback should be classified before implementation.

## Feedback Types

Use these feedback types:

```text
bug
workflow confusion
missing MVP expectation
future feature request
data issue
performance issue
support issue
security/privacy concern
```

## Severity Levels

Use:

```text
critical - blocks pilot usage or creates serious risk
high - serious issue affecting important workflow
medium - workaround exists but user experience is poor
low - polish or future improvement
```

## Priority Decision

Priority should consider:

- severity
- number of users affected
- workflow importance
- pilot impact
- safety/security risk
- effort to fix
- whether it is inside MVP scope

## Triage Statuses

Use:

```text
new
needs clarification
accepted for fix
accepted limitation
future backlog
duplicate
not planned
resolved
```

## Required Feedback Fields

Capture:

```text
reported_by
user_role
organization
feedback_type
severity
screen_or_workflow
summary
details
steps_to_reproduce
expected_result
actual_result
attachment_or_log_reference
triage_status
owner
next_action
```

## Bug Handling

A bug should be fixed during pilot when:

- it blocks a critical MVP workflow
- it creates data risk
- it affects multiple pilot users
- it prevents UAT completion

## Workflow Confusion Handling

Workflow confusion should be treated seriously.

If users cannot complete a core workflow without guidance, consider it a product issue, not only a training issue.

## Feature Request Handling

Feature requests should be separated from bugs.

Classify feature requests as:

```text
MVP gap
post-pilot improvement
future module
not aligned
```

## Accepted Limitation Handling

An accepted limitation must be documented clearly.

It should include:

- limitation summary
- reason accepted
- workaround if any
- planned follow-up if any

## Final Rule

Pilot feedback should drive learning and prioritization. Do not turn every request into immediate scope expansion.
