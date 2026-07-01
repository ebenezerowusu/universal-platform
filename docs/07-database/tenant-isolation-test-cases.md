# Tenant Isolation Test Cases

## Purpose

This document records test cases for multi-organization data separation.

Universal Platform serves many organizations. Features must prove that records are read and written using the correct organization context.

## Test Principle

Any organization-owned feature should be tested with at least two organizations.

## Common Setup

Recommended setup:

1. Create Organization A.
2. Create Organization B.
3. Create User A for Organization A.
4. Create User B for Organization B.
5. Create sample records for both organizations.
6. Run the same operation from each organization context.
7. Confirm each result matches the selected organization context.

## Organization Tests

Test:

- user can list organizations they belong to
- organization detail uses selected organization context
- organization update uses selected organization context

## Member Tests

Test:

- member list uses selected organization context
- member detail uses selected organization context
- member search uses selected organization context
- member update uses selected organization context

## Visitor Tests

Test:

- visitor list uses selected organization context
- visitor follow-up uses selected organization context
- visitor conversion uses selected organization context

## Attendance Tests

Test:

- attendance session uses selected organization context
- attendance records belong to the selected session context
- attendance reports use selected organization context

## Structure Tests

Test:

- congregation list uses selected organization context
- service list uses selected organization context
- group list uses selected organization context
- group membership uses matching organization context

## Giving Tests

Test:

- giving list uses selected organization context
- giving reports use selected organization context
- payment references are linked using selected organization context

## File Tests

Test:

- file metadata uses selected organization context
- private file access uses selected organization context

## Audit Tests

Test:

- organization audit query uses selected organization context
- platform-level audit review uses platform-level access rules

## Expected Result Direction

When the context does not match, the API should return a safe standard response.

The exact error behavior should be documented per endpoint.

## Final Rule

Every organization-owned feature must prove tenant isolation before production use.
