# Sprint 18 Communication Governance Roadmap

## Purpose

This document defines Sprint 18 for Communication Governance.

Sprint 18 should add the foundations needed to make communication safe, respectful, auditable, and scalable before advanced campaign features are considered.

## Sprint Goal

Build the foundation for recipient preferences, opt-out handling, message templates, recipient segmentation, and safe send checks.

## Why This Sprint

Sprint 16 prepared Communication and SMS Wallet.

Sprint 17 prepared provider readiness and delivery reconciliation.

Sprint 18 ensures communication remains controlled before scale increases:

- recipients can be excluded safely
- message templates improve consistency
- recipient segments reduce manual targeting errors
- send behavior can be reviewed and audited
- future campaigns have governance foundations

## Work Package 1: Recipient Preferences

Prepare foundation for communication preferences by organization recipient.

Suggested preference areas:

```text
sms_allowed
sms_opted_out
preferred_language
preferred_channel_future
updated_by
updated_at
```

## Work Package 2: Opt-Out Handling

Prepare opt-out behavior so recipients can be excluded from future sends.

MVP may allow admin-managed opt-out first, with future self-service or inbound reply handling deferred.

## Work Package 3: Message Templates

Prepare reusable templates for common organization messages.

Examples:

- service reminder
- visitor follow-up
- group notice
- event reminder
- general announcement

## Work Package 4: Recipient Segments

Prepare simple segments such as:

- active members
- visitors
- selected group
- manual list
- custom saved segment foundation

## Work Package 5: Safe Send Rules

Before sending, confirm:

- user has permission
- module and feature are available
- wallet has enough units where required
- recipients are eligible
- opted-out recipients are excluded
- message is auditable

## Work Package 6: Flutter Foundation

Prepare screens or placeholders for:

- preferences
- templates
- segments
- recipient preview
- send review

## Out of Scope

Do not implement:

- advanced automation builder
- WhatsApp or email campaigns
- AI message generation
- inbound opt-out processing unless approved
- complex marketing analytics

## Completion Standard

Sprint 18 is complete when communication can respect preferences, use templates, preview recipient segments, and apply safe send checks before delivery.

## Final Rule

Communication should not scale until preferences, opt-outs, templates, segments, and safe send rules are in place.
