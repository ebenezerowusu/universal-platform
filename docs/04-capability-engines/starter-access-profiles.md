# Starter Access Profiles

## Purpose

This document defines starter access profiles for Universal Platform.

Access profiles help new organizations begin with useful permission groupings while still allowing custom roles later.

## Principle

An access profile is a starting point.

The Permission Engine remains the source of authorization truth.

## Platform Access Profiles

### Platform Administrator

Purpose:

- Manage platform-level configuration and platform records.

Typical access areas:

- organizations
- modules
- subscription plans
- provider settings
- platform audit records

### Platform Support

Purpose:

- Assist with support and troubleshooting.

Typical access areas:

- organization lookup
- module status review
- limited platform activity review

## Organization Access Profiles

### Organization Owner

Purpose:

- Manage the full organization account.

Typical access areas:

- organization profile
- users
- roles
- settings
- modules
- subscription visibility

### Organization Administrator

Purpose:

- Manage daily organization operations.

Typical access areas:

- organization profile view
- user view
- settings view
- module catalog view

### Finance User

Purpose:

- Manage financial and payment-related work where allowed.

Typical access areas:

- giving records
- payment records
- financial reports

### Communication User

Purpose:

- Manage announcements and communication work where allowed.

Typical access areas:

- announcements
- messages
- campaigns
- delivery reports

## Religious Access Profiles

### Branch Leader

Purpose:

- Manage assigned branch operations.

Default scope:

```text
branch
```

Typical access areas:

- member view
- visitor view
- attendance
- group view
- branch reports

### Group Leader

Purpose:

- Manage assigned group operations.

Default scope:

```text
own_group
```

Typical access areas:

- assigned group view
- attendance marking
- assigned follow-up tasks

### Care User

Purpose:

- Support care and follow-up operations.

Typical access areas:

- care records
- prayer records
- follow-up tasks

## Custom Profiles

Organizations should be able to create custom access profiles based on starters.

Customized profiles should not unexpectedly change when starter profiles are updated later.

## Final Rule

Starter access profiles help setup. Backend permission checks decide actual access.
