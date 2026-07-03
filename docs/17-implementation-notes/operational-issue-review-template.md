# Operational Issue Review Template

## Purpose

Use this template when a significant bug, production issue, access-control concern, privacy concern, deployment issue, or reliability problem is discovered.

This document supports factual tracking, mitigation, root cause review, and follow-up work.

## Issue Context

```text
Issue ID:
Date discovered:
Discovered by:
Environment:
Affected version/commit:
Related issue/PR:
Severity:
Status:
```

## Issue Type

Select all that apply:

```text
[ ] Application bug
[ ] Access-control concern
[ ] Privacy concern
[ ] Tenant isolation concern
[ ] Payment issue
[ ] SMS/notification issue
[ ] Data migration issue
[ ] Deployment issue
[ ] Performance issue
[ ] Availability issue
[ ] Other
```

## Summary

Describe what happened in clear, factual language.

## Impact

Describe the known or suspected impact.

Include:

- affected users or organizations, using safe labels only
- affected module/foundation
- affected data category, if any
- duration, if known
- business impact

## Severity

Use one of:

```text
low
medium
high
critical
```

Explain why.

## Immediate Mitigation

Document what was done immediately to reduce risk.

```text
<actions taken>
```

## Root Cause

Document the confirmed root cause.

If unknown, state:

```text
Root cause not confirmed yet.
```

## Timeline

```text
YYYY-MM-DD HH:MM - Event
YYYY-MM-DD HH:MM - Event
```

## Data And Privacy Review

Confirm:

```text
[ ] No privacy impact identified
[ ] Privacy impact suspected
[ ] Privacy impact confirmed
[ ] Restricted data category involved
[ ] No restricted data category involved
[ ] Further review required
```

Do not include raw confidential data in this document.

## Permission And Tenant Isolation Review

Confirm:

```text
[ ] Permission checks worked as expected
[ ] Permission checks failed or were missing
[ ] Tenant isolation worked as expected
[ ] Tenant isolation failed or was incomplete
[ ] Further review required
```

## Fix Summary

Describe the fix or planned fix.

```text
<fix summary>
```

## Files Changed

```text
<list files or none yet>
```

## Tests Added Or Updated

```text
<list tests>
```

## Checks Run

```text
cargo fmt
cargo check
cargo test
manual verification placeholder
review checklist placeholder
```

## Audit And Event Review

Audit events involved:

```text
<audit events or none>
```

Domain/platform events involved:

```text
<events or none>
```

## User Communication

State whether communication is required.

```text
required / not required / under review
```

Do not draft external notices here unless approved.

## Follow-Up Actions

- 

## Prevention

Describe what should change to prevent recurrence.

Examples:

- stronger validation
- additional permission test
- tenant isolation test
- new monitoring alert
- documentation update
- migration correction
- deployment checklist update

## Closure Criteria

```text
[ ] Fix merged
[ ] Tests passing
[ ] Impact reviewed
[ ] Audit reviewed
[ ] Follow-up issues created
[ ] Communication decision recorded
[ ] Issue closed
```

## Final Rule

Operational issue notes must be factual, safe, and privacy-aware. Never include confidential values or raw restricted data.
