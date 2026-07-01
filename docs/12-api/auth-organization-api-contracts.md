# Auth and Organization API Contracts

## Purpose

This document defines initial API contracts for authentication and organization context.

These contracts are planning-level and should be refined during implementation and OpenAPI generation.

## Response Standard

All responses must follow the platform response standards.

Success:

```json
{
  "success": true,
  "data": {},
  "meta": {
    "traceId": "..."
  }
}
```

Error:

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "The request is invalid.",
    "details": {}
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Login

```text
POST /api/v1/auth/login
```

Request:

```json
{
  "identifier": "user@example.com",
  "password": "password"
}
```

Response data:

```json
{
  "accessToken": "...",
  "refreshToken": "...",
  "user": {
    "id": "...",
    "displayName": "...",
    "email": "user@example.com"
  }
}
```

Possible errors:

- INVALID_CREDENTIALS
- ACCOUNT_DISABLED
- VALIDATION_ERROR

## Current User

```text
GET /api/v1/auth/me
```

Response data:

```json
{
  "user": {
    "id": "...",
    "displayName": "...",
    "email": "..."
  },
  "organizations": []
}
```

Possible errors:

- AUTHENTICATION_REQUIRED
- SESSION_EXPIRED

## List User Organizations

```text
GET /api/v1/organizations
```

Response data:

```json
{
  "items": [
    {
      "id": "...",
      "name": "Example Organization",
      "slug": "example-organization",
      "status": "active",
      "roleSummary": []
    }
  ]
}
```

## Create Organization

```text
POST /api/v1/organizations
```

Request:

```json
{
  "name": "Example Organization",
  "type": "religious",
  "country": "GH",
  "currency": "GHS",
  "timezone": "Africa/Accra",
  "defaultLanguage": "en"
}
```

Response data:

```json
{
  "organization": {
    "id": "...",
    "name": "Example Organization",
    "status": "active"
  }
}
```

Possible errors:

- VALIDATION_ERROR
- RESOURCE_ALREADY_EXISTS

## Get Organization

```text
GET /api/v1/organizations/{organizationId}
```

Possible errors:

- AUTHENTICATION_REQUIRED
- TENANT_ACCESS_DENIED
- RESOURCE_NOT_FOUND

## Update Organization

```text
PATCH /api/v1/organizations/{organizationId}
```

Request example:

```json
{
  "name": "Updated Organization Name",
  "timezone": "Africa/Accra",
  "defaultLanguage": "en"
}
```

Possible errors:

- AUTHENTICATION_REQUIRED
- TENANT_ACCESS_DENIED
- PERMISSION_DENIED
- VALIDATION_ERROR

## Organization Context Rule

Any route with `{organizationId}` must verify:

1. User is authenticated.
2. User belongs to the organization.
3. User has required permission for the operation.

## Final Rule

Authentication proves identity. Organization routes must still prove organization access and permissions.
