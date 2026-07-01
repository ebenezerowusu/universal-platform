# Flutter UI/UX Standards

## Purpose

This document defines UI/UX standards for Universal Platform's Flutter applications.

The platform will use Flutter for mobile and desktop clients. The UI must support multi-tenant, multi-domain, module-aware, role-aware, country-aware, and language-aware behavior.

## Client Direction

Initial clients:

- Flutter Mobile App
- Flutter Desktop App

Future clients may include:

- Web portal
- Public admin portal
- Partner applications

## UI Principle

The UI must adapt to:

- Active organization
- Installed modules
- User permissions
- Subscription plan
- Platform type: mobile or desktop
- Country settings
- Language settings
- Organization branding

The UI must not contain core business logic.

## Shared Codebase Strategy

Use a shared Flutter codebase where practical.

However, mobile and desktop experiences must not be identical.

Mobile should prioritize:

- Member/customer self-service
- Notifications
- QR/GPS check-in
- Payments
- Profile
- Events
- Marketplace browsing later

Desktop should prioritize:

- Organization administration
- Dashboards
- Member/staff/product management
- Reports
- Finance/admin workflows
- POS later
- Bulk operations

## Module-Aware Navigation

Navigation should be generated or filtered using backend metadata.

Inputs:

- Installed modules
- Active modules
- User permissions
- Subscription access
- Platform type

Example:

A user without `religious.members.view` should not see the Members menu.

However, hiding the menu is not security. Backend permission checks remain mandatory.

## Role-Based UI

The UI should render based on permissions, not hardcoded roles.

Do not write UI conditions such as:

```text
if user is pastor
```

Prefer:

```text
if user has religious.members.view
```

Terminology such as Pastor, Imam, Leader, Member, Worshipper should come from configuration/localization.

## Domain-Aware UI

Domain features should be isolated.

Religious screens should not be mixed into Commerce UI unless the organization has both modules and the user has permission.

Recommended organization:

```text
features/
  core/
  auth/
  organizations/
  religious/
  commerce/
  pos/
  finance/
  hr/
  shared/
```

Exact folder structure may evolve, but feature boundaries must remain clear.

## State Management

Choose a consistent state management strategy before heavy UI development.

The chosen approach must support:

- Authentication state
- Organization switching
- Module metadata
- Permissions
- Offline-ready workflows later
- Form state
- API loading/error states

## Design System

Create a shared design system with:

- Colors
- Typography
- Spacing
- Buttons
- Inputs
- Cards
- Tables
- Dialogs
- Navigation components
- Loading states
- Empty states
- Error states
- Form validation patterns

Organization branding may customize approved theme values but should not break usability.

## Responsive Rules

The UI must adapt to:

- Small mobile screens
- Tablets
- Desktop layouts
- Wide dashboard screens

Desktop should use layouts like:

- Sidebar navigation
- Tables
- Split panes
- Dashboard grids
- Bulk action toolbars

Mobile should use:

- Bottom navigation where appropriate
- Simplified forms
- QR scanner flows
- Touch-friendly interactions
- Offline-friendly input where needed

## Localization

All user-facing strings must be localization-ready.

Do not hardcode English strings deep inside components.

Support future languages such as:

- English
- Twi
- French
- Swahili

User-generated content should not be auto-translated by default.

## Forms

Forms must support:

- Validation
- Required fields
- Server-side errors
- Loading states
- Draft state where useful
- Dynamic custom fields
- Accessibility labels

Dynamic forms should be driven by backend definitions when possible.

## Offline-Ready Direction

Some workflows should be designed with future offline support in mind.

Examples:

- Attendance marking
- Visitor registration
- Basic member lookup
- POS sales later

Offline support does not need to be in MVP, but UI architecture should not make it impossible.

## Error Handling

The UI should display safe, useful errors.

Do not show raw provider errors, stack traces, or SQL errors.

Use API error codes to show friendly messages.

## Accessibility

UI should consider:

- readable font sizes
- sufficient contrast
- button tap targets
- screen reader labels where practical
- keyboard navigation for desktop

## Security Rules

The UI must not store provider secrets.

The UI must not trust local permission checks as final authorization.

The UI must not expose hidden admin functionality through client-only checks.

## First UI Development Order

After backend foundations are stable:

1. Design system foundation
2. Authentication screens
3. Organization selector/context
4. Permission/module-aware navigation
5. Dashboard shell
6. Admin layout for desktop
7. Mobile shell
8. First Religious Domain screens after APIs are ready

## Final Rule

The UI is a client of the platform, not the owner of business rules.
