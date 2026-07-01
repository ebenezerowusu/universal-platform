# Database Relationship Map

## Purpose

This document provides a text-based relationship map for the first Universal Platform data model.

It helps developers understand how Core, Engines, and Religious MVP records relate.

## Principle

Platform Core owns generic platform records.

Domains own business-specific records.

Organization-owned records must connect back to organization context.

## Core Relationship Map

```text
users
  -> organization_users
    -> organizations
```

A user can belong to many organizations.

An organization can have many users.

## Organization Foundation

```text
organizations
  -> organization_users
  -> organization_settings
  -> organization_modules
  -> organization_subscriptions
  -> audit_logs
```

Organization is the central tenant boundary for most business records.

## Permission Foundation

```text
permissions
  -> role_permissions
    -> roles
      -> organization_user_roles
        -> organization_users
```

Permissions define available actions.

Roles group permissions.

Organization users receive roles.

## Module Registry Foundation

```text
modules
  -> organization_modules
    -> organizations
```

Module definitions are platform-level.

Organization module records decide which modules are available to an organization.

## Subscription Foundation

```text
subscription_plans
  -> plan_features
  -> plan_limits
  -> organization_subscriptions
    -> organizations
```

Plan records define commercial access.

Organization subscription records connect plans to organizations.

## Audit Foundation

```text
audit_logs
  -> organizations
  -> users
```

Audit logs may reference actor user, organization, entity type, entity id, and metadata.

## Storage Foundation

```text
storage_files
  -> organizations
  -> users
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

## Religious Member Foundation

```text
organizations
  -> religious_members
    -> religious_member_status_history
```

A religious member belongs to one organization context.

A member may optionally link to a global user account later.

## Religious Structure Foundation

```text
organizations
  -> religious_congregations
    -> religious_services
    -> religious_groups
      -> religious_group_members
        -> religious_members
```

Congregations group people.

Services represent scheduled gatherings.

Groups represent smaller units such as departments, cells, and study groups.

## Religious Visitor Foundation

```text
organizations
  -> religious_visitors
    -> religious_visitor_follow_ups
```

Visitors can later be converted to members through an application service.

## Religious Attendance Foundation

```text
organizations
  -> religious_attendance_sessions
    -> religious_attendance_records
      -> religious_members
```

Attendance sessions may reference services, groups, events, or programs depending on attendance target type.

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

## Final Rule

Relationships should preserve ownership boundaries: Core owns generic records, Engines own reusable capability records, and Domains own business records.
