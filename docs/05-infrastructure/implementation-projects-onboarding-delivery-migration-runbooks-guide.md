# Implementation Projects Onboarding Delivery Migration Runbooks Guide

## Purpose

This document defines operating guidance for implementation projects, onboarding milestones, migration runbooks, setup tasks, acceptance checks, go-live readiness, implementation issues/risks, delivery links, implementation notes, and implementation audit history.

The goal is to make onboarding and migration delivery repeatable, reviewable, and traceable.

## Principle

Implementation delivery should coordinate onboarding work.

It should not replace project management, import/export, data quality, support operations, or customer success.

## Data Sources

Implementation delivery operations may use:

- implementation project metadata
- onboarding milestone/checklist placeholder
- migration runbook metadata placeholder
- migration task link
- configuration/setup task placeholder
- acceptance/go-live readiness placeholder
- implementation issue/risk placeholder
- delivery linkage placeholder
- implementation note placeholder
- audit records

## Project Direction

Implementation project records should capture:

```text
organization_id
project_type
implementation_phase
owner_safe_label
target_go_live_placeholder
status
```

## Milestone Direction

Onboarding milestones may include:

- organization setup
- admin user setup
- module enablement
- configuration review
- training/help readiness
- pilot usage
- go-live readiness

## Migration Runbook Direction

Migration runbook placeholders should capture:

- source system summary
- migration scope
- migration owner
- validation checklist
- rollback/contingency placeholder
- status

## Migration Link Direction

Migration links may connect implementation projects to:

- import jobs
- export packages
- migration validation tasks
- data quality reviews
- duplicate/merge reviews
- migration audit records

Implementation delivery should not execute destructive migration actions directly.

## Setup Task Direction

Setup tasks may include:

- roles/permissions
- module settings
- payment/SMS/provider settings
- localization/timezone/currency
- notification preferences
- branch/network setup

## Go-Live Readiness Direction

Go-live readiness placeholders may include:

- stakeholder sign-off placeholder
- data quality acceptance placeholder
- training/help readiness placeholder
- support readiness placeholder
- rollback readiness placeholder
- final readiness review

Go-live readiness should require human review.

## Issue and Risk Direction

Implementation issues/risks may include:

- missing data
- configuration blocker
- user access blocker
- training blocker
- migration blocker
- customer decision pending

## Delivery Link Direction

Delivery links may connect to:

- customer success playbooks
- support cases
- knowledge base articles
- product analytics onboarding funnels
- policy/training acknowledgements

## Notes Direction

Implementation notes should be safe, permission-controlled, and free of unnecessary sensitive data.

## Audit Direction

Audit should record:

- implementation project created or updated
- onboarding milestone updated
- migration runbook created or updated
- migration task link changed
- setup task created or completed placeholder
- readiness review recorded
- implementation issue/risk created or resolved placeholder
- go-live readiness status changed

## Data Quality Warnings

Warn or flag when:

- project has no owner
- milestone is overdue placeholder
- migration runbook has no validation checklist
- data quality review is missing before go-live readiness
- setup task is blocked
- readiness review marks ready with unresolved critical issue
- implementation note contains sensitive details placeholder

## Final Rule

Implementation delivery operations must preserve tenant privacy, avoid automatic go-live approval, and keep onboarding, migration, support, and customer success work traceable.
