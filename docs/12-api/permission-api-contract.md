# Permission API Contract

## Purpose

This document defines the MVP Permission API contract.

Permission answers:

```text
What can this user do in this organization?
```

## Owner

```text
Permission Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}
```

## List Permissions

```text
GET /api/v1/organizations/{organizationId}/permissions
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "key": "organization.settings.view",
        "name": "View organization settings",
        "description": "Allows viewing organization settings"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## List Roles

```text
GET /api/v1/organizations/{organizationId}/roles
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "...",
        "name": "Organization Administrator",
        "description": "...",
        "permissions": []
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Create Role

```text
POST /api/v1/organizations/{organizationId}/roles
```

### Request

```json
{
  "name": "Branch Leader",
  "description": "Can manage assigned branch workflows"
}
```

## Assign Permission To Role

```text
POST /api/v1/organizations/{organizationId}/roles/{roleId}/permissions
```

### Request

```json
{
  "permissionKeys": [
    "religious.members.view"
  ]
}
```

## Assign Role To Organization User

```text
POST /api/v1/organizations/{organizationId}/users/{organizationUserId}/roles
```

### Request

```json
{
  "roleIds": ["..."]
}
```

## Permission Check Internal Contract

Application services should use an internal service contract similar to:

```text
can(user_id, organization_id, permission_key, scope_context) -> allow/deny
```

API handlers should not manually inspect role permission data.

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- organization not found
- user is not part of organization
- permission denied
- role not found
- invalid permission key

## Sprint 3 Scope Note

Sprint 3 should implement the permission foundation.

Advanced scopes such as branch, group, subtree, and own records may be prepared but should not be overbuilt unless required by the acceptance criteria.

## Final Rule

Permission APIs manage roles and permission assignments. Application services must use Permission Engine checks instead of duplicating authorization logic.
