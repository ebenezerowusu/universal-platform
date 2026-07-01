# Module Registry API Contract

## Purpose

This document defines the MVP Module Registry API contract.

Module Registry answers:

```text
Which platform capabilities are available to this organization?
```

## Owner

```text
Module Registry Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/modules
```

## List Organization Modules

```text
GET /api/v1/organizations/{organizationId}/modules
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "key": "religious.members",
        "name": "Religious Members",
        "status": "active",
        "version": "1.0.0",
        "domain": "religious"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## List Module Catalog

```text
GET /api/v1/organizations/{organizationId}/modules/catalog
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "key": "religious.attendance",
        "name": "Religious Attendance",
        "description": "Attendance foundation for Religious Domain",
        "domain": "religious",
        "version": "1.0.0",
        "available": true
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Install Module

```text
POST /api/v1/organizations/{organizationId}/modules/{moduleKey}/install
```

### Success Response

```json
{
  "success": true,
  "data": {
    "module": {
      "key": "religious.members",
      "status": "installed"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Activate Module

```text
POST /api/v1/organizations/{organizationId}/modules/{moduleKey}/activate
```

## Deactivate Module

```text
POST /api/v1/organizations/{organizationId}/modules/{moduleKey}/deactivate
```

## Internal Availability Contract

Application services should use an internal contract similar to:

```text
is_module_available(organization_id, module_key) -> available/unavailable
```

Domain services must not manually inspect module tables.

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- organization not found
- permission denied
- module not found
- module dependency missing
- module already installed
- module inactive

## Audit Direction

Important module changes should be recorded through Audit Engine.

Examples:

- module installed
- module activated
- module deactivated

## Final Rule

Module Registry controls module availability. Domains should check module availability through Module Registry instead of creating their own module state logic.
