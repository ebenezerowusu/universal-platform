# Flutter Mobile App Specification

## Purpose

This document defines the initial mobile app scope for Universal Platform.

The mobile app should focus on end-user and lightweight operational experiences rather than heavy administration.

## Mobile App Users

Possible users:

- Members
- Visitors
- Customers
- Staff
- Leaders
- Volunteers
- Future marketplace users

## Mobile App Principle

Mobile should be simple, fast, role-aware, module-aware, and organization-aware.

The mobile app must not contain business rules that belong to the backend.

## Core Mobile Features

Initial shared features:

- Login
- Organization selector
- Profile
- Notifications
- Language preference
- Module-aware home screen
- Settings

## Religious Mobile Features

Possible Religious Domain mobile features:

- Member profile
- Family linking
- Announcements
- Events
- Event registration
- Giving/payment history
- Prayer request submission
- Counseling request submission
- Attendance self check-in later
- Group membership view
- Visitor self-registration later

## Leader Mobile Features

For authorized leaders:

- View assigned group
- Mark attendance
- View follow-up tasks
- Contact assigned visitors
- View event attendees where permitted
- Send limited messages where permitted

## Future Commerce Mobile Features

Possible future features:

- Browse marketplace listings
- Cart
- Orders
- Payment
- Notifications

## Navigation

Navigation must be driven by:

- Active organization
- Installed modules
- User permissions
- Subscription access
- Platform configuration

## Offline Direction

Mobile architecture should consider future offline support for:

- Attendance marking
- Visitor registration
- Basic profile lookup
- POS later if needed

Offline support can be phased after MVP.

## Security Rules

- Do not store provider secrets in the app.
- Do not rely on UI hiding for authorization.
- Protect stored tokens.
- Do not expose confidential records without backend permission checks.

## Final Rule

Mobile is for fast user and leader workflows. Heavy administration belongs mainly to desktop/admin experiences.
