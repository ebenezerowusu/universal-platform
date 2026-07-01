# Storage Engine Specification

## Purpose

The Storage Engine provides one platform-wide way to store, retrieve, secure, and manage files across all domains.

Domains must not write directly to local disk, S3, Cloudflare R2, Azure Blob Storage, or any other provider.

## Engine Position

```text
Domain Module
  -> Storage Engine
    -> Storage Provider Interface
      -> Storage Adapter
        -> Local Storage / Object Storage Provider
```

## Responsibilities

The Storage Engine is responsible for:

- File upload handling
- File metadata storage
- File ownership tracking
- Tenant-aware access control
- Storage provider abstraction
- File download URLs
- File visibility rules
- File validation rules
- Storage quotas
- Storage audit support
- Future migration from local storage to object storage

## Non-Responsibilities

The Storage Engine does not decide domain-specific meaning of a file.

Examples:

- A member profile photo belongs to the Religious Domain.
- A product image belongs to the Commerce/POS Domain.
- A staff document belongs to the HR Domain.

The Storage Engine stores and protects the file. The domain decides why the file exists.

## Supported Storage Providers

Initial and future adapters:

- Local storage adapter for MVP
- S3-compatible storage adapter
- Cloudflare R2 adapter
- Azure Blob Storage adapter
- Future providers

## File Ownership

Every file should track ownership context.

Candidate fields:

- id
- organization_id nullable for platform-level files
- owner_type
- owner_id
- uploaded_by
- storage_provider
- storage_key
- original_filename
- safe_filename
- mime_type
- size_bytes
- visibility
- metadata JSONB nullable
- created_at
- updated_at
- deleted_at nullable

## Visibility Levels

Possible visibility values:

- private
- organization
- public
- temporary

Default should be private unless the domain explicitly requests otherwise.

## Upload Flow

```text
Domain requests upload
  -> Storage Engine validates file rules
  -> Storage Engine resolves provider
  -> Adapter stores file
  -> Metadata saved in PostgreSQL
  -> FileUploaded event published
  -> Audit log recorded if sensitive
```

## Access Flow

```text
Client requests file
  -> Authenticate user where required
  -> Resolve organization context
  -> Check file ownership and visibility
  -> Generate access response or temporary URL
```

## Validation Rules

The Storage Engine should support configurable validation:

- Allowed file types
- Maximum file size
- Allowed visibility
- Domain-specific categories
- Organization storage quotas

## Storage Migration

The MVP may start with local storage.

Later, files may move to object storage.

This move must not change domain logic.

Only the Storage Engine adapter/configuration should change.

## Events Published

- FileUploaded
- FileDeleted
- FileMetadataUpdated
- FileAccessed optional for sensitive records
- StorageQuotaReached

## Security Requirements

- Validate file size and type.
- Do not trust original filenames.
- Enforce tenant ownership.
- Do not expose private file paths directly.
- Sensitive files require permission checks.
- File access for confidential records should be auditable.

## Tenant Isolation

Organization-owned files must be scoped to the organization.

A user from Organization A must not access private files from Organization B.

## Testing Requirements

Tests must cover:

- Successful upload
- Invalid file type rejection
- Oversized file rejection
- Private file access allowed to authorized user
- Private file access denied to wrong organization
- Provider adapter mock upload
- Storage quota check
- Metadata persistence

## Final Rule

Domains own file meaning. The Storage Engine owns file handling, access control, and provider abstraction.
