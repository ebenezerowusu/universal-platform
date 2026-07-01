# Sprint 8 Quality Gates

## Purpose

This document defines quality gates for Sprint 8 implementation.

Sprint 8 should stabilize the MVP for a controlled first pilot without expanding product scope.

## Gate 1: Scope Control

Sprint 8 may include:

- MVP bug fixes discovered during testing
- missing loading, empty, error, and unavailable states
- API contract mismatch fixes
- seed data fixes for pilot demo
- smoke test improvements
- release-candidate documentation updates
- pilot-readiness notes
- small UX fixes that do not expand MVP scope

Sprint 8 must not include:

- new major modules
- advanced Religious features
- QR/GPS attendance
- full offline sync
- live provider integrations unless explicitly approved
- POS/Commerce/Finance/HR feature work
- large refactors not required for pilot stability

## Gate 2: End-to-End Flow Check

Confirm the main MVP journey works:

```text
Login
  -> select organization
    -> Religious dashboard
      -> members
      -> visitors
      -> structure
      -> attendance
      -> basic summary/report
```

## Gate 3: API Alignment Check

Confirm Flutter and backend agree on:

- route paths
- request payloads
- response payloads
- error shape
- pagination shape
- validation behavior

## Gate 4: UI State Check

Confirm MVP screens handle:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## Gate 5: Security and Boundary Check

Confirm:

- unauthenticated access is blocked
- organization boundary is enforced
- permission denials are handled safely
- sensitive values are not exposed in general lists
- important actions are auditable where implemented

## Gate 6: Seed and Demo Data Check

Confirm:

- seed data is safe
- seed data is repeatable
- pilot/demo data does not contain real private records
- required modules and permissions are enabled for demo organization

## Gate 7: Release Candidate Check

Confirm:

- backend checks pass
- Flutter checks pass where practical
- smoke tests pass or failures are documented
- known limitations are documented
- release-candidate runbook is followed

## Gate 8: UAT Check

Confirm:

- UAT checklist was executed or scheduled
- critical UAT issues are resolved
- high UAT issues are resolved or accepted
- user confusion blocking core workflows is not ignored

## Gate 9: Pilot Readiness Report

Confirm a pilot-readiness report exists using:

```text
docs/17-implementation-notes/pilot-readiness-report-template.md
```

## Final Rule

Sprint 8 is complete only when the MVP is stable enough for controlled pilot testing with clear known limitations.
