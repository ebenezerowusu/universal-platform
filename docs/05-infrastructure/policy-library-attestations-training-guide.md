# Policy Library Attestations Training Guide

## Purpose

This document defines operating guidance for policy metadata, policy versions, publication reviews, attestation requests, acknowledgement records, training assignments, audience targets, evidence links, and policy audit history.

The goal is to make policy communication and acknowledgement traceable.

## Principle

Policy acknowledgement proves that guidance was assigned, made available, and acknowledged.

It should not be presented as legal advice, legal certification, or a full learning management system.

## Data Sources

Policy library operations may use:

- policy document metadata
- policy version placeholder
- publication/review placeholder
- attestation request
- acknowledgement record
- training assignment placeholder
- training completion placeholder
- audience targeting placeholder
- evidence link
- audit records

## Policy Direction

Policy records should capture:

```text
policy_key
organization_id
owner_module_domain
document_reference_placeholder
category
status
```

## Version Direction

Policy version placeholders should capture:

- version label
- document reference
- effective date placeholder
- review date placeholder
- replaced version placeholder
- status

## Publication Direction

Publication review placeholders may include:

- draft
- review requested
- approved placeholder
- published placeholder
- retired placeholder

## Attestation Direction

Attestation requests should capture:

```text
policy_version_id
target_audience_placeholder
due_date_placeholder
status
created_by
```

## Acknowledgement Direction

Acknowledgement records should capture:

```text
attestation_request_id
recipient_reference_placeholder
acknowledgement_status
acknowledged_at_placeholder
source_channel_placeholder
```

## Training Direction

Training assignment placeholders may include:

- policy-related training
- role-based training
- branch/network training
- onboarding training
- refresher training

## Audience Direction

Audience targets may include:

- users
- roles
- workforce groups
- branch/network groups
- module-specific actors

## Evidence Direction

Evidence links may connect policy activity to:

- risk/control checks
- acknowledgement records
- training completion placeholders
- policy publication events
- document/media references

## Notification and Workflow Direction

Notification and workflow integration may later support:

- policy publication notifications
- attestation reminders
- overdue acknowledgement tasks
- review approval workflows
- escalation for missing acknowledgement

## Audit Direction

Audit should record:

- policy created or updated
- policy version created or retired placeholder
- policy published placeholder
- attestation request created
- acknowledgement captured
- training assignment created
- training completion captured placeholder
- evidence link added or removed placeholder

## Data Quality Warnings

Warn or flag when:

- policy has no document reference
- published policy has no active version
- attestation has no audience target
- acknowledgement is overdue
- training assignment has no due date where required
- evidence link is inaccessible
- policy review date is overdue

## Final Rule

Policy library operations must preserve document permissions, audience targeting, evidence linkage, and audit history.
