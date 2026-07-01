# Database Relationship Map

## Purpose

This document provides a text-based relationship map for the MVP database model.

It helps developers understand how Core, Engines, and the first Religious Domain tables relate before implementation begins.

## Principle

Platform Core owns generic platform records.

Capability Engines own reusable capability records.

Domains own business-specific records.

Organization-owned records must connect back to organization context.

## High-Level Core Map

```text
users
  -> organization_users
      -> organizations
          -> organization_settings
          -> organization_modules
          -> organization_subscriptions
          -> audit_logs
```

## Identity and Organization

```text
users
  -> user_credentials
  -> user_sessions
  -> organization_users

organizations
  -> organization_users
  -> organization_settings
  -> organization_invitations
```

Meaning:

- `users` are global identities.
- `organization_users` connects users to organizations.
- A user can belong to many organizations.
- An organization can have many users.

## Permission Relationships

```text
permissions
  -> role_permissions
      -> roles
          -> organization_user_roles
              -> organization_users
```

Meaning:

- permissions are stable platform action keys.
- roles group permissions.
- organization users receive roles.
- application services use Permission Engine to check actions.

## Module and Subscription Relationships

```text
modules
  -> organization_modules
      -> organizations

subscription_plans
  -> plan_features
  -> plan_limits
  -> organization_subscriptions
      -> organizations
```

Meaning:

- modules define available functional areas.
- organization modules define what an organization has enabled.
- subscription plans define commercial access and limits.
- organization subscriptions connect organizations to plans.

## Audit Relationship

```text
organizations
  -> audit_logs

users
  -> audit_logs as actor_user_id
```

Meaning:

- audit logs may belong to an organization.
- audit logs may record the acting user.
- platform-level audit records may not always have an organization.

## Engine Relationship Direction

Capability engines usually reference organizations and records from domains or Core.

Examples:

```text
organizations
  -> payment_transactions
  -> sms_wallets
  -> sms_messages
  -> storage_files
  -> notification_records
  -> workflow_tasks
```

Engines should not depend on Religious Domain tables directly unless the relationship is expressed through generic owner fields.

## Storage Foundation

```text
storage_files
  -> organizations
  -> users nullable
```

File metadata should store organization context and owner information.

The domain owns file meaning. Storage Engine owns file handling.

## Payment Foundation

```text
payment_transactions
  -> organizations
  -> users nullable
```

Payment transaction records may reference a domain owner using owner type and owner id fields.

## SMS Foundation

```text
sms_wallets
  -> organizations

sms_messages
  -> organizations
  -> sms_campaigns nullable
```

SMS wallets and messages belong to organizations.

## Religious Domain Map

```text
organizations
  -> religious_settings
  -> religious_members
      -> religious_member_status_history
      -> religious_group_members
      -> religious_attendance_records
  -> religious_visitors
      -> religious_visitor_follow_ups
  -> religious_congregations
      -> religious_services
      -> religious_groups
  -> religious_attendance_sessions
      -> religious_attendance_records
```

## Religious Structure Map

```text
religious_congregations
  -> religious_services
  -> religious_groups

religious_groups
  -> religious_group_members
      -> religious_members
```

Meaning:

- congregations define people grouping.
- services define scheduled gatherings.
- groups define smaller units such as departments, cells, and study groups.
- group membership connects members to groups.

## Religious Attendance Map

```text
religious_attendance_sessions
  -> religious_attendance_records
      -> religious_members
```

Meaning:

- attendance sessions define the event or meeting being tracked.
- attendance records store each member's attendance status for that session.

## Religious Visitor Conversion Map

```text
religious_visitors
  -> religious_members
```

Meaning:

- visitor conversion creates a member record.
- conversion should be handled by an application service.
- the relationship should be traceable through conversion metadata or history where implemented.

## Religious Giving Foundation Later

```text
organizations
  -> religious_giving_categories
  -> religious_giving_records
      -> religious_members nullable
      -> payment_transactions nullable
```

Giving records belong to Religious Domain.

Payment processing belongs to Payment Engine.

## Ownership Rule

Organization-owned tables must include organization context unless a documented exception exists.

Core tables must not depend on Religious Domain tables.

Domain tables may depend on Core identities, organizations, modules, and engine records through clear boundaries.

## Final Rule

Relationships should preserve ownership boundaries: Core owns generic records, Engines own reusable capability records, and Domains own business records.
