# API Error Catalog

## Purpose

This document defines common API error codes for Universal Platform.

Stable error codes help clients, developers, and AI agents handle failures consistently.

## Error Response Shape

Standard error response:

```json
{
  "success": false,
  "error": {
    "code": "PERMISSION_DENIED",
    "message": "You do not have permission to perform this action.",
    "details": {}
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Authentication Errors

### AUTHENTICATION_REQUIRED

The request requires authentication.

### INVALID_CREDENTIALS

The supplied login credentials are not valid.

### SESSION_EXPIRED

The session or token has expired.

### ACCOUNT_DISABLED

The user account is disabled or suspended.

## Tenant and Permission Errors

### TENANT_ACCESS_DENIED

The user does not have access to the requested organization.

### PERMISSION_DENIED

The user does not have the required permission.

### SCOPE_DENIED

The user has the permission but not for the requested scope or resource.

## Validation Errors

### VALIDATION_ERROR

The request payload or query parameters are invalid.

### INVALID_IDENTIFIER

The supplied identifier is malformed or not accepted.

### INVALID_STATE_TRANSITION

The requested state change is not allowed.

## Resource Errors

### RESOURCE_NOT_FOUND

The requested resource does not exist or is not visible to the user.

### RESOURCE_ALREADY_EXISTS

A conflicting resource already exists.

### RESOURCE_IN_USE

The resource cannot be changed because it is currently referenced by other records.

## Module and Subscription Errors

### MODULE_NOT_INSTALLED

The required module is not installed for the organization.

### MODULE_NOT_ACTIVE

The required module is installed but not active.

### FEATURE_NOT_AVAILABLE

The feature is not available for the organization's current configuration or plan.

### SUBSCRIPTION_LIMIT_REACHED

The organization has reached a plan limit.

### SUBSCRIPTION_INACTIVE

The organization subscription is inactive, expired, or restricted.

## Provider and Infrastructure Errors

### PROVIDER_ERROR

A configured external provider returned an error.

### PROVIDER_UNAVAILABLE

A provider is temporarily unavailable.

### STORAGE_ERROR

A file storage operation failed.

### DATABASE_ERROR

A database operation failed internally.

### QUEUE_ERROR

A background job or queue operation failed.

## Rate and Safety Errors

### RATE_LIMITED

The request exceeded allowed request limits.

### REQUEST_TOO_LARGE

The request payload or uploaded file is too large.

## Internal Errors

### INTERNAL_ERROR

An unexpected server-side error occurred.

Clients should show a safe generic message and include trace ID when reporting the issue.

## Final Rule

Error codes are part of the API contract. Do not change them casually once clients depend on them.
