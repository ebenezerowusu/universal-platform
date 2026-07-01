# Scoped Query Test Plan

## Purpose

This document defines tests for scoped database queries in Universal Platform.

Most records in the platform belong to an organization context. Queries must consistently apply that context.

## Principle

Any feature that reads or writes organization records should include tests for selected organization context.

## Common Test Setup

Use this setup for scoped query tests:

1. Create two sample organizations.
2. Create sample users.
3. Create sample records for each organization.
4. Run list and detail queries for each context.
5. Verify only matching records are returned.

## General Query Tests

Required checks:

- List query returns records for selected context.
- Detail query returns record only for selected context.
- Search query uses selected context.
- Update action uses selected context.
- Report query uses selected context.

## Member Query Tests

Required checks:

- Member list uses selected context.
- Member detail uses selected context.
- Member search uses selected context.
- Member update uses selected context.

## Visitor Query Tests

Required checks:

- Visitor list uses selected context.
- Visitor detail uses selected context.
- Visitor follow-up uses selected context.
- Visitor conversion uses selected context.

## Attendance Query Tests

Required checks:

- Attendance session uses selected context.
- Attendance record uses matching session context.
- Attendance report uses selected context.

## Structure Query Tests

Required checks:

- Congregation query uses selected context.
- Service query uses selected context.
- Group query uses selected context.
- Group member assignment uses matching context.

## Giving Query Tests

Required checks:

- Giving query uses selected context.
- Giving report uses selected context.
- Payment reference connection uses selected context.

## File Query Tests

Required checks:

- File metadata query uses selected context.
- Private file access uses selected context.

## Audit Query Tests

Required checks:

- Audit query uses selected context.
- Platform-level audit access uses platform-level permission.

## Standard Error Direction

When selected context is not valid for the user or record, return a standard safe error response.

Possible errors:

- TENANT_ACCESS_DENIED
- RESOURCE_NOT_FOUND
- PERMISSION_DENIED

## Final Rule

Scoped query tests are required for organization-owned records.
