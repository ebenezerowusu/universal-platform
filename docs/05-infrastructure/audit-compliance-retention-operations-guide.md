# Audit Compliance Retention Operations Guide

## Purpose

This document defines operating guidance for audit events, access reviews, retention policies, archive/delete request placeholders, and compliance checklist placeholders.

The goal is to keep sensitive actions explainable while preventing unsafe deletion or cross-organization governance access.

## Principle

Every sensitive action should answer:

```text
who acted
what action happened
which record was affected
when it happened
why it happened where available
whether the action was permitted
```

## Data Sources

Governance workflows may use:

- audit event
- sensitive-action category
- access review placeholder
- retention policy metadata
- archive/delete request placeholder
- compliance checklist item
- audit records for governance actions

## Audit Event Direction

Audit events should capture:

```text
organization_id
actor_id optional
action
target_type
target_id optional
category
severity placeholder
metadata placeholder
created_at
```

Sensitive metadata should not be exposed unless the user has the correct permission.

## Access Review Direction

Access reviews should focus first on high-risk permissions:

- admin permissions
- export permissions
- finance permissions
- HR/workforce permissions
- document access permissions
- inactive users with active permissions

## Retention Policy Direction

Retention policy metadata should define:

```text
policy_key
module
target_record_type
retention_period_placeholder
archive_behavior_placeholder
delete_behavior_placeholder
status
```

## Archive/Delete Request Direction

Archive/delete request placeholders should capture:

```text
request_type
target_type
target_id
requested_by
reason optional
status
created_at
resolved_by optional
resolved_at optional
```

Do not implement automatic destructive deletion in MVP.

## Compliance Checklist Direction

Checklist placeholders may include:

- audit logging enabled
- admin access reviewed
- export permissions reviewed
- retention policies drafted
- sensitive actions monitored
- archive/delete requests reviewed

## Data Safety Direction

Governance operations should prevent:

- cross-organization audit visibility
- unauthorized sensitive audit search
- destructive deletion without review
- retention policy changes without audit
- false legal compliance guarantees

## Audit Direction

Audit should record:

- sensitive audit query executed where policy requires it
- retention policy changed
- retention request created
- retention request resolved placeholder
- access review completed placeholder
- compliance checklist updated

## Data Quality Warnings

Warn or flag when:

- sensitive action has no audit event
- inactive user still has privileged permission
- retention policy is missing for sensitive domain
- retention request references missing record
- governance checklist is incomplete

## Final Rule

Governance operations must protect audit visibility, preserve decision history, and avoid destructive actions unless explicitly designed and approved in a future sprint.
