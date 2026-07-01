# Import/Export Engine Specification

## Purpose

The Import/Export Engine provides reusable file import, validation, preview, duplicate detection, export, and background processing capabilities across domains.

It prevents each domain from building separate CSV/Excel handling logic.

## Responsibilities

The Import/Export Engine is responsible for:

- CSV import foundation
- Excel import foundation
- Column mapping
- Validation
- Preview before commit
- Duplicate detection support
- Import job tracking
- Export job tracking
- Error reports
- Background processing for large jobs
- Audit support for sensitive imports/exports

## Non-Responsibilities

The engine does not decide domain-specific meaning of imported rows.

Examples:

- Religious Domain defines member import rules.
- POS Domain defines product import rules.
- HR Domain defines staff import rules.

The engine handles the import/export process and delegates domain validation rules to the owning domain.

## Example Use Cases

Religious Domain:

- Import members
- Import visitors
- Import family relationship hints
- Export attendance reports
- Export giving reports

POS Domain:

- Import products
- Import inventory
- Export sales reports

HR Domain:

- Import staff
- Export leave balances

Finance Domain:

- Import chart of accounts
- Export financial reports

## Import Flow

```text
User uploads file
  -> Storage Engine stores file
  -> Import/Export Engine creates import job
  -> Columns are detected
  -> User maps columns
  -> Preview generated
  -> Validation errors shown
  -> User confirms import
  -> Background job processes rows
  -> ImportCompleted event published
```

## Export Flow

```text
User requests export
  -> Permission checked
  -> Export job created
  -> Report/query runs
  -> File generated
  -> Storage Engine stores output
  -> ExportCompleted event published
```

## Suggested Entities

Candidate entities:

- import_jobs
- import_job_rows
- import_mappings
- export_jobs
- export_files
- import_templates

## Import Job Statuses

Possible statuses:

- uploaded
- mapping_required
- preview_ready
- ready_to_process
- processing
- completed
- completed_with_errors
- failed
- cancelled

## Export Job Statuses

Possible statuses:

- queued
- processing
- completed
- failed
- expired

## Duplicate Detection

The engine should support duplicate detection hooks.

Examples:

- Member duplicate by phone, email, name plus date of birth
- Product duplicate by SKU or barcode
- Staff duplicate by email or staff number

Domains define duplicate rules.

The engine coordinates preview and reporting.

## Error Reporting

Import errors should be clear and row-specific.

Example fields:

- row_number
- column
- error_code
- message
- value

## Security Requirements

- Import requires permission.
- Export requires permission.
- Sensitive exports should be audited.
- Uploaded import files must use Storage Engine.
- Export files should expire where appropriate.
- Tenant context must be enforced.

## Tenant Isolation

Import and export jobs must belong to an organization unless platform-level.

Users must not view or download another organization's import/export files.

## Events Published

- ImportUploaded
- ImportPreviewGenerated
- ImportStarted
- ImportCompleted
- ImportFailed
- ExportStarted
- ExportCompleted
- ExportFailed

## Testing Requirements

Tests must cover:

- File upload creates import job
- Column mapping works
- Validation errors are reported
- Preview does not commit data
- Confirmed import commits data
- Export enforces permission
- Tenant isolation
- Sensitive export audit

## Final Rule

Domains define data meaning. The Import/Export Engine owns the import/export process.
