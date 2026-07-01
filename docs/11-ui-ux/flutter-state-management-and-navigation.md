# Flutter State Management and Navigation

## Purpose

This document defines the preferred direction for managing Flutter app state and navigation in Universal Platform.

It exists to keep mobile and desktop/admin experiences consistent while still allowing each platform to use layouts that fit the device.

## Principle

Flutter screens should be simple presentation layers.

The backend provides the main platform summaries needed by the app, and Flutter uses those summaries to decide what to display.

## Main App State

The app may keep these shared state areas:

- current user summary
- selected organization
- current language
- organization summary
- module summary
- feature summary
- navigation summary
- notification count

## State Refresh

The app should refresh organization-related state when the selected organization changes.

Typical refresh areas:

- organization summary
- module summary
- feature summary
- navigation summary
- screen data for the selected organization

## Navigation Direction

Navigation should be generated from a small set of app state inputs.

Important inputs:

- whether the user has started a session
- selected organization
- module summary
- feature summary
- mobile or desktop layout

## Mobile Direction

Mobile navigation should focus on quick workflows.

Examples:

- profile
- announcements
- events
- giving history
- attendance actions
- group activities
- leader quick actions

## Desktop Direction

Desktop navigation should focus on management workflows.

Examples:

- dashboards
- tables
- reports
- settings
- users
- modules
- domain administration screens

## Screen States

Screens should support common states:

- loading
- empty
- unavailable
- network issue
- expired session

## Component Reuse

Shared components should be preferred for:

- page headers
- cards
- lists
- tables
- filters
- empty states
- status badges
- action buttons

## Testing Direction

Flutter tests should cover:

- app start state
- organization selection
- mobile layout
- desktop layout
- loading state
- empty state
- unavailable feature state

## Final Rule

Flutter navigation should be dynamic, platform-aware, and based on backend-provided summaries. Avoid one-off hardcoded menus that cannot adapt as modules and domains grow.
