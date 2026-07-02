# Notification Center UI Contract

## Purpose

This document defines the MVP UI contract for Notification Center and User Preferences foundation.

The screens should help users review notifications, mark messages read, archive messages, and configure basic preferences.

## Navigation Direction

Notification screens should appear when:

- user is authenticated
- organization is selected
- notification feature is available
- user has required permission
- feature availability allows notifications

## Notification Inbox

Should show:

- notification title
- safe summary
- category
- unread/read status
- created date
- action to view detail

## Notification Detail

Should show:

- title
- safe body/summary
- category
- domain reference hint where safe
- created date
- read state
- archive action

## Unread Filter

Should support:

- all notifications
- unread only
- category filter
- archived view placeholder

## Preferences Screen

Should support:

- category list
- in-app toggle
- SMS placeholder toggle where available
- muted category placeholder
- quiet-hours placeholder

## Delivery Warning Placeholder

Admin users may see warnings such as:

- delivery failed
- skipped by preference
- template unavailable
- source domain unavailable

## Empty States

Should show clear messages for:

- no notifications
- no unread notifications
- no archived notifications
- preferences unavailable

## Access Safety

If the linked source record is no longer accessible, show a generic unavailable state.

Do not reveal restricted domain information through notification text or route hints.

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 32:

- full marketing campaign UI
- push provider setup UI
- email provider setup UI
- WhatsApp provider setup UI
- advanced personalization UI
- unrestricted broadcast messaging UI

## Final Rule

Notification UI should centralize safe user messages while respecting preferences, access, and provider boundaries.
