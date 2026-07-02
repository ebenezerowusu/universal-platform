# Sprint 36 Audit Compliance Retention Review

## Purpose

This document defines review checks for Sprint 36.

Sprint 36 implements Audit, Compliance, and Data Retention foundation.

## Review 1: Scope

Sprint 36 may include:

- audit/compliance feature registration foundation
- audit event metadata foundation
- audit event query foundation
- sensitive action category foundation
- access review placeholder where practical
- retention policy metadata foundation
- retention action placeholder where practical
- safe deletion/archive request placeholder where practical
- compliance flag/checklist placeholder where practical
- permission and feature checks
- audit records for compliance/retention actions
- API and UI foundations

Sprint 36 should not include:

- legal advice engine
- automatic destructive deletion
- full GRC/compliance suite
- government filing workflows
- automated regulatory classification
- cross-organization audit visibility
- provider-specific archive storage calls inside domain modules
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Audit Foundation owns generic audit events and sensitive-action categories
- Retention Foundation owns retention metadata and safe action requests
- domain modules emit audit events and retention candidates
- Permission Engine controls access decisions
- Platform Core does not contain domain-specific compliance rules

## Review 3: Organization Boundaries

Confirm:

- audit events are organization-scoped
- retention policies are organization-scoped or platform-defined safely
- retention requests are organization-scoped
- access reviews are organization-scoped
- audit search cannot cross organization boundaries

## Review 4: Sensitive Access Rules

Confirm:

- audit views require permission
- sensitive metadata requires sensitive audit permission
- governance actions are auditable
- inactive privileged users can be flagged where practical
- compliance checklist updates require permission

## Review 5: Retention Rules

Confirm:

- retention policies are metadata only in MVP
- archive/delete requests are placeholders
- destructive deletion is not automatic
- retention action requests preserve requester, reason, and status
- retention policy changes create audit records

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/audit-compliance-retention-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/audit-compliance-retention-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- audit event query scope
- sensitive permission denied behavior
- retention policy create/update
- retention request creation
- access review placeholder creation
- compliance checklist update
- organization boundary enforcement
- destructive deletion not available

## Final Rule

Sprint 36 is complete when organizations can review audit events, filter sensitive actions, manage retention metadata, request safe archive/delete placeholders, and track compliance checklist placeholders through permission-protected and auditable workflows.
