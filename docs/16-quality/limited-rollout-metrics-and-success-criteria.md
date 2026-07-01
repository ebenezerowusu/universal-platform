# Limited Rollout Metrics and Success Criteria

## Purpose

This document defines metrics and success criteria for limited rollout.

The goal is to decide launch readiness using evidence instead of assumption.

## Principle

Rollout success should be measured by stable usage, workflow completion, support load, and user confidence.

## Organization Metrics

Track:

```text
organizations_onboarded
organizations_active
organizations_paused
organizations_waiting_setup
```

## User Metrics

Track:

```text
active_users
successful_logins
failed_logins
users_with_support_requests
```

## Workflow Metrics

Track completion for:

- login
- organization selection
- member creation
- visitor creation
- visitor conversion where available
- congregation/service/group setup
- attendance session creation
- manual attendance marking

## Support Metrics

Track:

```text
support_requests_total
support_requests_by_organization
support_requests_by_severity
average_first_response_time
open_critical_issues
open_high_issues
repeated_confusion_items
```

## Quality Metrics

Track:

```text
smoke_tests_passed
smoke_tests_failed
end_to_end_tests_passed
api_contract_mismatches
flutter_error_state_gaps
data_correction_requests
```

## Success Criteria

Limited rollout is successful when:

- core workflows complete for most active organizations
- critical issues are resolved
- high issues are resolved or accepted
- smoke tests are repeatable
- support volume is manageable
- users understand known limitations
- no serious organization boundary issue is observed

## Warning Signals

Pause growth or stabilize further when:

- login issues repeat across organizations
- organization selection causes confusion repeatedly
- member, visitor, or attendance flows fail often
- support load grows faster than the team can handle
- data correction requests are frequent
- smoke tests fail during onboarding

## Decision Options

Use metrics to choose one:

```text
ready for broader launch
ready for another limited rollout wave
needs more hardening
pause expansion
```

## Final Rule

Metrics should protect the launch. Do not expand usage when the numbers show unstable onboarding, support, or core workflow completion.
