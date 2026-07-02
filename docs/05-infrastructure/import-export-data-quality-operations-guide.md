# Import Export Data Quality Operations Guide

## Purpose

This document defines operating guidance for import jobs, export requests, validation results, file references, and rollback notes.

The goal is to keep bulk data movement safe, explainable, and permission-protected.

## Principle

Imports should validate before execution.

Exports should check permissions before producing files.

No domain should accept arbitrary bulk data without a supported template and validation path.

## Data Sources

Import/export workflows may use:

- import template
- import job
- source document reference
- validation result
- field mapping placeholder
- dry-run preview summary
- execution summary
- rollback note placeholder
- export request
- output document reference placeholder
- audit records

## Template Direction

Templates should define:

```text
template_key
domain_module
expected_columns
required_columns
supported_file_types
required_permission
status
```

## Validation Direction

Validation should check:

- required columns
- supported values
- duplicate records
- missing references
- row-level permission issues
- source document access
- organization ownership

## Dry-Run Direction

Dry-run preview should show:

```text
total_rows
valid_rows
warning_rows
error_rows
blocked_rows
sample_messages
```

Execution should not proceed when blocked errors exist unless an explicitly permitted override is implemented later.

## Export Direction

Exports should check:

- export permission
- source module access
- source record access
- filter validity
- document/output permissions

## File Boundary Direction

Import source files and export outputs should reference Document Media Library records.

Storage Engine owns provider-specific file behavior.

Do not store provider-specific file paths inside domain import logic.

## Rollback Note Direction

Rollback notes should capture:

```text
job_id
created_by
created_at
summary
affected_records_summary optional
manual_action_required optional
```

Do not promise automatic destructive rollback in MVP.

## Audit Direction

Audit should record:

- import job submitted
- validation completed
- import execution requested
- import cancelled
- rollback note added
- export requested
- export cancelled
- template changed

## Data Quality Warning Types

Suggested warning types:

```text
missing_required_column
invalid_value
duplicate_record
reference_not_found
permission_limited_row
source_document_unavailable
manual_review
```

## Final Rule

Bulk data movement must be traceable from source file to validation results to execution or export output.
