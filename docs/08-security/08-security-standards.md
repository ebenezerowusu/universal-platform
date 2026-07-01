# Security Standards

## Purpose

Security is a foundation requirement for Universal Platform.

The platform will manage sensitive organizational, financial, member, staff, communication, and operational data. Security must be designed from the beginning, not added later.

## Security Principles

- Secure by default
- Least privilege
- Tenant isolation
- Defense in depth
- Audit sensitive actions
- Never trust the client
- Never expose provider secrets
- Validate all input
- Protect sensitive data

## Authentication

Authentication confirms who the user is.

The platform should support:

- Email/password authentication
- Secure password hashing
- Password reset
- Session/token management
- Future OAuth/OpenID support
- Future MFA support for sensitive roles

Passwords must never be stored in plain text.

## Authorization

Authorization confirms what the user can do.

All protected actions must use the Permission Engine.

Authorization must consider:

- User identity
- Organization context
- Role
- Permission
- Scope
- Installed modules
- Subscription limits
- Resource ownership

## Tenant Isolation

Tenant isolation is mandatory.

Every organization-owned resource must be protected so another organization cannot access it.

Tenant isolation must be enforced in:

- Application services
- Repository queries
- Permission checks
- API handlers
- Tests

Missing tenant filters are critical security bugs.

## JWT / Token Rules

If JWT is used:

- Use strong signing keys.
- Keep token expiry reasonable.
- Do not store sensitive secrets in token payload.
- Validate issuer/audience where applicable.
- Support token revocation strategy for high-risk cases.

## Secrets Management

Secrets must not be committed to git.

Examples of secrets:

- Database passwords
- JWT signing keys
- SMS provider API keys
- Payment provider API keys
- Email provider credentials
- Object storage credentials

Use environment variables or secure secret management.

## Provider Credentials

Provider credentials are platform-level infrastructure secrets.

Organizations must never see raw SMS/payment provider credentials.

Provider configuration UI should be restricted to platform administrators.

## Input Validation

All external input must be validated server-side.

Validation applies to:

- API request bodies
- Query parameters
- File uploads
- Webhook payloads
- Import files
- Search filters
- IDs and references

## Webhook Security

Payment and SMS webhooks must be verified.

Requirements:

- Verify signatures where providers support them.
- Validate provider references.
- Ensure idempotency.
- Reject duplicate event processing.
- Log suspicious callbacks.

## File Upload Security

File uploads must be controlled.

Requirements:

- Validate file type
- Validate file size
- Generate safe storage names
- Do not trust original filenames
- Prevent executable uploads where inappropriate
- Scan files in the future if needed
- Enforce tenant ownership

## Rate Limiting

Rate limiting should protect:

- Login
- Password reset
- SMS sending
- Payment initialization
- Public endpoints
- Import endpoints
- Search endpoints

## Audit Logs

Sensitive actions must be audited.

Examples:

- Login failures where useful
- Permission changes
- Role assignment
- Provider configuration changes
- Payment status changes
- SMS wallet changes
- Bulk SMS sending
- Financial report export
- Confidential counseling access
- Member deletion or transfer
- Branch connection/disconnection

Audit logs should include:

- Actor
- Organization
- Action
- Entity type
- Entity ID
- Timestamp
- IP address where available
- User agent where available
- Before/after values where appropriate

## Sensitive Data

Some data requires extra access controls.

Examples:

- Financial records
- Confidential counseling notes
- Prayer requests marked confidential
- Staff records
- Personal contact information
- Children/minor records
- Payment metadata

## Error Safety

API errors must not expose:

- Stack traces
- SQL queries
- Provider secrets
- Internal infrastructure paths
- Raw provider credentials

Use structured safe errors.

## Backups and Disaster Recovery

Backups must be protected.

Database backups may contain sensitive data and must not be publicly accessible.

Restore processes must be documented and tested.

## Security Testing

Security-related tests must cover:

- Tenant isolation
- Permission enforcement
- Scope enforcement
- Invalid token rejection
- Wrong organization access
- Webhook verification
- Provider secret protection
- File upload validation

## Final Rule

A feature is not complete until it is secure, tenant-aware, permission-aware, and auditable where necessary.
