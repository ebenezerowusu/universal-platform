# SMS Engine Specification

## Purpose

The SMS Engine provides one platform-wide way to sell, send, track, audit, and report SMS messages across all domains.

SMS is both a communication capability and a revenue stream for the platform.

Domains must not integrate directly with SMS providers.

## Engine Position

```text
Domain Module
  -> SMS Engine
    -> SMS Provider Resolver
      -> SMS Provider Adapter
        -> External SMS Provider
```

## Responsibilities

The SMS Engine is responsible for:

- SMS wallet management
- SMS package purchases
- SMS balance tracking
- SMS sending
- SMS campaign logging
- Delivery status tracking
- Provider abstraction
- Country/provider resolution
- Sender ID management
- SMS templates
- Low balance alerts
- Usage reports
- Provider fallback
- SMS audit logs

## Non-Responsibilities

The SMS Engine does not decide religious birthday logic, visitor follow-up logic, marketplace order logic, or HR reminder logic.

Domains decide why a message should be sent.

The SMS Engine decides how the message is sent and recorded.

## Domains That Use It

Examples:

- Religious Domain: birthdays, announcements, event reminders, visitor follow-ups
- Commerce Domain: order updates, delivery notifications
- HR Domain: staff reminders
- Subscription Module: billing reminders
- Platform Admin: system notices

## Provider Adapters

Potential providers:

- Arkessel
- Hubtel
- Africa's Talking
- Twilio
- Termii
- Future providers

Arkessel may be used for Ghana, but this must be configured at the provider level, not hardcoded into domains.

## Provider Resolution

Provider selection should be configuration-driven.

Inputs may include:

- Organization country
- Message destination country
- Platform provider configuration
- Sender ID availability
- Provider health
- Fallback rules
- Cost rules

Example:

```text
Organization Country: Ghana
Destination: Ghana number
Resolved Provider: Arkessel
Fallback Provider: Hubtel
```

## Organization Experience

Organizations should not manage low-level provider credentials.

They should see:

- SMS balance
- Buy SMS package
- Send SMS
- Campaign history
- Delivery status
- Low balance alerts

They should not see:

- Provider API keys
- Provider raw errors
- Provider technical payloads

## SMS Wallet

Each organization should have an SMS wallet.

Wallet data may include:

- organization_id
- balance
- total_purchased
- total_used
- reserved_balance
- low_balance_threshold
- status

SMS sending should check balance before sending unless the platform allows postpaid billing for selected organizations.

## SMS Package Sales

The system admin can define SMS packages.

Package fields may include:

- name
- country
- currency
- sms_units
- price
- validity_days nullable
- active

Buying SMS credits should use the Payment Engine.

Flow:

```text
Organization buys SMS package
  -> Payment Engine processes payment
  -> PaymentCompleted event
  -> SMS Engine credits wallet
  -> SmsWalletCredited event
```

## SMS Lifecycle

Possible statuses:

- queued
- sending
- sent
- delivered
- failed
- rejected
- expired
- unknown

## Sending Flow

```text
Domain requests SMS
  -> SMS Engine validates organization and balance
  -> SMS Engine resolves provider
  -> SMS Engine creates message log
  -> Provider adapter sends message
  -> SMS Engine updates status
  -> SMS Engine deducts/reserves credits
  -> SMS Engine publishes SmsSent or SmsFailed event
  -> Delivery callback updates final delivery status
```

## Key Entities

Candidate entities:

- sms_wallets
- sms_packages
- sms_wallet_transactions
- sms_messages
- sms_campaigns
- sms_provider_configs
- sms_provider_events
- sms_sender_ids
- sms_templates

## Message Types

Examples:

- transactional
- announcement
- reminder
- campaign
- verification
- system

## Templates

Templates should support localization and variables.

Example:

```text
Hello {{name}}, this is a reminder for {{event_name}} on {{date}}.
```

Templates must be approved where required by policy or provider rules.

## Events Published

- SmsSendRequested
- SmsQueued
- SmsSent
- SmsDelivered
- SmsFailed
- SmsWalletCredited
- SmsWalletDebited
- SmsLowBalance
- SmsProviderFailed

## Events Consumed

- PaymentCompleted
- PaymentFailed
- OrganizationCreated
- ProviderConfigurationChanged

## Security Requirements

- Provider API keys must be stored securely.
- Organizations must not access provider credentials.
- SMS sending must enforce permissions.
- Sensitive message content should be protected in logs.
- Rate limits should prevent abuse.
- Audit logs should track bulk sends and wallet changes.

## Tenant Isolation

Every SMS wallet, campaign, message, and template must belong to an organization unless it is platform-level.

Organization users must not access another organization's SMS records.

## Consent and Preferences

The platform should support communication preferences.

Members/customers may opt into or out of certain message types where appropriate.

Important communication may have different rules than marketing communication.

## Testing Requirements

Tests must cover:

- Wallet crediting
- Balance deduction
- Insufficient balance
- Provider resolution
- Provider fallback
- Successful send
- Failed send
- Delivery callback
- Tenant isolation
- Permission checks
- Bulk send limits

## Final Rule

Domains decide the message purpose. The SMS Engine owns sending, billing, provider selection, and tracking.
