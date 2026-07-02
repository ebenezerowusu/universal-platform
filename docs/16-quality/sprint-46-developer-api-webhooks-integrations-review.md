# Sprint 46 Developer API Webhooks Integrations Review

## Purpose

This document defines review checks for Sprint 46.

Sprint 46 implements Developer API Access, Webhooks, and Integration Operations foundation.

## Review 1: Scope

Sprint 46 may include:

- developer/integration feature registration foundation
- API client/application metadata foundation
- scoped access key placeholder foundation
- API scope registry placeholder where practical
- webhook subscription foundation
- webhook event catalog foundation
- webhook delivery attempt foundation
- webhook retry/dead-letter placeholder where practical
- inbound integration request placeholder where practical
- rate limit/quota metadata placeholder where practical
- integration audit/event history foundation
- permission and feature checks
- audit records for sensitive developer access actions
- API and UI foundations

Sprint 46 should not include:

- unrestricted API key generation
- raw secret display after creation
- external OAuth provider implementation
- public app marketplace
- arbitrary script execution
- provider-specific integration SDK calls inside domain modules
- cross-organization webhook visibility
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Developer Integration Foundation owns API clients, key placeholders, scopes, webhooks, deliveries, inbound placeholders, and integration audit history
- domain modules own business events and source records
- Notification/Event Engine handles event distribution where used
- Permission Engine controls developer access
- Audit Engine records sensitive integration actions
- Platform Core does not contain provider-specific integration SDK behavior

## Review 3: Organization Boundaries

Confirm:

- API clients are organization-scoped
- access key placeholders are organization-scoped
- webhook subscriptions are organization-scoped
- delivery attempts are organization-scoped
- inbound integration placeholders are organization-scoped where linked
- developer records cannot cross organization boundaries

## Review 4: Access and Secret-Safety Rules

Confirm:

- access keys have explicit scopes
- broad scopes are flagged where practical
- sensitive key material is not shown in normal views
- revoked/expired key placeholders cannot remain active
- actor cannot assign scopes beyond permission
- all sensitive developer actions are audited

## Review 5: Webhook Rules

Confirm warnings exist where practical for:

- unsupported event subscription
- invalid endpoint placeholder
- repeated failed delivery
- delivery stuck in retry placeholder
- dead-letter placeholder state
- disabled webhook still receiving events

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/developer-api-webhooks-integrations-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/developer-integrations-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- API client creation/update
- access key placeholder creation
- access key revoke/rotate placeholder
- scope assignment validation
- webhook subscription creation/update
- delivery attempt creation
- retry/dead-letter placeholder behavior
- inbound integration placeholder review
- organization boundary enforcement
- permission denied behavior
- audit record creation

## Final Rule

Sprint 46 is complete when API clients, scoped access key placeholders, webhook subscriptions, event catalogs, delivery attempts, retry/dead-letter placeholders, inbound integration placeholders, rate-limit metadata, and integration audit history are available through organization-scoped, permission-protected, secret-safe, and auditable foundations.
