# Core Migration Plan

## Purpose

This document defines the recommended migration order for Platform Core.

The goal is to create the database foundation before any product domain tables.

## Migration Principle

Core migrations must be created before domain migrations.

Do not create Religious, Commerce, POS, Finance, or HR tables before identity, organization, permissions, modules, subscriptions, and audit foundations exist.

## Recommended Migration Order

### 1. Identity

Create:

- users
- user_credentials
- user_sessions or refresh_tokens

### 2. Organizations

Create:

- organizations
- organization_users
- organization_settings
- organization_invitations later

### 3. Permissions

Create:

- permissions
- roles
- role_permissions
- organization_user_roles

### 4. Modules

Create:

- modules
- module_versions later
- organization_modules
- module_dependencies later
- module_permissions later

### 5. Subscriptions

Create:

- subscription_plans
- subscription_plan_features
- subscription_plan_limits
- organization_subscriptions
- subscription_usage_counters later

### 6. Audit

Create:

- audit_logs

### 7. Provider Registry

Create:

- provider_definitions
- provider_configs
- regional_provider_rules later

### 8. Engine Tables

Create foundation tables for:

- payment transactions
- SMS wallets
- notification records
- file metadata
- workflow tasks
- report exports
- import/export jobs

Engine tables may be phased based on implementation order.

## Migration Review Rules

Before merging migrations, review:

- tenant ownership
- foreign keys
- unique constraints
- indexes
- default values
- rollback/recovery notes
- existing data impact

## First Migration Batch Goal

The first batch should support:

- user creation
- organization creation
- user membership in organization
- role and permission assignment
- audit logging
- module registration
- subscription plan assignment

## Final Rule

The core migration plan must enable a safe multi-tenant platform before any domain feature is added.
