# Sprint 35 Import Export Data Migration Review

## Purpose

This document defines review checks for Sprint 35.

Sprint 35 implements Import, Export, and Data Migration foundation.

## Review 1: Scope

Sprint 35 may include:

- import/export feature registration foundation
- import template metadata foundation
- export request foundation
- import job foundation
- validation result foundation
- field mapping placeholder where practical
- dry-run/preview placeholder where practical
- import execution placeholder where practical
- rollback note placeholder where practical
- import/export file reference through Document/Storage boundaries
- permission and feature checks
- audit records for import/export actions
- API and UI foundations

Sprint 35 should not include:

- unrestricted direct database imports
- automatic destructive migrations
- full ETL platform
- external data warehouse integration
- AI data cleaning
- cross-organization imports
- provider-specific file access inside domain modules
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Import/Export Foundation owns job orchestration and templates
- domain modules own supported schemas and validation rules
- Document Media Library owns file metadata
- Storage Engine owns provider-specific file behavior
- Permission Engine controls access decisions
- Platform Core does not contain domain-specific import mapping rules

## Review 3: Organization Boundaries

Confirm:

- import jobs are organization-scoped
- export requests are organization-scoped
- source and output document references are organization-scoped
- imports cannot write across organization boundaries
- exports cannot include records from another organization

## Review 4: Validation Rules

Confirm:

- imports validate before execution
- required columns are checked
- invalid values are flagged
- duplicate records are flagged where practical
- blocked errors prevent execution unless explicit override exists later

## Review 5: Export Rules

Confirm:

- exports require explicit permission
- source module access is checked
- source record access is checked where practical
- output document placeholder is protected
- sensitive exports are auditable where required

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/import-export-data-migration-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/import-export-data-migration-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- import template retrieval
- import job creation
- source document permission check
- validation preview
- blocked validation result
- execution blocked until validation
- export request permission
- organization boundary enforcement
- rollback note creation

## Final Rule

Sprint 35 is complete when organizations can submit import jobs, validate data, preview outcomes, request exports, and track job history through permission-protected and auditable workflows.
