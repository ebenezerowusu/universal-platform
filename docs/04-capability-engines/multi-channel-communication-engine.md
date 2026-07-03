# Multi-Channel Communication Engine

## Purpose

This document defines the Multi-Channel Communication Engine for Universal Platform.

Organizations should be able to send messages to members, customers, staff, visitors, partners, and other permitted audiences through multiple paid and configurable channels.

The first paid communication channel was SMS. This engine expands the same business model to Email and WhatsApp.

## Supported Channels

Initial channels:

```text
SMS
Email
WhatsApp
In-app notification
Push notification placeholder
```

Revenue-generating channels:

```text
SMS
Email
WhatsApp
```

In-app notifications may be included in platform plans and do not require external provider cost by default.

## Core Rule

Domains must not call communication providers directly.

Correct flow:

```text
Domain Module
  -> Multi-Channel Communication Engine
    -> Channel Resolver
      -> Provider Adapter
```

Examples:

```text
Religious Domain
  -> Communication Engine
    -> SMS Adapter / Email Adapter / WhatsApp Adapter

Commerce Domain
  -> Communication Engine
    -> Email Adapter / WhatsApp Adapter

HR Domain
  -> Communication Engine
    -> Email Adapter / SMS Adapter
```

## Business Model

The platform may sell communication credits or packages per channel.

Examples:

```text
SMS package
Email package
WhatsApp package
Multi-channel bundle
```

Each organization may have separate balances or usage ledgers per channel.

Suggested balance types:

```text
sms_credits
email_credits
whatsapp_credits
multi_channel_credits_placeholder
```

## Organization-Facing Features

Organizations should be able to:

- buy SMS credits/packages
- buy Email credits/packages
- buy WhatsApp credits/packages
- send messages to permitted audiences
- create templates
- schedule messages
- view delivery status
- view failed messages
- view credit usage
- view message history
- configure sender identity where supported
- manage opt-out/communication preferences

## Target Audiences

Possible audiences:

- entire organization
- branch
- hierarchy subtree
- congregation
- group
- service attendees
- visitors
- members
- customers
- staff
- families/households
- leaders
- custom filters

Every audience query must respect tenant boundaries and permissions.

## Channel Provider Direction

SMS providers may include country-specific adapters such as Arkessel, Hubtel, Africa's Talking, Termii, or others.

Email providers may include adapters such as SMTP, SendGrid, Amazon SES, Mailgun, Brevo, or others.

WhatsApp providers may include adapters such as Meta WhatsApp Cloud API, Twilio WhatsApp, Africa's Talking WhatsApp where available, or other approved providers.

Provider names must not be hardcoded into domain modules.

## Channel Selection

A message request may specify:

```text
preferred_channel
fallback_channels
recipient_type
message_template
language
country
priority
```

Example fallback:

```text
WhatsApp -> SMS -> Email
```

Fallback rules must be configurable by organization and country.

## Template Direction

Templates may be required for some channels.

WhatsApp often requires approved templates for business-initiated messages. The platform should support template approval metadata without hardcoding provider-specific rules into domains.

Template fields:

```text
template_key
channel
template_type
language
status
provider_template_reference_placeholder
approval_status_placeholder
created_by
updated_by
```

## Wallet And Ledger Direction

Each paid channel should have a usage ledger.

Ledger examples:

```text
communication_wallet
communication_credit_package
communication_credit_ledger
communication_message
communication_delivery_attempt
```

Ledger actions:

```text
package_purchased
credits_added
message_reserved
message_sent
message_failed
credits_reversed_placeholder
manual_adjustment_placeholder
```

## Permissions

Suggested permissions:

```text
communication.wallet.view
communication.wallet.manage
communication.packages.view
communication.packages.manage
communication.messages.view
communication.messages.send
communication.templates.view
communication.templates.manage
communication.delivery.view
communication.usage.view
communication.preferences.manage
```

Channel-specific permissions may be added where needed:

```text
communication.sms.send
communication.email.send
communication.whatsapp.send
```

## Events Published

```text
communication.package.purchased
communication.wallet.credited
communication.message.queued
communication.message.sent
communication.message.failed
communication.message.delivered
communication.template.created
communication.template.approved_placeholder
communication.credits.adjusted_placeholder
```

## Audit Events

```text
audit.communication.package_purchased
audit.communication.wallet_adjusted
audit.communication.message_sent
audit.communication.template_changed
audit.communication.sender_identity_changed
audit.communication.preference_changed
```

## Privacy And Consent

Communication must respect:

- recipient preferences
- opt-out rules
- consent rules where applicable
- quiet hours where configured
- restricted audience permissions
- privacy-safe message content rules

Sensitive messages should require stricter permission and audit review.

## Reporting

Reports may include:

- channel usage by organization
- channel usage by branch/group/audience
- package purchases
- credit balance
- sent messages
- failed messages
- delivery rates
- cost/revenue summary placeholder

## Out Of Scope For First Implementation

Do not implement immediately:

- provider-specific WhatsApp onboarding automation
- email reputation management
- automated marketing campaign suite
- advanced segmentation engine
- cross-tenant recipient lists
- direct provider calls from domain modules
- hidden message sending without audit

## Final Rule

SMS, Email, and WhatsApp should be treated as configurable paid communication channels powered by a shared Communication Engine, not separate hardcoded features inside domains.
