# API Test Plan

## Purpose

This document defines the initial API test plan for Universal Platform.

API tests should confirm that routes follow documented contracts and return consistent responses.

## Principle

API tests should verify behavior from the client boundary.

They should complement application service and repository tests.

## Common Test Areas

Important API groups should test:

- success response shape
- standard error response shape
- request validation
- missing record behavior
- pagination behavior where applicable
- module availability behavior where applicable
- feature availability behavior where applicable

## Health API Tests

Test:

- health route returns success
- response format is standard

## Auth API Tests

Test:

- login returns expected response for valid input
- login returns safe response for invalid input
- current user route returns safe user summary when valid context exists

## Organization API Tests

Test:

- create organization
- list organizations for current user
- get organization detail
- update organization

## Module API Tests

Test:

- list modules
- install module
- activate module
- inactive module behavior

## Religious MVP API Tests

Test:

- create member
- list members
- get member detail
- update member
- create visitor
- convert visitor
- create congregation
- create service
- create group
- create attendance session
- mark attendance

## Engine API Tests

When implemented, test:

- create payment transaction
- list payment transactions
- get SMS wallet
- create SMS message request
- create file metadata
- request file access
- run report
- create workflow task

## Final Rule

API tests should prove documented API contracts from the outside.
