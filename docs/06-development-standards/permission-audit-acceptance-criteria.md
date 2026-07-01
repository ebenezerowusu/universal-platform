# Permission and Audit Acceptance Criteria

## Purpose

This document defines completion criteria for Permission Engine and Audit Engine foundations.

These foundations must be reliable before module-protected and domain-protected features are built.

## Permission Completion Criteria

Permission foundation is complete when:

- permissions table exists
- roles table exists
- role permissions table exists
- organization user roles table exists
- initial permission seed exists
- permission check service exists
- selected use cases call permission checks

## Permission Quality Criteria

Permission implementation must:

- keep authorization logic server-side
- support organization-specific roles
- support stable permission keys
- return consistent allow/deny decisions
- prepare for future scope checks
- include allow and deny tests

## Audit Completion Criteria

Audit foundation is complete when:

- audit_logs table exists
- audit service exists
- application services can write audit records
- actor context can be recorded
- organization context can be recorded
- entity type and id can be recorded

## Audit Quality Criteria

Audit implementation must:

- avoid storing unnecessary sensitive values
- support important action history
- include trace or correlation metadata where available
- be reusable by Core, Engines, and Domains
- include tests for audit writes

## API Integration Criteria

Selected routes or use cases should demonstrate:

- action allowed when required permission exists
- action not allowed when required permission is missing
- audit record written for important change

## Test Criteria

Tests should cover:

- permission seed repeatability
- role assignment
- permission check success
- permission check failure
- audit write success
- audit metadata shape

## Out of Scope

Do not include at this stage:

- full branch/group scope logic
- full module registry
- full subscription checks
- Religious Domain features
- provider integrations

## Final Rule

Permissions decide what users can do. Audit records what important actions happened. Both are required before serious domain work.
