# Multi-Channel Communication Wallet API Contract

## Purpose

This document defines the MVP API direction for paid SMS, Email, and WhatsApp communication channels.

Organizations should be able to buy channel packages, hold channel balances, send messages, manage templates, and review delivery/usage history through a shared Communication Engine.

## Owner

```text
Multi-Channel Communication Engine
Notification Engine
SMS Engine
Provider Connection Foundation
Payment Engine
Audit Engine
Permission Engine
Privacy/Consent Foundation
```

## Base Path

```text
/api/v1/organizations/{organizationId}/communication
```

## Dashboard

```text
GET /api/v1/organizations/{organizationId}/communication/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "smsCredits": 1200,
    "emailCredits": 5000,
    "whatsappCredits": 300,
    "messagesSentThisMonth": 2400,
    "failedMessagesThisMonth": 35
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Channel Wallets

```text
GET /api/v1/organizations/{organizationId}/communication/wallets
GET /api/v1/organizations/{organizationId}/communication/wallets/{channel}
GET /api/v1/organizations/{organizationId}/communication/wallets/{channel}/ledger
```

Supported `{channel}` values:

```text
sms
email
whatsapp
multi_channel_placeholder
```

## Channel Packages

```text
GET /api/v1/organizations/{organizationId}/communication/packages
POST /api/v1/organizations/{organizationId}/communication/packages/{packageId}/purchase-placeholder
GET /api/v1/organizations/{organizationId}/communication/package-purchases
GET /api/v1/organizations/{organizationId}/communication/package-purchases/{purchaseId}
```

Package types:

```text
sms_package
email_package
whatsapp_package
multi_channel_bundle_placeholder
```

## Messages

```text
GET /api/v1/organizations/{organizationId}/communication/messages
POST /api/v1/organizations/{organizationId}/communication/messages
GET /api/v1/organizations/{organizationId}/communication/messages/{messageId}
POST /api/v1/organizations/{organizationId}/communication/messages/{messageId}/send-placeholder
POST /api/v1/organizations/{organizationId}/communication/messages/{messageId}/cancel-placeholder
```

### Message Request Direction

```json
{
  "channel": "whatsapp",
  "fallbackChannels": ["sms", "email"],
  "audienceType": "group",
  "audienceId": "...",
  "templateId": "...",
  "subject": "Optional for email",
  "body": "Message body placeholder",
  "scheduledAt": null
}
```

## Templates

```text
GET /api/v1/organizations/{organizationId}/communication/templates
POST /api/v1/organizations/{organizationId}/communication/templates
GET /api/v1/organizations/{organizationId}/communication/templates/{templateId}
PATCH /api/v1/organizations/{organizationId}/communication/templates/{templateId}
POST /api/v1/organizations/{organizationId}/communication/templates/{templateId}/submit-for-approval-placeholder
POST /api/v1/organizations/{organizationId}/communication/templates/{templateId}/mark-approved-placeholder
```

Template channels:

```text
sms
email
whatsapp
in_app
push_placeholder
```

## Delivery Attempts

```text
GET /api/v1/organizations/{organizationId}/communication/delivery-attempts
GET /api/v1/organizations/{organizationId}/communication/messages/{messageId}/delivery-attempts
POST /api/v1/organizations/{organizationId}/communication/delivery-attempts/{attemptId}/retry-placeholder
```

## Sender Identities

```text
GET /api/v1/organizations/{organizationId}/communication/sender-identities
POST /api/v1/organizations/{organizationId}/communication/sender-identities
PATCH /api/v1/organizations/{organizationId}/communication/sender-identities/{senderIdentityId}
POST /api/v1/organizations/{organizationId}/communication/sender-identities/{senderIdentityId}/verify-placeholder
```

Sender identity examples:

```text
sms_sender_name
email_from_address
whatsapp_business_profile_placeholder
```

## Communication Preferences

```text
GET /api/v1/organizations/{organizationId}/communication/preferences
PATCH /api/v1/organizations/{organizationId}/communication/preferences
GET /api/v1/organizations/{organizationId}/communication/recipient-preferences
PATCH /api/v1/organizations/{organizationId}/communication/recipient-preferences/{recipientId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- communication feature availability
- channel package availability
- channel balance before send
- recipient consent/preference where applicable
- audience permission
- template approval where required
- sender identity status where required
- provider connection availability
- audit logging for purchases, sends, templates, and sender identity changes

## Status Direction

Suggested message statuses:

```text
draft
scheduled
queued
sent
partially_sent
failed
cancelled
manual_review
```

Suggested delivery statuses:

```text
pending
sent
delivered
failed
reversed_placeholder
manual_review
```

Suggested template statuses:

```text
draft
pending_approval
approved_placeholder
rejected_placeholder
archived
manual_review
```

## Error Direction

Expected error areas:

```text
insufficient credits
channel unavailable
template not approved
sender identity not verified
recipient preference blocked
permission denied
provider connection unavailable
audience not found
message not found
```

## Final Rule

SMS, Email, and WhatsApp routes must use the shared Communication Engine and must not expose provider-specific behavior to domain modules.
