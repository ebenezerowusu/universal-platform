# Developer Integrations UI Contract

## Purpose

This document defines the MVP UI contract for Developer API Access, Webhooks, and Integration Operations foundation.

The screens should help permitted users manage API clients, access key placeholders, webhook subscriptions, event catalogs, delivery attempts, inbound integration placeholders, rate limit metadata, and integration audit history.

## Navigation Direction

Developer screens should appear when:

- user is authenticated
- organization is selected
- developer integration feature is available
- user has required permission
- feature availability allows developer access

## Developer Dashboard

Should show:

- API client count
- active access key count
- webhook subscription count
- failed delivery count
- rate limit status placeholder
- recent integration audit events

## API Client List

Should show:

- client name
- client type placeholder
- owner safe label
- status
- last used placeholder
- created date
- action to view detail

## API Client Detail

Should show:

- client metadata
- access key metadata
- assigned scopes
- rate limit placeholder
- webhook subscriptions linked where available
- audit history summary

## Access Key Placeholder List

Should show:

- key label
- scope count
- status
- created by safe label
- created date
- last used placeholder
- rotate/revoke placeholder actions where permitted

Sensitive key material should not be shown in normal list or detail views.

## Webhook Event Catalog

Should show:

- event key
- module/domain
- description
- payload schema placeholder
- status

## Webhook Subscription List

Should show:

- subscription name
- endpoint safe label
- subscribed event count
- status
- last delivery status placeholder
- action to view detail

## Webhook Subscription Detail

Should show:

- endpoint safe label
- event list
- signing reference placeholder
- status
- recent delivery attempts
- disable placeholder action where permitted

## Delivery Attempt List

Should show:

- event key
- subscription
- status
- attempt count
- response status placeholder
- next retry placeholder
- retry/dead-letter actions where permitted

## Inbound Integration Placeholder

Should show:

- source placeholder
- request type
- status
- received date
- review action where permitted

## Integration Audit History

Should show:

- event type
- actor safe label
- target safe label
- timestamp
- reason/metadata safe summary

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- updating progress
- permission denied

## MVP Exclusions

Do not implement in Sprint 46:

- public app marketplace UI
- external OAuth provider setup UI
- arbitrary script execution UI
- provider-specific SDK setup UI inside domains
- cross-tenant integration browser UI

## Final Rule

Developer UI should make integrations manageable and observable without exposing sensitive material or allowing unsafe cross-tenant access.
