# Password and Token Standards

## Purpose

This document records the platform direction for password-based login and session token handling.

It complements `credential-lifecycle-standards.md` and keeps the original documentation topic available in the security section.

## Core Direction

Password and token handling must be implemented only inside the Identity foundation.

Other domains should not create their own login, password, or token systems.

## Password Handling Direction

Implementation should follow these principles:

- Do not store plain passwords.
- Do not return password-related internal values through APIs.
- Do not write password values to logs.
- Validate login input before processing.
- Keep account status checks on the server.
- Return safe and consistent login responses.

## Token Handling Direction

Implementation should follow these principles:

- Use short-lived access where practical.
- Keep longer-lived session material revocable.
- Store only safe token references where server-side tracking is required.
- Do not place full authorization rules inside client-held token data.
- Refresh permission and organization access from server-side state where needed.

## Separation of Responsibility

Authentication answers:

```text
Who is the user?
```

Authorization answers:

```text
What can the user do in this organization?
```

These must remain separate.

## Runtime Configuration

Token-related runtime secrets must come from environment or secret configuration.

They must not be committed to the repository.

## Testing Direction

Implementation should include tests for:

- valid login flow
- invalid login flow
- inactive account handling
- expired session handling
- revoked session handling
- protected route without a valid session context

## Final Rule

Passwords and tokens are Identity concerns. They must not leak into domains, UI rules, provider adapters, or organization business logic.
