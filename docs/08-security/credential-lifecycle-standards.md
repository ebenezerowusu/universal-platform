# Credential Lifecycle Standards

## Purpose

This document defines credential lifecycle standards for Universal Platform.

The platform must handle login credentials, session records, recovery records, and access state carefully.

## Principle

Authentication data must identify users safely without exposing secret values or replacing authorization checks.

## Credential Records

Credential records should support:

- user ownership
- credential type
- safe verifier or reference
- status
- created time
- updated time
- disabled time where applicable

Do not expose credential verifier values through APIs.

## Login Handling

Login handling should:

- validate request shape
- check account state
- verify submitted credential using the approved server-side method
- create or refresh session state where applicable
- return only safe user/session data

Login failure messages should be safe and not reveal unnecessary account details.

## Session Handling

Session records should support:

- expiry
- revocation
- refresh behavior where selected
- future device tracking
- future suspicious activity review

Access decisions must still be made on the server using organization, module, subscription, and permission checks.

## Recovery Handling

Account recovery records should be:

- time-limited
- single-use where practical
- stored safely
- audited where appropriate

Recovery responses should avoid exposing unnecessary account information.

## Configuration

Runtime signing or verification secrets must come from environment or secret configuration.

They must not be committed to the repository.

## Logging Rules

Logs should never include:

- submitted credential values
- secret verifier values
- raw recovery tokens
- sensitive session material

## Testing Requirements

Tests should cover:

- successful login
- failed login
- inactive account behavior
- expired session behavior
- revoked session behavior
- protected route without valid authentication
- safe error response

## Final Rule

Credentials and sessions identify the user. Authorization still requires organization, module, subscription, and permission checks.
