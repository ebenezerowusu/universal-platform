# Seed Implementation Plan

## Purpose

This document defines the implementation plan for seed data in Universal Platform.

Seed data should initialize stable platform records without creating duplicates or overwriting organization-specific choices unexpectedly.

## Seed Principle

Seed data must be repeatable.

Running seed more than once should not create duplicate platform records.

## First Seed Areas

Initial seed work should cover:

- supported languages
- core modules
- engine modules
- platform permissions
- organization permissions
- starter subscription plans
- Religious MVP modules
- Religious MVP permission keys

## Stable Keys

Seed records should use stable keys.

Examples:

```text
module_key
permission_key
plan_code
language_code
provider_key
feature_key
```

## Seed Order

Recommended order:

1. Languages and regional defaults
2. Core modules
3. Engine modules
4. Domain modules
5. Permissions
6. Subscription plans
7. Plan features and limits
8. Starter access profiles later

## Environment Awareness

Production seed should include only required platform defaults.

Development seed may include sample organizations, users, members, groups, and attendance records.

## Update Behavior

For existing seed records, choose one of:

- skip existing record
- update safe descriptive fields
- fail if a protected field changed unexpectedly

Do not silently overwrite organization custom settings.

## Testing Requirements

Tests should cover:

- seed can run once
- seed can run repeatedly
- required permissions exist
- required modules exist
- starter plans exist
- development sample data is not required for production

## Final Rule

Seed data should make the platform ready to run without becoming hidden business logic.
