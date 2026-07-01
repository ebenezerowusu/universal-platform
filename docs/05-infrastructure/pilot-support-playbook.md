# Pilot Support Playbook

## Purpose

This document defines support guidance during the controlled pilot.

The goal is to help the team respond consistently to pilot issues without turning support into uncontrolled feature development.

## Support Principle

Support should protect the pilot experience while preserving MVP scope.

## Support Channels

Define the pilot support channel before launch.

Examples:

- WhatsApp group
- email
- shared issue form
- internal support tracker

The selected support channel should be documented in the pilot readiness report.

## Issue Categories

Use these support categories:

- login/access
- organization selection
- permission/role
- member workflow
- visitor workflow
- structure workflow
- attendance workflow
- UI confusion
- data correction
- unexpected error
- feature request

## Severity Levels

Use:

```text
critical - blocks pilot usage
high - serious workflow issue
medium - workaround exists
low - polish or future improvement
```

## Response Direction

For every support issue, capture:

```text
reported_by
user_role
organization
screen_or_workflow
issue_summary
severity
steps_to_reproduce
expected_result
actual_result
screenshot_or_log_reference optional
support_owner
status
```

## Common Issue: Login Problem

Check:

- user exists
- account status
- correct login identifier
- session/token behavior
- backend auth route status

## Common Issue: Organization Missing

Check:

- user is connected to organization
- organization status
- organization membership status
- selected organization state in Flutter

## Common Issue: Permission Denied

Check:

- user role assignment
- role permissions
- required permission key
- module availability
- feature availability

## Common Issue: Religious Module Unavailable

Check:

- module installed
- module active
- plan/feature availability
- Flutter unavailable state

## Common Issue: Data Not Showing

Check:

- selected organization
- backend organization scoping
- filters/search
- pagination
- API response shape

## Common Issue: Attendance Marking Issue

Check:

- session exists
- session status
- member exists
- duplicate attendance record
- permission for attendance action

## Feature Requests

Feature requests should be logged separately from pilot blockers.

Do not implement feature requests during pilot unless they are approved as pilot-critical.

## Escalation

Escalate immediately when:

- pilot users cannot log in
- organization data boundary appears broken
- data is lost or corrupted
- critical workflows fail for multiple users
- sensitive data is exposed unexpectedly

## Final Rule

Support should create clarity. Every pilot issue should become either a fix, accepted limitation, future feature request, or user guidance update.
