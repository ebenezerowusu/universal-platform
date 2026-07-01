# Post-Launch Adoption Metrics

## Purpose

This document defines adoption metrics to review after launch.

The goal is to understand whether users are adopting the MVP and completing important workflows.

## Principle

Adoption should be measured through meaningful usage, not only signups.

## Organization Metrics

Track:

```text
organizations_created
organizations_active
organizations_inactive
organizations_requesting_support
```

## User Metrics

Track:

```text
users_created
users_active
successful_sign_ins
failed_sign_ins
users_returning_after_first_use
```

## Core Workflow Metrics

Track:

- organization selection completed
- Religious dashboard opened
- members created
- visitors created
- visitors converted where available
- congregations/services/groups created where available
- attendance sessions created
- attendance records marked where available

## Support Metrics

Track:

```text
support_requests_total
support_requests_by_workflow
support_requests_by_severity
repeat_support_themes
average_response_time
open_critical_items
open_high_items
```

## Quality Metrics

Track:

```text
reported_errors
workflow_failures
data_correction_requests
unavailable_state_confusion
api_client_errors
```

## Adoption Signals

Positive signals:

- users return after first use
- members and visitors are created without support help
- attendance workflow is used successfully
- support requests reduce over time
- repeated confusion decreases

Warning signals:

- users sign in once and do not return
- organization selection repeatedly confuses users
- core workflows require support guidance
- data correction requests increase
- errors repeat across organizations

## Review Cadence

Review metrics:

- daily during the first launch week
- weekly after launch stabilizes
- before approving major feature expansion

## Final Rule

Use adoption metrics to guide the roadmap. Do not assume launch success without evidence of repeated useful activity.
