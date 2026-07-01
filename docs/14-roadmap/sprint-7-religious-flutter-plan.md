# Sprint 7 Religious Flutter Plan

## Purpose

This document defines Sprint 7 for the first Religious MVP Flutter workflows.

Sprint 7 should connect the Flutter client foundation to the Religious MVP backend foundation.

## Sprint Goal

Create the first usable Religious Domain screens in Flutter without expanding beyond MVP scope.

## Prerequisites

Before Sprint 7 begins:

- Sprint 6 Flutter foundation is complete
- API client foundation exists
- app shell exists
- selected organization state exists
- Religious backend routes are stable enough for integration
- module and feature unavailable states are supported or planned

## Work Package 1: Religious Navigation

Add Religious navigation entry points based on module availability.

Expected result:

- Religious item appears only when the module is available
- unavailable state appears safely when module is not available

## Work Package 2: Religious Dashboard

Create a simple Religious dashboard or summary placeholder.

Possible summary cards:

- members
- visitors
- groups
- attendance sessions

## Work Package 3: Member Screens

Create:

- member list
- member detail placeholder or foundation
- create member form foundation

## Work Package 4: Visitor Screens

Create:

- visitor list
- visitor detail placeholder or foundation
- create visitor form foundation
- convert visitor action foundation where backend is ready

## Work Package 5: Structure Screens

Create list screens or placeholders for:

- congregations
- services
- groups

## Work Package 6: Attendance Screens

Create:

- attendance session list
- create attendance session foundation where backend is ready
- mark attendance foundation where backend is ready

## Work Package 7: API Integration

Add Flutter API client methods for Religious MVP contracts.

Use existing standard response/error parsing.

## Work Package 8: UI States

Each screen should support:

- loading
- empty
- error
- unavailable
- retry where useful

## Work Package 9: Tests

Add tests where practical for:

- Religious navigation entry
- member list rendering
- create member form validation foundation
- visitor list rendering
- unavailable module state
- API response parsing

## Out of Scope

Do not implement:

- QR attendance
- GPS attendance
- full offline sync
- payment or giving UI
- SMS campaign UI
- full family/household UI
- full care/counseling UI
- POS or Commerce UI

## Completion Standard

Sprint 7 is complete when the first Religious MVP workflows are reachable from the Flutter shell and use the shared API client, state, navigation, and design system foundations.

## Final Rule

Sprint 7 should make the MVP usable, not complete every Religious feature.
