# Limited Rollout Runbook

## Purpose

This document defines the runbook for onboarding a small number of organizations after the first controlled pilot.

The goal is to make rollout repeatable, observable, and supportable.

## Rollout Principle

Limited rollout is not public launch.

Keep the number of organizations small until setup, support, and smoke testing are predictable.

## Step 1: Confirm Rollout Decision

Confirm the decision from the wider rollout readiness checklist is:

```text
proceed to wider pilot
```

or:

```text
proceed to limited rollout
```

Do not proceed if the decision is stabilize further or pause rollout.

## Step 2: Select Organizations

Select organizations that are suitable for early rollout.

Prefer organizations that:

- understand the MVP scope
- accept documented limitations
- can provide clear feedback
- have an identified owner/admin
- can participate in support and review

## Step 3: Prepare Organization Setup

For each organization, prepare:

- organization profile
- country/timezone/currency/language
- owner/admin user
- Religious module availability
- starter plan/features
- starter access profiles
- starter Religious settings

## Step 4: Run Setup Smoke Test

After setup, run:

```text
docs/16-quality/production-smoke-test-checklist.md
```

## Step 5: Confirm User Onboarding

Confirm:

- users can log in
- users can select the correct organization
- users understand included features
- users understand excluded features
- users know the support channel

## Step 6: Monitor First Use

During first use, record:

- support issues
- confusing workflows
- failed workflows
- data correction requests
- feature requests
- accepted limitations

## Step 7: Review Organization Health

After initial use, review:

- login success
- core workflow completion
- support volume
- issue severity
- user confidence

## Step 8: Decide Next Organization

Onboard the next organization only when current rollout behavior is stable enough.

## Final Rule

Limited rollout should progress gradually. Do not add more organizations faster than the support and testing process can handle.
