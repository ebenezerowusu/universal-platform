# Seed Data Strategy

## Purpose

This document defines how seed data should be handled in Universal Platform.

Seed data helps initialize stable platform records such as permissions, modules, plans, and default role templates.

## Seed Data Principle

Seed data should be repeatable and safe to run more than once.

Do not create duplicate records when seed commands run repeatedly.

## Seed Data Categories

### Platform Seed Data

Examples:

- Permission keys
- Core modules
- Engine modules
- Default subscription plans
- Provider definitions
- Supported languages
- Regional defaults

### Organization Seed Data

Examples:

- Default organization roles
- Default organization settings
- Default Religious Domain settings
- Default giving categories where appropriate

### Development Seed Data

Examples:

- Sample organization
- Sample users
- Sample Religious members
- Sample groups

Development seed data must not be used as production truth.

## Idempotency

Seed operations should use stable keys.

Examples:

```text
permission_key
module_key
plan_code
provider_key
language_code
```

If a record already exists, update safe fields or skip according to seed policy.

## Environment Rules

Production seed data should only include required platform defaults.

Development seed data may include sample records for testing and UI work.

## Ordering

Recommended seed order:

1. Supported languages and regional defaults
2. Provider definitions
3. Modules
4. Permissions
5. Subscription plans
6. Plan features and limits
7. Default role templates
8. Development sample data where allowed

## Safety Rules

- Do not seed real secrets.
- Do not seed real customer data.
- Do not overwrite production organization custom settings accidentally.
- Do not create records without stable identifiers.

## Testing Requirements

Tests should cover:

- Seed can run once
- Seed can run multiple times without duplicates
- Required permissions exist
- Required modules exist
- Default plans exist

## Final Rule

Seed data initializes platform foundations. It must be stable, repeatable, and environment-aware.
