# Post-Pilot Issue Prioritization Model

## Purpose

This document defines how post-pilot issues should be prioritized.

The goal is to focus hardening work on the issues that matter most before wider rollout.

## Principle

Priority should be evidence-driven.

Do not prioritize only by who speaks loudest or by what is easiest to build.

## Priority Inputs

Consider these inputs:

- severity
- number of users affected
- importance of affected workflow
- frequency of occurrence
- workaround availability
- risk to data quality
- risk to user trust
- effort to fix
- alignment with MVP scope

## Severity Levels

Use:

```text
critical - blocks pilot or creates serious risk
high - serious workflow issue
medium - workaround exists but experience is poor
low - polish or future improvement
```

## Priority Buckets

Use these priority buckets:

```text
P0 - must fix before wider pilot
P1 - fix before limited rollout if possible
P2 - schedule soon after rollout
P3 - future improvement
P4 - not planned or outside scope
```

## P0 Criteria

An issue is P0 when it:

- blocks login or organization access
- causes wrong organization data to appear
- prevents core member, visitor, or attendance flow
- creates serious data quality risk
- causes repeated pilot failure
- exposes information users should not see

## P1 Criteria

An issue is P1 when it:

- affects an important MVP workflow
- causes repeated user confusion
- has a workaround but is still painful
- weakens pilot confidence
- is needed for a wider pilot to succeed

## P2 Criteria

An issue is P2 when it:

- improves usability
- reduces support load
- fixes non-blocking edge cases
- improves test coverage or maintainability

## P3 Criteria

An issue is P3 when it:

- is useful but not urgent
- improves polish
- supports future growth
- does not block current rollout

## P4 Criteria

An issue is P4 when it:

- is outside MVP direction
- conflicts with platform principles
- duplicates another request
- belongs to a future domain/module

## Decision Record

Every accepted issue should record:

```text
priority
reason
owner
target sprint
accepted limitation if deferred
```

## Final Rule

Post-pilot priority should protect users, data quality, and MVP confidence before adding new features.
