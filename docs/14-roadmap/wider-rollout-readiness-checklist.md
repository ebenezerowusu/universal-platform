# Wider Rollout Readiness Checklist

## Purpose

This document defines the checklist for moving from controlled pilot to wider pilot or limited rollout.

The goal is to scale usage only after the MVP has enough evidence of stability.

## Readiness Principle

Do not widen rollout because the pilot happened. Widen rollout because the pilot proved the MVP is ready enough.

## Product Readiness

Confirm:

- core MVP workflows work end to end
- critical pilot issues are fixed
- high pilot issues are fixed or accepted
- confusing workflows are improved or documented
- accepted limitations are clear

## Technical Readiness

Confirm:

- backend checks pass
- Flutter checks pass where practical
- smoke tests pass
- end-to-end tests pass or known gaps are documented
- logs are usable for support
- migrations are repeatable
- seed data process is safe

## Data Readiness

Confirm:

- organization boundaries are verified
- member and visitor records are scoped correctly
- attendance data is scoped correctly
- data correction process exists
- no unapproved private data is used for test/demo

## Support Readiness

Confirm:

- support playbook is updated
- issue categories are clear
- severity rules are clear
- support owner is identified
- escalation path is known
- known limitations can be explained clearly

## Security Readiness

Confirm:

- authentication behavior is stable
- organization access behavior is stable
- permission behavior is stable
- sensitive data exposure has been reviewed
- audit behavior is available where implemented

## Operational Readiness

Confirm:

- deployment process is documented
- pause/rollback process is documented
- release candidate reference is known
- system health can be checked
- support team knows how to report incidents

## User Readiness

Confirm:

- onboarding instructions exist
- pilot users understand included features
- unavailable features are explained clearly
- training or walkthrough is prepared where useful

## Decision Options

Choose one:

```text
proceed to wider pilot
run another controlled pilot
stabilize further
pause rollout
```

## Final Rule

Wider rollout should be earned by evidence from pilot usage, not assumed from implementation completion.
