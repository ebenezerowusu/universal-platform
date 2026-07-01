# Identity and Organization Acceptance Criteria

## Purpose

This document defines completion criteria for Identity and Organization foundations.

These foundations must be reliable before Permission Engine, Module Registry, and domain features build on them.

## Identity Completion Criteria

Identity is complete for the foundation stage when:

- user table exists
- credential/session foundation exists
- user repository exists
- login use case exists
- current user endpoint exists
- authentication context can be extracted for protected routes
- safe user summary response exists

## Identity Quality Criteria

Identity implementation must:

- keep credential handling inside Identity layer
- avoid exposing credential internals through APIs
- avoid logging sensitive credential values
- return standard API responses
- return standard API errors
- include tests for valid and invalid login flows

## Organization Completion Criteria

Organization foundation is complete when:

- organizations table exists
- organization users table exists
- organization settings foundation exists
- organization repository exists
- create organization use case exists
- list current user organizations use case exists
- get organization detail use case exists
- organization membership check exists

## Organization Quality Criteria

Organization implementation must:

- keep organization logic in Core
- avoid domain-specific organization behavior
- support selected organization context
- support future multi-organization users
- use standard API responses
- include tests for organization creation and listing

## API Completion Criteria

The following route groups should exist at foundation level:

- login
- current user
- list organizations
- create organization
- get organization
- update organization foundation where planned

## Test Criteria

Tests should cover:

- login success
- login failure
- current user response
- organization creation
- organization membership creation
- organization listing for current user
- organization detail using selected context

## Out of Scope

Do not include at this stage:

- full role and permission system
- Religious Domain features
- payment provider integration
- SMS provider integration
- Flutter UI

## Final Rule

Identity proves who the user is. Organization proves where the user is operating. Both must work before authorization and domains are added.
