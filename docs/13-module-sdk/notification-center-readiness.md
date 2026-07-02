# Notification Center Readiness

## Purpose

This document prepares the Notification Center and User Preferences foundation for implementation.

It ensures notifications are reusable across domains, preference-aware, provider-agnostic, and safe for users.

## Wave Summary

Build foundations for in-app notifications, notification categories, template metadata, user preferences, organization defaults, delivery logs, and domain trigger references.

## Problem Being Solved

Many modules need to notify users about reminders, approvals, orders, deliveries, attendance, finance, and system updates.

Without a shared notification center, each module will create its own messaging logic and bypass user preferences.

## Evidence

This wave is supported by:

- Notification Engine direction
- SMS Engine direction
- Calendar reminder needs
- Commerce order status needs
- Delivery status needs
- HR approval and attendance needs
- Finance alert needs
- Religious event reminder needs

## Primary Users

- organization users
- organization owner/admin
- department or domain managers
- customer or guest user later where supported
- platform support/admin user

## MVP Scope

Included:

- in-app notification foundation
- notification type/category foundation
- template metadata foundation
- user preference foundation
- organization default preference foundation where practical
- delivery log foundation
- domain trigger reference foundation
- read/archive foundation

Excluded:

- full marketing campaign engine
- push provider integration
- email provider integration
- WhatsApp provider integration
- advanced personalization
- unrestricted broadcast messaging
- bulk SMS sales expansion

## Domain Ownership

Notification Center owns user-facing notification records and preferences.

Notification Engine owns notification orchestration.

SMS Engine owns SMS delivery where used.

Domain modules own their source events and request notifications through the engine.

Platform Core must not contain domain-specific notification content rules.

## Required Engines

- Notification Engine
- Permission Engine
- Audit Engine where required
- SMS Engine where SMS placeholders are used
- Configuration Engine
- Feature Flag or License Engine

## Suggested Permissions

```text
notifications.view
notifications.manage
notifications.preferences.view
notifications.preferences.manage
notifications.delivery_logs.view
notifications.templates.manage
notifications.admin.manage
```

## Data Ownership

Records should include:

- notification
- notification category
- template metadata
- user preference
- organization default preference
- delivery log
- domain trigger reference

## API Direction

Planned API groups:

- notification inbox
- notification detail
- read/archive actions
- preferences
- organization defaults
- delivery logs
- template metadata
- domain trigger references

## UI Direction

Planned screens:

- notification inbox
- notification detail
- unread filter
- archived notifications
- notification preferences
- delivery warning placeholder

## Audit Direction

Audit important actions where policy requires:

- notification template changed
- organization default preferences changed
- user notification preferences changed
- broadcast-like notification requested
- delivery failure manually resolved

## Revenue Direction

This wave supports premium communication, SMS workflows, reminder automation, and future push/email/WhatsApp add-ons.

## Risks

- notifications bypassing user preferences
- domains calling providers directly
- sensitive data inside notification snippets
- duplicate notifications
- unrestricted broadcast messaging too early

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Notification Center must centralize user-facing messages and preferences while provider-specific delivery remains behind engines.
