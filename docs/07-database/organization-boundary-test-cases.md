# Organization Boundary Test Cases

## Purpose

This document defines test cases for organization boundary behavior.

The platform must consistently apply the selected organization context when reading, writing, listing, reporting, and exporting records.

## Principle

Organization-owned data should always be handled with organization context.

## Base Scenario

Use two organizations in tests:

- Organization A
- Organization B

Create sample users and records for each organization.

Then verify that operations return only data for the selected organization context.

## Required Checks

### Lists

Verify list endpoints return only records for the selected organization context.

Examples:

- members
- visitors
- groups
- services
- attendance sessions
- giving records

### Details

Verify detail endpoints use both organization context and record id.

### Updates

Verify update actions use selected organization context.

### Reports

Verify report filters apply selected organization context before other filters.

### Assignments

Verify assignments use matching organization context.

Examples:

- member to group
- visitor follow-up
- leader assignment
- attendance record to session

### Files

Verify file metadata and file access use selected organization context.

### Audit

Verify organization activity records are queried using selected organization context.

## Response Direction

If the selected organization context does not apply, return a safe platform response.

The exact response code should follow API standards for the endpoint.

## Final Rule

Organization boundary tests must exist for every organization-owned feature.
