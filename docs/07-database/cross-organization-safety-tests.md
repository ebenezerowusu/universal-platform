# Cross-Organization Safety Tests

## Purpose

This document defines safety tests for records that belong to organizations.

It complements the tenant isolation and organization boundary test documents.

## Principle

A feature should be tested using more than one organization when the feature stores organization-owned records.

## Test Setup

Create:

- two organizations
- one or more users for each organization
- sample records for each organization

Then verify that operations respect the selected organization context.

## Resource Areas

Apply this testing pattern to:

- organizations
- members
- visitors
- congregations
- services
- groups
- attendance
- giving
- files
- reports
- audit records

## List Checks

List operations should return records for the selected organization context.

## Detail Checks

Detail operations should use organization context and record id together.

## Write Checks

Create, update, and assignment operations should use organization context consistently.

## Report Checks

Reports should apply organization context before report-specific filters.

## File Checks

Private file metadata and access should use organization context.

## Final Rule

Cross-organization safety must be proven through tests before organization-owned features are considered complete.
