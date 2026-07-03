# Module Dependency Map

## Purpose

This document maps module and foundation dependencies across Universal Platform.

Use it to prevent circular dependencies, Core pollution, and incorrect domain coupling.

## Dependency Direction Rule

```text
Platform Core -> Capability Engines -> Domains -> UI/API Consumers
```

Domains may depend on Core and capability engines.

Core must never depend on domains.

## Core Layer

Core foundations:

```text
Identity
Organization/Tenant
Permission
Audit
Configuration
Module Registry
Subscription Entitlement
Localization
Notification
Workflow
Storage
Provider Connections
Jobs/Scheduler
```

Core may expose capability interfaces, but it must remain domain-agnostic.

## Capability Engine Dependencies

| Engine | Depends On | Used By |
|---|---|---|
| Permission | Identity, Organization | Every protected module |
| Audit | Identity, Organization | Every sensitive module |
| Configuration | Organization, Audit | Feature Flags, Providers, Domains |
| Feature Flags | Configuration, Module Registry | Product, Experimentation, Domains |
| Notification | Identity, Preferences, Jobs | Support, Workflow, Product Communication |
| SMS | Provider Connections, Payment, Audit | Religious, Support, Communication |
| Payment | Provider Connections, Audit, Country Ops | Finance, Commerce, Billing, SMS |
| Storage | Provider Connections, Permission | Documents, Media, Imports |
| Workflow | Permission, Audit, Notification | Governance, Finance, Support, Implementation |
| Reporting | Permission, Audit | Finance, Product Analytics, Religious, Commerce |
| Search | Permission, Privacy | Knowledge Base, Documents, Marketplace |
| Jobs/Scheduler | Config, Audit | Imports, Notifications, Reports, Webhooks |
| Provider Connections | Credential References, Country Ops | Payment, SMS, Storage, Integrations |

## Domain Dependency Map

| Domain/Foundation | Must Depend On | Must Not Depend On |
|---|---|---|
| Religious Domain | Identity, Organization, Permission, Audit, Attendance, Payment, SMS, Reporting | Commerce internals, Partner internals, Provider secrets |
| Finance Domain | Payment, Workflow, Audit, Reporting | Religious-specific logic, Provider direct APIs |
| Commerce/POS | Payment, Inventory, Audit, Reporting | Religious internals, Partner internals |
| Marketplace | Commerce, Payment, Notification, Fulfillment | HR internals, Partner internals |
| Logistics | Fulfillment, Notification, Audit | Commerce database internals directly |
| HR | Identity, Organization, Attendance, Workflow, Audit | Religious member lifecycle |
| Scheduling | Notification, Permission, Audit | Payment provider internals |
| Documents/Media | Storage, Permission, Privacy, Audit | Domain-specific rules in Core |
| Support | Identity, Organization, Notification, Workflow, Audit | Billing mutation without Billing foundation |
| Customer Success | Product Analytics, Support, Billing visibility, Audit | Billing mutation, raw surveillance |
| Implementation Delivery | Customer Success, Data Migration, Data Quality, Workflow, Audit | Direct destructive migration execution |
| Solution Blueprints | Config, Module Registry, Permission, Audit | Core domain behavior, arbitrary scripts |
| Partner Ecosystem | Access Lifecycle, Permission, Audit, Implementation Delivery | Unrestricted tenant data access |
| Partner Enablement | Knowledge Base, Policy Library, Partner Ecosystem, Audit | Legal accreditation engine |
| Partner Referrals | Partner Ecosystem, Partner Enablement, Billing visibility, Customer Success | CRM replacement, commission payout |

## Data Governance Dependencies

| Foundation | Depends On | Used By |
|---|---|---|
| Privacy/Consent | Identity, Organization, Permission, Audit | Product Analytics, Exports, Support |
| Data Quality | Import/Export, Audit | Migration, Customer Success, Reporting |
| Lineage/History | Import/Export, Audit | Data Quality, Portability, Reporting |
| Portability/Tenant Exit | Privacy, Lineage, Billing, Support | Organization Lifecycle |
| Organization Network | Organization, Permission, Audit, Governance | Religious branch networks, reporting visibility |
| Governance Authority | Permission, Workflow, Audit | Finance approvals, network decisions, partner delegation |
| Risk/Compliance | Audit, Evidence, Workflow | Policy Library, Operator Control Plane |
| Policy Library | Documents, Notification, Audit | Partner Enablement, Governance, Compliance |

## Product Growth Dependencies

| Foundation | Depends On | Used By |
|---|---|---|
| Product Communication | Notification, Feature Flags, Audit | Customer Success, Experimentation |
| Feedback | Identity, Organization, Support, Audit | Product Analytics, Roadmap Intake |
| Product Analytics | Reporting, Privacy, Feature Flags, Audit | Experimentation, Customer Success |
| Experimentation | Feature Flags, Product Analytics, Audit | Product Communication, Customer Success |
| Customer Success | Product Analytics, Support, Billing visibility, Knowledge Base | Implementation Delivery, Partner Referrals |

## Partner Dependency Chain

```text
Partner Ecosystem
  -> Partner Enablement
  -> Partner Referrals
  -> Implementation Delivery
  -> Customer Success
  -> Subscription/Billing visibility
```

Rules:

- Certification does not grant access.
- Delegation grants scoped access after tenant approval.
- Referrals do not create committed revenue.
- Partner status updates must be safe summaries.

## Implementation Dependency Order

Recommended build order:

```text
1. Backend scaffold
2. Identity
3. Organization/Tenant
4. Permission
5. Audit
6. Module Registry
7. Configuration
8. Provider Connections
9. Notification
10. Payment/SMS basics
11. Religious MVP slice
12. Reporting basics
13. Support/Customer Success later
14. Partner and marketplace foundations later
```

## Forbidden Dependencies

Do not allow:

```text
Core -> Religious
Core -> Commerce
Core -> Partner
Domain -> Provider secret storage
Domain -> Raw payment provider API
Domain -> Raw SMS provider API
Partner -> unrestricted tenant data
Analytics -> raw user surveillance
Blueprint -> arbitrary script execution
Implementation Delivery -> destructive migration execution
```

## Final Rule

Dependencies should flow from generic platform capabilities to domain-specific behavior, never the reverse.
