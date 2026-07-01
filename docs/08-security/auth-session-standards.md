# Auth Session Standards

## Purpose

This document defines authentication session standards for Universal Platform.

Authentication must identify the user safely and must remain separate from authorization.

## Login Principles

- Do not store raw secret credentials.
- Do not write secret credential values to logs.
- Return safe error messages.
- Keep account status checks server-side.
- Keep organization access checks separate from login.

## Session Principles

- Session access should expire.
- Long-lived access should be revocable.
- Runtime secrets must come from configuration.
- Session records should support future device tracking.
- Authorization must use current server-side rules.

## Auth Payload Direction

Authentication payloads should include only what is required to identify the user and session.

Avoid placing full authorization rules into long-lived client payloads.

Permissions, modules, and subscriptions can change and should be checked server-side.

## Logout Direction

Logout should invalidate or revoke the relevant server-side session reference where supported.

## Account Recovery Direction

Account recovery should use short-lived records and safe user-facing messages.

Do not expose unnecessary account details through recovery responses.

## Testing Requirements

Tests should cover:

- Login success
- Login failure
- Disabled account behavior
- Session expiry
- Session revocation
- Protected route without authentication
- Protected route with valid authentication

## Final Rule

Authentication proves identity. Organization access and permissions must still be verified separately.
