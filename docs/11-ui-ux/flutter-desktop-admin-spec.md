# Flutter Desktop Admin Specification

## Purpose

This document defines the initial desktop/admin app scope for Universal Platform.

The desktop app should focus on administration, management, reporting, and operational workflows.

## Desktop App Users

Possible users:

- Organization administrators
- Branch administrators
- Leaders
- Finance officers
- HR officers
- POS operators later
- Platform administrators

## Desktop App Principle

Desktop should provide powerful management workflows while remaining module-aware, permission-aware, and organization-aware.

Business logic must remain in the backend.

## Core Desktop Features

Initial shared features:

- Login
- Organization selector
- Dashboard shell
- Sidebar navigation
- Module-aware menus
- Role/permission-aware actions
- User profile
- Settings

## Platform Admin Features

Possible platform admin features:

- Organizations
- Subscription plans
- Module registry
- SMS packages
- Provider configuration
- Platform audit logs
- Platform reports

## Organization Admin Features

Possible organization admin features:

- Organization settings
- Users and roles
- Installed modules
- Subscription status
- Audit logs
- Reports

## Religious Admin Features

Possible Religious Domain admin features:

- Members
- Visitors
- Branch hierarchy
- Congregations
- Services
- Groups
- Education programs
- Attendance sessions
- Giving and pledges
- Communication
- Events
- Care/prayer/counseling/welfare
- Reports and dashboards

## Desktop UI Patterns

Recommended patterns:

- Sidebar navigation
- Data tables
- Search and filters
- Bulk actions
- Detail drawers
- Modal dialogs
- Report filters
- Export actions
- Audit/history panels

## Security Rules

- Menus should be filtered by permission.
- Backend must still enforce authorization.
- Sensitive data should not be cached carelessly.
- Confidential modules require explicit access checks.

## Final Rule

Desktop/admin is for management depth. Mobile is for speed and focused workflows. Both consume the same backend contracts.
