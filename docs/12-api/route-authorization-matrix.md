# Route Authorization Matrix

## Purpose

This document maps important API route groups to authentication, organization context, module access, and permission checks.

It helps implementation avoid inconsistent authorization behavior.

## Matrix Principle

Every protected route must define its access requirements before implementation.

## Public Routes

| Route Group | Authentication | Organization Context | Permission |
|---|---:|---:|---|
| Health | No | No | None |
| Login | No | No | None |
| Password recovery | No | No | None |

## Authenticated User Routes

| Route Group | Authentication | Organization Context | Permission |
|---|---:|---:|---|
| Current user profile | Yes | No | None |
| User organizations | Yes | No | None |

## Organization Routes

| Route Group | Authentication | Organization Context | Permission |
|---|---:|---:|---|
| View organization | Yes | Yes | organizations.profile.view |
| Update organization | Yes | Yes | organizations.profile.update |
| View organization users | Yes | Yes | organizations.users.view |
| Invite organization user | Yes | Yes | organizations.users.invite |
| Manage roles | Yes | Yes | organizations.roles.manage |

## Module Routes

| Route Group | Authentication | Organization Context | Permission |
|---|---:|---:|---|
| View available modules | Yes | Yes | modules.catalog.view |
| Install module | Yes | Yes | modules.install |
| Activate module | Yes | Yes | modules.activate |
| Deactivate module | Yes | Yes | modules.deactivate |

## Religious MVP Routes

| Route Group | Authentication | Organization Context | Module | Permission |
|---|---:|---:|---|---|
| List members | Yes | Yes | religious.members | religious.members.view |
| Create member | Yes | Yes | religious.members | religious.members.create |
| Update member | Yes | Yes | religious.members | religious.members.update |
| List visitors | Yes | Yes | religious.visitors | religious.visitors.view |
| Create visitor | Yes | Yes | religious.visitors | religious.visitors.create |
| Convert visitor | Yes | Yes | religious.visitors | religious.visitors.convert |
| List congregations | Yes | Yes | religious.structure | religious.congregations.view |
| Manage congregations | Yes | Yes | religious.structure | religious.congregations.manage |
| List groups | Yes | Yes | religious.structure | religious.groups.view |
| Manage groups | Yes | Yes | religious.structure | religious.groups.manage |
| Create attendance session | Yes | Yes | religious.attendance | religious.attendance.mark |
| Mark attendance | Yes | Yes | religious.attendance | religious.attendance.mark |
| View attendance reports | Yes | Yes | religious.attendance | religious.attendance.reports.view |

## Sensitive Routes

Sensitive routes may require additional checks such as scope, audit, or feature availability.

Examples:

- financial reports
- sensitive member exports
- confidential care records
- provider configuration
- platform audit records

## Final Rule

Route authorization must be explicit. Do not rely on implicit controller or UI behavior.
