# Sprint 8 MVP Stabilization Plan

## Purpose

This document defines Sprint 8 for MVP stabilization and controlled pilot readiness.

Sprint 8 should not expand the product. It should make the existing MVP reliable enough for a first pilot.

## Sprint Goal

Prepare the MVP for a controlled pilot organization by fixing critical issues, validating core flows, and confirming release-candidate readiness.

## Prerequisites

Before Sprint 8 begins:

- backend foundation is implemented
- Identity and Organization are implemented
- Permission and Audit foundations are implemented
- Module Registry and Subscription foundations are implemented
- Religious MVP backend foundation is implemented
- Flutter client foundation is implemented
- Religious MVP Flutter workflows are implemented

## Work Package 1: End-to-End Flow Review

Validate the main MVP flow:

```text
Login
  -> select organization
    -> view Religious dashboard
      -> create member
        -> create visitor
          -> convert visitor to member
            -> create congregation/service/group
              -> create attendance session
                -> mark attendance
                  -> view basic summary/report
```

## Work Package 2: API Contract Alignment

Confirm Flutter and backend agree on:

- route paths
- request payloads
- response payloads
- error shapes
- pagination shape
- validation behavior

## Work Package 3: UI State Completion

Complete missing states for MVP screens:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## Work Package 4: Seed and Demo Data

Prepare safe pilot/demo seed data.

Seed data should support:

- initial organization
- owner/admin user
- Religious modules enabled
- starter roles/permissions
- sample congregation/service/group where appropriate

Do not include real private data.

## Work Package 5: Smoke Testing

Prepare and run smoke tests for:

- backend health
- login
- organization list
- module availability
- member list/create
- visitor list/create/convert
- attendance session create/list
- attendance mark
- Flutter startup

## Work Package 6: Pilot Readiness Review

Create a pilot-readiness report using:

```text
docs/17-implementation-notes/pilot-readiness-report-template.md
```

## Out of Scope

Do not add:

- new major modules
- advanced Religious features
- QR/GPS attendance
- full offline sync
- provider integrations unless approved
- POS/Commerce/Finance/HR feature work

## Completion Standard

Sprint 8 is complete when the MVP can be demonstrated end-to-end with known limitations documented.

## Final Rule

Stabilize what already exists. Do not expand the MVP unless a missing item blocks the first controlled pilot.
