# Initial Migration Order

## Purpose

This document defines the recommended first migration order for Universal Platform.

The goal is to avoid circular dependencies and ensure Core foundations exist before engines and domains depend on them.

## Principle

Build data foundations in dependency order.

Core comes first. Engines come after Core. Domains come after Core and required Engines.

## Phase 1: Runtime Migration Setup

Create migration infrastructure before feature migrations.

Expected result:

- migration folder exists
- migration runner is selected
- local migration command is documented

## Phase 2: Identity Foundation

Create:

```text
users
user_credentials
user_sessions
```

Purpose:

- global user identity
- login foundation
- session foundation

## Phase 3: Organization Foundation

Create:

```text
organizations
organization_users
organization_settings
organization_invitations
```

Purpose:

- organization accounts
- user membership
- selected organization context
- organization-level settings

## Phase 4: Permission Foundation

Create:

```text
permissions
roles
role_permissions
organization_user_roles
```

Purpose:

- stable permission keys
- organization roles
- user role assignment

## Phase 5: Audit Foundation

Create:

```text
audit_logs
```

Purpose:

- important action history
- accountability foundation

## Phase 6: Module Registry Foundation

Create:

```text
modules
organization_modules
module_dependencies optional
module_settings optional
```

Purpose:

- module catalog
- organization module activation

## Phase 7: Subscription and Plan Foundation

Create:

```text
subscription_plans
plan_features
plan_limits
organization_subscriptions
```

Purpose:

- plan access
- feature and limit foundation

## Phase 8: Engine Foundation Tables

Create only the MVP-needed engine tables.

Examples:

```text
storage_files
payment_transactions
sms_wallets
sms_messages
notification_records
workflow_tasks
```

Purpose:

- shared capability records
- local/mock foundation before live providers

## Phase 9: Religious Domain Foundation

Create:

```text
religious_settings
religious_members
religious_member_status_history
religious_congregations
religious_services
religious_groups
religious_group_members
religious_visitors
religious_visitor_follow_ups
religious_attendance_sessions
religious_attendance_records
```

Purpose:

- first business domain MVP

## Phase 10: Seed Data

Seed after tables exist.

Seed order:

1. languages or regional defaults where needed
2. modules
3. permissions
4. starter plans
5. plan features and limits
6. default organization role templates where used
7. Religious MVP module keys and permission keys

## Final Rule

Never create domain migrations before the Core and Engine foundations they depend on.
