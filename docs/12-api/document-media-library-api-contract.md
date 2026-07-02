# Document Media Library API Contract

## Purpose

This document defines the MVP API contract for Document and Media Library foundation.

The API should support libraries, folders, document metadata, media records, storage references, domain attachments, and access checks while preserving organization boundaries.

## Owner

```text
Document Media Library
Storage Engine
Permission Engine
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/documents
```

## Libraries and Folders

```text
GET /api/v1/organizations/{organizationId}/documents/libraries
POST /api/v1/organizations/{organizationId}/documents/libraries
GET /api/v1/organizations/{organizationId}/documents/folders
POST /api/v1/organizations/{organizationId}/documents/folders
GET /api/v1/organizations/{organizationId}/documents/folders/{folderId}
PATCH /api/v1/organizations/{organizationId}/documents/folders/{folderId}
```

### Create Folder Request

```json
{
  "libraryId": "...",
  "parentFolderId": "optional",
  "name": "Finance Receipts",
  "visibility": "organization_internal",
  "status": "active"
}
```

## Document Records

```text
GET /api/v1/organizations/{organizationId}/documents/items
POST /api/v1/organizations/{organizationId}/documents/items
GET /api/v1/organizations/{organizationId}/documents/items/{documentId}
PATCH /api/v1/organizations/{organizationId}/documents/items/{documentId}
```

### Create Document Metadata Request

```json
{
  "folderId": "optional",
  "title": "Receipt July 2026",
  "description": "Optional description",
  "fileType": "pdf",
  "category": "finance_receipt",
  "accessLevel": "restricted",
  "status": "active"
}
```

## Upload Intent Placeholder

```text
POST /api/v1/organizations/{organizationId}/documents/upload-intents
```

### Request Direction

```json
{
  "title": "Product image",
  "fileType": "image",
  "category": "commerce_product_image",
  "folderId": "optional"
}
```

This route should prepare metadata and delegate actual storage behavior to Storage Engine.

## Storage Reference

```text
PATCH /api/v1/organizations/{organizationId}/documents/items/{documentId}/storage-reference
```

### Request Direction

```json
{
  "storageProvider": "local",
  "storageKey": "provider-neutral-key",
  "sizeBytes": 12345,
  "checksum": "optional"
}
```

## Domain Attachments

```text
GET /api/v1/organizations/{organizationId}/documents/items/{documentId}/attachments
POST /api/v1/organizations/{organizationId}/documents/items/{documentId}/attachments
DELETE /api/v1/organizations/{organizationId}/documents/items/{documentId}/attachments/{attachmentId}
```

### Create Attachment Request

```json
{
  "domainType": "finance_receipt",
  "domainRecordId": "...",
  "label": "Receipt attachment"
}
```

## Access Update

```text
PATCH /api/v1/organizations/{organizationId}/documents/items/{documentId}/access
```

### Request Direction

```json
{
  "accessLevel": "organization_internal"
}
```

## View or Download Permission Check

```text
GET /api/v1/organizations/{organizationId}/documents/items/{documentId}/view-intent
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "document": {
      "id": "...",
      "title": "Receipt July 2026",
      "fileType": "pdf",
      "accessAllowed": true,
      "storageReferenceAvailable": true
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- document/media feature availability
- required permission
- organization ownership
- access level rules

Domain attachment routes must not allow cross-organization links.

## Status Direction

Suggested document statuses:

```text
draft
active
archived
deleted_placeholder
manual_review
```

Suggested access levels:

```text
private
organization_internal
restricted
public_placeholder
```

## Audit Direction

Audit should be written for:

- library/folder created or updated
- document record created or updated
- storage reference attached
- domain attachment created or removed
- access level changed
- view/download action where policy requires it

## Error Direction

Use standard response shape.

Expected error areas:

- document not found
- folder not found
- storage reference missing
- access denied
- domain reference invalid
- upload intent failed
- permission denied
- organization not found

## Final Rule

Document APIs must preserve organization scope, access checks, auditability, and Storage Engine boundaries.
