# Communication and SMS Feature Wave Readiness

## Purpose

This document prepares the Communication and SMS Wallet feature wave for implementation.

It follows the feature wave readiness template and keeps communication provider-agnostic.

## Wave Summary

Build a Communication module foundation with SMS wallet, package catalog, message send request, recipient groups, and message history.

## Problem Being Solved

Organizations need a simple way to communicate with members, visitors, groups, and future platform users.

They also need a clear SMS wallet and usage history so SMS can become a recurring revenue path.

## Evidence

This wave is supported by:

- operational need for member and visitor follow-up
- revenue potential through SMS packages
- reuse potential across Religious, Commerce, HR, and future domains
- existing SMS Engine direction
- need for provider abstraction

## Primary Users

- organization owner/admin
- communication manager
- Religious leader
- group leader where permitted
- support/admin user

## MVP Scope

Included:

- Communication module registration
- SMS wallet balance foundation
- SMS package catalog foundation
- SMS transaction ledger foundation
- message compose foundation
- message send request foundation
- message history foundation
- mock/local SMS provider foundation

Excluded:

- WhatsApp integration
- email campaigns
- advanced automation
- provider credential UI
- real payment collection unless approved
- advanced analytics

## Domain Ownership

Communication Domain with shared SMS Engine and Notification Engine support.

Platform Core must not contain SMS business workflow rules.

## Required Engines

- Module Registry Engine
- Permission Engine
- Audit Engine
- SMS Engine
- Notification Engine where useful
- Subscription or Feature Flag Engine
- Payment Engine later if purchase collection is enabled

## Module Manifest Direction

Suggested module id:

```text
communication
```

Suggested permissions:

```text
communication.view
communication.sms_wallet.view
communication.sms_wallet.manage
communication.messages.view
communication.messages.create
communication.messages.send
communication.history.view
```

Suggested navigation:

```text
Communication
SMS Wallet
Messages
History
```

## Data Ownership

Organization-owned records should include:

- SMS wallet
- SMS package request
- SMS transaction
- message draft
- message send request
- message recipient
- message status record

All records must be scoped by organization.

## API Direction

Planned API groups:

- wallet summary
- package catalog
- transaction history
- message draft/send request
- recipient preview
- message history

## Flutter Direction

Planned screens:

- Communication dashboard
- SMS wallet summary
- package catalog
- compose message
- recipient preview
- message history

## Permission Direction

Protected actions:

- viewing wallet
- managing packages
- creating messages
- sending messages
- viewing message history

## Audit Direction

Audit important actions:

- package request created
- wallet adjusted
- message send requested
- message cancelled where supported
- manual adjustment by support/admin

## Revenue Direction

Near-term revenue direction:

```text
usage-based add-on
SMS wallet package sales
```

Payment collection can be deferred until the Payment Engine flow is approved.

## Risks

- provider coupling risk
- wallet balance mismatch risk
- support confusion around failed messages
- overbuilding campaigns too early
- payment flow complexity if added too soon

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Communication and SMS Wallet should become reusable platform capability. It must not be implemented as a one-off Religious-only feature.
