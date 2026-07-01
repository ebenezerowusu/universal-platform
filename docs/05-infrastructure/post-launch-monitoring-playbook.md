# Post-Launch Monitoring Playbook

## Purpose

This document defines what the team should watch immediately after launch.

The goal is to detect problems early and respond before they affect many users.

## Monitoring Principle

The first launch window should be observed closely.

Do not assume that a successful deployment means the user experience is healthy.

## Monitoring Areas

Track:

- application health
- successful user sessions
- organization usage
- Religious dashboard usage
- member and visitor workflows
- attendance workflows
- support requests
- repeated user confusion
- unexpected error patterns

## First-Hour Review

During the first hour, check:

- app is reachable
- backend is responding
- support channel is active
- no repeated core workflow failures are reported
- issue owners are available

## First-Day Review

During the first day, review:

- active organizations
- active users
- support volume
- failed workflows
- data correction requests
- common feedback themes
- critical or high issues

## Issue Response Direction

For serious issues, follow:

```text
docs/05-infrastructure/incident-response-playbook.md
```

For support items, follow:

```text
docs/05-infrastructure/pilot-support-playbook.md
```

## Launch Status Options

Use:

```text
healthy
healthy with limitations
watch closely
pause expansion
needs fix
```

## Reporting

A launch monitoring note should include:

```text
time window
active organizations
active users
support requests
critical issues
high issues
workflow concerns
next actions
```

## Final Rule

Post-launch monitoring should focus on user confidence and core workflow health, not vanity metrics.
