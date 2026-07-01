# SMS Provider Adapter Contract

## Purpose

This document defines the contract for SMS provider adapters.

Provider adapters allow SMS Engine to use different providers without exposing provider-specific behavior to domains or Flutter.

## Principle

SMS Engine owns the platform contract.

Providers only implement adapters.

## Adapter Responsibilities

Each provider adapter should support where possible:

- send SMS
- normalize send result
- normalize provider errors
- estimate or report message units
- provider health check
- delivery status lookup
- callback signature verification where supported

## Adapter Must Not Own

Provider adapters must not own:

- organization business workflows
- wallet business rules
- Communication Domain logic
- Flutter UI behavior
- subscription or feature checks

## Send Request Shape

SMS Engine should call adapters with a normalized request similar to:

```text
provider_account_context
message_id
organization_id
sender_id optional
recipients
message_body
metadata
```

Provider adapters may transform this into provider-specific payload internally.

## Send Result Shape

Adapters should return normalized result information:

```text
provider_message_reference
accepted_count
rejected_count
estimated_units
provider_status
normalized_status
provider_response_summary
```

## Normalized Statuses

Use platform statuses:

```text
queued
submitted
sent
delivered
failed
expired
unknown
```

Provider-specific statuses must be mapped into these values.

## Error Normalization

Provider errors should be mapped to safe categories:

```text
provider_unavailable
invalid_recipient
insufficient_provider_credit
rate_limited
rejected_by_provider
unknown_provider_error
```

Do not expose provider secrets or raw credentials in errors.

## Health Check Direction

Provider health checks should return:

```text
healthy
degraded
unavailable
unknown
```

## Callback Direction

If a provider supports callbacks, the adapter should normalize callback data into:

```text
provider_message_reference
normalized_status
provider_status
status_time
raw_reference optional
```

## Security Direction

Provider credentials must be stored and accessed through infrastructure configuration or secure secrets flow.

Do not commit live provider credentials.

## Testing Direction

Each adapter should include tests for:

- successful send normalization
- provider error normalization
- delivery status mapping
- health check mapping
- invalid recipient behavior where applicable

## Final Rule

The rest of the platform should not care which SMS provider is used. That is the adapter's job.
