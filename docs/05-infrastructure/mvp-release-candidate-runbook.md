# MVP Release Candidate Runbook

## Purpose

This document defines the runbook for preparing an MVP release candidate.

A release candidate is not automatically production-ready. It is a controlled build that can be tested and reviewed for pilot readiness.

## Release Candidate Principle

A release candidate should be stable, traceable, testable, and documented.

## Step 1: Confirm Scope

Confirm the release candidate includes only MVP-approved features.

Review:

```text
docs/14-roadmap/mvp-scope-lock.md
```

## Step 2: Confirm Branch and PR Status

Confirm:

- all required MVP PRs are merged
- no critical PR is pending
- stabilization branch is up to date
- CI checks pass where available

## Step 3: Confirm Backend Checks

Confirm:

- backend builds
- tests pass
- health endpoint works
- migrations run cleanly
- seed data runs cleanly
- core APIs respond correctly

## Step 4: Confirm Flutter Checks

Confirm:

- Flutter project builds where applicable
- Flutter tests pass where applicable
- login flow works
- organization selection works
- Religious MVP screens load
- standard API errors display safely

## Step 5: Confirm Data Setup

Confirm pilot/demo data exists for:

- organization
- owner/admin user
- modules
- permissions and roles
- Religious settings
- optional sample structure data

Do not use real private data in demo seed sets.

## Step 6: Confirm End-to-End Tests

Run the MVP end-to-end test plan:

```text
docs/16-quality/mvp-end-to-end-test-plan.md
```

## Step 7: Confirm UAT Readiness

Prepare UAT checklist:

```text
docs/16-quality/user-acceptance-testing-checklist.md
```

## Step 8: Confirm Known Limitations

Document known limitations clearly.

Examples:

- QR attendance deferred
- GPS attendance deferred
- live SMS deferred
- live payments deferred
- offline sync deferred

## Step 9: Create Pilot Readiness Report

Use:

```text
docs/17-implementation-notes/pilot-readiness-report-template.md
```

## Step 10: Release Candidate Decision

Mark release candidate status as one of:

```text
ready for pilot testing
ready with accepted limitations
not ready
```

## Final Rule

Do not call the MVP ready because it builds. Call it ready only when core workflows have been verified and known limitations are documented.
