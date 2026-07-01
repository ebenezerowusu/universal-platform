# Seed Data Plan

## Purpose

This document defines the initial seed data required for Universal Platform.

Seed data should make the platform usable after the first migrations run.

## Seed Data Principle

Seed only stable platform defaults.

Do not seed organization-specific business data unless it is required for a test/demo environment.

## Core Seed Groups

Initial seed groups:

- platform permissions
- platform roles
- core modules
- capability engine modules
- Religious Domain modules
- subscription plans
- default plan features
- default plan limits
- supported languages
- default regional settings

## Permissions

Seed permissions from:

```text
docs/04-capability-engines/initial-permission-catalog.md
```

Permission seed data should include:

- permission_key
- description
- module_key
- category
- status

## Roles

Initial platform roles:

- platform_owner
- platform_admin
- platform_support

Initial organization role templates:

- organization_owner
- organization_admin
- branch_admin
- leader
- finance_user
- viewer

Role templates should be customizable per organization later.

## Modules

Seed modules from:

```text
docs/13-module-sdk/initial-module-catalog.md
```

Module seed data should include:

- module_key
- domain_key
- name
- description
- version
- is_core
- is_paid
- status

## Subscription Plans

Initial plan examples:

- free
- starter
- growth
- enterprise

Plan details can be refined later, but feature and limit structure should exist early.

## Languages and Region Defaults

Initial language examples:

- en
- tw
- fr
- sw

Initial region settings may include:

- country code
- default currency
- default timezone
- default language

## Environment-Specific Seeds

Production seeds should include only stable required data.

Development/demo seeds may include sample organizations, users, and records.

Do not mix demo data with production seed scripts.

## Idempotency

Seed scripts must be idempotent.

Running seed scripts multiple times should not create duplicate permissions, modules, or plans.

Use stable keys for upserts.

## Final Rule

Seed data should create stable platform foundations, not hidden business behavior.
