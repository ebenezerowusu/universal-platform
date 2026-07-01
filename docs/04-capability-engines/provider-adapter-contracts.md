# Provider Adapter Contracts

## Purpose

This document defines common rules for provider adapters in Universal Platform.

Provider adapters allow the platform to connect to payment, SMS, storage, email, and other external services without leaking provider-specific logic into domains.

## Adapter Principle

Domains must not call external providers directly.

Correct flow:

```text
Domain
  -> Capability Engine
    -> Provider Interface
      -> Provider Adapter
```

## Adapter Responsibilities

A provider adapter is responsible for:

- Translating platform requests to provider requests
- Translating provider responses to platform responses
- Handling provider-specific errors
- Hiding provider credentials
- Supporting provider health checks where useful
- Logging safe operational metadata

## Adapter Non-Responsibilities

A provider adapter must not own domain business logic.

Examples:

- Payment adapter should not decide whether a tithe is valid.
- SMS adapter should not decide who receives birthday messages.
- Storage adapter should not decide whether a member document is visible.

Those decisions belong to domains and engines.

## Common Adapter Interface Ideas

Adapters should expose stable operations through engine-specific interfaces.

Payment adapter examples:

- initialize_payment
- verify_payment
- handle_callback
- refund_payment later

SMS adapter examples:

- send_message
- check_delivery_status
- validate_sender

Storage adapter examples:

- store_file
- retrieve_file
- generate_temporary_access
- delete_file_reference

## Provider Resolution

Provider choice should be resolved by the relevant engine.

Inputs may include:

- organization configuration
- regional settings
- currency
- message destination
- provider status
- feature availability

## Error Mapping

Provider-specific errors should be mapped to platform error types.

Examples:

- PROVIDER_ERROR
- PROVIDER_UNAVAILABLE
- VALIDATION_ERROR
- PAYMENT_VERIFICATION_FAILED where applicable
- STORAGE_ERROR

Do not expose raw provider secrets, signatures, or sensitive payloads to clients.

## Idempotency

Provider adapters should support idempotency where relevant.

Important areas:

- payment initialization
- payment callbacks
- SMS sending
- file operations

## Testing Requirements

Adapters must be testable with mocks or sandbox implementations.

Tests should cover:

- successful provider call
- provider failure
- response mapping
- error mapping
- retry-safe behavior where relevant
- no domain logic inside adapter

## Final Rule

Provider adapters translate between the platform and providers. They do not own product rules.
