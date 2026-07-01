# Communication Governance Feature Readiness

## Purpose

This document prepares the Communication Governance feature wave for implementation.

It ensures communication growth is controlled before advanced messaging or campaigns are introduced.

## Wave Summary

Build foundations for recipient preferences, opt-out handling, reusable message templates, recipient segments, recipient preview, and safe-send checks.

## Problem Being Solved

Organizations need communication workflows that are respectful, consistent, and controlled.

Without governance, messages may be sent to wrong recipients, opted-out recipients, or poorly targeted groups.

## Evidence

This wave is supported by:

- SMS wallet and provider readiness already planned
- need to protect recipient trust
- need to reduce support issues from wrong recipients
- need for future campaign foundations
- need for reusable templates across domains

## Primary Users

- organization owner/admin
- communication manager
- Religious leader
- group leader where permitted
- support/admin user

## MVP Scope

Included:

- recipient preference foundation
- admin-managed opt-out foundation
- message template foundation
- simple recipient segments
- recipient preview filters
- safe-send rule foundation
- audit direction

Excluded:

- advanced automation
- inbound opt-out processing
- WhatsApp/email campaigns
- AI generated messages
- complex analytics
- public marketing flows

## Domain Ownership

Communication Domain owns communication governance workflows.

SMS Engine owns sending infrastructure.

Platform Core must not own communication-specific business rules.

## Required Engines

- Permission Engine
- Audit Engine
- SMS Engine
- Notification Engine where useful
- Configuration Engine
- Feature Flag or License Engine
- Reporting Engine later

## Suggested Permissions

```text
communication.preferences.view
communication.preferences.manage
communication.templates.view
communication.templates.manage
communication.segments.view
communication.segments.manage
communication.recipient_preview.view
communication.messages.review_send
```

## Data Ownership

Organization-owned records should include:

- communication preference
- communication template
- recipient segment
- recipient segment rule
- send review snapshot

All records must be scoped by organization.

## API Direction

Planned API groups:

- preferences
- templates
- segments
- recipient preview
- send review

## Flutter Direction

Planned screens:

- communication preferences
- template list/create/edit
- segment list/create/edit
- recipient preview
- send review

## Audit Direction

Audit important actions:

- recipient preference changed
- opt-out applied or removed
- template created or updated
- segment created or updated
- send review accepted

## Revenue Direction

Governance may be included in base Communication module initially.

Advanced templates, saved segments, or campaign tools may become premium later.

## Risks

- overbuilding campaign tools too early
- confusing opt-out behavior
- sending to excluded recipients
- exposing private recipient details unnecessarily
- hardcoding Religious-only recipient rules

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Communication governance should be reusable across domains and must protect recipient preferences before message volume grows.
