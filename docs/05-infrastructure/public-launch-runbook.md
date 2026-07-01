# Public Launch Runbook

## Purpose

This document defines the runbook for launch-day execution.

The goal is to launch with control, visibility, and a clear pause path.

## Launch Principle

Launch only when readiness has been proven by rollout evidence, not by optimism.

## Step 1: Confirm Launch Decision

Confirm the launch decision is recorded using:

```text
docs/17-implementation-notes/launch-decision-record-template.md
```

Proceed only when the decision is:

```text
launch approved
launch approved with limitations
```

## Step 2: Confirm Release Reference

Record:

```text
release_tag_or_commit
backend_version
flutter_version
migration_state
release_owner
support_owner
```

## Step 3: Confirm Known Limitations

Confirm known limitations are documented and support can explain them clearly.

Examples:

- QR attendance deferred
- GPS attendance deferred
- live SMS deferred unless approved
- live payments deferred unless approved
- full offline sync deferred

## Step 4: Confirm Backup Confidence

Confirm:

- backup source is known
- restore drill has acceptable result or approved limitation
- rollback/pause path is known

## Step 5: Run Launch-Day Smoke Tests

Run:

```text
docs/16-quality/launch-day-smoke-test-checklist.md
```

Do not proceed if core launch smoke tests fail.

## Step 6: Open Launch Window

When checks pass:

- confirm support is available
- confirm monitoring is active
- confirm issue reporting path is ready
- open access according to approved launch scope

## Step 7: Monitor First Hours

Track:

- login success
- organization creation or access
- Religious dashboard usage
- member/visitor workflows
- attendance workflows
- support requests
- unexpected errors

## Step 8: Decide Continue, Pause, or Fix

Choose one:

```text
continue launch
continue with limitations
pause launch
fix forward
rollback release candidate
```

## Step 9: Record Launch Outcome

Record:

- launch status
- issues observed
- support volume
- decisions made
- next actions

## Final Rule

Do not expand launch access while core smoke tests, support, or monitoring are uncertain.
