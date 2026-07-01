# Webhook Standards

## Purpose

This document defines webhook standards for Universal Platform.

Webhooks allow external services such as payment or communication providers to notify the platform about completed events.

## Webhook Principle

Webhook handlers should be secure, idempotent, observable, and isolated from domain business rules.

## Route Pattern

Recommended pattern:

```text
/api/v1/webhooks/{engine}/{provider}
```

Examples:

```text
/api/v1/webhooks/payments/hubtel
/api/v1/webhooks/sms/arkessel
```

Provider names are examples. Provider behavior must still be handled through adapters.

## Handler Responsibilities

Webhook handlers should:

- receive provider callback
- verify provider authenticity where supported
- normalize provider payload
- pass to the relevant engine
- return provider-compatible response

Webhook handlers must not:

- update domain records directly without engine/service coordination
- trust unverified payment status
- expose raw provider errors to users
- duplicate provider-specific logic across domains

## Idempotency

Webhook processing must handle repeated delivery.

Use provider reference, transaction reference, event id, or another stable key where available.

Duplicate callbacks should not create duplicate critical effects.

## Logging

Webhook logs should include:

- provider
- event type
- reference
- trace id
- processing result

Do not log provider secrets.

## Events

After verification and processing, engines may publish internal events such as:

- PaymentCompleted
- PaymentFailed
- SmsDelivered
- SmsFailed

Domains should react to internal events instead of raw provider callbacks.

## Testing Requirements

Tests should cover:

- valid callback
- invalid callback
- duplicate callback
- provider error mapping
- internal event publication

## Final Rule

Webhooks enter through engines, not through domain shortcuts.
