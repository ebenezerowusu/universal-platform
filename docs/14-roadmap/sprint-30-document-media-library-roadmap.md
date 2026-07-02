# Sprint 30 Document Media Library Roadmap

## Purpose

This document defines Sprint 30 for Document and Media Library foundation.

Sprint 30 should create a reusable layer for document metadata, media references, access rules, domain attachments, and storage-provider abstraction.

## Sprint Goal

Enable organizations to create document/media records, organize them, link them to domain records, control access, and audit file-related actions without letting every domain implement storage logic independently.

## Why This Sprint

Many modules now need file and media handling:

- HR references
- finance receipts
- product images
- delivery proof placeholders
- religious sermons/media
- event files
- import/export outputs
- organization documents

Without a shared document library, file handling will become duplicated and inconsistent.

## Work Package 1: Library and Folder Foundation

Prepare organization-owned library/folder records with:

- name
- parent folder optional
- visibility
- status

## Work Package 2: Document Metadata

Prepare document/media metadata with:

- title
- description optional
- file type
- category
- size placeholder
- storage reference
- owner organization
- status

## Work Package 3: Storage Reference

Store provider-neutral storage references.

Storage Engine owns provider-specific behavior.

## Work Package 4: Domain Attachment Links

Prepare domain link records such as:

```text
finance_receipt
hr_reference
commerce_product_image
delivery_proof_placeholder
religious_media
event_file
custom
```

A document may link to a domain record without owning that domain's business logic.

## Work Package 5: Access and Visibility

Prepare access levels:

```text
private
organization_internal
restricted
public_placeholder
```

Public sharing should remain limited in MVP.

## Work Package 6: Upload Intent Placeholder

Prepare upload intent direction without binding domains to a storage provider.

Actual file storage should go through Storage Engine.

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- document library
- folder list
- document detail
- upload placeholder
- domain attachments
- access warnings

## Out of Scope

Do not implement:

- live virus scanning engine
- full OCR/document AI
- advanced video streaming
- e-signature workflows
- unrestricted public sharing
- provider-specific file handling inside domains

## Completion Standard

Sprint 30 is complete when organizations can manage document/media metadata, link documents to domain records, enforce basic access rules, and keep storage-provider logic behind Storage Engine.

## Final Rule

Document and media handling must be centralized, auditable, and reusable across modules.
