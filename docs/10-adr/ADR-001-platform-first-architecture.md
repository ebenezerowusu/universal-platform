# ADR-001: Platform-First Architecture

## Status

Accepted

## Context

The project started from the idea of building a religious/church management system, but the long-term vision expanded into a modular platform that can support multiple organization types and business domains.

Building only for one domain would create technical debt and make future expansion difficult.

The platform must support domains such as Religious Organisation, Commerce, POS, Finance, HR, Logistics, and future domains without rewriting the Platform Core.

## Decision

Universal Platform will be built as a multi-tenant, modular, extensible platform.

The Platform Core must remain domain-agnostic.

Business logic must live inside domains and modules.

Reusable capabilities must be implemented as capability engines.

External providers must be accessed through adapters.

## Consequences

### Positive

- The platform can expand into new industries.
- Core services can be reused across domains.
- New modules can be added without rewriting the core.
- Provider and country-specific behavior can be configured.
- The architecture supports long-term growth.

### Negative / Cost

- Initial planning takes longer.
- Developers must follow stricter architecture rules.
- AI coding agents need clear documentation before implementation.
- Some simple features may require more disciplined design upfront.

## Final Principle

Build the platform once. Expand it forever.
