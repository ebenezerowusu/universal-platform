# Organization Lifecycle Provisioning Guide

## Purpose

This document defines operating guidance for organization provisioning, lifecycle statuses, activation, suspension, reactivation, admin bootstrap, regional bootstrap, module bootstrap, billing linkage, onboarding linkage, and tenant isolation validation.

The goal is to make organization onboarding repeatable, safe, auditable, and tenant-isolated.

## Principle

An organization should not become active until required lifecycle, ownership, regional, module, billing, and tenant-isolation checks have been considered.

## Data Sources

Lifecycle operations may use:

- provisioning request
- organization lifecycle status
- lifecycle action history
- admin bootstrap record
- regional bootstrap record
- module bootstrap record placeholder
- billing linkage placeholder
- onboarding linkage placeholder
- tenant isolation validation placeholder
- audit records

## Provisioning Direction

Provisioning requests should capture:

```text
organization_name
requested_country_code
requested_modules
owner_user_id optional
source_channel_placeholder
status
created_at
```

## Lifecycle Direction

Lifecycle actions should capture:

```text
organization_id
action
old_status
new_status
actor
reason optional
created_at
```

## Admin Bootstrap Direction

Admin bootstrap should verify:

- first owner/admin exists
- owner/admin belongs to organization
- required role assignment exists
- invite status is tracked where applicable
- ownership creation is audited

## Regional Bootstrap Direction

Regional bootstrap should verify:

- country selected
- currency selected
- timezone selected
- default language selected
- supported languages configured
- formatting metadata available

## Module Bootstrap Direction

Module bootstrap should verify:

- requested modules are known
- installed modules are organization-scoped
- required feature entitlement exists where applicable
- disabled modules do not appear active

## Billing Linkage Direction

Billing linkage should verify:

- subscription belongs to organization
- billing status is compatible with module access
- entitlement changes are recorded
- trial/activation placeholders are tracked where applicable

## Onboarding Linkage Direction

Onboarding linkage may connect to:

- support onboarding tracker
- operational checklist
- organization health review
- support case placeholder
- blocker notes

## Tenant Isolation Validation Direction

Tenant validation should check where practical:

- organization id is present on tenant-owned records
- admin user belongs to organization
- module access is organization-scoped
- billing/subscription reference belongs to organization
- regional settings belong to organization
- support/onboarding records belong to organization

## Audit Direction

Audit should record:

- provisioning request created
- provisioning approved/rejected placeholder
- lifecycle status changed
- admin bootstrap changed
- regional bootstrap changed
- module bootstrap changed
- billing linkage changed
- tenant validation run placeholder

## Data Quality Warnings

Warn or flag when:

- organization has no owner/admin
- organization has no regional settings
- activated organization has no module access
- module access exists without entitlement
- billing linkage points to another organization
- tenant validation has failed checks
- organization is active but onboarding is blocked

## Final Rule

Organization lifecycle operations must protect tenant boundaries and make readiness visible before activation or reactivation.
