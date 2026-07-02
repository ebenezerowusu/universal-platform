# Authentication Session Device Security API Contract

## Purpose

This document defines the MVP API contract for Authentication Session, Device, and Security Control foundation.

The API should support security dashboard summaries, sessions, devices, revocation actions, MFA placeholders, security events, suspicious access review placeholders, and token/access key lifecycle placeholders while preserving user, organization, and permission boundaries.

## Owner

```text
Identity Foundation
Session Security Foundation
Permission Engine
Audit Engine
Notification Engine where security notifications are used
```

## Base Path

```text
/api/v1/security
```

Organization-admin views should use organization-scoped routes where appropriate:

```text
/api/v1/organizations/{organizationId}/security
```

## Security Dashboard

```text
GET /api/v1/security/dashboard-summary
GET /api/v1/organizations/{organizationId}/security/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "activeSessions": 3,
    "knownDevices": 2,
    "recentSecurityEvents": 5,
    "mfaStatus": "not_enabled_placeholder",
    "suspiciousAccessReviews": 1
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Sessions

```text
GET /api/v1/security/sessions
GET /api/v1/security/sessions/{sessionId}
POST /api/v1/security/sessions/{sessionId}/revoke
POST /api/v1/security/sessions/revoke-all-other
```

Organization-admin placeholder routes:

```text
GET /api/v1/organizations/{organizationId}/security/sessions
POST /api/v1/organizations/{organizationId}/security/sessions/{sessionId}/revoke-placeholder
```

## Devices

```text
GET /api/v1/security/devices
GET /api/v1/security/devices/{deviceId}
PATCH /api/v1/security/devices/{deviceId}
POST /api/v1/security/devices/{deviceId}/forget-placeholder
```

Organization-admin placeholder route:

```text
GET /api/v1/organizations/{organizationId}/security/devices
```

## MFA Readiness Placeholder

```text
GET /api/v1/security/mfa-readiness
POST /api/v1/security/mfa-readiness/enable-placeholder
POST /api/v1/security/mfa-readiness/disable-placeholder
```

Organization policy placeholder:

```text
GET /api/v1/organizations/{organizationId}/security/mfa-policy-placeholder
PATCH /api/v1/organizations/{organizationId}/security/mfa-policy-placeholder
```

## Security Events

```text
GET /api/v1/security/events
GET /api/v1/security/events/{eventId}
GET /api/v1/organizations/{organizationId}/security/events
GET /api/v1/organizations/{organizationId}/security/events/{eventId}
```

### Query Parameters

```text
startDate
endDate
eventType
severity
userId
deviceId
status
page
perPage
```

## Suspicious Access Review Placeholder

```text
GET /api/v1/organizations/{organizationId}/security/suspicious-access-reviews
POST /api/v1/organizations/{organizationId}/security/suspicious-access-reviews
GET /api/v1/organizations/{organizationId}/security/suspicious-access-reviews/{reviewId}
POST /api/v1/organizations/{organizationId}/security/suspicious-access-reviews/{reviewId}/resolve-placeholder
```

## Token and Access Key Placeholder

```text
GET /api/v1/security/access-keys-placeholder
POST /api/v1/security/access-keys-placeholder
POST /api/v1/security/access-keys-placeholder/{keyId}/rotate-placeholder
POST /api/v1/security/access-keys-placeholder/{keyId}/revoke-placeholder
```

Do not expose raw secrets after creation placeholder.

## Required Checks

Routes should check:

- authenticated user
- user owns personal session/device record for personal routes
- organization membership for organization-scoped routes
- security feature availability
- required permission for organization/admin security views
- target user belongs to same organization where applicable

Security records must not cross organization boundaries.

## Status Direction

Suggested session statuses:

```text
active
revoked
expired_placeholder
suspicious_placeholder
manual_review
```

Suggested device statuses:

```text
known_placeholder
trusted_placeholder
untrusted_placeholder
forgotten_placeholder
manual_review
```

Suggested security event types:

```text
login_success
login_failure_placeholder
session_revoked
new_device_detected_placeholder
password_changed_placeholder
mfa_enabled_placeholder
mfa_disabled_placeholder
privileged_access_event
suspicious_access_placeholder
token_created_placeholder
token_revoked_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- session revoked
- all sessions revoked
- organization-admin session revoke placeholder
- device trust changed placeholder
- MFA setting changed placeholder
- suspicious access review resolved placeholder
- token/access key created placeholder
- token/access key revoked placeholder

## Error Direction

Use standard response shape.

Expected error areas:

- session not found
- device not found
- security event not found
- suspicious access review not found
- MFA placeholder unavailable
- access key placeholder not found
- security permission denied
- cross-organization access denied

## Final Rule

Session and device security APIs must be user-safe, organization-aware, permission-protected, provider-agnostic, and auditable.
