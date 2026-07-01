# Sprint 16 Communication and SMS Wallet Roadmap

## Purpose

This document defines Sprint 16 for the Communication and SMS Wallet feature wave.

This wave is selected because it supports real organization operations, recurring revenue, and future module communication needs.

## Sprint Goal

Build the foundation for organization communication and SMS wallet usage without coupling the platform to a specific SMS provider.

## Why This Wave

Communication and SMS support:

- Religious member and visitor follow-up
- attendance reminders
- event notices
- administrative alerts
- future Commerce order notifications
- future HR notifications
- wallet/package revenue

## Work Package 1: Communication Module Foundation

Prepare module registration, permissions, navigation, and feature availability for communication.

Suggested module areas:

```text
communication.dashboard
communication.sms_wallet
communication.messages
communication.recipients
communication.history
```

## Work Package 2: SMS Wallet Foundation

Prepare organization-level wallet foundation:

- balance summary
- package catalog
- package purchase request foundation
- transaction ledger
- usage records

Payment collection may remain mocked or deferred until Payment Engine readiness is approved.

## Work Package 3: Message Send Foundation

Prepare foundation for:

- message draft
- recipient selection
- send request
- delivery status placeholder
- send history

## Work Package 4: Recipient Groups

Prepare simple recipient group sources:

- manual phone list
- all active Religious members where permitted
- selected group members where permitted
- visitors where permitted

## Work Package 5: Provider Abstraction

All sending must go through SMS Engine.

Do not call providers directly from Communication Domain or Flutter.

Use local/mock provider for MVP testing.

## Work Package 6: Flutter Foundation

Prepare screens or placeholders for:

- communication dashboard
- wallet balance
- package catalog
- compose message
- message history

## Work Package 7: Audit, Permission, and Feature Checks

Important actions should be auditable.

Protected actions should use Permission Engine.

Feature availability should control wallet and send behavior.

## Out of Scope

Do not implement:

- WhatsApp integration
- email campaigns
- advanced campaign analytics
- direct provider configuration by organization users
- real payment collection unless approved
- complex automation rules

## Completion Standard

Sprint 16 is complete when the platform has a provider-agnostic communication and SMS wallet foundation with APIs, basic UI direction, permissions, audit direction, and test coverage.

## Final Rule

Communication should become a reusable platform capability, not a Religious-only feature.
