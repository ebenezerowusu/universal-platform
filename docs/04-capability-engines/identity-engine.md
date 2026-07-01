# Identity Engine Specification

## Purpose

The Identity Engine manages user identity, authentication, credential handling, sessions or tokens, and account lifecycle across Universal Platform.

Identity is platform-level. A user may belong to many organizations.

## Responsibilities

The Identity Engine is responsible for:

- User accounts
- Authentication
- Credential verification
- Password hashing
- Token or session creation
- Password reset foundation
- Account status
- Login history where useful
- Future multi-factor authentication
- Future OAuth/OpenID integration

## Non-Responsibilities

The Identity Engine does not decide what an authenticated user can do inside an organization.

Authorization belongs to the Permission Engine.

The Identity Engine answers: who is this user?

The Permission Engine answers: what can this user do here?

## Core Concepts

### User

A global platform identity.

A user can belong to one or many organizations.

### Credential

Authentication method for a user.

Initial credential type:

- email and password

Future credential types:

- phone login
- OAuth provider
- passkey
- SSO

### Session or Token

Represents authenticated access from a client.

The exact token/session strategy should be finalized before implementation.

## Suggested Entities

Candidate entities:

- users
- user_credentials
- user_sessions or refresh_tokens
- password_reset_tokens
- login_attempts optional
- user_security_events optional

## User Fields

Candidate fields:

- id
- display_name
- email
- phone nullable
- status
- email_verified_at nullable
- phone_verified_at nullable
- created_at
- updated_at
- deleted_at nullable

Possible statuses:

- active
- pending_verification
- suspended
- disabled

## Authentication Flow

```text
User submits credentials
  -> Identity Engine validates input
  -> Credential is verified
  -> User status is checked
  -> Token/session is issued
  -> Login event recorded where needed
```

## Organization Context

Authentication does not automatically select organization authority.

After login, the client may need to select an organization if the user belongs to multiple organizations.

Organization access is resolved by the Organization Engine and Permission Engine.

## Password Rules

Passwords must be hashed using an approved password hashing algorithm.

Plain-text passwords must never be stored or logged.

Password reset tokens must expire.

## Token Rules

Tokens must not contain sensitive secrets.

Token payloads should contain only necessary identity and session information.

Organization-level authorization should be checked server-side, not trusted only from token claims.

## Events Published

- UserCreated
- UserUpdated
- UserLoggedIn
- UserLoggedOut
- PasswordResetRequested
- PasswordChanged
- UserSuspended
- UserReactivated

## Security Requirements

- Protect passwords.
- Rate-limit login attempts.
- Do not reveal whether an email exists during password reset where possible.
- Support secure token expiry.
- Audit sensitive identity events where appropriate.
- Never trust the client for final authorization.

## Testing Requirements

Tests must cover:

- User creation
- Password hashing
- Login success
- Login failure
- Suspended user cannot login
- Token/session creation
- Password reset token expiry
- Multi-organization user flow foundation

## Final Rule

Identity is global. Organization access is contextual. Permissions are enforced separately.
