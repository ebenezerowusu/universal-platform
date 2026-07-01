# Production Readiness Checklist

## Purpose

This document defines the production readiness checklist for Universal Platform.

Production readiness means the platform is not only working, but also secure, observable, recoverable, and supportable.

## Application Readiness

Verify:

- Backend starts reliably.
- Health endpoint works.
- Configuration is environment-specific.
- Standard API response format is used.
- Standard error format is used.
- Trace IDs are available.

## Database Readiness

Verify:

- Migrations are applied.
- Database backups exist.
- Recovery path is documented.
- Indexes are reviewed for critical queries.
- Tenant isolation is tested.
- Seed data is safe and repeatable.

## Security Readiness

Verify:

- Authentication is enforced.
- Organization access is enforced.
- Permissions are enforced.
- Sensitive data access is controlled.
- Secrets are not committed.
- Error messages are safe.
- Rate limiting exists for high-risk routes.

## Operational Readiness

Verify:

- Logs are structured.
- Health checks exist.
- Important jobs are observable.
- Failed jobs can be inspected.
- Provider failures are visible.
- Deployment runbook exists.

## Backup and Recovery Readiness

Verify:

- Database backup strategy exists.
- File backup strategy exists if local storage is used.
- Restore process is documented.
- Restore has been tested before launch.

## Product Readiness

Verify:

- MVP scope is clear.
- Unsupported features are hidden or disabled.
- Module availability is enforced.
- Subscription checks work where needed.
- User-facing flows are tested.

## Support Readiness

Verify:

- Common errors are documented.
- Logs include trace IDs.
- Admins can identify organization context.
- Audit records exist for sensitive actions.

## Final Rule

Do not treat production launch as only deployment. Production launch requires security, observability, recovery, and support readiness.
