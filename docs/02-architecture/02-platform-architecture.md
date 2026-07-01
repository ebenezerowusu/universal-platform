# Platform Architecture Overview

## Purpose

This document defines the high-level architecture of Universal Platform.

The platform must be multi-tenant, modular, provider-agnostic, country-aware, infrastructure-independent, and extensible across business domains.

## Architecture Summary

```text
Client Applications
  -> Platform API Gateway
    -> Platform Core
      -> Capability Engines
        -> Domains and Modules
          -> Infrastructure Adapters
```

## Main Layers

### 1. Client Applications

The first clients are:

- Flutter Mobile App
- Flutter Desktop App

Future clients may include:

- Web Portal
- Public API consumers
- Partner integrations
- Third-party module UIs

Clients must not contain business logic. They should call backend APIs and render UI based on permissions, installed modules, and configuration.

### 2. Platform API Layer

The API layer exposes backend capabilities to clients.

Responsibilities:

- Request routing
- Authentication enforcement
- Authorization enforcement
- Validation
- Error formatting
- API versioning
- Rate limiting
- Request tracing

The API layer must not contain domain business logic. It should delegate to application services.

### 3. Platform Core

The Platform Core provides shared foundation capabilities.

It includes:

- Identity
- Organizations / Tenants
- Roles and Permissions
- Module Registry
- Subscriptions and Plans
- Configuration
- Events
- Audit Logs
- Localization
- Provider Registry

The core must remain domain-agnostic.

### 4. Capability Engines

Capability Engines are reusable services used by domains.

Examples:

- Authentication Engine
- Authorization Engine
- Payment Engine
- SMS Engine
- Notification Engine
- Workflow Engine
- Storage Engine
- Reporting Engine
- Search Engine
- Audit Engine
- Localization Engine
- Timeline Engine
- Import/Export Engine

Engines expose stable interfaces. Domains use the interfaces, not the underlying providers.

### 5. Domains and Modules

Domains contain business-specific logic.

Initial domains:

- Religious Domain
- Commerce Domain
- Finance Domain
- HR Domain

Future domains can be added later without modifying the Platform Core.

Each domain may contain modules.

Example:

```text
Religious Domain
  - Member Management
  - Visitors
  - Attendance
  - Congregations
  - Groups
  - Bible Study
  - Giving
  - Prayer and Counseling
  - Welfare
```

### 6. Infrastructure Adapters

Infrastructure adapters implement technical integrations.

Examples:

- PostgreSQL repositories
- Redis cache and queue adapters
- Local storage adapter
- S3/R2 storage adapters
- Arkessel SMS adapter
- Hubtel payment adapter
- Paystack payment adapter

Infrastructure must depend on application contracts. Business logic must not depend directly on infrastructure implementations.

## Clean Architecture Direction

The platform should follow Clean Architecture principles:

```text
Domain Layer
  <- Application Layer
    <- Interface/API Layer
      <- Infrastructure Layer
```

Business rules live in the Domain Layer and Application Layer.

Infrastructure details live outside the business layer.

## Domain-Driven Design Direction

Each major business area should be treated as a bounded context.

Examples:

- Identity Context
- Organization Context
- Subscription Context
- Religious Context
- Commerce Context
- Finance Context
- HR Context

Bounded contexts communicate through explicit contracts and events.

## Event Bus

The platform should support event-driven communication.

Example flow:

```text
PaymentCompleted
  -> Notification Engine sends receipt
  -> Reporting Engine updates analytics
  -> Domain module updates business state
```

Events reduce direct coupling between modules.

## Module System

Every installable module should eventually define:

- Module ID
- Version
- Name
- Description
- Required permissions
- Navigation entries
- Settings schema
- Events published
- Events consumed
- Database migrations
- Background jobs
- API routes

This will later become the Module SDK.

## Multi-Tenancy

The platform must be multi-tenant from day one.

Every organization has its own configuration, modules, users, roles, subscriptions, and data boundaries.

Tenant isolation must be enforced at the repository/application-service level and verified by tests.

## Provider Abstraction

Provider integrations must be hidden behind engines.

Example:

```text
Domain Module
  -> Payment Engine
    -> Provider Adapter
      -> Hubtel / Paystack / Flutterwave / M-Pesa
```

Domains never directly call provider APIs.

## Deployment Direction

The MVP may run on one VPS:

```text
Nginx
Rust + Axum API
PostgreSQL
Redis
Workers
Local Storage
```

Later, infrastructure can be separated:

- Dedicated PostgreSQL server
- Dedicated Redis server
- Separate worker servers
- Object storage
- Multiple API nodes
- Load balancer

Application code should not change when this happens.

## Architecture Rule

If a feature cannot answer these questions clearly, it should not be implemented yet:

1. Is it Platform Core, Capability Engine, Domain, Module, or Infrastructure?
2. Is it reusable across domains?
3. Is it configurable?
4. Does it respect tenant isolation?
5. Does it call providers only through adapters?
6. Does it publish or consume events where appropriate?
7. Does it preserve the principle: Build the platform once. Expand it forever?
