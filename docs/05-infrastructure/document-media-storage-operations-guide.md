# Document Media Storage Operations Guide

## Purpose

This document defines operating guidance for document and media records.

The goal is to keep file metadata, storage references, access rules, and domain attachments consistent across the platform.

## Principle

Document records should store metadata and access rules.

Storage Engine should own provider-specific file behavior.

Domain modules should link to document records instead of storing provider paths directly.

## Data Sources

Document/media workflows may use:

- document library
- folder
- document/media record
- storage reference
- upload intent placeholder
- domain attachment link
- access level
- audit records

## Storage Reference Direction

Storage references should be provider-neutral where possible.

Capture:

```text
document_id
storage_provider
storage_key
size_bytes optional
checksum optional
created_at
```

Do not expose raw provider credentials or private paths to clients.

## Domain Attachment Direction

Domain attachments should capture:

```text
document_id
domain_type
domain_record_id
organization_id
label
created_by
created_at
```

Domain attachments must not cross organization boundaries.

## Access Direction

Access checks should consider:

- organization membership
- document access level
- user permissions
- domain-specific permission where needed
- status of document

## Public Sharing Direction

Public sharing should remain a placeholder in MVP.

Do not expose unrestricted public links until policy, expiry, and audit rules are defined.

## Retention Direction

For MVP, prefer archive or deleted placeholder behavior over hard delete.

Actual storage cleanup should be planned explicitly.

## Audit Direction

Audit should record:

- document record created or updated
- storage reference attached
- domain attachment created or removed
- access level changed
- document archived or deleted placeholder
- view/download where policy requires it

## Data Quality Warnings

Warn or flag when:

- document has metadata but no storage reference
- storage reference exists without document metadata
- domain attachment points to missing domain record
- document is public placeholder without approval
- archived document is still linked to active workflow

## Final Rule

Document and media operations should protect access, preserve metadata, and avoid provider-specific file paths leaking into domain logic.
