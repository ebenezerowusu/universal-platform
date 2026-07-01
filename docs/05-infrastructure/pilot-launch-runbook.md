# Pilot Launch Runbook

## Purpose

This document defines the runbook for launching a controlled pilot organization.

The goal is to make pilot setup repeatable and safe.

## Pilot Launch Principle

A pilot should be small, observable, and reversible.

Do not treat pilot launch as a full public release.

## Step 1: Confirm Pilot Decision

Confirm the pilot readiness report decision is one of:

```text
ready for pilot testing
ready with accepted limitations
```

Do not proceed if the decision is:

```text
not ready
```

## Step 2: Confirm Pilot Scope

Confirm pilot users understand what is included:

- login
- organization selection
- Religious dashboard
- members
- visitors
- structure
- attendance
- basic summaries

Confirm exclusions are clear:

- QR attendance
- GPS attendance
- live SMS
- live payments
- POS/Commerce
- HR/Finance
- full offline sync

## Step 3: Prepare Organization

Confirm:

- pilot organization exists
- organization status is active
- timezone/country/currency/language are correct
- Religious modules are active
- starter plan/features are active

## Step 4: Prepare Users

Confirm:

- owner/admin user exists
- required pilot users exist
- users can log in
- users are connected to pilot organization
- role/permission assignments are correct

## Step 5: Prepare Data

Confirm:

- seed/demo data is safe
- no real private data is used without approval
- sample congregation/service/group exists where needed
- test members/visitors are clearly marked where used

## Step 6: Verify System Health

Confirm:

- backend health endpoint works
- database migrations are applied
- Flutter app starts
- key MVP routes respond
- logs are available for troubleshooting

## Step 7: Run Smoke Test

Run core smoke flow:

```text
login
select organization
open Religious dashboard
create member
create visitor
convert visitor where available
create attendance session
mark attendance
view summary where available
```

## Step 8: Share Pilot Instructions

Share with pilot users:

- login details or account setup process
- what to test
- what not to expect yet
- how to report issues
- support contact

## Step 9: Monitor Pilot

During pilot:

- monitor support issues
- record bugs
- capture user confusion
- separate feature requests from pilot blockers
- avoid uncontrolled scope expansion

## Step 10: Close Pilot Window

At pilot close:

- gather feedback
- complete post-pilot review
- classify issues
- decide next release direction

## Final Rule

Pilot launch should be deliberate. If setup is unclear or support is not ready, pause before adding users.
