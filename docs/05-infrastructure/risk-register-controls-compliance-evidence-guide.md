# Risk Register Controls Compliance Evidence Guide

## Purpose

This document defines operating guidance for risk registers, risk assessments, control objectives, control checks, evidence links, exceptions, remediation actions, checklist mappings, review cycles, and risk/control audit history.

The goal is to make operational risks and controls visible, reviewable, and evidence-backed.

## Principle

Risk and controls should organize operational evidence.

They should not replace legal review, audit logs, monitoring systems, incident response, or governance workflows.

## Data Sources

Risk and controls operations may use:

- risk register metadata
- risk assessment placeholder
- control objective metadata
- control check placeholder
- evidence item link
- control exception placeholder
- remediation action placeholder
- compliance checklist mapping placeholder
- review cycle placeholder
- audit records

## Risk Direction

Risk records should capture:

```text
risk_key
organization_id
owner_safe_label
risk_category
severity_placeholder
likelihood_placeholder
status
```

## Assessment Direction

Risk assessments should capture:

- inherent risk placeholder
- residual risk placeholder
- impact summary
- likelihood summary
- review date placeholder
- reviewer safe label

## Control Direction

Control objectives should capture:

```text
control_key
control_category
mapped_domain_module
owner_safe_label
status
review_frequency_placeholder
```

## Control Check Direction

Control checks may include:

- manual check
- scheduled check placeholder
- evidence-required check
- monitoring-linked check
- audit-linked check

## Evidence Direction

Evidence links may reference:

- audit events
- incident records
- backup drill records
- privacy/export reviews
- governance decisions
- provider connection health checks
- document/media references

Evidence access should respect permissions and privacy masking.

## Exception Direction

Exceptions should capture:

- control reference
- exception summary
- severity placeholder
- review status
- remediation link placeholder
- decision safe summary

## Remediation Direction

Remediation actions should capture:

- action owner
- due date placeholder
- linked risk/control/exception
- status
- completion evidence placeholder

## Checklist Mapping Direction

Checklist mappings may group controls for internal readiness views.

Checklist mappings must not claim legal or regulatory certification.

## Review Cycle Direction

Review cycles may define:

- review frequency placeholder
- next review date placeholder
- responsible owner
- scope summary
- status

## Audit Direction

Audit should record:

- risk created or updated
- risk assessment updated
- control objective created or updated
- control check completed placeholder
- evidence linked or removed placeholder
- exception created or reviewed
- remediation action created or completed
- checklist mapping changed

## Data Quality Warnings

Warn or flag when:

- high risk has no control mapping
- control has no recent check
- evidence link is missing or inaccessible
- exception has no remediation action
- remediation is overdue
- checklist mapping has open exceptions
- review cycle is overdue

## Final Rule

Risk and controls operations must preserve organization boundaries, evidence permissions, audit history, and honest readiness language.
