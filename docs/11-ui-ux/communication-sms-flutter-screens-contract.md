# Communication and SMS Flutter Screens Contract

## Purpose

This document defines the MVP Flutter screen contract for Communication and SMS Wallet foundation.

The screens should use the shared app shell, API client, selected organization state, module availability, feature availability, and design system components.

## Navigation Entry

Communication navigation should appear when:

- user is authenticated
- organization is selected
- Communication module is available
- backend navigation or module summary allows the entry

If unavailable, show a safe unavailable state.

## Communication Dashboard

Purpose:

- show communication entry point
- show SMS wallet summary
- show quick links to compose message and history

Possible cards:

- SMS balance
- package catalog
- recent messages
- draft messages

## SMS Wallet Screen

Should show:

- available units
- reserved units where applicable
- package catalog link
- transaction history link
- package request action where permitted

## Package Catalog Screen

Should show:

- package name
- units
- price
- status
- request package action where permitted

MVP may create a package request without live payment collection.

## Compose Message Screen

Should support:

- channel selection set to SMS for MVP
- title or internal label
- message body
- recipient source
- manual recipient list foundation
- recipient preview
- estimated units
- send action where permitted

## Recipient Preview

Should show:

- recipient count
- estimated units
- unavailable or permission state where needed

Do not expose raw private recipient lists more than necessary.

## Message History Screen

Should show:

- message title
- channel
- status
- recipient count
- created date
- sent date where available

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 16:

- WhatsApp screens
- email campaign screens
- provider configuration screens
- advanced campaign analytics
- automation builder
- live payment collection unless approved

## Final Rule

Flutter screens should make Communication and SMS Wallet usable without exposing provider-specific implementation details.
