# Capability Engine Overview

## Purpose

Capability Engines are reusable platform services used by multiple domains.

They prevent duplicated logic and keep domain modules from depending directly on infrastructure or third-party providers.

## Engine Rule

Domains must use engines for shared capabilities.

Domains must not bypass engines to call infrastructure or providers directly.

```text
Domain -> Capability Engine -> Interface -> Adapter -> Provider/Infrastructure
```

## Initial Engines

### Identity Engine

Handles authentication, sessions, tokens, and user identity.

### Organization Engine

Handles organizations, tenants, branches, organization settings, and tenant context.

### Permission Engine

Handles roles, permissions, scopes, and authorization checks.

### Module Registry Engine

Handles module registration, installation, activation, and feature availability.

### Subscription Engine

Handles plans, limits, feature access, renewals, and billing rules.

### Payment Engine

Handles payment initiation, verification, callbacks/webhooks, receipts, and provider abstraction.

Potential adapters:

- Hubtel
- Paystack
- Flutterwave
- M-Pesa
- Future providers

### SMS Engine

Handles SMS wallet, SMS sending, delivery reports, campaigns, templates, and provider abstraction.

Potential adapters:

- Arkessel
- Hubtel
- Africa's Talking
- Twilio
- Termii
- Future providers

### Notification Engine

Handles in-app, push, SMS-triggered, email-triggered, and event-based notifications.

### Workflow Engine

Handles approvals, follow-ups, task automation, reminders, and configurable workflows.

### Storage Engine

Handles file upload, download, metadata, provider switching, and storage security.

Potential adapters:

- Local storage
- S3-compatible storage
- Cloudflare R2
- Azure Blob Storage

### Audit Engine

Handles audit logging for sensitive and important actions.

### Reporting Engine

Handles dashboards, reports, aggregation, exports, and future analytics.

### Localization Engine

Handles languages, translations, date formats, country settings, currency, and terminology.

### Timeline Engine

Handles timeline/activity history for entities across domains.

Examples:

- Member timeline
- Order timeline
- Product timeline
- Payment timeline
- Staff timeline

### Import/Export Engine

Handles CSV/Excel imports, validation, duplicate detection, previews, exports, and report downloads.

## Engine Specification Template

Each engine document should define:

- Purpose
- Responsibilities
- Non-responsibilities
- Interfaces
- Key entities
- Events published
- Events consumed
- Configuration
- Security requirements
- Tenant isolation requirements
- Provider adapters if any
- Example domain usage
- Testing requirements

## Provider-Based Engine Pattern

Example SMS flow:

```text
Religious Domain birthday workflow
  -> SMS Engine
    -> Check SMS wallet
    -> Resolve provider by country/configuration
    -> Arkessel Adapter for Ghana
    -> Store message log
    -> Publish SmsSent/SmsFailed event
```

Example payment flow:

```text
Marketplace checkout
  -> Payment Engine
    -> Resolve provider by country/configuration
    -> Initialize payment
    -> Verify callback
    -> Publish PaymentCompleted event
```

## Engine Design Principle

Engines should be reusable, configurable, tenant-aware, secure, and provider-agnostic.

If a capability can serve more than one domain, it should be considered an engine before becoming domain-specific code.
