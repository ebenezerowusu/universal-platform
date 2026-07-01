# ADR-006: Use Flutter for Shared Mobile and Desktop Clients

## Status

Accepted

## Context

Universal Platform needs mobile and desktop clients.

Mobile users may include members, customers, staff, riders, and end users.

Desktop users may include organization administrators, platform administrators, finance officers, HR officers, POS operators, and managers.

A shared technology stack can reduce development effort, but the user experience must still be adapted for each platform.

## Decision

Use Flutter as the primary frontend technology for mobile and desktop clients.

The Flutter codebase should share design system, API clients, authentication flow, localization, and reusable components where practical.

Mobile and desktop layouts should be adapted to their use cases.

## Consequences

### Positive

- Shared codebase and faster development.
- Consistent design system.
- Easier localization strategy.
- Shared API integration layer.
- Mobile and desktop can evolve together.

### Cost

- Requires discipline to avoid mixing admin-heavy desktop code into mobile flows.
- Requires responsive layouts.
- Requires module-aware navigation.
- Some platform-specific functionality may need separate implementations.

## Implementation Rules

- Do not put business logic in Flutter UI.
- Use backend permissions to control real access.
- Use module metadata for navigation.
- Use responsive layouts.
- Keep feature modules separated.
- Support localization from the beginning.

## Final Rule

Flutter is the client technology, not the business architecture. The backend APIs and platform contracts remain the source of truth.
