# Tenant Growth Operations Guide

## Purpose

This document defines operational guidance for supporting more organizations on Universal Platform.

The goal is to grow usage without losing setup quality, support quality, or platform boundaries.

## Principle

Growth should be repeatable.

If every new organization needs custom manual setup, the process must be improved before scaling further.

## Organization Intake

Capture the same setup details for each organization:

```text
organization_name
country
timezone
currency
default_language
owner_admin_name
support_contact
expected_users
expected_core_workflows
known_special_requirements
```

## Setup Consistency

Each organization should follow a common setup path:

- create organization
- assign owner/admin
- activate modules
- assign starter plan/features
- assign starter access profiles
- configure Religious settings where applicable
- run smoke test
- record onboarding completion

## Growth Controls

Before adding more organizations, confirm:

- support load is manageable
- smoke tests are passing
- onboarding process is not creating repeated confusion
- data correction volume is acceptable
- no open critical issue affects core workflows

## Operational Review Cadence

During growth, review weekly or per rollout wave:

- organizations onboarded
- active users
- support requests
- critical/high issues
- smoke test failures
- accepted limitations
- repeated setup problems

## Standardization Opportunities

Look for repeated manual steps that should become:

- admin tooling
- seed scripts
- setup checklist items
- API improvements
- better onboarding screens
- clearer support documentation

## Warning Signs

Slow down growth when:

- support issues repeat across organizations
- users do not understand MVP boundaries
- setup requires custom fixes each time
- smoke tests fail often
- data correction requests increase
- core workflows produce inconsistent results

## Final Rule

Tenant growth should be controlled by operational readiness, not by demand alone.
