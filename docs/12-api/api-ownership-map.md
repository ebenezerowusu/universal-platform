# API Ownership Map

## Purpose

This document maps API route groups to their owning platform area.

It helps developers and coding agents place routes, services, tests, and documentation in the correct layer.

## Ownership Principle

Every API route group should have a clear owner.

A route owner is responsible for behavior, documentation, tests, and future changes.

## Platform Core Routes

### Auth Routes

Owner:

```text
Core / Identity
```

Examples:

```text
/api/v1/auth/login
/api/v1/auth/me
/api/v1/auth/logout
```

### Organization Routes

Owner:

```text
Core / Organization Engine
```

Examples:

```text
/api/v1/organizations
/api/v1/organizations/{organizationId}
/api/v1/organizations/{organizationId}/users
```

### Permission and Role Routes

Owner:

```text
Permission Engine
```

Examples:

```text
/api/v1/organizations/{organizationId}/roles
/api/v1/organizations/{organizationId}/permissions
```

### Module Routes

Owner:

```text
Module Registry Engine
```

Examples:

```text
/api/v1/organizations/{organizationId}/modules
```

### Subscription Routes

Owner:

```text
Subscription Engine
```

Examples:

```text
/api/v1/organizations/{organizationId}/subscription
/api/v1/platform/subscription-plans
```

## Capability Engine Routes

### Payment Routes

Owner:

```text
Payment Engine
```

### SMS Routes

Owner:

```text
SMS Engine
```

### Storage Routes

Owner:

```text
Storage Engine
```

### Notification Routes

Owner:

```text
Notification Engine
```

### Reporting Routes

Owner:

```text
Reporting Engine
```

### Workflow Routes

Owner:

```text
Workflow Engine
```

## Religious Domain Routes

Owner:

```text
Religious Domain
```

Examples:

```text
/api/v1/organizations/{organizationId}/religious/members
/api/v1/organizations/{organizationId}/religious/visitors
/api/v1/organizations/{organizationId}/religious/groups
/api/v1/organizations/{organizationId}/religious/attendance
```

## Ownership Rules

- API route owner owns the application service.
- Route owner must update API docs when behavior changes.
- Route owner must add tests for new behavior.
- Shared engines may be used by domains, but domains should not own engine routes.

## Final Rule

Every API route must have one clear owner. Shared behavior belongs in engines, not duplicated across route groups.
