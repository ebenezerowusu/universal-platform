# Core Data Model

## Purpose

This document defines the first Platform Core data model at a planning level.

The goal is to make implementation consistent before migrations are written.

## Core Principle

The Core data model must remain domain-agnostic.

Core tables should support identity, organizations, permissions, modules, subscriptions, configuration, and audit. They must not include religious, commerce, POS, HR, or finance-specific business concepts.

## Identity Tables

### users

Purpose:

- Global platform identity.

Candidate columns:

- id
- display_name
- email
- phone
- status
- email_verified_at
- phone_verified_at
- created_at
- updated_at
- deleted_at

### user_credentials

Purpose:

- Store authentication credential records.

Candidate columns:

- id
- user_id
- credential_type
- credential_identifier
- credential_secret_hash
- status
- created_at
- updated_at

### refresh_tokens or user_sessions

Purpose:

- Track refresh/session state where the chosen auth design requires it.

Candidate columns:

- id
- user_id
- token_hash
- expires_at
- revoked_at
- created_at

## Organization Tables

### organizations

Purpose:

- Tenant/customer record.

Candidate columns:

- id
- name
- legal_name
- slug
- type
- country
- currency
- timezone
- default_language
- status
- created_at
- updated_at
- deleted_at

### organization_users

Purpose:

- Link users to organizations.

Candidate columns:

- id
- organization_id
- user_id
- status
- joined_at
- invited_by
- created_at
- updated_at

### organization_settings

Purpose:

- Store organization-specific settings.

Candidate columns:

- id
- organization_id
- setting_key
- setting_value
- value_type
- created_at
- updated_at

## Permission Tables

### permissions

Purpose:

- Define permission keys available in the platform.

Candidate columns:

- id
- permission_key
- description
- module_key
- created_at
- updated_at

### roles

Purpose:

- Define organization-scoped or platform roles.

Candidate columns:

- id
- organization_id nullable
- name
- description
- role_type
- status
- created_at
- updated_at

### role_permissions

Purpose:

- Link roles to permissions.

Candidate columns:

- id
- role_id
- permission_id
- created_at

### organization_user_roles

Purpose:

- Assign roles to users inside organizations.

Candidate columns:

- id
- organization_id
- user_id
- role_id
- scope_type
- scope_id nullable
- created_at
- updated_at

## Module Tables

### modules

Purpose:

- Register platform modules.

Candidate columns:

- id
- module_key
- domain_key
- name
- description
- version
- status
- is_core
- is_paid
- created_at
- updated_at

### organization_modules

Purpose:

- Track modules installed for organizations.

Candidate columns:

- id
- organization_id
- module_id
- status
- installed_by
- installed_at
- activated_at
- suspended_at
- configuration
- created_at
- updated_at

## Subscription Tables

### subscription_plans

Purpose:

- Define SaaS plans.

Candidate columns:

- id
- code
- name
- description
- currency
- price
- billing_interval
- status
- trial_days
- created_at
- updated_at

### subscription_plan_features

Purpose:

- Define features enabled by plan.

Candidate columns:

- id
- plan_id
- feature_key
- enabled
- created_at
- updated_at

### subscription_plan_limits

Purpose:

- Define quantity limits by plan.

Candidate columns:

- id
- plan_id
- limit_key
- limit_value
- created_at
- updated_at

### organization_subscriptions

Purpose:

- Track active organization subscription.

Candidate columns:

- id
- organization_id
- plan_id
- status
- starts_at
- trial_ends_at
- current_period_start
- current_period_end
- cancelled_at
- created_at
- updated_at

## Audit Table

### audit_logs

Purpose:

- Record important and sensitive actions.

Candidate columns:

- id
- organization_id nullable
- actor_user_id nullable
- actor_type
- action
- entity_type
- entity_id
- severity
- before_values
- after_values
- metadata
- ip_address
- user_agent
- trace_id
- created_at

## Provider Registry Tables

### provider_definitions

Purpose:

- Register available provider types.

Candidate columns:

- id
- provider_key
- provider_type
- name
- status
- created_at
- updated_at

### provider_configs

Purpose:

- Store provider configuration metadata.

Candidate columns:

- id
- organization_id nullable
- provider_definition_id
- config_scope
- status
- encrypted_config_reference
- metadata
- created_at
- updated_at

## Final Rule

Core data supports the platform foundation. Domain data belongs in domain-specific tables.
