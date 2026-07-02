# Notification Delivery Operations Guide

## Purpose

This document defines operating guidance for notification records, preferences, delivery logs, and provider boundaries.

The goal is to make notification delivery explainable, preference-aware, and provider-agnostic.

## Principle

Domain modules should request notifications through Notification Engine.

They should not call SMS, email, push, or WhatsApp providers directly.

## Data Sources

Notification workflows may use:

- notification record
- notification category
- template metadata
- user preference
- organization default preference
- domain trigger reference
- delivery log
- audit records

## Preference Direction

Before creating or delivering a notification, check:

- recipient exists and is eligible
- category preference
- organization default preference
- channel availability
- quiet-hours placeholder where supported
- domain-specific opt-out rules where needed later

## Delivery Log Direction

Delivery logs should capture:

```text
notification_id
channel
status
provider_placeholder
attempted_at
failure_reason optional
skipped_reason optional
```

## Domain Trigger Direction

Domain trigger references should capture:

```text
domain_type
domain_record_id
organization_id
triggered_by
triggered_at
```

Domain references must not cross organization boundaries.

## Duplicate Prevention

Before creating a notification, check:

- domain trigger reference
- recipient
- category
- idempotency key where available
- duplicate window where policy later defines it

## Safe Content Direction

Notification titles and summaries should contain safe content.

Do not include private domain details unless the recipient has access.

## Provider Direction

Sprint 32 should not require push, email, or WhatsApp provider integrations.

SMS behavior should continue through SMS Engine where used.

## Audit Direction

Audit may record:

- organization default preference changed
- template metadata changed
- broadcast-like request created
- delivery failure manually resolved
- provider configuration changed later

## Data Quality Warnings

Warn or flag when:

- notification has no eligible recipient
- delivery skipped by preference
- delivery failed repeatedly
- domain reference points to missing record
- notification content contains unsafe fields by policy later

## Final Rule

Notification delivery must be explainable and must respect preferences before provider delivery is attempted.
