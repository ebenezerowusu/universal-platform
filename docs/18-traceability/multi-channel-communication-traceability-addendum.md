# Multi-Channel Communication Traceability Addendum

## Purpose

This document adds explicit traceability for Email and WhatsApp as paid organization communication channels alongside SMS.

SMS was already documented as a paid wallet/package feature. This addendum extends that model to Email and WhatsApp.

## Feature To Engine Traceability

| Feature | Primary Owner | Required Engines | Supporting Foundations |
|---|---|---|---|
| SMS messaging | Multi-Channel Communication Engine | SMS Engine, Provider Connections, Payment, Audit | Notification, Privacy/Consent |
| Email messaging | Multi-Channel Communication Engine | Email Adapter, Provider Connections, Payment, Audit | Notification, Privacy/Consent |
| WhatsApp messaging | Multi-Channel Communication Engine | WhatsApp Adapter, Provider Connections, Payment, Audit | Notification, Privacy/Consent |
| Communication packages | Multi-Channel Communication Engine | Payment, Subscription/Billing, Audit | Revenue Operations |
| Communication wallets | Multi-Channel Communication Engine | Payment, Audit, Reporting | Organization/Tenant |
| Message templates | Multi-Channel Communication Engine | Permission, Audit | Localization, Provider Connections |
| Delivery tracking | Multi-Channel Communication Engine | Jobs/Scheduler, Audit | Provider Connections |
| Recipient preferences | Multi-Channel Communication Engine | Privacy/Consent, Permission | Identity, Organization |

## Permissions

```text
communication.dashboard.view
communication.wallet.view
communication.wallet.manage
communication.packages.view
communication.packages.manage
communication.messages.view
communication.messages.send
communication.sms.send
communication.email.send
communication.whatsapp.send
communication.templates.view
communication.templates.manage
communication.delivery.view
communication.usage.view
communication.sender_identities.view
communication.sender_identities.manage
communication.preferences.view
communication.preferences.manage
```

## API Route Groups

```text
GET /api/v1/organizations/{organizationId}/communication/dashboard-summary
GET /api/v1/organizations/{organizationId}/communication/wallets
GET /api/v1/organizations/{organizationId}/communication/wallets/{channel}
GET /api/v1/organizations/{organizationId}/communication/wallets/{channel}/ledger
GET /api/v1/organizations/{organizationId}/communication/packages
POST /api/v1/organizations/{organizationId}/communication/packages/{packageId}/purchase-placeholder
GET /api/v1/organizations/{organizationId}/communication/package-purchases
GET /api/v1/organizations/{organizationId}/communication/messages
POST /api/v1/organizations/{organizationId}/communication/messages
GET /api/v1/organizations/{organizationId}/communication/messages/{messageId}
POST /api/v1/organizations/{organizationId}/communication/messages/{messageId}/send-placeholder
POST /api/v1/organizations/{organizationId}/communication/messages/{messageId}/cancel-placeholder
GET /api/v1/organizations/{organizationId}/communication/templates
POST /api/v1/organizations/{organizationId}/communication/templates
GET /api/v1/organizations/{organizationId}/communication/templates/{templateId}
PATCH /api/v1/organizations/{organizationId}/communication/templates/{templateId}
GET /api/v1/organizations/{organizationId}/communication/delivery-attempts
GET /api/v1/organizations/{organizationId}/communication/sender-identities
POST /api/v1/organizations/{organizationId}/communication/sender-identities
GET /api/v1/organizations/{organizationId}/communication/preferences
PATCH /api/v1/organizations/{organizationId}/communication/preferences
```

## Entities

```text
CommunicationWallet
CommunicationCreditPackage
CommunicationPackagePurchase
CommunicationCreditLedger
CommunicationMessage
CommunicationRecipient
CommunicationDeliveryAttempt
CommunicationTemplate
CommunicationSenderIdentity
CommunicationRecipientPreference
CommunicationChannelProviderConfigPlaceholder
```

## Channels

```text
sms
email
whatsapp
in_app
push_placeholder
multi_channel_placeholder
```

## Package Types

```text
sms_package
email_package
whatsapp_package
multi_channel_bundle_placeholder
```

## Events

```text
communication.package.purchased
communication.wallet.credited
communication.message.created
communication.message.queued
communication.message.sent
communication.message.delivered
communication.message.failed
communication.delivery.retry_requested
communication.template.created
communication.template.updated
communication.template.approved_placeholder
communication.sender_identity.created
communication.sender_identity.verified_placeholder
communication.preference.changed
```

## Audit Events

```text
audit.communication.package_purchased
audit.communication.wallet_adjusted
audit.communication.message_created
audit.communication.message_sent
audit.communication.delivery_retry_requested
audit.communication.template_changed
audit.communication.sender_identity_changed
audit.communication.preference_changed
```

## Revenue Direction

The platform may sell:

```text
SMS credits/packages
Email credits/packages
WhatsApp credits/packages
Multi-channel communication bundles
Premium sender identity setup placeholder
Premium template setup/review placeholder
```

## Domain Usage

Religious Domain examples:

- send service announcements by SMS, Email, or WhatsApp
- send Bible Study reminders by WhatsApp/SMS
- send giving receipts by Email/SMS
- send birthday or anniversary messages by SMS/WhatsApp
- send visitor follow-up messages by WhatsApp/Email/SMS

Commerce examples:

- order confirmation by Email/WhatsApp/SMS
- delivery updates by WhatsApp/SMS
- customer campaign placeholders by Email/WhatsApp

HR examples:

- staff announcements by Email
- attendance or schedule reminders by WhatsApp/SMS

## Rules

- Domains must not call Email, WhatsApp, or SMS providers directly.
- Sending must check channel balance or package entitlement.
- Sending must check permission and recipient preference where applicable.
- WhatsApp template approval metadata should be supported where required.
- Provider-specific behavior must stay behind adapters.
- Paid communication usage must be ledgered and auditable.

## Final Rule

Email and WhatsApp are first-class paid communication channels alongside SMS, implemented through the shared Multi-Channel Communication Engine.
