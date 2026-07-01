# Sprint 6 Flutter Foundation Plan

## Purpose

This document defines Sprint 6 for the Flutter foundation.

Sprint 6 should begin the shared Flutter client foundation after the first backend platform and Religious MVP APIs are sufficiently stable.

## Sprint Goal

Create the Flutter application shell that can support mobile and desktop/admin experiences from a shared codebase.

## Prerequisites

Before Sprint 6 begins:

- backend scaffold exists
- Identity foundation works
- Organization foundation works
- Permission and module summaries are available or planned
- Religious MVP route contracts are stable enough for UI integration
- shared design system direction exists

## Work Package 1: Flutter Project Structure

Create the first Flutter project structure for shared mobile and desktop/admin work.

Suggested areas:

- app bootstrap
- shared design system
- core services
- feature modules
- routing
- localization
- platform layouts

## Work Package 2: App Shell

Create the first app shell.

Expected areas:

- startup screen
- authenticated shell placeholder
- organization selection placeholder
- mobile layout placeholder
- desktop/admin layout placeholder

## Work Package 3: API Client Foundation

Create the first API client structure.

Expected areas:

- base URL configuration
- standard response handling
- standard error handling
- request metadata support
- session context attachment where applicable

## Work Package 4: State Foundation

Create shared state containers for:

- current user summary
- selected organization
- language
- module summary
- feature summary
- navigation summary

## Work Package 5: Design System Foundation

Create reusable components for:

- page header
- buttons
- text fields
- cards
- loading state
- empty state
- error state
- status badge

## Work Package 6: First Screens

Create first screens:

- login screen
- organization selector screen
- mobile home placeholder
- desktop/admin dashboard placeholder

## Work Package 7: Tests

Add tests for:

- app starts
- login screen renders
- organization selection state updates
- mobile layout renders
- desktop layout renders
- loading and empty states render

## Out of Scope

Do not implement yet:

- full Religious member screens
- payment screens
- SMS screens
- offline sync
- POS or Commerce UI
- production app store packaging

## Completion Standard

Sprint 6 is complete when the Flutter shell runs, basic navigation works, API client foundation exists, and shared UI components are ready for feature screens.

## Final Rule

Sprint 6 creates the client foundation. It should not rush into feature-heavy UI before the shell, state, and design system are stable.
