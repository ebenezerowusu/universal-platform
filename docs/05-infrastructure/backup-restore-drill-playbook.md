# Backup and Restore Drill Playbook

## Purpose

This document defines a practical backup and restore verification process.

The goal is to prove that the platform can recover important data before broader launch.

## Principle

A backup is not trusted until restore has been tested.

## When To Run

Run a restore drill:

- before broader launch
- after major migration changes
- after database hosting changes
- before moving to a larger rollout wave
- on a recurring schedule after launch

## Drill Scope

For MVP, verify restore confidence for:

- organizations
- users and organization memberships
- roles and permissions
- modules and features
- Religious members
- Religious visitors
- congregations/services/groups
- attendance sessions and records

## Step 1: Confirm Backup Source

Record:

```text
backup_date
backup_location
backup_type
created_by_or_process
retention_period
```

## Step 2: Prepare Restore Environment

Use a safe restore environment.

Do not restore over live production data during a drill.

## Step 3: Restore Data

Restore the selected backup into the safe environment.

Record any restore warnings or errors.

## Step 4: Verify Core Data

Confirm restored data includes expected:

- organization records
- user records
- module state
- Religious member records
- visitor records
- attendance records

## Step 5: Run Smoke Tests

Run relevant smoke tests after restore:

```text
docs/16-quality/production-smoke-test-checklist.md
```

## Step 6: Record Result

Use:

```text
passed
failed
partial
blocked
```

Record:

- restore duration
- missing data if any
- errors observed
- follow-up actions

## Step 7: Review Recovery Confidence

Decide whether backup and restore confidence is:

```text
acceptable
acceptable with limitations
not acceptable
```

## Final Rule

Do not rely on backups for launch confidence until at least one restore path has been tested and documented.
