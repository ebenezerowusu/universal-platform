# Sprint 10 Post-Pilot Hardening Plan

## Purpose

This document defines Sprint 10 for post-pilot hardening.

Sprint 10 should use controlled pilot feedback to improve MVP reliability, usability, and rollout readiness.

## Sprint Goal

Fix the most important pilot findings and prepare the MVP for a wider pilot or limited rollout.

## Prerequisites

Before Sprint 10 begins:

- controlled pilot has been completed
- post-pilot review exists
- pilot feedback has been triaged
- critical pilot issues are known
- accepted limitations are documented
- product owner or architecture lead has approved the hardening scope

## Work Package 1: Pilot Issue Review

Review all pilot findings and classify them as:

```text
must fix before wider pilot
should fix soon
accepted limitation
future backlog
not planned
```

## Work Package 2: Critical and High Fixes

Fix critical and high issues that block wider pilot readiness.

Examples:

- login or access problems
- organization selection confusion
- member workflow failure
- visitor workflow failure
- attendance workflow failure
- serious validation issue
- confusing unavailable feature behavior

## Work Package 3: UX Clarity Improvements

Improve areas where pilot users were confused.

Examples:

- clearer empty states
- clearer validation messages
- better success feedback
- better unavailable feature messages
- clearer navigation labels

## Work Package 4: Data Correction Support

Prepare safe processes for correcting pilot data issues.

Follow:

```text
docs/05-infrastructure/data-correction-playbook.md
```

## Work Package 5: Tests and Regression Coverage

Add or improve tests for pilot-discovered gaps.

Focus on:

- flows that failed during pilot
- organization boundary cases
- permission-related cases
- API/Flutter mismatch cases
- validation and error states

## Work Package 6: Wider Rollout Readiness

Review:

```text
docs/14-roadmap/wider-rollout-readiness-checklist.md
```

Use it to decide whether the MVP can move beyond the first pilot.

## Out of Scope

Do not add:

- unrelated modules
- broad new feature work
- provider integrations without approval
- full offline sync unless approved as rollout-critical
- large architecture changes unless pilot findings prove they are necessary

## Completion Standard

Sprint 10 is complete when pilot-blocking issues are resolved or formally accepted, and the MVP has a clear readiness decision for wider pilot or limited rollout.

## Final Rule

Post-pilot hardening should be evidence-driven. Fix what the pilot proved matters.
