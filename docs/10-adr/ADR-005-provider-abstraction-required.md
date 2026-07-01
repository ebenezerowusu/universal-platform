# ADR-005: Provider Abstraction Is Required

## Status

Accepted

## Context

Universal Platform will integrate with external providers for SMS, payments, storage, email, maps, search, and other infrastructure services.

Different countries and organizations may require different providers.

Examples:

- Ghana SMS may use Arkessel.
- Ghana payments may use Hubtel or Paystack.
- Kenya payments may use M-Pesa.
- Storage may begin locally and later move to S3 or Cloudflare R2.

If domains call providers directly, the platform will become difficult to maintain and expand.

## Decision

All external providers must be accessed through capability engines and provider adapters.

Provider-specific logic must not appear in domain modules.

Correct pattern:

```text
Domain
  -> Capability Engine
    -> Provider Interface
      -> Provider Adapter
        -> External Provider
```

Incorrect pattern:

```text
Domain
  -> External Provider API
```

## Examples

Religious birthday SMS:

```text
Religious Domain
  -> SMS Engine
    -> SMS Provider Resolver
      -> Arkessel Adapter
```

Marketplace checkout:

```text
Commerce Domain
  -> Payment Engine
    -> Payment Provider Resolver
      -> Hubtel Adapter
```

File upload:

```text
Any Domain
  -> Storage Engine
    -> Local Storage Adapter or S3/R2 Adapter
```

## Consequences

### Positive

- New providers can be added without changing domain logic.
- Countries can use different providers.
- Provider fallback becomes possible.
- Provider credentials remain hidden from domains and organizations.
- Testing becomes easier with mock adapters.
- The platform remains future-proof.

### Negative / Cost

- More upfront interface design.
- Simple integrations take slightly longer.
- Developers must resist direct SDK/API calls.

## Final Rule

No provider integration is complete unless it is behind an engine and adapter.
