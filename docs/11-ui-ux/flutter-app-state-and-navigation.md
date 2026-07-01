# Flutter App State and Navigation

## Purpose

This document defines how Flutter screens should organize shared app state and navigation.

The same Flutter codebase should support mobile and desktop/admin layouts.

## Principle

The app should use backend-provided summaries to decide what to show.

Backend services remain responsible for business behavior.

## App State

Shared state may include:

- user summary
- selected organization
- language
- organization summary
- module summary
- feature summary
- navigation summary

## Organization Change

When the selected organization changes, reload organization-specific screen data.

## Mobile Direction

Mobile should focus on simple and fast workflows.

Examples:

- profile
- announcements
- events
- giving history
- attendance actions
- leader quick actions

## Desktop Direction

Desktop/admin should focus on management workflows.

Examples:

- dashboards
- tables
- reports
- settings
- user management
- module management

## Screen States

Screens should support:

- loading
- empty state
- unavailable feature state
- network issue state

## Final Rule

Flutter navigation should be dynamic and platform-aware. Avoid building one-off menus that cannot adapt to organizations, modules, language, or device type.
