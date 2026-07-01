# Religious MVP Migration Plan

## Purpose

This document defines the recommended migration order for the first Religious Domain MVP.

The migration plan helps implementation avoid circular dependencies and keeps the first domain build predictable.

## Prerequisites

Before these migrations are created, Core migrations should already exist for:

- users
- organizations
- organization users
- permissions
- roles
- modules
- subscriptions
- audit logs
- storage file metadata where needed

## Migration Principle

Religious Domain tables must depend on Platform Core, not the other way around.

Every organization-owned Religious table should include organization context.

## Recommended Migration Order

### 1. Religious Settings

Create table for Religious organization settings.

Purpose:

- terminology
- domain-level defaults
- future attendance and member settings

### 2. Members

Create:

- religious_members
- religious_member_status_history

Purpose:

- member profile foundation
- status tracking

### 3. Structure

Create:

- religious_congregations
- religious_services
- religious_groups
- religious_group_members

Purpose:

- organizational ministry structure
- group membership

### 4. Visitors

Create:

- religious_visitors
- religious_visitor_follow_ups
- religious_visitor_status_history where useful

Purpose:

- visitor registration
- follow-up foundation
- conversion flow

### 5. Attendance

Create:

- religious_attendance_sessions
- religious_attendance_records
- religious_attendance_edit_history where useful

Purpose:

- manual attendance foundation
- audit-friendly correction history

### 6. Basic Reporting Support

Create summary tables only if the first implementation needs them.

For MVP, prefer simple indexed queries first.

## Index Direction

Common indexes:

```text
organization_id
organization_id + status
organization_id + created_at
organization_id + branch_id
organization_id + congregation_id
organization_id + session_date
```

## Constraint Direction

Use constraints for important rules.

Examples:

- group member must reference a member in the same organization context at application level
- attendance should avoid duplicate member record for same session
- member code should be unique per organization where used

## Seed Direction

Seed only safe defaults:

- Religious module records
- Religious permission keys
- default Religious settings where useful

Do not seed real member or visitor data outside development sample data.

## Final Rule

Religious migrations must be built after Core and must never force Platform Core to know Religious Domain concepts.
