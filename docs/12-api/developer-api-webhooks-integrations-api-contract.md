# Developer API Webhooks Integrations API Contract

## Purpose

This document defines the MVP API contract for Developer API Access, Webhooks, and Integration Operations foundation.

The API should support developer dashboard summaries, API client metadata, scoped access key placeholders, API scope registry placeholders, webhook event catalogs, webhook subscriptions, delivery attempts, inbound integration placeholders, rate limit metadata, and integration audit history while preserving organization boundaries.

## Owner

```text
Developer Integration Foundation
Permission Engine
Audit Engine
Notification/Event Engine where outbound events are emitted
Security Session Foundation where token/access key lifecycle aligns
```

## Base Path

```text
/api/v1/organizations/{organizationId}/developer
```

## Developer Dashboard

```text
GET /api/v1/organizations/{organizationId}/developer/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "apiClients": 2,
    "activeAccessKeys": 3,
    "webhookSubscriptions": 4,
    "failedDeliveries": 1,
    "rateLimitStatus": "normal_placeholder"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## API Clients

```text
GET /api/v1/organizations/{organizationId}/developer/api-clients
POST /api/v1/organizations/{organizationId}/developer/api-clients
GET /api/v1/organizations/{organizationId}/developer/api-clients/{clientId}
PATCH /api/v1/organizations/{organizationId}/developer/api-clients/{clientId}
POST /api/v1/organizations/{organizationId}/developer/api-clients/{clientId}/disable-placeholder
```

### Create API Client Request

```json
{
  "name": "Reporting integration",
  "clientTypePlaceholder": "server_to_server",
  "ownerUserId": "optional",
  "notes": "Optional note"
}
```

## Access Key Placeholders

```text
GET /api/v1/organizations/{organizationId}/developer/api-clients/{clientId}/access-keys
POST /api/v1/organizations/{organizationId}/developer/api-clients/{clientId}/access-keys
POST /api/v1/organizations/{organizationId}/developer/access-keys/{keyId}/rotate-placeholder
POST /api/v1/organizations/{organizationId}/developer/access-keys/{keyId}/revoke-placeholder
```

Raw secrets should only be returned once during creation placeholder and never displayed again.

## API Scope Registry Placeholder

```text
GET /api/v1/organizations/{organizationId}/developer/scopes
POST /api/v1/organizations/{organizationId}/developer/scopes
PATCH /api/v1/organizations/{organizationId}/developer/scopes/{scopeKey}
```

## Webhook Event Catalog

```text
GET /api/v1/organizations/{organizationId}/developer/webhook-events
POST /api/v1/organizations/{organizationId}/developer/webhook-events
PATCH /api/v1/organizations/{organizationId}/developer/webhook-events/{eventKey}
```

## Webhook Subscriptions

```text
GET /api/v1/organizations/{organizationId}/developer/webhooks
POST /api/v1/organizations/{organizationId}/developer/webhooks
GET /api/v1/organizations/{organizationId}/developer/webhooks/{subscriptionId}
PATCH /api/v1/organizations/{organizationId}/developer/webhooks/{subscriptionId}
POST /api/v1/organizations/{organizationId}/developer/webhooks/{subscriptionId}/disable-placeholder
```

### Create Webhook Subscription Request

```json
{
  "name": "Order status webhook",
  "endpointUrlPlaceholder": "https://example.com/webhooks/platform",
  "eventKeys": ["order.completed_placeholder"],
  "signingSecretReferencePlaceholder": "generated_placeholder"
}
```

## Delivery Attempts

```text
GET /api/v1/organizations/{organizationId}/developer/webhook-deliveries
GET /api/v1/organizations/{organizationId}/developer/webhook-deliveries/{deliveryId}
POST /api/v1/organizations/{organizationId}/developer/webhook-deliveries/{deliveryId}/retry-placeholder
POST /api/v1/organizations/{organizationId}/developer/webhook-deliveries/{deliveryId}/mark-dead-letter-placeholder
```

## Inbound Integration Placeholder

```text
GET /api/v1/organizations/{organizationId}/developer/inbound-integrations
POST /api/v1/organizations/{organizationId}/developer/inbound-integrations
GET /api/v1/organizations/{organizationId}/developer/inbound-integrations/{requestId}
POST /api/v1/organizations/{organizationId}/developer/inbound-integrations/{requestId}/review-placeholder
```

## Rate Limit and Quota Metadata Placeholder

```text
GET /api/v1/organizations/{organizationId}/developer/rate-limits
PATCH /api/v1/organizations/{organizationId}/developer/api-clients/{clientId}/rate-limit-placeholder
```

## Integration Audit History

```text
GET /api/v1/organizations/{organizationId}/developer/audit-history
GET /api/v1/organizations/{organizationId}/developer/api-clients/{clientId}/audit-history
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- developer integration feature availability
- required permission
- API client belongs to organization
- webhook subscription belongs to organization
- scope assignment is allowed for actor
- raw secrets are not returned after creation placeholder

Developer records must not cross organization boundaries.

## Status Direction

Suggested API client statuses:

```text
active
inactive
suspended_placeholder
revoked_placeholder
manual_review
```

Suggested access key statuses:

```text
active
rotated_placeholder
revoked
expired_placeholder
manual_review
```

Suggested webhook delivery statuses:

```text
queued_placeholder
sent_placeholder
failed
retrying_placeholder
dead_letter_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- API client created or updated
- access key created placeholder
- access key rotated placeholder
- access key revoked placeholder
- webhook subscription created or updated
- webhook disabled
- delivery retried placeholder
- inbound integration reviewed placeholder
- scope assignment changed placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- API client not found
- access key not found
- webhook subscription not found
- webhook delivery not found
- scope not found
- invalid scope assignment
- raw secret unavailable after creation
- developer permission denied
- organization not found

## Final Rule

Developer integration APIs must be organization-scoped, permission-protected, secret-safe, auditable, and centralized through shared integration foundations.
