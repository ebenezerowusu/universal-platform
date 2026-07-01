# Incident Response Playbook

## Purpose

This document defines a basic response process for serious issues during limited rollout.

The goal is to respond calmly, protect users, and preserve data confidence.

## Principle

Serious rollout issues should be handled through a clear process.

Do not guess, hide, or continue expanding rollout while core issues are unresolved.

## Incident Categories

Use these categories:

- login/access issue
- organization setup issue
- data display issue
- member or visitor workflow issue
- attendance workflow issue
- performance issue
- deployment issue
- support overload
- unexpected application error

## Severity Levels

Use:

```text
critical - core usage blocked or serious risk
high - important workflow affected
medium - workaround exists
low - minor issue or polish
```

## Required Incident Fields

Capture:

```text
incident_id
reported_at
reported_by
organization
affected_workflow
severity
summary
current_status
owner
next_update_time
resolution_summary
```

## Response Steps

### Step 1: Acknowledge

Confirm the issue has been received and assigned.

### Step 2: Classify

Classify category and severity.

### Step 3: Contain

If needed, pause new organization onboarding until the issue is understood.

### Step 4: Investigate

Review logs, affected workflow, recent changes, and reproduction steps.

### Step 5: Decide

Choose one action:

```text
fix forward
pause rollout
rollback release candidate
accept limitation with guidance
```

### Step 6: Communicate

Give clear status to affected pilot users.

### Step 7: Resolve

Apply fix or operational action.

### Step 8: Review

Record what happened, why it happened, and how to prevent recurrence.

## Communication Direction

Communicate:

- what is affected
- who is affected
- what users should avoid temporarily
- next update timing
- resolution once complete

## Final Rule

During limited rollout, user trust matters more than speed. Pause expansion when serious issues are not understood.
