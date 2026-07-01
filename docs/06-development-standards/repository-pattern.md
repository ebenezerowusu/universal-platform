# Repository Pattern Guide

## Purpose

This document defines the repository pattern for Universal Platform.

Repositories provide data access boundaries and help keep database details out of application services and domain logic.

## Principle

Application services should depend on repository contracts, not scattered database queries.

## Repository Responsibilities

Repositories may:

- read records
- write records
- apply tenant filters
- support pagination
- support safe filtering and sorting
- map database rows to application models

Repositories must not:

- own business workflow decisions
- check high-level permissions by themselves
- call external providers
- publish domain events
- decide subscription access

## Tenant Filtering

Organization-owned repositories must require organization context for organization-owned data.

Example principle:

```text
find_member_by_id(organization_id, member_id)
```

Avoid repository methods that can accidentally fetch organization-owned records without organization scope.

## Pagination

List repository methods should support pagination and must avoid unbounded queries.

## Transactions

Repositories should support being used inside transactions where needed.

Application services usually decide transaction boundaries.

## Naming Direction

Repository names should match aggregate or resource areas.

Examples:

```text
UserRepository
OrganizationRepository
PermissionRepository
ModuleRepository
ReligiousMemberRepository
AttendanceRepository
```

## Testing

Repository tests should cover:

- create record
- fetch record by id
- organization boundary behavior
- pagination
- filtering
- uniqueness constraints
- soft delete behavior where applicable

## Final Rule

Repositories protect data access consistency. They must be tenant-aware and predictable.
