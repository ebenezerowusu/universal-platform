# Data Correction Playbook

## Purpose

This document defines a safe process for correcting data issues discovered during pilot or early rollout.

The goal is to correct data carefully without hiding mistakes or creating new problems.

## Principle

Data correction should be intentional, documented, and reviewable.

Do not make silent manual changes without a record of why the change happened.

## When To Use

Use this playbook for:

- incorrect member data
- incorrect visitor data
- duplicate records
- wrong attendance record
- wrong group assignment
- organization setup mistake
- role or permission assignment mistake
- seed/demo data issue

## Correction Request Fields

Capture:

```text
requester
organization
affected_record_type
affected_record_id
issue_summary
requested_change
reason
urgency
support_owner
approval_status
completed_at
```

## Correction Steps

### Step 1: Confirm Issue

Confirm the issue exists and affects the correct organization.

### Step 2: Confirm Ownership

Confirm the requester is allowed to request the correction.

### Step 3: Confirm Impact

Check whether the correction affects:

- reports
- attendance summaries
- member counts
- visitor conversion history
- audit history
- related records

### Step 4: Choose Correction Method

Prefer application-level correction where available.

Use direct database correction only when necessary and approved.

### Step 5: Record the Change

Record what changed, who approved it, and why.

Where Audit Engine supports it, write an audit record.

### Step 6: Verify Result

Confirm the corrected data appears properly in backend and Flutter where relevant.

## Duplicate Record Direction

For duplicate records:

- do not delete immediately unless policy allows
- prefer merge or deactivate approach where supported
- preserve useful history where possible
- document the selected action

## Attendance Correction Direction

Attendance corrections should preserve context:

- original status
- corrected status
- who requested correction
- who approved correction
- reason

## Unsafe Corrections

Avoid:

- changing organization ownership without review
- deleting records without backup or approval
- editing audit records
- changing payment-like records manually
- making changes without a request record

## Final Rule

Data correction should build trust. Every correction should be explainable after it happens.
