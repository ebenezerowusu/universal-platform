# Sprint 43 Organization Lifecycle Provisioning Roadmap

## Purpose

This document defines Sprint 43 for Organization Lifecycle, Provisioning, and Tenant Operations foundation.

Sprint 43 should create a reusable tenant lifecycle layer for provisioning requests, organization statuses, activation, suspension, reactivation, admin bootstrap, regional defaults, module bootstrap, billing linkage, onboarding linkage, and tenant isolation validation.

## Sprint Goal

Enable the platform to create, activate, suspend, reactivate, and operationally track organizations through permission-protected, auditable, and configuration-driven workflows.

## Why This Sprint

The platform now has many foundations that affect tenant readiness:

- organization and identity foundations
- regional settings
- module registry and feature entitlement
- subscription billing
- payment operations
- support onboarding
- monitoring and recovery
- audit and governance
- domain modules such as Religious, Commerce, HR, Finance, and Logistics

Without a shared lifecycle/provisioning foundation, organization setup will become manual, inconsistent, and unsafe.

## Work Package 1: Provisioning Request Foundation

Prepare provisioning request records with:

- organization name
- requested country
- requested modules
- owner/admin user reference
- status
- source channel placeholder
- created at

## Work Package 2: Organization Lifecycle Status

Prepare lifecycle statuses such as:

```text
draft
provisioning_requested
provisioning_in_progress
active
suspended
reactivation_requested
archived_placeholder
manual_review
```

Do not implement destructive organization deletion.

## Work Package 3: Activation and Suspension Placeholders

Prepare action records for:

- activation requested
- activated
- suspension requested
- suspended
- reactivation requested
- reactivated
- archive placeholder

## Work Package 4: Admin Bootstrap

Prepare organization owner/admin bootstrap direction:

- create or link first admin user
- assign owner/admin role placeholder
- invite flow placeholder
- audit ownership creation

## Work Package 5: Regional Settings Bootstrap

Prepare regional defaults:

- country
- currency
- timezone
- default language
- supported languages
- formatting metadata

Read regional configuration instead of hardcoding any country.

## Work Package 6: Module and Billing Linkage

Prepare linkage placeholders for:

- requested modules
- installed modules
- plan/subscription reference
- entitlement check status
- billing readiness placeholder

## Work Package 7: Onboarding and Support Linkage

Prepare linkage to support operations:

- onboarding checklist
- support case placeholder
- health review placeholder
- blocker note placeholder

## Work Package 8: Tenant Isolation Validation

Prepare validation placeholders for:

- organization id present on tenant-owned records
- admin user belongs to organization
- module access is organization-scoped
- billing/subscription reference belongs to organization
- regional settings belong to organization

## Work Package 9: Flutter Foundation

Prepare screens or placeholders for:

- organization lifecycle dashboard
- provisioning request list
- provisioning request detail
- activation/suspension actions
- regional bootstrap summary
- module bootstrap summary
- tenant isolation validation summary

## Out of Scope

Do not implement:

- unsafe cross-tenant data access
- destructive organization deletion
- automatic production data migration
- direct payment provider calls during provisioning
- country-specific provisioning logic inside Platform Core
- provider-specific infrastructure provisioning scripts inside domain modules

## Completion Standard

Sprint 43 is complete when provisioning requests, organization lifecycle statuses, activation/suspension placeholders, admin bootstrap, regional defaults, module/billing linkage, onboarding linkage, and tenant-isolation validation placeholders are available through permission-protected and auditable foundations.

## Final Rule

Organization lifecycle must be safe enough to onboard many tenants without weakening tenant isolation or hardcoding country/domain behavior.
