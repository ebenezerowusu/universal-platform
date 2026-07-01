# Pilot Pause and Rollback Playbook

## Purpose

This document defines when and how to pause a pilot or roll back a release candidate.

The goal is to protect users, data quality, and trust during early rollout.

## Principle

It is better to pause early than to allow a confusing or risky pilot to continue uncontrolled.

## Pause Triggers

Pause pilot expansion when:

- users cannot log in reliably
- organization access is incorrect
- important data is missing or incorrect
- member, visitor, or attendance flows fail repeatedly
- users report serious confusion that blocks core workflows
- support cannot keep up with pilot issues
- release candidate behavior differs from documented limitations

## Rollback Triggers

Consider rollback when:

- a release candidate creates repeated failed workflows
- a migration creates unexpected data issues
- Flutter and backend contracts are badly mismatched
- a fix introduces wider instability
- support team cannot safely guide pilot users

## Pause Steps

1. Stop adding new pilot users.
2. Inform current pilot users that the pilot is paused for review.
3. Record the reason for pause.
4. Capture affected workflows.
5. Triage the issue severity.
6. Decide whether to fix forward or roll back.
7. Update pilot-readiness notes.

## Rollback Steps

1. Identify the last stable release candidate.
2. Confirm database state and migration impact.
3. Confirm whether data correction is needed.
4. Restore application version or disable problematic feature path.
5. Verify health checks and core flows.
6. Inform pilot users of status.
7. Record the rollback decision and follow-up work.

## Fix Forward Criteria

Fix forward may be acceptable when:

- issue is small and understood
- data impact is low
- fix can be tested quickly
- rollback would create more confusion
- affected users can be guided safely

## Rollback Criteria

Rollback is preferred when:

- impact is broad
- data behavior is uncertain
- fix is risky
- users cannot complete core workflows
- issue affects trust or safety

## Communication Direction

Pilot communication should be calm and clear.

Explain:

- what is affected
- what users should avoid temporarily
- when the team will provide the next update
- whether data entered so far is safe or under review

## Final Rule

Pause and rollback decisions should be based on user impact and data confidence, not pride. Protect the pilot experience first.
