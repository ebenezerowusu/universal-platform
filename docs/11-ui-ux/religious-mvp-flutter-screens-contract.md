# Religious MVP Flutter Screens Contract

## Purpose

This document defines the MVP Flutter screen contract for the Religious Domain.

It describes the first usable screens without expanding into advanced Religious features.

## Principle

Religious Flutter screens should use the shared app shell, API client, selected organization state, module availability, feature availability, and design system components.

Do not create disconnected screens that bypass the platform client foundation.

## Navigation Entry

Religious navigation should appear when:

- user is authenticated
- organization is selected
- Religious module is available
- backend navigation or module summary allows the entry

If unavailable, show a safe unavailable state instead of broken links.

## Religious Dashboard Screen

Purpose:

- provide a simple entry point to Religious MVP workflows

Possible cards:

- members
- visitors
- groups
- attendance sessions

## Member List Screen

Should support:

- loading state
- empty state
- error state
- member search foundation where practical
- status filter foundation where practical
- create member action where permitted

## Member Detail Screen

MVP may include either:

- detail placeholder with member summary
- full detail foundation if backend is ready

Should avoid advanced tabs until MVP flow is stable.

## Create Member Screen

Should support foundation fields:

- first name
- last name
- phone
- email
- gender where included
- date of birth where included
- status

Validation should be client-friendly but backend remains the source of truth.

## Visitor List Screen

Should support:

- loading state
- empty state
- error state
- visitor status filter foundation
- create visitor action where permitted

## Create Visitor Screen

Should support foundation fields:

- first name
- last name
- phone
- email
- visit date
- source
- notes

## Visitor Conversion Action

Where backend is ready, support visitor conversion to member.

The UI should show clear confirmation before conversion.

## Congregation, Service, and Group Screens

MVP may include simple list and create foundations for:

- congregations
- services
- groups

Do not overbuild hierarchy management in Sprint 7.

## Attendance Screens

MVP should support foundations for:

- attendance session list
- create attendance session where backend is ready
- manual attendance marking where backend is ready

Do not implement QR, GPS, or offline attendance in Sprint 7.

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- retry where useful

## Final Rule

Religious Flutter screens should make the MVP usable while staying within platform boundaries and Sprint 7 scope.
