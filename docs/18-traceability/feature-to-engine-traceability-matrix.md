# Feature To Engine Traceability Matrix

## Purpose

This document maps major platform features to the capability engines and foundations responsible for implementing them.

Use this matrix before adding new code so features are placed in the correct engine or domain module.

## Core Rule

```text
Core provides capabilities.
Domains provide business logic.
Configuration controls behavior.
Adapters connect external providers.
```

## Traceability Matrix

| Feature Area | Primary Owner | Required Engines | Supporting Foundations |
|---|---|---|---|
| Authentication | Identity Foundation | Session/Security, Audit | Config, Notification |
| User profiles | Identity Foundation | Permission, Audit | Data Privacy |
| Organizations and tenancy | Organization/Tenant Foundation | Permission, Audit | Lifecycle, Data Privacy |
| Organization lifecycle | Organization Lifecycle Foundation | Workflow, Audit | Support, Billing |
| Invitations and memberships | Access Lifecycle Foundation | Identity, Permission, Notification, Audit | Security Controls |
| Permissions and roles | Permission Engine | Audit | Module Registry, Governance |
| Audit history | Audit Engine | Permission | Compliance, Data Retention |
| Module installation | Module Registry Foundation | Subscription, Permission, Audit | Config, Feature Flags |
| Feature availability | Configuration/Feature Flag Foundation | Module Registry, Permission | Subscription |
| Runtime rules | Configuration/Feature Flag Foundation | Audit | Workflow, Product Analytics |
| Notifications | Notification Engine | Preferences, Audit | Email/SMS adapters later |
| SMS wallet and sending | SMS Engine | Payment, Audit, Notification | Provider Connections, Country Ops |
| Payments | Payment Engine | Provider Connections, Audit | Finance, Billing, Country Ops |
| Storage and media | Storage Engine | Permission, Audit | Privacy, Data Retention |
| Search | Search Engine | Permission, Audit | Data Privacy |
| Reporting | Reporting/Analytics Foundation | Permission, Audit | Product Analytics, Finance |
| Workflows and approvals | Workflow Engine | Permission, Audit, Notification | Governance Authority |
| Background jobs | Jobs/Scheduler Foundation | Audit, Config | Monitoring |
| Provider connections | Provider Connection Foundation | Credential Safety, Audit | Country Ops |
| Religious organization management | Religious Domain | Attendance, Payment, SMS, Workflow | Reporting, Audit |
| Members and visitors | Religious Domain | Identity, Permission, Audit | Privacy, Data Quality |
| Congregations and groups | Religious Domain | Permission, Audit | Reporting |
| Bible study programs | Religious Domain | Attendance, Permission, Reporting | Notification |
| Attendance | Attendance Capability | Audit, Reporting | QR/GPS later, Offline Sync later |
| Giving and church income | Finance Domain | Payment, Audit, Reporting | Subscription/Billing if needed |
| Finance expenses and budgets | Finance Domain | Workflow, Audit, Reporting | Governance Authority |
| Commerce/POS | Commerce Domain | Inventory, Payment, Reporting, Audit | Module Registry |
| Marketplace orders | Commerce Domain | Payment, Notification, Workflow, Audit | Logistics |
| Fulfillment and pickup | Fulfillment Foundation | Workflow, Notification, Audit | Marketplace |
| Delivery and logistics | Logistics Domain | Assignment, Notification, Audit | Fulfillment, Maps adapter later |
| HR and workforce | HR Domain | Attendance, Workflow, Audit | Permission, Reporting |
| Calendar and scheduling | Scheduling Foundation | Notification, Permission, Audit | Workflow |
| Documents and media library | Document/Media Foundation | Storage, Permission, Audit | Search, Privacy |
| Import/export/migration | Data Migration Foundation | Data Quality, Audit, Jobs | Storage |
| Data privacy and consent | Privacy Governance Foundation | Permission, Audit | Data Retention |
| Data quality and duplicates | Data Quality Foundation | Audit, Workflow | Import/Export |
| Data lineage and history | Lineage Foundation | Audit | Import/Export, Reporting |
| Tenant exit and handover | Data Portability Foundation | Privacy, Lineage, Audit | Billing, Support |
| Organization branch network | Organization Network Foundation | Permission, Audit, Workflow | Data Sharing |
| Governance authority | Governance Foundation | Workflow, Permission, Audit | Organization Network |
| Risk and controls | Risk/Compliance Foundation | Audit, Evidence, Workflow | Policy Library |
| Policy attestations | Policy Library Foundation | Notification, Audit | Knowledge Base |
| Knowledge base | Knowledge Base Foundation | Search, Permission, Audit | Support |
| Support operations | Support Foundation | Notification, Workflow, Audit | Customer Success |
| Product communication | Product Communication Foundation | Notification, Targeting, Audit | Feature Flags |
| Feedback and requests | Feedback Foundation | Audit, Product Analytics | Support, Roadmap Intake |
| Product analytics | Product Analytics Foundation | Reporting, Privacy, Audit | Feature Flags |
| Experimentation | Experimentation Foundation | Feature Flags, Product Analytics, Audit | Feedback, Support |
| Customer success | Customer Success Foundation | Product Analytics, Support, Billing, Audit | Knowledge Base |
| Implementation delivery | Implementation Delivery Foundation | Data Migration, Data Quality, Workflow, Audit | Customer Success |
| Solution blueprints | Solution Blueprint Foundation | Config, Module Registry, Permission, Audit | Implementation Delivery |
| Partner ecosystem | Partner Ecosystem Foundation | Permission, Access Lifecycle, Audit | Implementation Delivery |
| Partner enablement | Partner Enablement Foundation | Knowledge Base, Policy Library, Audit | Partner Ecosystem |
| Partner referrals | Partner Referral Foundation | Partner Ecosystem, Billing, Audit | Customer Success, Implementation |
| Developer APIs and webhooks | Developer API Foundation | Permission, Audit, Jobs | Event Catalogue |
| Operator control plane | Platform Operations Foundation | Permission, Audit, Monitoring | Support, Billing |
| Commercial packaging | Commercialization Foundation | Subscription, Billing, Module Registry | Partner Referrals |

## Placement Rules

- New identity behavior belongs in Identity, not individual domains.
- New tenant ownership behavior belongs in Organization/Tenant Foundation.
- New provider logic belongs behind Provider Connections or adapter crates.
- New workflow behavior belongs in Workflow Engine.
- New reporting behavior belongs in Reporting or Product Analytics depending on purpose.
- New religious behavior belongs in Religious Domain, not Core.
- New commerce behavior belongs in Commerce Domain, not Core.
- New partner behavior belongs in Partner foundations, not Organization Core.

## Final Rule

When in doubt, create a reusable capability engine first, then let domains consume it.
